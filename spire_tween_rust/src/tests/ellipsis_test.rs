use std::f32::consts::PI;

use godot::classes::Sprite2D;
use godot::prelude::*;
use spire_tween::prelude::*;

use super::util::*;
use crate::impl_test_base;

const CENTER: Vector2 = Vector2::new(960.0, 540.0);

#[derive(GodotClass)]
#[class(init, base = Node2D)]
pub struct EllipsisTests {
    base: Base<Node2D>,
    #[init(node = "Ball")]
    ball: OnReady<Gd<Sprite2D>>,
    #[init(val = RcPtr::new(TimeTracker::new()))]
    time_tracker: RcPtr<TimeTracker>,
}

impl_test_base! { INode2D, EllipsisTests }

impl ITestClass for EllipsisTests {
    const PREFAB_PATH: &'static str = "res://examples/tests/ellipsis.tscn";

    fn test_list() -> Vec<fn(&mut Self) -> PinnedTestTask> {
        vec![
            Self::test_circle,
            Self::test_circle_inverted,
            Self::test_ellipsis,
            Self::test_ellipsis_varying,
            Self::test_ellipsis_varying_inverted,
        ]
    }

    fn time_tracker(&self) -> &RcPtr<TimeTracker> { &self.time_tracker }
}

impl EllipsisTests {
    fn test_circle(&mut self) -> PinnedTestTask {
        let handle = self.ball.do_ellipsis(CENTER, 0.0, 2.0 * PI, Vector2::splat(256.0), Vector2::splat(256.0), 4.0).register();
        let gd = handle.gd_handle.as_ref().unwrap().clone();
        let tracker = RcPtr::clone(&self.time_tracker);
        Box::pin(async move { tracker.wait_finished(&gd, 4.0).await; })
    }

    fn test_circle_inverted(&mut self) -> PinnedTestTask {
        let handle = self.ball.do_ellipsis(CENTER, 2.0 * PI, 0.0, Vector2::splat(256.0), Vector2::splat(256.0), 4.0).register();
        let gd = handle.gd_handle.as_ref().unwrap().clone();
        let tracker = RcPtr::clone(&self.time_tracker);
        Box::pin(async move { tracker.wait_finished(&gd, 4.0).await; })
    }

    fn test_ellipsis(&mut self) -> PinnedTestTask {
        let r = Vector2::new(256.0, 192.0);
        let handle = self.ball.do_ellipsis(CENTER, 0.0, 2.0 * PI, r, r, 4.0).register();
        let gd = handle.gd_handle.as_ref().unwrap().clone();
        let tracker = RcPtr::clone(&self.time_tracker);
        Box::pin(async move { tracker.wait_finished(&gd, 4.0).await; })
    }

    fn test_ellipsis_varying(&mut self) -> PinnedTestTask {
        let from_r = Vector2::new(256.0, 192.0);
        let to_r = Vector2::ZERO;
        let handle = self.ball.do_ellipsis(CENTER, 0.0, 8.0 * PI, from_r, to_r, 12.0).register();
        let gd = handle.gd_handle.as_ref().unwrap().clone();
        let tracker = RcPtr::clone(&self.time_tracker);
        Box::pin(async move { tracker.wait_finished(&gd, 12.0).await; })
    }

    fn test_ellipsis_varying_inverted(&mut self) -> PinnedTestTask {
        let from_r = Vector2::ZERO;
        let to_r = Vector2::new(128.0, 256.0);
        let handle = self.ball.do_ellipsis(CENTER, 0.0, 8.0 * PI, from_r, to_r, 12.0).register();
        let gd = handle.gd_handle.as_ref().unwrap().clone();
        let tracker = RcPtr::clone(&self.time_tracker);
        Box::pin(async move { tracker.wait_finished(&gd, 12.0).await; })
    }
}
