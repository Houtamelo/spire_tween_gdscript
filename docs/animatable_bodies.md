# Move platforms and doors with physics-clock tweens

Use the existing Node2D/Node3D position and rotation tweens on an
`AnimatableBody2D` or `AnimatableBody3D`. Keep `sync_to_physics` enabled (its
default) and select Spire's physics process mode. Godot synchronizes the body's
transform with physics and estimates its linear/angular velocity so the moving
collider can affect other bodies.

## GDScript

Attach collision shapes to the animatable body, configure collision layers and
masks for the objects it should interact with, then create the tween from
`_ready()` or later:

```gdscript
extends Node

@onready var platform: AnimatableBody2D = $Platform

func _ready() -> void:
    DoNode2D.position(platform, platform.position + Vector2(200, 0), 2.0)\
        .set_ease(Spire.EASE_LINEAR)\
        .set_process_mode(Spire.PROCESS_MODE_PHYSICS)
```

The 3D equivalent uses `DoNode3D.position` and a `Vector3` target.
`position` uses parent-local coordinates; `global_position` uses world
coordinates. Both work with the corresponding animatable body.

For a door with its collision shape offset from the body's origin, rotate the
body around that origin. Set the physics clock on the **root sequence** when
composing movement:

```gdscript
extends Node

@onready var door: AnimatableBody3D = $Door

func open_door() -> SpireSequence:
    var sequence := Spire.sequence()\
        .set_process_mode(Spire.PROCESS_MODE_PHYSICS)\
        .set_default_children_ease(Spire.EASE_LINEAR)
    sequence.append_interval(0.1)
    sequence.append(DoNode3D.rotation(
        door, door.rotation + Vector3(0, PI / 2, 0), 1.0))
    return sequence
```

Rotation values are radians. Once adopted, children are advanced by the
sequence; setting a child's process mode alone does not select the root's clock.

## Rust

Native shortcuts accept the inherited Node2D/Node3D APIs directly. Register the
builder to start automatic processing:

```rust
use godot::{classes::AnimatableBody2D, prelude::*};
use spire_tween::prelude::*;

fn move_platform(platform: Gd<AnimatableBody2D>) {
    let target = platform.get_position() + Vector2::new(200.0, 0.0);
    platform
        .do_position(target, 2.0)
        .with_ease(EaseKind::Basic(Ease::Linear))
        .with_process_mode(ProcessMode::Physics)
        .register();
}
```

For sequences, set `.with_process_mode(ProcessMode::Physics)` on the root
builder and call `set_default_ease(Ease::Linear)` before appending children.

## Playback and collision behavior

- `pause()` holds the tween's progress and `play()` resumes it. `stop()` stops
  further tween writes; it does not restore the starting pose or complete the
  movement. Physics can still consume the last requested pose, and other bodies
  retain their own simulated motion.
- Freeing the target cancels its property tween without emitting `finished`.
- `finished` reports tween completion. The physics engine still needs to
  consume the final synchronized transform; collision queries made directly
  from the signal are not evidence that the final physics step has run.
- An animatable body follows the scripted trajectory and can push other bodies.
  Contacts do not make it stop at an obstacle. Tween completion does not mean
  the route was unobstructed.
- Contact detection depends on collision shapes, layers/masks, movement per
  physics step and the selected physics backend. This workflow does not promise
  swept collision detection for large translations or rotations.
- Do not combine `sync_to_physics` with `move_and_collide()` on the same body.
  For a simulated `RigidBody`, direct transform animation is a different
  operation from controlling its motion through `_integrate_forces()`.

See Godot's [AnimatableBody2D](https://docs.godotengine.org/en/4.4/classes/class_animatablebody2d.html),
[AnimatableBody3D](https://docs.godotengine.org/en/4.4/classes/class_animatablebody3d.html)
and [rigid-body control](https://docs.godotengine.org/en/4.4/tutorials/physics/rigid_body.html)
documentation for the engine's contracts.

## Integration coverage

The native and GDScript suites each contain five `test_animatable_body_*`
cases. They run the actual physics simulation in isolated 2D/3D worlds:

- Translation pushes a dynamic rigid body and emits a contact signal identifying
  the animatable body. A body at the same starting location with disabled
  collision layers/masks remains stationary.
- Rotation of an offset door collider, driven by a physics-clock sequence,
  produces the same contact/displacement checks in both dimensions.
- Additional checks cover a non-origin parent with a world-position target,
  pause/resume, endpoint synchronization, stop preserving the current pose and
  cancellation when either target is freed.

These tests validate scripted animatable-body motion. Collision-blocked
movement and dynamic rigid-body steering remain separate capabilities.
