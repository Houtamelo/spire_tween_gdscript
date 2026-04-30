//! Integration tests for the 3D-only tween templates: `do_follow_3d`,
//! `do_ellipsis_3d`, `do_bone_position`, `do_bone_scale`.
//!
//! Uses a Node3D-rooted scene with a Skeleton3D child that has two bones
//! pre-configured in the .tscn (with rest poses and enabled state) so the
//! bone tests can verify the actual pose values, not just that the tween
//! emitted `finished`.

use std::f32::consts::PI;

use godot::classes::{Node3D, Skeleton3D};
use godot::prelude::*;
use spire_tween::prelude::*;

use super::util::*;
use crate::impl_test_base;

#[derive(GodotClass)]
#[class(init, base = Node3D)]
pub struct Templates3DTests {
    base: Base<Node3D>,
    #[init(node = "Skeleton3D")]
    skeleton: OnReady<Gd<Skeleton3D>>,
    #[init(node = "Leader")]
    leader: OnReady<Gd<Node3D>>,
    #[init(node = "Follower")]
    follower: OnReady<Gd<Node3D>>,
    #[init(val = RcPtr::new(TimeTracker::new()))]
    time_tracker: RcPtr<TimeTracker>,
}

impl_test_base! { INode3D, Templates3DTests }

impl ITestClass for Templates3DTests {
    const PREFAB_PATH: &'static str = "res://examples/tests/templates_3d.tscn";

    fn test_list() -> Vec<fn(&mut Self) -> PinnedTestTask> {
        vec![
            Self::test_do_bone_position,
            Self::test_do_bone_scale,
            Self::test_do_follow_3d,
            Self::test_do_ellipsis_3d,
        ]
    }

    fn time_tracker(&self) -> &RcPtr<TimeTracker> { &self.time_tracker }
}

/// Compares two `Vector3`s component-wise within `eps`. Returns true if all components match.
fn vec3_close(a: Vector3, b: Vector3, eps: f32) -> bool {
    (a.x - b.x).abs() <= eps && (a.y - b.y).abs() <= eps && (a.z - b.z).abs() <= eps
}

impl Templates3DTests {
    /// Verifies `do_bone_position` actually writes to a bone's pose position.
    /// Uses `test_bone_a` (pre-configured in templates_3d.tscn).
    fn test_do_bone_position(&mut self) -> PinnedTestTask {
        let mut skeleton = self.skeleton.clone();
        let bone_idx = skeleton.find_bone("test_bone_a");
        assert!(bone_idx >= 0, "test_bone_a must exist in templates_3d.tscn (got {bone_idx})");

        // Reset to a known starting state.
        skeleton.set_bone_pose_position(bone_idx, Vector3::ZERO);
        assert_eq!(
            skeleton.get_bone_pose_position(bone_idx),
            Vector3::ZERO,
            "set/get round-trip on a configured bone"
        );

        let target = Vector3::new(1.0, 2.0, 3.0);
        let handle = skeleton.do_bone_position(bone_idx, target, 1.0).register();
        let tracker = RcPtr::clone(&self.time_tracker);
        assert_finished_timing(&handle, &tracker, 1.0);

        Box::pin(async move {
            wait_finished(&handle, &tracker, 1.0).await;
            let final_pos = skeleton.get_bone_pose_position(bone_idx);
            assert!(
                vec3_close(final_pos, target, 0.001),
                "do_bone_position must end at target {target}, got {final_pos}"
            );
        })
    }

    /// Verifies `do_bone_scale` actually writes to a bone's pose scale.
    /// Uses `test_bone_b` (pre-configured in templates_3d.tscn).
    fn test_do_bone_scale(&mut self) -> PinnedTestTask {
        let mut skeleton = self.skeleton.clone();
        let bone_idx = skeleton.find_bone("test_bone_b");
        assert!(bone_idx >= 0, "test_bone_b must exist in templates_3d.tscn (got {bone_idx})");

        // Sanity: starting pose scale is (1, 1, 1) per the .tscn.
        skeleton.set_bone_pose_scale(bone_idx, Vector3::ONE);
        assert_eq!(
            skeleton.get_bone_pose_scale(bone_idx),
            Vector3::ONE,
            "set/get round-trip must work on a configured bone"
        );

        let target = Vector3::new(2.0, 0.5, 3.0);
        let handle = skeleton.do_bone_scale(bone_idx, target, 1.0).register();
        let tracker = RcPtr::clone(&self.time_tracker);
        assert_finished_timing(&handle, &tracker, 1.0);

        Box::pin(async move {
            wait_finished(&handle, &tracker, 1.0).await;
            let final_scale = skeleton.get_bone_pose_scale(bone_idx);
            assert!(
                vec3_close(final_scale, target, 0.001),
                "do_bone_scale must end at target {target}, got {final_scale}"
            );
        })
    }

    /// Verifies `do_follow` (3D variant) catches up to a moving Node3D leader.
    fn test_do_follow_3d(&mut self) -> PinnedTestTask {
        let leader = self.leader.clone();
        let follower = self.follower.clone();

        let mut leader_mut = leader.clone();
        leader_mut.set_global_position(Vector3::ZERO);
        let mut follower_mut = follower.clone();
        follower_mut.set_global_position(Vector3::ZERO);

        let leader_target = Vector3::new(5.0, 0.0, 0.0);

        let leader_tween = leader.clone().do_global_position(leader_target, 2.0).register();
        let follow_tween = follower.do_follow(leader.clone(), 2.0).as_speed_based().register();

        let tracker = RcPtr::clone(&self.time_tracker);

        Box::pin(async move {
            wait_finished(&leader_tween, &tracker, 2.0).await;
            assert_eq!(leader.get_global_position(), leader_target);

            while !follow_tween.is_stopped() {
                next_frame().await;
            }

            let dist = follower.get_global_position().distance_to(leader_target);
            assert!(
                dist < 0.01,
                "follower failed to converge: distance {dist} from leader at {leader_target}"
            );
        })
    }

    /// Verifies `do_ellipsis_3d` traces the expected ellipsoidal path.
    ///
    /// Setup: center=ORIGIN, axis=UP, from_radius=to_radius=(2,2,0), angle 0..2*PI.
    /// With rz=0, the leader traces a flat circle in the plane perpendicular to UP.
    /// The implementation's local axes for axis=UP work out to u=(1,0,0), v=(0,0,-1),
    /// so position = (2*cos(theta), 0, -2*sin(theta)).
    ///
    /// Sample points checked:
    ///  - theta=0:      (2, 0, 0)
    ///  - theta=PI/2:   (0, 0, -2)
    ///  - theta=PI:     (-2, 0, 0)
    ///  - theta=3PI/2:  (0, 0, 2)
    ///  - theta=2PI:    (2, 0, 0)  (back to start)
    fn test_do_ellipsis_3d(&mut self) -> PinnedTestTask {
        let leader = self.leader.clone();
        let mut leader_mut = leader.clone();
        leader_mut.set_global_position(Vector3::ZERO);

        let center = Vector3::ZERO;
        let radius = Vector3::new(2.0, 2.0, 0.0);
        let axis = Vector3::UP;
        let duration = 4.0;

        let handle = leader
            .clone()
            .do_ellipsis(center, 0.0, 2.0 * PI, radius, radius, axis, duration)
            .register();

        let tracker = RcPtr::clone(&self.time_tracker);
        assert_finished_timing(&handle, &tracker, duration);

        Box::pin(async move {
            // Sample mid-tween at the quarter mark (theta ~= PI/2, expected ~(0, 0, -2)).
            tracker.wait_seconds(duration * 0.25).await;
            let pos_quarter = leader.get_global_position();
            assert!(
                vec3_close(pos_quarter, Vector3::new(0.0, 0.0, -2.0), 0.05),
                "at t=duration/4, leader should be near (0, 0, -2), got {pos_quarter}"
            );

            // Sample at half-way (theta = PI, expected (-2, 0, 0)).
            tracker.wait_seconds(duration * 0.25).await;
            let pos_half = leader.get_global_position();
            assert!(
                vec3_close(pos_half, Vector3::new(-2.0, 0.0, 0.0), 0.05),
                "at t=duration/2, leader should be near (-2, 0, 0), got {pos_half}"
            );

            // Wait for finish (theta = 2*PI, expected back at start (2, 0, 0)).
            wait_finished(&handle, &tracker, duration).await;
            let pos_end = leader.get_global_position();
            assert!(
                vec3_close(pos_end, Vector3::new(2.0, 0.0, 0.0), 0.05),
                "at finish, leader should be back at (2, 0, 0), got {pos_end}"
            );
        })
    }
}
