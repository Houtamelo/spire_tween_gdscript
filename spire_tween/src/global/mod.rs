use super::*;

mod cfg;
mod deferred_op;
mod node_status;

use anyhow::Result;
use deferred_op::*;
pub use node_status::*;

#[cfg(feature = "indexmap")]
mod index_map_impl;
#[cfg(feature = "indexmap")]
pub use index_map_impl::*;
#[cfg(feature = "dashmap")]
mod dash_map_impl;
#[cfg(feature = "dashmap")]
pub use dash_map_impl::*;

// Note: We check for strong_count > 1 because we are literally holding one reference here.
fn tween_process(tween: &AnyTween, delta_time: f64, is_tree_paused: bool) -> bool {
    use AnyTween::*;
    use ObjectValidityResult::*;

    let result = match tween {
        Property(t) => {
            delegate_property_tween! { t.to_mut().handle_bound_nodes_validity(is_tree_paused) }
        }
        Method(t) => {
            delegate_method_tween! { t.to_mut().handle_bound_nodes_validity(is_tree_paused) }
        }
        DelayedCall(t) => t.to_mut().handle_bound_nodes_validity(is_tree_paused),
        Sequence(t) => t.to_mut().handle_bound_nodes_validity(is_tree_paused),
    };

    match result {
        CanProcess => {
            match tween.get_state() {
                State::Stopped | State::Paused => tween.strong_count() > 1,

                State::Playing => {
                    let process_result = match tween {
                        Property(t) => {
                            delegate_property_tween! { t.to_mut().process(delta_time, is_tree_paused) }
                        }
                        Method(t) => {
                            delegate_method_tween! { t.to_mut().process(delta_time, is_tree_paused) }
                        }
                        DelayedCall(t) => t.to_mut().process(delta_time, is_tree_paused),
                        Sequence(t) => t.to_mut().process(delta_time, is_tree_paused),
                    };

                    match process_result {
                        AdvanceTimeResult::Playing => true,
                        // If a tween is paused and there aren't any references to it, nothing will ever be able
                        // to unpause it, so we can safely stop referencing it.
                        AdvanceTimeResult::Paused => tween.strong_count() > 1,

                        // Tween is done, check if we can stop referencing it.
                        AdvanceTimeResult::Completed { .. } => tween.strong_count() > 1,
                        AdvanceTimeResult::ShouldDespawn => false,
                    }
                }
            }
        }
        DontProcess => tween.strong_count() > 1,
        SomeObjectsDead => false,
    }
}

unsafe impl Send for TweensMap {}

unsafe impl Sync for TweensMap {}

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

fn try_init_singleton() -> Result<()> {
    let main_loop = Engine::singleton().get_main_loop().ok_or_else(|| {
        anyhow!(
            "[SpireTween] Error: `Engine::get_main_loop()` returned null, \
        SpireTween needs to access the main loop to register its singleton."
        )
    })?;

    let scene_tree = main_loop.try_cast::<SceneTree>().map_err(|_| {
        anyhow!(
            "[SpireTween] Error: `Engine::get_main_loop()` did not return a `SceneTree` object, \
        SpireTween only supports `SceneTree` as the main loop."
        )
    })?;

    let mut root = scene_tree.get_root().ok_or_else(|| {
        anyhow!(
            "Error: `SceneTree::get_root()` returned null, \
        SpireTween needs to access the root to register its singleton."
        )
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

#[derive(GodotClass)]
#[class(init, base = Node, internal)]
pub(crate) struct TweensController {
    base: Base<Node>,
    #[init(val = OnReady::from_base_fn(|base| base.get_tree().expect("Expected SceneTree to be accessible in `_ready()`.")))]
    scene_tree: OnReady<Gd<SceneTree>>,
    last_process_ticks_usec: u64,
    last_physics_process_ticks_usec: u64,
}

#[godot_api]
impl INode for TweensController {
    fn ready(&mut self) {
        self.last_process_ticks_usec = Time::singleton().get_ticks_usec();
        self.last_physics_process_ticks_usec = self.last_process_ticks_usec;

        let mut base = self.base_mut();
        base.set_process_mode(node::ProcessMode::ALWAYS);
        // Makes sure we tick after everything else.
        base.set_process_priority(i32::MAX);
        base.set_physics_process_priority(i32::MAX);
    }

    fn process(&mut self, delta_time: f64) {
        let is_tree_paused = self.scene_tree.is_paused();

        let curr_ticks_usec = Time::singleton().get_ticks_usec();
        let unscaled_delta_time =
            (curr_ticks_usec - self.last_process_ticks_usec) as f64 / 1_000_000.0;
        self.last_process_ticks_usec = curr_ticks_usec;

        #[allow(unused_unsafe)]
        unsafe {
            TM.tick_process(is_tree_paused, delta_time, unscaled_delta_time)
        }
    }

    fn physics_process(&mut self, delta_time: f64) {
        let is_tree_paused = self.scene_tree.is_paused();

        let curr_ticks_usec = Time::singleton().get_ticks_usec();
        let unscaled_delta_time =
            (curr_ticks_usec - self.last_physics_process_ticks_usec) as f64 / 1_000_000.0;
        self.last_physics_process_ticks_usec = curr_ticks_usec;

        #[allow(unused_unsafe)]
        unsafe {
            TM.tick_physics(is_tree_paused, delta_time, unscaled_delta_time)
        }
    }
}
