pub(super) use dashmap::{DashMap, DashSet, Entry, mapref::one::RefMut};

use super::*;

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
    root_tweens:   DashSet<AnyTween, ahash::RandomState>,
    tracked_nodes: DashMap<i64, NodeState>,
    deferred_ops:  DashMap<*const (), DeferredOp>,
}

fn panic_if_locked(set: &DashSet<AnyTween, ahash::RandomState>, addr: *const ()) {
    let shard_idx = set.determine_map(&addr);
    if let Some(shard) = set.shards().get(shard_idx) {
        assert!(!shard.is_locked(), "Shard {shard_idx} is locked! Addr: {addr:?}");
    }
}

impl TweensMap {
    pub fn node_bind(&self, node: Gd<Node>, weak: impl Into<WeakAnyTween>) {
        let mut entry = self.node_ensure_tracked(node);
        let weak = weak.into();
        debug_assert!(!entry.bound_tweens.contains(&weak));
        entry.bound_tweens.insert(weak);
    }

    pub fn node_unbind<T: ITweenable>(&self, node: Gd<Node>, tween: &RcPtr<SpireTween<T>>) {
        let address = tween.address();
        let node_id = node.instance_id_unchecked().to_i64();
        if let Some(mut state) = self.tracked_nodes.get_mut(&node_id) {
            let pre_len = state.bound_tweens.len();
            state
                .bound_tweens
                .retain(|t| !std::ptr::eq(t.address(), address));
            let post_len = state.bound_tweens.len();
            debug_assert_eq!(pre_len - 1, post_len);
        }
    }

    pub fn node_get_status_fresh(&self, node: Gd<Node>) -> NodeStatus {
        let mut entry = self.node_ensure_tracked(node);

        if let NodeStatus::OutsideTreeMaybeDead = entry.status
            && !is_instance_id_valid(node.instance_id_unchecked().to_i64())
        {
            entry.status = NodeStatus::Dead;
        }

        entry.status
    }

    pub fn node_bound_tweens_kill(&self, node: Gd<Node>) {
        let Entry::Occupied(mut entry) = self
            .tracked_nodes
            .entry(node.instance_id_unchecked().to_i64())
        else {
            return
        };

        let bound_tweens = std::mem::take(&mut entry.get_mut().bound_tweens);
        let deferred_ops = &self.deferred_ops;
        for weak in &bound_tweens {
            deferred_ops.insert(weak.address(), DeferredOp::Remove);
        }

        drop(entry); // Release lock before passing context back to Godot.

        for mut tween in bound_tweens.iter().filter_map(WeakAnyTween::upgrade) {
            tween.stop();
        }
    }

    pub fn node_bound_tweens_force_complete(&self, node: Gd<Node>) {
        let Entry::Occupied(mut entry) = self
            .tracked_nodes
            .entry(node.instance_id_unchecked().to_i64())
        else {
            return;
        };

        // Unlike `kill`, we don't want to remove the tweens from the bound list here, so we clone instead of take.
        let bound_tweens = entry
            .get_mut()
            .bound_tweens
            .iter()
            .cloned()
            .collect::<SmallVec<[WeakAnyTween; 4]>>();

        drop(entry); // Release lock before passing context back to Godot.

        for mut tween in bound_tweens.iter().filter_map(WeakAnyTween::upgrade) {
            tween.force_complete();
        }
    }

    /// Unregisters a tween, it will no longer update on ticks.
    #[inline]
    pub fn tween_unregister<T: Address>(&self, tween: &T) {
        #[cfg(debug_assertions)]
        godot_print!("Unregistering tween: {}-{:?}", type_name::<T>(), tween.address());
        //debug_assert!(self.tween_is_registered(tween));
        self.deferred_ops
            .insert(tween.address(), DeferredOp::Remove);
    }

    /// Makes a tween update on every tick.
    #[inline]
    pub fn tween_register(&self, tween: impl Into<AnyTween>) {
        let tween = tween.into();
        #[cfg(debug_assertions)]
        godot_print!("Registering tween: {}-{:?}", tween.inner_type_name(), tween.address());
        //debug_assert!(!self.tween_is_registered(&tween));
        self.check_binds(&tween);
        self.deferred_ops
            .insert(tween.address(), DeferredOp::Add(tween));
    }

    #[inline]
    pub fn tween_notify_sequence_child(&self, tween: &AnyTween) {
        self.check_binds(tween);
        self.tween_unregister(tween);
    }

    #[inline]
    pub fn tween_is_registered(&self, tween: &impl Address) -> bool {
        let address = tween.address();

        let is_root = self.root_tweens.contains(&address);
        // If root, ensure it's not queued for removal.
        if is_root {
            self.deferred_ops
                .get(&address)
                .is_none_or(|op| !matches!(&*op, DeferredOp::Remove))
        }
        // Otherwise, check if it's queued for addition.
        else {
            self.deferred_ops
                .get(&address)
                .is_some_and(|op| matches!(&*op, DeferredOp::Add(_)))
        }
    }
}

impl TweensMap {
    fn node_ensure_tracked(&self, node: Gd<Node>) -> RefMut<i64, NodeState> {
        let node_id = node.instance_id_unchecked().to_i64();

        self.tracked_nodes.entry(node_id).or_insert_with(|| {
            let status = eval_node_status(node);

            if let NodeStatus::InsideTree | NodeStatus::OutsideTreeMaybeDead = status {
                node.signals().tree_exiting().connect(move || {
                    let Entry::Occupied(mut entry) = TM.tracked_nodes.entry(node_id)
                    else { return };

                    if !node.is_queued_for_deletion() {
                        entry.get_mut().status = NodeStatus::OutsideTreeMaybeDead;
                        return;
                    }

                    let entry_mut = entry.get_mut();
                    entry_mut.status = NodeStatus::Dead;
                    let bound_tweens = std::mem::take(&mut entry_mut.bound_tweens);

                    let deferred_ops = &TM.deferred_ops;
                    for weak in &bound_tweens {
                        deferred_ops.insert(weak.address(), DeferredOp::Remove);
                    }

                    drop(entry); // Release lock before passing context back to Godot.

                    for mut tween in bound_tweens.iter().filter_map(WeakAnyTween::upgrade) {
                        tween.stop();
                    }
                });

                node.signals().tree_entered().connect(move || {
                    if let Some(mut value) = TM.tracked_nodes.get_mut(&node_id) {
                        value.status = NodeStatus::InsideTree;
                    }
                });
            };

            NodeState {
                bound_tweens: Default::default(),
                status,
            }
        })
    }

    pub fn check_binds(&self, tween: &AnyTween) {
        match tween {
            AnyTween::Property(tween) => {
                if let ObjectOrNode::Node(owner) = tween.get_owner() {
                    self.node_ensure_tracked(*owner);
                }
            }
            AnyTween::Method(tween) => {
                if let Some(ObjectOrNode::Node(owner)) = tween.get_owner() {
                    self.node_ensure_tracked(*owner);
                }
            }
            AnyTween::DelayedCall(_) => {}
            AnyTween::Sequence(seq) => {
                for inner_tween in seq.iter_inner_tweens_non_recursive() {
                    self.check_binds(&inner_tween);
                }
            }
        }

        let weak = tween.downgrade();
        for obj in tween.get_bound_nodes() {
            self.node_bind(*obj, weak.clone());
        }
    }
}

impl TweensMap {
    fn handle_deferred_ops(&self) {
        for op in self.deferred_ops.iter() {
            let addr = op.key();
            match &*op {
                DeferredOp::Add(tween) => {
                    //debug_assert!(!self.root_tweens.contains(addr));
                    self.root_tweens.insert(tween.clone());
                    //debug_assert!(self.root_tweens.contains(addr));
                }
                DeferredOp::Remove => {
                    //debug_assert!(self.root_tweens.contains(addr));
                    self.root_tweens.remove(addr);
                    //debug_assert!(!self.root_tweens.contains(addr));
                }
            }
        }

        self.deferred_ops.clear();
    }

    pub(super) fn tick_process(&self, is_tree_paused: bool, delta_time: f64) {
        self.handle_deferred_ops();

        self.root_tweens.retain(|tween| {
            let should_retain = match tween.get_process_mode() {
                // Reference count for this is tested in `tick_physics`.
                ProcessMode::Physics => true,
                // User might not be referencing this anymore, release the reference if that's the case.
                ProcessMode::Manual => tween.strong_count() > 1,
                ProcessMode::Idle => tween_process(tween, delta_time, is_tree_paused),
            };

            #[cfg(debug_assertions)]
            if !should_retain {
                godot_print!(
                    "Tween unregistered due to retain = false: {}-{:?}",
                    tween.inner_type_name(),
                    tween.address()
                );
            }

            should_retain
        });

        self.handle_deferred_ops();
    }

    pub(super) fn tick_physics(&self, is_tree_paused: bool, delta_time: f64) {
        self.handle_deferred_ops();

        self.root_tweens.retain(|tween| {
            match tween.get_process_mode() {
                // Reference count for this is tested in `tick_process`.
                ProcessMode::Idle => true,
                // User might not be referencing this anymore, release the reference if that's the case.
                ProcessMode::Manual => tween.strong_count() > 1,
                ProcessMode::Physics => tween_process(tween, delta_time, is_tree_paused),
            }
        });

        self.handle_deferred_ops();
    }
}
