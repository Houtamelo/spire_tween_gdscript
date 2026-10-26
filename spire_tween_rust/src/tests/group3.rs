//! Numerical and lifecycle checks against real Godot objects.
use godot::{classes::*, prelude::*};
use spire_tween::prelude::*;

use super::{misc::MiscTests, util::PinnedTestTask};
type Driver = TweenPtr<LerpMethodData<f64>>;
fn manual(t: SpireTween<LerpMethodData<f64>>) -> Driver {
    TweenPtr::new(
        t.with_process_mode(ProcessMode::Manual)
            .with_ease(EaseKind::Basic(Ease::Linear)),
    )
}
fn step(t: &Driver, delta: f64) { let _ = t.custom_step(delta, false); }
fn near(a: f64, b: f64) {
    assert!((a - b).abs() < 0.0002, "{a} != {b}");
}
fn v2(a: Vector2, b: Vector2) { near(a.distance_to(b) as f64, 0.0); }
fn v3(a: Vector3, b: Vector3) { near(a.distance_to(b) as f64, 0.0); }
fn quat(a: Quaternion, b: Quaternion) {
    assert!(a.dot(b).abs() > 0.99999, "{a:?} != {b:?}");
}
const FULL: ShakeRandomnessMode = ShakeRandomnessMode::Full;

impl MiscTests {
    pub(super) fn test_group3_global_easing(&mut self) -> PinnedTestTask {
        let n = Node2D::new_alloc();
        self.base_mut().add_child(&n);
        let original = Ease::get_global_default();
        Ease::set_global_default(Ease::OutQuad);
        let mut t = TweenPtr::new(
            n.do_shake(1.0, Vector2::new(3.0, 4.0), 4, 0.0, false, FULL, true)
                .with_process_mode(ProcessMode::Manual),
        );
        step(&t, 0.125);
        let first = n.get_position().length();
        Ease::set_global_default(Ease::InQuad);
        step(&t, 0.125);
        let changed = n.get_position().length();
        t.force_complete();
        let mut explicit = manual(n.do_shake(1.0, Vector2::new(3.0, 4.0), 4, 0.0, false, FULL, true));
        step(&explicit, 0.125);
        let overridden = n.get_position().length();
        explicit.force_complete();
        Ease::set_global_default(Ease::OutQuad);
        let mut punch = TweenPtr::new(
            n.do_punch_position(Vector2::new(8.0, 0.0), 1.0, 4, 1.0, true)
                .with_process_mode(ProcessMode::Manual),
        );
        step(&punch, 0.05);
        let punch_eased = n.get_position().x;
        Ease::set_global_default(Ease::Linear);
        step(&punch, 0.05);
        let punch_changed = n.get_position().x;
        punch.force_complete();
        Ease::set_global_default(original);
        near(first as f64, 4.6875);
        near(changed as f64, 1.25);
        near(overridden as f64, 2.5);
        near(punch_eased as f64, 7.8);
        near(punch_changed as f64, 8.0);
        n.free();
        Box::pin(async {})
    }
    pub(super) fn test_group3_punch(&mut self) -> PinnedTestTask {
        let mut n = Node2D::new_alloc();
        self.base_mut().add_child(&n);
        n.set_position(Vector2::new(2.0, 3.0));
        let mut t = manual(n.do_punch_position(Vector2::new(8.0, 0.0), 1.0, 4, 0.5, true));
        step(&t, 0.1);
        v2(n.get_position(), Vector2::new(10.0, 3.0));
        step(&t, 0.2);
        v2(n.get_position(), Vector2::new(-1.0, 3.0));
        step(&t, 0.3);
        v2(n.get_position(), Vector2::new(6.0, 3.0));
        t.force_complete();
        v2(n.get_position(), Vector2::new(2.0, 3.0));
        let mut scale = manual(n.do_punch_scale(Vector2::ONE, 1.0, 4, 0.0, false));
        step(&scale, 0.1);
        v2(n.get_scale(), Vector2::splat(2.0));
        step(&scale, 0.2);
        v2(n.get_scale(), Vector2::ONE);
        scale.force_complete();
        let mut rotation = manual(n.do_punch_rotation(0.4, 1.0, 4, 1.0, true));
        step(&rotation, 0.1);
        near(n.get_rotation() as f64, 0.4);
        rotation.force_complete();
        let ui = Control::new_alloc();
        self.base_mut().add_child(&ui);
        let mut effects = [
            manual(ui.do_punch_position(Vector2::ONE, 1.0, 4, 1.0, true)),
            manual(ui.do_punch_scale(Vector2::ONE, 1.0, 4, 1.0, true)),
            manual(ui.do_punch_rotation(0.4, 1.0, 4, 1.0, true)),
        ];
        for t in &effects {
            step(t, 0.1);
        }
        v2(ui.get_position(), Vector2::ONE);
        v2(ui.get_scale(), Vector2::splat(2.0));
        near(ui.get_rotation() as f64, 0.4);
        for t in &mut effects {
            t.force_complete();
        }
        v2(ui.get_position(), Vector2::ZERO);
        v2(ui.get_scale(), Vector2::ONE);
        near(ui.get_rotation() as f64, 0.0);
        ui.free();
        n.free();
        Box::pin(async {})
    }
    pub(super) fn test_group3_punch_3d(&mut self) -> PinnedTestTask {
        let n = Node3D::new_alloc();
        self.base_mut().add_child(&n);
        let mut pos = manual(n.do_punch_position(Vector3::new(2.0, 3.0, 4.0), 1.0, 4, 1.0, true));
        let mut scale = manual(n.do_punch_scale(Vector3::ONE, 1.0, 4, 1.0, false));
        let mut rot = manual(n.do_punch_rotation(Vector3::new(0.2, 0.3, 0.4), 1.0, 4, 1.0, false));
        for t in [&pos, &scale, &rot] {
            step(t, 0.1);
        }
        v3(n.get_position(), Vector3::new(2.0, 3.0, 4.0));
        v3(n.get_scale(), Vector3::splat(2.0));
        quat(n.get_quaternion(), Quaternion::from_euler(Vector3::new(0.2, 0.3, 0.4)));
        for t in [&mut pos, &mut scale, &mut rot] {
            t.force_complete();
        }
        quat(n.get_quaternion(), Quaternion::IDENTITY);
        v3(n.get_scale(), Vector3::ONE);
        let mut blend = manual(n.do_blendable_punch_rotation(Vector3::new(0.3, 0.0, 0.0), 1.0, 4, 1.0));
        step(&blend, 0.1);
        quat(n.get_quaternion(), Quaternion::from_euler(Vector3::new(0.3, 0.0, 0.0)));
        blend.force_complete();
        quat(n.get_quaternion(), Quaternion::IDENTITY);
        n.free();
        Box::pin(async {})
    }
    pub(super) fn test_group3_effect_lifecycle(&mut self) -> PinnedTestTask {
        let mut n = Node2D::new_alloc();
        self.base_mut().add_child(&n);
        let mut t = manual(n.do_punch_position(Vector2::new(8.0, 0.0), 1.0, 4, 1.0, true));
        n.set_position(Vector2::new(2.0, 0.0));
        step(&t, 0.1);
        let shifted = n.get_position() + Vector2::new(0.0, 5.0);
        n.set_position(shifted);
        t.stop();
        v2(n.get_position(), Vector2::new(10.0, 5.0));
        t.force_complete();
        v2(n.get_position(), Vector2::new(10.0, 5.0)); // Stopped tweens remain untouched.
        t.play();
        step(&t, 0.1);
        v2(n.get_position(), Vector2::new(10.0, 5.0));
        t.force_complete();
        let t = manual(
            n.do_punch_position(Vector2::new(8.0, 0.0), 1.0, 4, 1.0, true)
                .with_loops(2, LoopMode::Yoyo),
        );
        step(&t, 0.1);
        let outward = n.get_position();
        step(&t, 0.9);
        step(&t, 0.9);
        v2(n.get_position(), outward);
        step(&t, 0.1);
        v2(n.get_position(), Vector2::new(2.0, 5.0));
        n.free();
        Box::pin(async {})
    }
    pub(super) fn test_group3_shake(&mut self) -> PinnedTestTask {
        let n = Node2D::new_alloc();
        self.base_mut().add_child(&n);
        let mut scale = manual(n.do_shake_scale(1.0, Vector2::new(3.0, 0.0), 4, 0.0, false, FULL, true));
        step(&scale, 0.25);
        near((n.get_scale() - Vector2::ONE).length() as f64, 3.0);
        scale.force_complete();
        v2(n.get_scale(), Vector2::ONE);
        let mut rot = manual(n.do_shake_rotation(1.0, 0.3, 4, 0.0, false, FULL, true));
        step(&rot, 0.25);
        near((n.get_rotation() as f64).abs(), 0.3);
        rot.force_complete();
        near(n.get_rotation() as f64, 0.0);
        let ui = Control::new_alloc();
        self.base_mut().add_child(&ui);
        let mut scale = manual(ui.do_shake_scale(1.0, Vector2::new(0.0, 2.0), 4, 0.0, false, FULL, false));
        let mut rot = manual(ui.do_shake_rotation(1.0, 0.2, 4, 0.0, false, FULL, false));
        step(&scale, 0.25);
        step(&rot, 0.25);
        near((ui.get_scale() - Vector2::ONE).length() as f64, 2.0);
        near((ui.get_rotation() as f64).abs(), 0.2);
        scale.force_complete();
        rot.force_complete();
        v2(ui.get_scale(), Vector2::ONE);
        ui.free();
        n.free();
        Box::pin(async {})
    }
    pub(super) fn test_group3_shake_3d_camera(&mut self) -> PinnedTestTask {
        let n = Node3D::new_alloc();
        self.base_mut().add_child(&n);
        let mut pos = manual(n.do_shake(1.0, Vector3::new(0.0, 0.0, 3.0), 4, 90.0, false, FULL, true));
        step(&pos, 0.25);
        near(n.get_position().length() as f64, 3.0);
        near(n.get_position().x as f64, 0.0);
        near(n.get_position().y as f64, 0.0);
        pos.force_complete();
        v3(n.get_position(), Vector3::ZERO);
        let mut scale = manual(n.do_shake_scale(1.0, Vector3::new(0.2, 0.0, 0.0), 4, 90.0, false, FULL, false));
        let mut rot = manual(n.do_shake_rotation(1.0, Vector3::new(0.0, 0.3, 0.0), 4, 90.0, false, FULL, true));
        step(&scale, 0.25);
        step(&rot, 0.25);
        near((n.get_scale() - Vector3::ONE).length() as f64, 0.2);
        near(n.get_quaternion().get_angle() as f64, 0.3);
        scale.force_complete();
        rot.force_complete();
        quat(n.get_quaternion(), Quaternion::IDENTITY);
        let mut camera = Camera3D::new_alloc();
        self.base_mut().add_child(&camera);
        camera.set_rotation(Vector3::new(0.0, 0.7, 0.0));
        let q = camera.get_quaternion();
        let mut t = manual(camera.do_shake(1.0, Vector3::new(3.0, 4.0, 9.0), 4, 0.0, false, FULL, true));
        step(&t, 0.25);
        near(camera.get_position().length() as f64, 5.0);
        near((q.inverse() * camera.get_position()).z as f64, 0.0);
        t.force_complete();
        camera.free();
        n.free();
        Box::pin(async {})
    }
    pub(super) fn test_group3_generic(&mut self) -> PinnedTestTask {
        for forced in [false, true] {
            for shake in [false, true] {
                let owner = Node3D::new_alloc();
                self.base_mut().add_child(&owner);
                let calls = RcPtr::new([0_usize; 4]);
                let reads = calls.clone();
                let mut writes = calls.clone();
                let getter = move || {
                    reads.to_mut()[0] += 1;
                    Vector3::ZERO
                };
                let setter = move |_| writes[1] += 1;
                let builder = if shake {
                    do_shake_vec3(getter, setter, 1.0, Vector3::ONE, 4, 90.0, true, FULL, true)
                } else {
                    do_punch_vec3(getter, setter, Vector3::ONE, 1.0, 4, 1.0, true)
                };
                let mut t = manual(builder.bound_to(owner.clone()));
                let mut stopped = calls.clone();
                t.stopped_connect(move || stopped[2] += 1, SpireFlags::empty());
                let mut finished = calls.clone();
                t.finished_connect(move || finished[3] += 1, SpireFlags::empty());
                owner.free();
                if forced {
                    t.force_complete();
                } else {
                    step(&t, 0.1);
                }
                assert!(t.is_stopped());
                assert_eq!(*calls, [0, 0, 1, 0], "Freed bindings must cancel before invoking generic accessors");
            }
        }
        let n = Node3D::new_alloc();
        self.base_mut().add_child(&n);
        let read = n.clone();
        let mut write = n.clone();
        let mut t = manual(
            do_punch_vec3(
                move || read.get_position(),
                move |p| write.set_position(p),
                Vector3::new(2.0, 0.0, 0.0),
                1.0,
                4,
                1.0,
                true,
            )
            .bound_to(n.clone()),
        );
        step(&t, 0.1);
        v3(n.get_position(), Vector3::new(2.0, 0.0, 0.0));
        t.force_complete();
        let read = n.clone();
        let mut write = n.clone();
        let mut t = manual(
            do_shake_vec3(
                move || read.get_position(),
                move |p| write.set_position(p),
                1.0,
                Vector3::new(0.0, 3.0, 0.0),
                4,
                0.0,
                false,
                FULL,
                true,
            )
            .bound_to(n.clone()),
        );
        step(&t, 0.25);
        near(n.get_position().length() as f64, 3.0);
        t.force_complete();
        n.free();
        step(&t, 0.1);
        assert!(t.is_stopped());
        Box::pin(async {})
    }
    pub(super) fn test_group3_quaternion(&mut self) -> PinnedTestTask {
        let mut parent = Node3D::new_alloc();
        let mut n = Node3D::new_alloc();
        self.base_mut().add_child(&parent);
        parent.add_child(&n);
        parent.set_rotation(Vector3::new(0.0, 0.5, 0.0));
        n.set_scale(Vector3::new(2.0, 3.0, 4.0));
        let target = Quaternion::from_euler(Vector3::new(0.0, 1.0, 0.0));
        let mut t = manual(n.do_rotate_quaternion(target, 1.0));
        step(&t, 0.5);
        quat(n.get_quaternion(), Quaternion::from_euler(Vector3::new(0.0, 0.5, 0.0)));
        t.force_complete();
        v3(n.get_scale(), Vector3::new(2.0, 3.0, 4.0));
        let mut t = manual(n.do_global_rotate_quaternion(Quaternion::IDENTITY, 1.0));
        t.force_complete();
        quat(n.get_global_basis().orthonormalized().get_quaternion(), Quaternion::IDENTITY);
        v3(n.get_scale(), Vector3::new(2.0, 3.0, 4.0));
        let target = Quaternion::from_euler(Vector3::new(0.3, 0.4, 0.2));
        let mut t = manual(n.do_global_rotate_quaternion(target, 1.0));
        t.force_complete();
        quat(n.get_global_basis().orthonormalized().get_quaternion(), target);
        v3(n.get_scale(), Vector3::new(2.0, 3.0, 4.0));
        let mut t = manual(n.do_rotate_quaternion(Quaternion::new(0.0, 0.0, 0.0, 0.0), 1.0));
        t.force_complete();
        quat(n.get_quaternion(), Quaternion::IDENTITY);
        parent.free();
        Box::pin(async {})
    }
    pub(super) fn test_group3_rotation_modes(&mut self) -> PinnedTestTask {
        let mut n = Node3D::new_alloc();
        self.base_mut().add_child(&n);
        for (mode, midpoint) in [(RotateMode::Fast, std::f32::consts::PI), (RotateMode::FastBeyond360, 0.0)] {
            n.set_rotation(Vector3::new(0.0, 170_f32.to_radians(), 0.0));
            let t = manual(n.do_rotate(Vector3::new(0.0, -170_f32.to_radians(), 0.0), 1.0, mode));
            step(&t, 0.5);
            quat(n.get_quaternion(), Quaternion::from_euler(Vector3::new(0.0, midpoint, 0.0)));
        }
        let start = Quaternion::from_euler(Vector3::new(0.6, 0.7, 0.0));
        let by = Vector3::new(0.0, 0.0, 0.8);
        for (mode, expected) in [
            (RotateMode::LocalAxisAdd, start * Quaternion::from_euler(by)),
            (RotateMode::WorldAxisAdd, Quaternion::from_euler(by) * start),
        ] {
            n.set_quaternion(start);
            let mut t = manual(n.do_rotate(by, 1.0, mode));
            t.force_complete();
            quat(n.get_quaternion(), expected);
        }
        n.set_rotation(Vector3::ZERO);
        let mut t = manual(n.do_global_rotate(Vector3::new(0.0, 1.0, 0.0), 1.0, RotateMode::Fast));
        t.force_complete();
        quat(n.get_quaternion(), Quaternion::from_euler(Vector3::new(0.0, 1.0, 0.0)));
        n.free();
        Box::pin(async {})
    }
    pub(super) fn test_group3_blendable_rotation(&mut self) -> PinnedTestTask {
        let mut n = Node3D::new_alloc();
        self.base_mut().add_child(&n);
        let start = Quaternion::from_euler(Vector3::new(0.3, 0.4, 0.0));
        let by = Vector3::new(0.0, 0.0, 0.8);
        n.set_quaternion(start);
        let mut local = manual(n.do_blendable_rotation(by, 1.0));
        step(&local, 0.5);
        quat(n.get_quaternion(), start * Quaternion::from_euler(by * 0.5));
        local.force_complete();
        quat(n.get_quaternion(), start * Quaternion::from_euler(by));
        n.set_quaternion(start);
        let mut global = manual(n.do_blendable_global_rotation(by, 1.0));
        step(&global, 0.5);
        quat(n.get_quaternion(), Quaternion::from_euler(by * 0.5) * start);
        global.force_complete();
        quat(n.get_quaternion(), Quaternion::from_euler(by) * start);
        n.free();
        Box::pin(async {})
    }
    pub(super) fn test_group3_look_at(&mut self) -> PinnedTestTask {
        let mut n = Node3D::new_alloc();
        self.base_mut().add_child(&n);
        let point = Vector3::new(4.0, 0.0, -4.0);
        let mut t = manual(n.do_look_at(point, 1.0, Vector3::UP, RotationAxis::All, false));
        step(&t, 0.5);
        n.set_position(Vector3::new(3.0, 0.0, 0.0));
        t.force_complete();
        v3(-(n.get_basis().col_c()), point.normalized());
        n.set_quaternion(Quaternion::IDENTITY);
        let mut t = manual(n.do_dynamic_look_at(point, 1.0, Vector3::UP, RotationAxis::Y, false));
        step(&t, 0.5);
        n.set_position(Vector3::new(2.0, 0.0, 0.0));
        t.force_complete();
        v3(-n.get_basis().col_c(), (point - n.get_position()).normalized());
        n.set_quaternion(Quaternion::from_euler(Vector3::new(0.2, 0.3, 0.4)));
        let mut locked = manual(n.do_look_at(Vector3::new(4.0, 3.0, -4.0), 1.0, Vector3::UP, RotationAxis::XY, false));
        step(&locked, 0.5);
        near(n.get_quaternion().get_euler().z as f64, 0.4);
        locked.force_complete();
        near(n.get_quaternion().get_euler().z as f64, 0.4);
        let mut t = manual(n.do_look_at(n.get_position(), 1.0, Vector3::ZERO, RotationAxis::All, false));
        let before = n.get_quaternion();
        t.force_complete();
        quat(n.get_quaternion(), before);
        let mut target = Node3D::new_alloc();
        self.base_mut().add_child(&target);
        target.set_position(point);
        let mut t = manual(n.do_look_at_target(target.clone(), 1.0, Vector3::UP, RotationAxis::All, true));
        step(&t, 0.5);
        target.set_position(Vector3::new(-3.0, 2.0, 5.0));
        t.force_complete();
        v3(n.get_basis().col_c(), (target.get_position() - n.get_position()).normalized());
        let t = manual(n.do_look_at_target(target.clone(), 1.0, Vector3::UP, RotationAxis::All, false));
        target.free();
        step(&t, 0.1);
        assert!(t.is_stopped());
        n.free();
        Box::pin(async {})
    }
    pub(super) fn test_group3_pivot(&mut self) -> PinnedTestTask {
        let mut n = Control::new_alloc();
        self.base_mut().add_child(&n);
        n.set_size(Vector2::new(100.0, 80.0));
        let mut t = manual(n.do_pivot(Vector2::ONE, 1.0));
        step(&t, 0.5);
        v2(n.get_pivot_offset(), Vector2::new(50.0, 40.0));
        n.set_size(Vector2::new(200.0, 160.0));
        t.force_complete();
        v2(n.get_pivot_offset(), Vector2::new(200.0, 160.0));
        let mut t = manual(n.do_pivot_x(0.25, 1.0));
        t.force_complete();
        v2(n.get_pivot_offset(), Vector2::new(50.0, 160.0));
        let mut t = manual(n.do_pivot_y(0.5, 1.0));
        t.force_complete();
        v2(n.get_pivot_offset(), Vector2::new(50.0, 80.0));
        let mut t = manual(n.do_pivot(Vector2::new(-1.0, f32::NAN), 1.0));
        t.force_complete();
        v2(n.get_pivot_offset(), Vector2::ZERO);
        n.free();
        Box::pin(async {})
    }
    pub(super) fn test_group3_scroll(&mut self) -> PinnedTestTask {
        let n = ScrollContainer::new_alloc();
        self.base_mut().add_child(&n);
        let mut h = n.get_h_scroll_bar().unwrap();
        let mut v = n.get_v_scroll_bar().unwrap();
        h.set_min(10.0);
        h.set_max(210.0);
        h.set_page(50.0);
        h.set_value(10.0);
        v.set_min(20.0);
        v.set_max(420.0);
        v.set_page(100.0);
        v.set_value(20.0);
        let mut t = manual(n.do_normalized_position(Vector2::new(1.0, 0.0), 1.0));
        step(&t, 0.5);
        near(h.get_value(), 85.0);
        near(v.get_value(), 170.0);
        h.set_max(410.0);
        t.force_complete();
        near(h.get_value(), 360.0);
        near(v.get_value(), 320.0);
        let mut t = manual(n.do_horizontal_normalized_position(0.0, 1.0));
        t.force_complete();
        near(h.get_value(), 10.0);
        near(v.get_value(), 320.0);
        let mut t = manual(n.do_vertical_normalized_position(1.0, 1.0));
        t.force_complete();
        near(v.get_value(), 20.0);
        let mut pixels = TweenPtr::new(n.do_scroll_horizontal(50, 1.0).with_process_mode(ProcessMode::Manual));
        pixels.force_complete();
        near(h.get_value(), 50.0);
        let mut pixels = TweenPtr::new(n.do_scroll_vertical(70, 1.0).with_process_mode(ProcessMode::Manual));
        pixels.force_complete();
        near(v.get_value(), 70.0);
        h.set_page(1000.0);
        let mut t = manual(n.do_horizontal_normalized_position(4.0, 1.0));
        t.force_complete();
        near(h.get_value(), 10.0);
        n.free();
        Box::pin(async {})
    }
    pub(super) fn test_group3_spiral(&mut self) -> PinnedTestTask {
        let n = Node3D::new_alloc();
        self.base_mut().add_child(&n);
        let mut t = manual(n.do_spiral(1.0, Vector3::BACK, SpiralMode::Expand, 1.0, std::f64::consts::TAU, 2.0, false));
        step(&t, 0.25);
        v3(n.get_position(), Vector3::new(0.0, 0.25, 0.5));
        t.force_complete();
        v3(n.get_position(), Vector3::new(1.0, 0.0, 2.0));
        let mut t = manual(n.do_spiral(1.0, Vector3::UP, SpiralMode::ExpandThenContract, 1.0, 10.0, 0.0, false));
        let start = n.get_position();
        step(&t, 0.5);
        near(n.get_position().distance_to(start) as f64, 1.0);
        t.force_complete();
        v3(n.get_position(), start);
        let mut t = manual(n.do_spiral(f64::NAN, Vector3::ZERO, SpiralMode::Expand, f64::NAN, -1.0, f64::NAN, true));
        step(&t, 0.25);
        v3(n.get_position(), n.get_position().round());
        t.force_complete();
        assert!(n.get_position().is_finite());
        n.free();
        Box::pin(async {})
    }
    pub(super) fn test_group3_cancellation_and_invalid(&mut self) -> PinnedTestTask {
        for forced in [false, true] {
            let n = Node3D::new_alloc();
            self.base_mut().add_child(&n);
            let mut tweens = [
                manual(n.do_punch_position(Vector3::new(f32::NAN, 1.0, 0.0), -1.0, 0, 3.0, true)),
                manual(n.do_shake(1.0, Vector3::new(-1.0, 2.0, 0.0), 0, f32::NAN, true, FULL, false)),
                manual(n.do_rotate_quaternion(Quaternion::IDENTITY, 1.0)),
                manual(n.do_spiral(1.0, Vector3::BACK, SpiralMode::Expand, 1.0, 10.0, 0.0, false)),
            ];
            let events = RcPtr::new([0, 0]);
            for t in &mut tweens {
                let mut s = events.clone();
                t.stopped_connect(move || s[0] += 1, SpireFlags::empty());
                let mut f = events.clone();
                t.finished_connect(move || f[1] += 1, SpireFlags::empty());
            }
            for t in &tweens {
                step(t, 0.1);
            }
            assert!(n.get_position().is_finite());
            n.free();
            for t in &mut tweens {
                if forced {
                    t.force_complete();
                } else {
                    step(t, 0.1);
                }
                assert!(t.is_stopped());
            }
            assert_eq!(*events, [4, 0]);
        }
        Box::pin(async {})
    }
}
