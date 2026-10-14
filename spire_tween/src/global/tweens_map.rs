use godot::classes::notify::ObjectNotification;
use indexmap::map::Entry;

use super::*;

type IndexMap<K, V> = indexmap::IndexMap<K, V, fxhash::FxBuildHasher>;
type IndexSet<T> = indexmap::IndexSet<T, fxhash::FxBuildHasher>;

pub static TM: LazyLock<TweensMap> = LazyLock::new(|| {
    if let Err(err) = try_init_singleton() {
        godot_error!("{err}");
    }

    TweensMap {
        root_tweens:   Default::default(),
        tracked_nodes: Default::default(),
        deferred_ops:  Default::default(),
    }
});

pub struct TweensMap {
    /// REMOVING/ADDING to `root_tweens` IS FORBIDDEN OUTSIDE `handle_deferred_ops()` and TICKING METHODS.
    root_tweens:   UnsafeCell<IndexSet<AnyTween>>,
    tracked_nodes: UnsafeCell<IndexMap<i64, NodeState>>,
    deferred_ops:  UnsafeCell<IndexMap<*const (), DeferredOp>>,
}

unsafe impl Send for TweensMap {}

unsafe impl Sync for TweensMap {}

/// Read-only observations used by the real-Godot integration tests.
/// Available only with `integration-tests`, outside the supported addon API.
#[cfg(feature = "integration-tests")]
#[doc(hidden)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BookkeepingSnapshot {
    pub tracked_nodes: usize,
    pub dead_nodes: usize,
    pub binding_entries: usize,
    pub expired_bindings: usize,
    pub root_tweens: usize,
    pub deferred_ops: usize,
}

#[cfg(feature = "integration-tests")]
impl TweensMap {
    /// Observe settled manager state on the main thread, outside processing.
    /// Does not upgrade weak pointers or change registrations/bindings.
    #[doc(hidden)]
    pub fn bookkeeping_snapshot(&self) -> BookkeepingSnapshot {
        let nodes = unsafe { &*self.tracked_nodes.get() };
        let mut snapshot = BookkeepingSnapshot {
            tracked_nodes: nodes.len(),
            dead_nodes: 0,
            binding_entries: 0,
            expired_bindings: 0,
            root_tweens: unsafe { (*self.root_tweens.get()).len() },
            deferred_ops: unsafe { (*self.deferred_ops.get()).len() },
        };
        for state in nodes.values() {
            snapshot.dead_nodes += usize::from(matches!(state.status, NodeStatus::Dead { .. }));
            snapshot.binding_entries += state.bound_tweens.len();
            snapshot.expired_bindings += state
                .bound_tweens
                .iter()
                .filter(|weak| match weak {
                    WeakAnyTween::Property(weak) => {
                        delegate_weak_property_tween! { weak => |pointer| pointer.is_expired() }
                    }
                    WeakAnyTween::Method(weak) => {
                        delegate_weak_method_tween! { weak => |pointer| pointer.is_expired() }
                    }
                    WeakAnyTween::DelayedCall(weak) => weak.is_expired(),
                    WeakAnyTween::Sequence(weak) => weak.is_expired(),
                })
                .count();
        }
        snapshot
    }

    /// Observe a cached record without creating it or consulting the node.
    #[doc(hidden)]
    pub fn bookkeeping_node_status(&self, id: i64) -> Option<NodeStatus> {
        unsafe { (*self.tracked_nodes.get()).get(&id).map(|state| state.status) }
    }
}

impl TweensMap {
    pub fn node_bind(&self, node: Gd<Node>, weak: impl Into<WeakAnyTween>) {
        let entry = unsafe { self.node_ensure_tracked(node) };
        let weak = weak.into();
        // Registration and sequence insertion revisit existing bindings.
        entry.bound_tweens.insert(weak);
    }

    pub fn node_unbind<T: ITweenable>(&self, node: Gd<Node>, tween: &TweenPtr<T>) {
        let node_id = node.instance_id_unchecked().to_i64();
        self.node_unbind_address(node_id, tween.address());
    }

    pub(crate) fn node_unbind_address(&self, node_id: i64, address: *const ()) {
        if let Some(state) = unsafe { (*self.tracked_nodes.get()).get_mut(&node_id) } {
            state.bound_tweens.retain(|t| !std::ptr::eq(t.address(), address));
        }
    }

    pub fn node_get_status_fresh(&self, node: Gd<Node>) -> NodeStatus {
        //let node_id = node.instance_id_unchecked().to_i64();
        let entry = unsafe { self.node_ensure_tracked(node) };

        // if let NodeStatus::OutsideTree = entry.status
        //     && !is_instance_id_valid(node_id)
        // {
        //     entry.status = NodeStatus::Dead {
        //         seen_by_process: false,
        //         seen_by_physics: false,
        //     };
        // }

        entry.status
    }

    pub fn node_bound_tweens_kill(&self, node: Gd<Node>) {
        let Entry::Occupied(mut entry) =
            (unsafe { (*self.tracked_nodes.get()).entry(node.instance_id_unchecked().to_i64()) })
        else {
            return
        };

        let bound_tweens = std::mem::take(&mut entry.get_mut().bound_tweens);
        unsafe {
            (*self.deferred_ops.get()).extend(bound_tweens.iter().map(|weak| (weak.address(), DeferredOp::Remove)))
        }

        #[allow(clippy::drop_non_drop)]
        drop(entry); // Release lock before passing context back to Godot.

        for mut tween in bound_tweens.iter().filter_map(WeakAnyTween::upgrade) {
            tween.stop();
        }
    }

    pub fn node_bound_tweens_force_complete(&self, node: Gd<Node>) {
        let Entry::Occupied(mut entry) =
            (unsafe { (*self.tracked_nodes.get()).entry(node.instance_id_unchecked().to_i64()) })
        else {
            return
        };

        // Unlike `kill`, we don't want to remove the tweens from the bound list here, so we clone instead of take.
        let bound_tweens = entry
            .get_mut()
            .bound_tweens
            .iter()
            .cloned()
            .collect::<SmallVec<[WeakAnyTween; 4]>>();

        #[allow(clippy::drop_non_drop)]
        drop(entry); // Release lock before passing context back to Godot.

        for mut tween in bound_tweens.iter().filter_map(WeakAnyTween::upgrade) {
            tween.force_complete();
        }
    }

    /// Unregisters a tween, it will no longer update on ticks.
    #[inline]
    pub fn tween_unregister<T: Address>(&self, tween: &T) {
        //#[cfg(debug_assertions)]
        //godot_print!("Unregistering tween: {}-{:?}", type_name::<T>(), tween.address());
        self.tween_unregister_address(tween.address());
    }

    pub(crate) fn tween_unregister_address(&self, address: *const ()) {
        unsafe {
            (*self.deferred_ops.get()).insert(address, DeferredOp::Remove);
        }
    }

    /// Makes a tween update on every tick.
    #[inline]
    pub fn tween_register(&self, tween: impl Into<AnyTween>) {
        let tween = tween.into();
        //#[cfg(debug_assertions)]
        //godot_print!("Registering tween: {}-{:?}", tween.inner_type_name(), tween.address());
        if !self.check_binds(&tween) {
            return;
        }
        unsafe {
            (*self.deferred_ops.get()).insert(tween.address(), DeferredOp::Add(tween));
        }
    }

    #[inline]
    pub fn tween_notify_sequence_child(&self, tween: &AnyTween) -> bool {
        if !self.check_binds(tween) {
            return false;
        }
        self.tween_unregister(tween);
        true
    }

    #[inline]
    pub fn tween_is_registered(&self, tween: &impl Address) -> bool {
        let address = tween.address();

        let is_root = unsafe { (*self.root_tweens.get()).contains(&address) };
        // If root, ensure it's not queued for removal.
        if is_root {
            unsafe {
                (*self.deferred_ops.get())
                    .get(&address)
                    .is_none_or(|op| !matches!(op, DeferredOp::Remove))
            }
        }
        // Otherwise, check if it's queued for addition.
        else {
            unsafe {
                (*self.deferred_ops.get())
                    .get(&address)
                    .is_some_and(|op| matches!(op, DeferredOp::Add(_)))
            }
        }
    }
}

impl TweensMap {
    unsafe fn notify_node_about_to_be_freed(&self, id: i64) {
        let mut entry = {
            let tracked_nodes_mut = &mut *self.tracked_nodes.get();

            match tracked_nodes_mut.entry(id) {
                Entry::Occupied(mut entry) => {
                    let state = entry.get_mut();
                    match state.status {
                        NodeStatus::InsideTree | NodeStatus::OutsideTree => {
                            state.status = NodeStatus::Dead {
                                seen_by_process: false,
                                seen_by_physics: false,
                            };
                        }
                        NodeStatus::Dead { .. } => {}
                    }

                    entry
                }
                Entry::Vacant(entry) => entry.insert_entry(NodeState {
                    bound_tweens: Default::default(),
                    status: NodeStatus::Dead {
                        seen_by_process: false,
                        seen_by_physics: false,
                    },
                }),
            }
        };

        let bound_tweens = std::mem::take(&mut entry.get_mut().bound_tweens);

        (*self.deferred_ops.get()).extend(bound_tweens.iter().map(|weak| (weak.address(), DeferredOp::Remove)));

        #[allow(clippy::drop_non_drop)]
        drop(entry); // Release lock before passing context back to Godot.

        for mut tween in bound_tweens.iter().filter_map(WeakAnyTween::upgrade) {
            tween.stop();
        }
    }

    unsafe fn notify_node_entering_tree(&self, id: i64) {
        let Entry::Occupied(mut entry) = (*self.tracked_nodes.get()).entry(id) else { return };

        match &mut entry.get_mut().status {
            status @ NodeStatus::OutsideTree => {
                *status = NodeStatus::InsideTree;
            }
            NodeStatus::InsideTree => {}
            NodeStatus::Dead { .. } => {}
        }
    }

    unsafe fn notify_node_exiting_tree(&self, id: i64) {
        let Entry::Occupied(mut entry) = (*self.tracked_nodes.get()).entry(id) else { return };

        match &mut entry.get_mut().status {
            status @ NodeStatus::InsideTree => {
                *status = NodeStatus::OutsideTree;
            }
            NodeStatus::OutsideTree => {}
            NodeStatus::Dead { .. } => {}
        }
    }

    unsafe fn node_ensure_tracked(&self, node: Gd<Node>) -> &mut NodeState {
        let node_id = node.instance_id_unchecked().to_i64();

        (*self.tracked_nodes.get()).entry(node_id).or_insert_with(|| {
            let status = eval_node_status(&node);

            match status {
                NodeStatus::InsideTree | NodeStatus::OutsideTree => {
                    node.signals()
                        .tree_entered()
                        .connect(move || (*TM).notify_node_entering_tree(node_id));

                    let tracker = Gd::from_object(NodeLifetimeTracker { node_id });
                    node.signals().tree_exiting().connect(move || {
                        let _keep_tracker_alive = &tracker;
                        (*TM).notify_node_exiting_tree(node_id);
                    });
                }
                NodeStatus::Dead { .. } => {}
            };

            NodeState {
                bound_tweens: Default::default(),
                status,
            }
        })
    }

    pub fn check_binds(&self, tween: &AnyTween) -> bool {
        if let AnyTween::Sequence(sequence) = tween {
            // Validate before applying any binding changes, without collecting a snapshot.
            for inner in sequence.iter_inner_tweens_recursive() {
                if let Err(error) = inner {
                    error.warn("Sequence binding traversal");
                    return false;
                }
            }

            for inner in sequence.iter_inner_tweens_recursive() {
                match inner {
                    Ok(inner) => self.check_one_bind(&inner),
                    Err(error) => {
                        error.warn("Sequence binding traversal");
                        return false;
                    }
                }
            }
        }
        self.check_one_bind(tween);
        true
    }

    fn check_one_bind(&self, tween: &AnyTween) {
        match tween {
            AnyTween::Property(tween) => {
                if let Some(ObjectOrNode::Node(owner)) = tween.get_owner()
                    && is_instance_id_valid(owner.instance_id_unchecked().to_i64())
                {
                    unsafe { self.node_ensure_tracked(owner.clone()) };
                }
            }
            AnyTween::Method(tween) => {
                if let Some(ObjectOrNode::Node(owner)) = tween.get_owner()
                    && is_instance_id_valid(owner.instance_id_unchecked().to_i64())
                {
                    unsafe { self.node_ensure_tracked(owner.clone()) };
                }
            }
            AnyTween::DelayedCall(_) | AnyTween::Sequence(_) => {}
        }

        let weak = tween.downgrade();
        for obj in tween.get_bound_nodes() {
            if is_instance_id_valid(obj.instance_id_unchecked().to_i64()) {
                self.node_bind(obj.clone(), weak.clone());
            }
        }
    }
}

impl TweensMap {
    fn handle_deferred_ops(&self) {
        let root_tweens = unsafe { &mut *self.root_tweens.get() };

        for (addr, op) in unsafe { (*self.deferred_ops.get()).drain(..) } {
            match op {
                DeferredOp::Add(tween) => root_tweens.insert(tween),
                DeferredOp::Remove => root_tweens.swap_remove(&addr),
            };
        }
    }

    pub(super) unsafe fn tick_process(&self, is_tree_paused: bool, scaled_delta_time: f64, unscaled_delta_time: f64) {
        {
            let tracked_nodes = &mut *self.tracked_nodes.get();
            for state in tracked_nodes.values_mut() {
                match &mut state.status {
                    NodeStatus::InsideTree | NodeStatus::OutsideTree => {}
                    NodeStatus::Dead {
                        seen_by_process,
                        seen_by_physics: _,
                    } => {
                        *seen_by_process = true;
                    }
                }
            }
        }
        self.handle_deferred_ops();

        let mut idx = 0;
        loop {
            let should_retain = {
                let Some(tween) = (*self.root_tweens.get()).get_index(idx) else { break };

                match tween.get_process_mode() {
                    // Reference count for this is tested in `tick_physics`.
                    ProcessMode::Physics => true,
                    // User might not be referencing this anymore, release the reference if that's the case.
                    ProcessMode::Manual => tween.has_external_owners(),
                    ProcessMode::Idle => {
                        let delta_time =
                            if tween.get_ignore_time_scale() { unscaled_delta_time } else { scaled_delta_time };

                        handle_tween_step(tween, delta_time, is_tree_paused)
                    }
                }
            };

            #[cfg(feature = "verbose-stdout")]
            if !should_retain {
                let tween = (*self.root_tweens.get()).get_index(idx).unwrap_unchecked();
                godot_print!(
                    "Tween unregistered due to retain = false: {}-{:?}",
                    tween.inner_type_name(),
                    tween.address()
                );
            }

            if should_retain {
                idx += 1;
            } else {
                // Don't bump idx, as we just removed it and placed a new item inside.
                (*self.root_tweens.get()).swap_remove_index(idx);
            }
        }

        self.handle_deferred_ops();
        self.prune_dead_nodes_seen_by_both();
        self.prune_weak_tweens();
    }

    pub(super) unsafe fn tick_physics(&self, is_tree_paused: bool, scaled_delta_time: f64, unscaled_delta_time: f64) {
        {
            let tracked_nodes = &mut *self.tracked_nodes.get();
            for state in tracked_nodes.values_mut() {
                match &mut state.status {
                    NodeStatus::InsideTree | NodeStatus::OutsideTree => {}
                    NodeStatus::Dead {
                        seen_by_process: _,
                        seen_by_physics,
                    } => {
                        *seen_by_physics = true;
                    }
                }
            }
        }

        self.handle_deferred_ops();

        let mut idx = 0;
        loop {
            let should_retain = {
                let Some(tween) = (*self.root_tweens.get()).get_index(idx) else { break };

                match tween.get_process_mode() {
                    // Reference count for this is tested in `tick_process`.
                    ProcessMode::Idle => true,
                    // User might not be referencing this anymore, release the reference if that's the case.
                    ProcessMode::Manual => tween.has_external_owners(),
                    ProcessMode::Physics => {
                        let delta_time =
                            if tween.get_ignore_time_scale() { unscaled_delta_time } else { scaled_delta_time };

                        handle_tween_step(tween, delta_time, is_tree_paused)
                    }
                }
            };

            #[cfg(feature = "verbose-stdout")]
            if !should_retain {
                let tween = (*self.root_tweens.get()).get_index(idx).unwrap_unchecked();
                godot_print!(
                    "Tween unregistered due to retain = false: {}-{:?}",
                    tween.inner_type_name(),
                    tween.address()
                );
            }

            if should_retain {
                idx += 1;
            } else {
                // Don't bump idx, as we just removed it and placed a new item inside.
                (*self.root_tweens.get()).swap_remove_index(idx);
            }
        }

        self.handle_deferred_ops();
        self.prune_dead_nodes_seen_by_both();
        self.prune_weak_tweens();
    }

    unsafe fn prune_dead_nodes_seen_by_both(&self) {
        let tracked_nodes = &mut *self.tracked_nodes.get();
        tracked_nodes.retain(|_, state| match state.status {
            NodeStatus::InsideTree | NodeStatus::OutsideTree => true,
            NodeStatus::Dead {
                seen_by_process: true,
                seen_by_physics: true,
            } => false,
            NodeStatus::Dead { .. } => true,
        });
    }

    unsafe fn prune_weak_tweens(&self) {
        let tracked_nodes = &mut *self.tracked_nodes.get();

        for state in tracked_nodes.values_mut() {
            state.bound_tweens.retain(|weak| !weak.is_expired());
        }
    }
}

fn handle_tween_step(tween: &AnyTween, delta_time: f64, is_tree_paused: bool) -> bool {
    match tween.custom_step(delta_time, is_tree_paused) {
        TweenStepResult::Stepped(result) => {
            match result {
                AdvanceTimeResult::Playing => true,
                // If a tween is paused and there aren't any references to it, nothing will ever be able
                // to unpause it, so we can safely stop referencing it.
                AdvanceTimeResult::Paused => tween.has_external_owners(),

                // Tween is done, check if we can stop referencing it.
                AdvanceTimeResult::Completed { .. } => tween.is_playing() || tween.has_external_owners(),
                AdvanceTimeResult::ShouldDespawn => false,
            }
        }
        TweenStepResult::AlreadyPaused | TweenStepResult::AlreadyStopped | TweenStepResult::DidNotStepDueToBounds => {
            tween.has_external_owners()
        }
        TweenStepResult::NoLongerValid => false,
    }
}

#[derive(GodotClass)]
#[class(base = RefCounted, no_init, internal)]
struct NodeLifetimeTracker {
    node_id: i64,
}

#[godot_api]
impl IRefCounted for NodeLifetimeTracker {
    fn on_notification(&mut self, what: ObjectNotification) {
        if what == ObjectNotification::PREDELETE {
            unsafe { TM.notify_node_about_to_be_freed(self.node_id) };
        }
    }
}
