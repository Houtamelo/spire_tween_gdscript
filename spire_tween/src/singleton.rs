use anyhow::Result;
use godot::classes::node::ProcessMode;

use super::*;
use crate::benchmarking::benchmark;

#[derive(GodotClass)]
#[class(init, base = Node, internal)]
pub(crate) struct TweensController {
    base: Base<Node>,
    #[init(val = OnReady::from_base_fn(|base| base.get_tree().expect("Expected SceneTree to be accessible in `_ready()`.")))]
    scene_tree: OnReady<Gd<SceneTree>>,
}

#[godot_api]
impl INode for TweensController {
    fn ready(&mut self) {
        let mut base = self.base_mut();
        base.set_process_mode(ProcessMode::ALWAYS);
        base.set_process_priority(256);
        base.set_physics_process_priority(256);
    }

    fn process(&mut self, delta_time: f64) {
        //self.tick_process(delta_time);
        benchmark! { Self::tick_process[self, delta_time] }
    }

    fn physics_process(&mut self, delta_time: f64) { self.tick_physics(delta_time); }

    fn exit_tree(&mut self) { benchmarking::print_results() }
}

impl TweensController {
    fn tick_process(&mut self, delta_time: f64) {
        let is_tree_paused = self.scene_tree.is_paused();

        TWEENS.retain(|_, tween| {
            match tween.get_process_mode() {
                SpireProcessMode::Physics => true,
                SpireProcessMode::Idle => tween_advance_time(tween, delta_time, is_tree_paused),
            }
        });
    }

    fn tick_physics(&mut self, delta_time: f64) {
        let is_tree_paused = self.scene_tree.is_paused();

        TWEENS.retain(|_, tween| {
            match tween.get_process_mode() {
                SpireProcessMode::Physics => tween_advance_time(tween, delta_time, is_tree_paused),
                SpireProcessMode::Idle => true,
            }
        });
    }
}

/// Returns `false` if the tween should despawn.
#[inline]
fn tween_advance_time(tween: &mut AnyTween, delta_time: f64, is_tree_paused: bool) -> bool {
    let bound_objects_status = eval_bound_objects_status(tween.get_bound_nodes());
    match (tween.get_pause_mode(), bound_objects_status) {
        (SpirePauseMode::Bound, BoundInstancesState::AllAliveOrInsideTree) => {
            for obj in tween.get_bound_nodes() {
                if obj
                    .try_cast::<Node>()
                    .is_ok_and(|node| !node.is_processing())
                {
                    return true;
                }
            }
        }
        (SpirePauseMode::Bound, BoundInstancesState::SomeOutsideTree) => return true,
        (SpirePauseMode::Process, BoundInstancesState::AllAliveOrInsideTree) => {}
        (SpirePauseMode::Process, BoundInstancesState::SomeOutsideTree) => {}
        (_, BoundInstancesState::SomeDead) => {
            tween.stop();
            return false;
        }
        (SpirePauseMode::Stop, _) => {
            if is_tree_paused {
                return true;
            }
        }
    }

    tween.advance_time(delta_time);

    match tween.get_state() {
        | TweenState::Playing | TweenState::Paused => true,
        TweenState::Stopped => false,
    }
}

enum BoundInstancesState {
    AllAliveOrInsideTree,
    SomeOutsideTree,
    SomeDead,
}

#[inline]
fn eval_bound_objects_status<'a>(
    bound_nodes: impl Iterator<Item = &'a Gd<Node>> + 'a,
) -> BoundInstancesState {
    let mut any_outside = false;

    for node in bound_nodes {
        match TWEENS.node_get_status_fresh(*node) {
            NodeStatus::InsideTree => {}
            NodeStatus::OutsideTreeMaybeDead => {
                any_outside = true;
            }
            NodeStatus::Dead => return BoundInstancesState::SomeDead,
        }
    }

    if any_outside {
        BoundInstancesState::SomeOutsideTree
    } else {
        BoundInstancesState::AllAliveOrInsideTree
    }
}

#[allow(unused)]
pub fn tween_ref<T, R>(id: TweenId, f: impl FnOnce(&SpireTween<T>) -> R) -> Result<R, FetchError>
where SpireTween<T>: FromEnumRef<AnyTween> {
    TWEENS
        .inspect(id, |any_tween| {
            let found_type = any_tween.inner_type_name();

            let tween = SpireTween::<T>::from_enum_ref(any_tween).ok_or_else(|| {
                FetchError::TypeMismatch {
                    expected: type_name::<T>(),
                    found: found_type,
                }
            })?;

            Ok(f(tween))
        })
        .ok_or(FetchError::NotFound)?
}

#[allow(unused)]
pub fn tween_mut<T, R>(
    id: TweenId,
    f: impl FnOnce(&mut SpireTween<T>) -> R,
) -> Result<R, FetchError>
where
    SpireTween<T>: FromEnumMut<AnyTween>,
{
    TWEENS
        .edit(id, |any_tween| {
            let found_type = any_tween.inner_type_name();

            let tween = SpireTween::<T>::from_enum_mut(any_tween).ok_or_else(|| {
                FetchError::TypeMismatch {
                    expected: type_name::<T>(),
                    found: found_type,
                }
            })?;

            Ok(f(tween))
        })
        .ok_or(FetchError::NotFound)?
}

pub fn tween_any_ref<R>(id: TweenId, f: impl FnOnce(&AnyTween) -> R) -> Result<R, FetchError> {
    TWEENS.inspect(id, f).ok_or(FetchError::NotFound)
}

pub fn tween_any_mut<R>(id: TweenId, f: impl FnOnce(&mut AnyTween) -> R) -> Result<R, FetchError> {
    TWEENS.edit(id, f).ok_or(FetchError::NotFound)
}

pub fn tween_property_ref<R>(
    id: TweenId,
    f: impl FnOnce(&PropertyTween) -> R,
) -> Result<R, FetchError> {
    TWEENS
        .inspect(id, |any_tween| {
            let found_type = any_tween.inner_type_name();
            match any_tween {
                AnyTween::Property(tween) => Ok(f(tween)),
                _ => {
                    Err(FetchError::TypeMismatch {
                        expected: "MethodTween",
                        found: found_type,
                    })
                }
            }
        })
        .ok_or(FetchError::NotFound)?
}

pub fn tween_property_mut<R>(
    id: TweenId,
    f: impl FnOnce(&mut PropertyTween) -> R,
) -> Result<R, FetchError> {
    TWEENS
        .edit(id, |any_tween| {
            let found_type = any_tween.inner_type_name();
            match any_tween {
                AnyTween::Property(tween) => Ok(f(tween)),
                _ => {
                    Err(FetchError::TypeMismatch {
                        expected: "MethodTween",
                        found: found_type,
                    })
                }
            }
        })
        .ok_or(FetchError::NotFound)?
}

pub fn tween_method_ref<R>(
    id: TweenId,
    f: impl FnOnce(&MethodTween) -> R,
) -> Result<R, FetchError> {
    TWEENS
        .inspect(id, |any_tween| {
            let found_type = any_tween.inner_type_name();
            match any_tween {
                AnyTween::Method(tween) => Ok(f(tween)),
                _ => {
                    Err(FetchError::TypeMismatch {
                        expected: "MethodTween",
                        found: found_type,
                    })
                }
            }
        })
        .ok_or(FetchError::NotFound)?
}

pub fn tween_method_mut<R>(
    id: TweenId,
    f: impl FnOnce(&mut MethodTween) -> R,
) -> Result<R, FetchError> {
    TWEENS
        .edit(id, |any_tween| {
            let found_type = any_tween.inner_type_name();
            match any_tween {
                AnyTween::Method(tween) => Ok(f(tween)),
                _ => {
                    Err(FetchError::TypeMismatch {
                        expected: "MethodTween",
                        found: found_type,
                    })
                }
            }
        })
        .ok_or(FetchError::NotFound)?
}

#[allow(unused)]
pub fn tween_sequence_ref<R>(
    id: TweenId,
    f: impl FnOnce(&SpireSequence) -> R,
) -> Result<R, FetchError> {
    TWEENS
        .inspect(id, |any_tween| {
            let found_type = any_tween.inner_type_name();
            match any_tween {
                AnyTween::Sequence(tween) => Ok(f(tween)),
                _ => {
                    Err(FetchError::TypeMismatch {
                        expected: "SequenceTween",
                        found: found_type,
                    })
                }
            }
        })
        .ok_or(FetchError::NotFound)?
}

pub fn tween_sequence_mut<R>(
    id: TweenId,
    f: impl FnOnce(&mut SpireSequence) -> R,
) -> Result<R, FetchError> {
    TWEENS
        .edit(id, |any_tween| {
            let found_type = any_tween.inner_type_name();
            match any_tween {
                AnyTween::Sequence(tween) => Ok(f(tween)),
                _ => {
                    Err(FetchError::TypeMismatch {
                        expected: "SequenceTween",
                        found: found_type,
                    })
                }
            }
        })
        .ok_or(FetchError::NotFound)?
}

pub fn tween_callable_ref<R>(
    id: TweenId,
    f: impl FnOnce(&SpireTween<CallableData>) -> R,
) -> Result<R, FetchError> {
    TWEENS
        .inspect(id, |any_tween| {
            let found_type = any_tween.inner_type_name();
            match any_tween {
                AnyTween::Callable(tween) => Ok(f(tween)),
                _ => {
                    Err(FetchError::TypeMismatch {
                        expected: "CallableTween",
                        found: found_type,
                    })
                }
            }
        })
        .ok_or(FetchError::NotFound)?
}

pub fn tween_callable_mut<R>(
    id: TweenId,
    f: impl FnOnce(&mut SpireTween<CallableData>) -> R,
) -> Result<R, FetchError> {
    TWEENS
        .edit(id, |any_tween| {
            let found_type = any_tween.inner_type_name();
            match any_tween {
                AnyTween::Callable(tween) => Ok(f(tween)),
                _ => {
                    Err(FetchError::TypeMismatch {
                        expected: "CallableTween",
                        found: found_type,
                    })
                }
            }
        })
        .ok_or(FetchError::NotFound)?
}
