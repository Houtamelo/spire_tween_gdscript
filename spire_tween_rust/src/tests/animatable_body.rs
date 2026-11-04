//! Real physics contacts, not manual tween steps or transform-only assertions.
use godot::{classes::*, prelude::*};
use spire_tween::prelude::*;

use super::{misc::MiscTests, util::PinnedTestTask};

async fn physics_frames(count: usize) {
    let tree = Engine::singleton().get_main_loop().unwrap().cast::<SceneTree>();
    for _ in 0..count {
        tree.signals().physics_frame().to_untyped().to_future::<()>().await;
    }
}

async fn finish<T: ITweenable>(tween: &TweenPtr<T>) {
    for _ in 0..150 {
        if tween.is_stopped() {
            break;
        }
        physics_frames(1).await;
    }
    assert!(tween.is_stopped(), "physics-clock tween did not finish");
    // finished is emitted by the setter's tick. AnimatableBody synchronization
    // and collision response must be allowed to consume that final transform.
    physics_frames(3).await;
}

fn world(parent: &mut Gd<Node>) -> Gd<SubViewport> {
    let mut viewport = SubViewport::new_alloc();
    viewport.set_world_2d(&World2D::new_gd());
    viewport.set_use_own_world_3d(true);
    parent.add_child(&viewport);
    viewport
}

fn shape_2d(parent: &mut Gd<Node2D>, size: Vector2, center: Vector2) {
    let mut shape = RectangleShape2D::new_gd();
    shape.set_size(size);
    let mut collider = CollisionShape2D::new_alloc();
    collider.set_shape(&shape);
    collider.set_position(center);
    parent.add_child(&collider);
}

fn shape_3d(parent: &mut Gd<Node3D>, size: Vector3, center: Vector3) {
    let mut shape = BoxShape3D::new_gd();
    shape.set_size(size);
    let mut collider = CollisionShape3D::new_alloc();
    collider.set_shape(&shape);
    collider.set_position(center);
    parent.add_child(&collider);
}

fn probe_2d(world: &mut Gd<SubViewport>, position: Vector2, collides: bool) -> Gd<RigidBody2D> {
    let mut probe = RigidBody2D::new_alloc();
    probe.set_position(position);
    probe.set_gravity_scale(0.0);
    probe.set_lock_rotation_enabled(true);
    probe.set_collision_layer(if collides { 2 } else { 0 });
    probe.set_collision_mask(if collides { 1 } else { 0 });
    probe.set_contact_monitor(true);
    probe.set_max_contacts_reported(16);
    shape_2d(&mut probe.clone().upcast(), Vector2::splat(16.0), Vector2::ZERO);
    world.add_child(&probe);
    probe
}

fn probe_3d(world: &mut Gd<SubViewport>, position: Vector3, collides: bool) -> Gd<RigidBody3D> {
    let mut probe = RigidBody3D::new_alloc();
    probe.set_position(position);
    probe.set_gravity_scale(0.0);
    probe.set_lock_rotation_enabled(true);
    probe.set_collision_layer(if collides { 2 } else { 0 });
    probe.set_collision_mask(if collides { 1 } else { 0 });
    probe.set_contact_monitor(true);
    probe.set_max_contacts_reported(16);
    shape_3d(&mut probe.clone().upcast(), Vector3::splat(0.5), Vector3::ZERO);
    world.add_child(&probe);
    probe
}

impl MiscTests {
    pub(super) fn test_animatable_body_2d_translation(&mut self) -> PinnedTestTask {
        let mut viewport = world(&mut self.base().clone().upcast());
        Box::pin(async move {
            let mut platform = AnimatableBody2D::new_alloc();
            platform.set_collision_layer(1);
            platform.set_collision_mask(2);
            shape_2d(&mut platform.clone().upcast(), Vector2::new(20.0, 80.0), Vector2::ZERO);
            viewport.add_child(&platform);
            assert!(platform.is_sync_to_physics_enabled());
            let start = Vector2::new(45.0, 0.0);
            let probe = probe_2d(&mut viewport, start, true);
            let control = probe_2d(&mut viewport, start, false);
            let contacts = RcPtr::new(0_usize);
            let count = contacts.clone();
            let id = platform.instance_id();
            probe.signals().body_entered().connect(move |body| {
                if body.instance_id() == id {
                    *count.to_mut() += 1;
                }
            });
            physics_frames(3).await;
            let tween = platform
                .do_position(Vector2::new(100.0, 0.0), 1.0)
                .with_ease(EaseKind::Basic(Ease::Linear))
                .with_process_mode(ProcessMode::Physics)
                .register();
            physics_frames(12).await;
            tween.to_mut().pause();
            physics_frames(2).await;
            let paused = platform.get_position();
            physics_frames(6).await;
            assert_eq!(platform.get_position(), paused);
            assert!(paused.x > 0.0 && paused.x < 100.0);
            tween.to_mut().play();
            finish(&tween).await;
            assert!(platform.get_position().distance_to(Vector2::new(100.0, 0.0)) < 0.01);
            assert!(*contacts > 0, "translation must generate real physics contacts");
            assert!(probe.get_position().x > 110.0, "collider must push the dynamic body");
            assert!(control.get_position().distance_to(start) < 0.01, "masked control must stay still");
            godot_print!("ANIMATABLE_2D translation contacts={} probe={}", *contacts, probe.get_position());
            viewport.free();
        })
    }

    pub(super) fn test_animatable_body_3d_translation(&mut self) -> PinnedTestTask {
        let mut viewport = world(&mut self.base().clone().upcast());
        Box::pin(async move {
            let mut parent = Node3D::new_alloc();
            parent.set_position(Vector3::new(10.0, 2.0, 0.0));
            viewport.add_child(&parent);
            let mut platform = AnimatableBody3D::new_alloc();
            platform.set_collision_layer(1);
            platform.set_collision_mask(2);
            shape_3d(&mut platform.clone().upcast(), Vector3::new(1.0, 4.0, 4.0), Vector3::ZERO);
            parent.add_child(&platform);
            assert!(platform.is_sync_to_physics_enabled());
            let origin = platform.get_global_position();
            let start = origin + Vector3::new(3.0, 0.0, 0.0);
            let probe = probe_3d(&mut viewport, start, true);
            let control = probe_3d(&mut viewport, start, false);
            let contacts = RcPtr::new(0_usize);
            let count = contacts.clone();
            let id = platform.instance_id();
            probe.signals().body_entered().connect(move |body| {
                if body.instance_id() == id {
                    *count.to_mut() += 1;
                }
            });
            physics_frames(3).await;
            let target = origin + Vector3::new(6.0, 0.0, 0.0);
            let tween = platform
                .do_global_position(target, 1.0)
                .with_ease(EaseKind::Basic(Ease::Linear))
                .with_process_mode(ProcessMode::Physics)
                .register();
            finish(&tween).await;
            assert!(platform.get_global_position().distance_to(target) < 0.01);
            assert!(*contacts > 0, "translation must generate real physics contacts");
            assert!(probe.get_position().x > target.x + 0.6, "collider must push the dynamic body");
            assert!(control.get_position().distance_to(start) < 0.01, "masked control must stay still");
            godot_print!("ANIMATABLE_3D translation contacts={} probe={}", *contacts, probe.get_position());
            viewport.free();
        })
    }

    pub(super) fn test_animatable_body_2d_rotation_sequence(&mut self) -> PinnedTestTask {
        let mut viewport = world(&mut self.base().clone().upcast());
        Box::pin(async move {
            let mut door = AnimatableBody2D::new_alloc();
            door.set_collision_layer(1);
            door.set_collision_mask(2);
            shape_2d(&mut door.clone().upcast(), Vector2::new(100.0, 10.0), Vector2::new(50.0, 0.0));
            viewport.add_child(&door);
            let start = Vector2::new(45.0, 45.0);
            let probe = probe_2d(&mut viewport, start, true);
            let control = probe_2d(&mut viewport, start, false);
            let contacts = RcPtr::new(0_usize);
            let count = contacts.clone();
            let id = door.instance_id();
            probe.signals().body_entered().connect(move |body| {
                if body.instance_id() == id {
                    *count.to_mut() += 1;
                }
            });
            physics_frames(3).await;
            let mut sequence = SpireTween::<Sequence>::new().with_process_mode(ProcessMode::Physics);
            sequence.set_default_ease(Ease::Linear);
            sequence.append_interval(0.1);
            sequence.append(door.do_rotation(std::f64::consts::FRAC_PI_2, 1.0));
            sequence.append_interval(0.1);
            let tween = sequence.register();
            finish(&tween).await;
            assert!((f64::from(door.get_rotation()) - std::f64::consts::FRAC_PI_2).abs() < 0.001);
            assert!(*contacts > 0, "rotating shape must generate real physics contacts");
            assert!(probe.get_position().distance_to(start) > 10.0, "door must push the dynamic body");
            assert!(control.get_position().distance_to(start) < 0.01, "masked control must stay still");
            godot_print!("ANIMATABLE_2D rotation contacts={} probe={}", *contacts, probe.get_position());
            viewport.free();
        })
    }

    pub(super) fn test_animatable_body_3d_rotation_sequence(&mut self) -> PinnedTestTask {
        let mut viewport = world(&mut self.base().clone().upcast());
        Box::pin(async move {
            let mut door = AnimatableBody3D::new_alloc();
            door.set_collision_layer(1);
            door.set_collision_mask(2);
            shape_3d(&mut door.clone().upcast(), Vector3::new(4.0, 0.4, 0.4), Vector3::new(2.0, 0.0, 0.0));
            viewport.add_child(&door);
            let start = Vector3::new(1.4, 1.4, 0.0);
            let probe = probe_3d(&mut viewport, start, true);
            let control = probe_3d(&mut viewport, start, false);
            let contacts = RcPtr::new(0_usize);
            let count = contacts.clone();
            let id = door.instance_id();
            probe.signals().body_entered().connect(move |body| {
                if body.instance_id() == id {
                    *count.to_mut() += 1;
                }
            });
            physics_frames(3).await;
            let target = Vector3::new(0.0, 0.0, std::f32::consts::FRAC_PI_2);
            let mut sequence = SpireTween::<Sequence>::new().with_process_mode(ProcessMode::Physics);
            sequence.set_default_ease(Ease::Linear);
            sequence.append_interval(0.1);
            sequence.append(door.do_rotation(target, 1.0));
            sequence.append_interval(0.1);
            let tween = sequence.register();
            finish(&tween).await;
            assert!(door.get_rotation().distance_to(target) < 0.001);
            assert!(*contacts > 0, "rotating shape must generate real physics contacts");
            assert!(probe.get_position().distance_to(start) > 0.5, "door must push the dynamic body");
            assert!(control.get_position().distance_to(start) < 0.01, "masked control must stay still");
            godot_print!("ANIMATABLE_3D rotation contacts={} probe={}", *contacts, probe.get_position());
            viewport.free();
        })
    }

    pub(super) fn test_animatable_body_stop_and_free(&mut self) -> PinnedTestTask {
        let mut viewport = world(&mut self.base().clone().upcast());
        Box::pin(async move {
            let body2 = AnimatableBody2D::new_alloc();
            let body3 = AnimatableBody3D::new_alloc();
            viewport.add_child(&body2);
            viewport.add_child(&body3);
            let t2 = body2
                .do_position(Vector2::new(100.0, 0.0), 1.0)
                .with_process_mode(ProcessMode::Physics)
                .register();
            let t3 = body3
                .do_position(Vector3::new(6.0, 0.0, 0.0), 1.0)
                .with_process_mode(ProcessMode::Physics)
                .register();
            let finishes = RcPtr::new(0);
            let count = finishes.clone();
            t2.to_mut()
                .finished_connect(move || *count.to_mut() += 1, SpireFlags::empty());
            let count = finishes.clone();
            t3.to_mut()
                .finished_connect(move || *count.to_mut() += 1, SpireFlags::empty());
            physics_frames(12).await;
            t2.to_mut().stop();
            t3.to_mut().stop();
            physics_frames(2).await;
            let stopped2 = body2.get_position();
            let stopped3 = body3.get_position();
            physics_frames(6).await;
            assert_eq!(body2.get_position(), stopped2);
            assert_eq!(body3.get_position(), stopped3);
            assert!(stopped2.x > 0.0 && stopped2.x < 100.0);
            assert!(stopped3.x > 0.0 && stopped3.x < 6.0);
            assert_eq!(*finishes, 0);
            t2.to_mut().play();
            t3.to_mut().play();
            body2.free();
            body3.free();
            physics_frames(3).await;
            assert!(t2.is_stopped() && t3.is_stopped(), "freed targets must cancel their tweens");
            assert_eq!(*finishes, 0);
            viewport.free();
        })
    }
}
