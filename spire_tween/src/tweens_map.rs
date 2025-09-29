use std::sync::LazyLock;

use anyhow::Result;
use dashmap::mapref::one::RefMut;

use super::*;

pub struct TweensMap {
    tweens: dashmap::DashMap<TweenId, AnyTween, ahash::RandomState>,
    // Shared tweens are used exclusively inside Sequences, they aren't "ticked" directly.
    shared_tweens: dashmap::DashMap<TweenId, PtrTween>,
    tracked_nodes: dashmap::DashMap<i64, NodeState>,
}

#[derive(Default)]
struct NodeState {
    bound_tweens: SmolSet<[TweenId; 1]>,
    status: NodeStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum NodeStatus {
    InsideTree,
    OutsideTreeMaybeDead,
    #[default]
    Dead,
}

#[inline]
fn eval_node_status(node: Gd<Node>) -> NodeStatus {
    let node_id = node.instance_id_unchecked().to_i64();

    if !is_instance_id_valid(node_id) {
        NodeStatus::Dead
    } else {
        match node.is_inside_tree() {
            true => NodeStatus::InsideTree,
            false => NodeStatus::OutsideTreeMaybeDead,
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
                    let Some(mut value) = TWEENS.tracked_nodes.get_mut(&node_id)
                    else { return };

                    if node.is_queued_for_deletion() {
                        value.status = NodeStatus::Dead;

                        for tween_id in &value.bound_tweens {
                            if let Some(mut tween) = TWEENS.remove(*tween_id) {
                                tween.stop();
                            }
                        }
                    } else {
                        value.status = NodeStatus::OutsideTreeMaybeDead;
                    }
                });

                node.signals().tree_entered().connect(move || {
                    if let Some(mut value) = TWEENS.tracked_nodes.get_mut(&node_id) {
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

    pub fn node_bind(&self, node: Gd<Node>, tween_id: TweenId) {
        let mut entry = self.node_ensure_tracked(node);
        entry.bound_tweens.insert(tween_id);
    }

    /// Returns true if the object was successfully bound to the tween (fails if tween doesn't exist).
    pub fn bind_bothways(&self, node: Gd<Node>, tween_id: TweenId) -> bool {
        self.node_bind(node, tween_id);

        self.edit(tween_id, |tween| {
            tween.bound_nodes_mut().insert(node);
        })
        .is_some()
    }

    pub fn node_unbind(&self, node: Gd<Node>, tween_id: TweenId) -> bool {
        let node_id = node.instance_id_unchecked().to_i64();
        if let Some(mut state) = self.tracked_nodes.get_mut(&node_id) {
            state.bound_tweens.remove(&tween_id);
        }

        self.edit(tween_id, |tween| tween.bound_nodes_mut().remove(&node))
            .is_some()
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

    pub fn object_remove_and_edit_bound_tweens(
        &self,
        object_id: i64,
        mut f: impl FnMut(&mut AnyTween),
    ) {
        let Some(status) = self.tracked_nodes.get(&object_id)
        else { return };

        for &tween_id in &status.bound_tweens {
            if let Some(mut tween) = self.remove(tween_id) {
                f(&mut tween);
            }
        }
    }

    #[allow(unused)]
    #[inline]
    pub fn iter(&self, mut f: impl FnMut(TweenId, &AnyTween)) {
        for entry in self.tweens.iter() {
            f(*entry.key(), &entry);
        }

        for entry in self.shared_tweens.iter() {
            f(*entry.key(), &entry);
        }
    }

    #[allow(unused)]
    #[inline]
    pub fn iter_mut(&self, mut f: impl FnMut(TweenId, &mut AnyTween)) {
        for mut entry in self.tweens.iter_mut() {
            f(*entry.key(), &mut entry);
        }

        for mut entry in self.shared_tweens.iter_mut() {
            f(*entry.key(), &mut entry);
        }
    }

    #[inline]
    pub fn retain(&self, mut f: impl FnMut(TweenId, &mut AnyTween) -> bool) {
        self.tweens.retain(|id, tween| f(*id, tween));
        self.shared_tweens.retain(|id, tween| f(*id, tween));
    }

    #[inline]
    pub fn remove(&self, id: TweenId) -> Option<MaybeShared> {
        self.tweens
            .remove(&id)
            .map(|(_, tween)| MaybeShared::Owned(tween))
            .or_else(|| {
                self.shared_tweens
                    .remove(&id)
                    .map(|(_, ptr)| MaybeShared::Shared(ptr))
            })
    }

    #[inline]
    pub fn get_or_make_shared(&self, id: TweenId) -> Option<PtrTween> {
        if let Some(ptr) = self.shared_tweens.get(&id) {
            Some(ptr.clone())
        } else if let Some((_, tween)) = self.tweens.remove(&id) {
            let ptr = PtrTween::new(tween, id);
            self.shared_tweens.insert(id, ptr.clone());
            Some(ptr)
        } else {
            None
        }
    }

    #[inline]
    pub fn register(&self, tween: AnyTween) -> TweenId {
        let id = TweenId::new();
        self.register_with_id(id, tween);
        id
    }

    #[inline]
    pub fn register_with_id(&self, id: TweenId, mut tween: AnyTween) {
        self.check_binds(id, &mut tween);
        self.tweens.insert(id, tween);
    }

    #[inline]
    pub fn register_shared(&self, tween: AnyTween) -> PtrTween {
        let id = TweenId::new();
        let ptr = PtrTween::new(tween, id);
        self.register_already_shared(ptr.clone());
        ptr
    }

    #[inline]
    pub fn register_already_shared(&self, mut tween: PtrTween) {
        self.check_binds(tween.id, &mut tween);
        self.shared_tweens.insert(tween.id, tween.clone());
    }

    fn check_binds(&self, id: TweenId, tween: &mut AnyTween) {
        match tween {
            AnyTween::Property(tween) => {
                if let ObjectOrNode::Node(owner) = tween.get_owner() {
                    self.node_ensure_tracked(*owner);
                }
            }
            AnyTween::Method(tween) => {
                if let ObjectOrNode::Node(owner) = tween.get_owner() {
                    self.node_ensure_tracked(*owner);
                }
            }
            AnyTween::Callable(tween) => {
                if let Some(ObjectOrNode::Node(owner)) = &tween.t.owner {
                    self.node_ensure_tracked(*owner);
                }
            }
            AnyTween::DelayedCall(_) => {}
            AnyTween::Sequence(seq) => {
                for mut tween in seq.iter_inner_tweens_non_recursive() {
                    self.check_binds(id, &mut tween);
                }
            }
        }

        for obj in tween.get_bound_nodes() {
            self.node_bind(*obj, id);
        }
    }

    #[allow(clippy::manual_map)]
    #[inline]
    pub fn inspect<R>(&self, id: TweenId, f: impl FnOnce(&AnyTween) -> R) -> Option<R> {
        if let Some(entry) = self.tweens.get(&id) {
            Some(f(&entry))
        } else if let Some(entry) = self.shared_tweens.get(&id) {
            Some(f(&entry))
        } else {
            None
        }
    }

    #[allow(clippy::manual_map)]
    #[inline]
    pub fn edit<R>(&self, id: TweenId, f: impl FnOnce(&mut AnyTween) -> R) -> Option<R> {
        if let Some(mut entry) = self.tweens.get_mut(&id) {
            Some(f(&mut entry))
        } else if let Some(mut entry) = self.shared_tweens.get_mut(&id) {
            Some(f(&mut entry))
        } else {
            None
        }
    }

    #[inline]
    pub fn is_valid(&self, id: TweenId) -> bool {
        self.tweens.contains_key(&id) || self.shared_tweens.contains_key(&id)
    }
}

unsafe impl Send for TweensMap {}

unsafe impl Sync for TweensMap {}

pub static TWEENS: LazyLock<TweensMap> = LazyLock::new(|| {
    if let Err(err) = try_init_singleton() {
        godot_error!("{err}");
    }

    TweensMap {
        tweens: Default::default(),
        shared_tweens: Default::default(),
        tracked_nodes: Default::default(),
    }
});

fn try_init_singleton() -> Result<()> {
    let main_loop = Engine::singleton().get_main_loop().ok_or_else(|| {
        anyhow!("[SpireTween] Error: `Engine::get_main_loop()` returned null, SpireTween needs to access the main loop to register its singleton.")
    })?;

    let scene_tree = main_loop.try_cast::<SceneTree>().map_err(|_| {
        anyhow!("[SpireTween] Error: `Engine::get_main_loop()` did not return a `SceneTree` object, SpireTween only supports `SceneTree` as the main loop.")
    })?;

    let mut root = scene_tree.get_root().ok_or_else(|| {
        anyhow!("Error: `SceneTree::get_root()` returned null, SpireTween needs to access the root to register its singleton.")
    })?;

    // Check if the controller has been manually registered by the user.
    if root
        .try_get_node_as::<TweensController>("tweens_controller")
        .is_none()
    {
        // Otherwise, create our own.
        let mut controller = TweensController::new_alloc();
        controller.set_name("tweens_controller");
        // Call deferred in case this is called outside the main thread.
        root.call_deferred("add_child", &[controller.to_variant()]);
    }

    Ok(())
}
