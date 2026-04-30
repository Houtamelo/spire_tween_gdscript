use std::f32::consts::PI;

use godot::classes::Sprite2D;
use godot::prelude::*;
use spire_tween::prelude::*;

use super::util::*;
use crate::impl_test_base;

const CENTER: Vector2 = Vector2::new(960.0, 540.0);

#[derive(GodotClass)]
#[class(init, base = Node2D)]
pub struct SpiralTests {
    base: Base<Node2D>,
    #[init(node = "Ball")]
    ball: OnReady<Gd<Sprite2D>>,
    #[init(val = RcPtr::new(TimeTracker::new()))]
    time_tracker: RcPtr<TimeTracker>,
}

impl_test_base! { INode2D, SpiralTests }

impl ITestClass for SpiralTests {
    const PREFAB_PATH: &'static str = "res://examples/tests/spiral.tscn";

    fn test_list() -> Vec<fn(&mut Self) -> PinnedTestTask> {
        vec![
            Self::test_spiral_logarithmic,
            Self::test_spiral_logarithmic_sheared,
            Self::test_spiral_logarithmic_inverted,
            Self::test_spiral_archimedean,
            Self::test_spiral_archimedean_sheared,
            Self::test_spiral_archimedean_inverted,
            Self::test_spiral_hyperbolic,
            Self::test_spiral_hyperbolic_sheared,
            Self::test_spiral_hyperbolic_inverted,
            Self::test_spiral_fermat,
            Self::test_spiral_fermat_sheared,
            Self::test_spiral_fermat_inverted,
        ]
    }

    fn time_tracker(&self) -> &RcPtr<TimeTracker> { &self.time_tracker }
}

impl SpiralTests {
    fn spawn_spiral(
        &self,
        from_angle: f32,
        to_angle: f32,
        scale: Vector2,
        shear: f32,
        duration: f64,
        mode: Spiral,
        log_growth: Vector2,
    ) -> RcPtr<SpireTween<LerpMethodData<f64>>> {
        self.ball.do_spiral(
            CENTER, from_angle, to_angle, scale, duration, 0.0, shear, mode, log_growth,
        ).register()
    }

    fn spiral_test(
        &mut self,
        from_angle: f32,
        to_angle: f32,
        scale: Vector2,
        shear: f32,
        duration: f64,
        mode: Spiral,
        log_growth: Vector2,
    ) -> PinnedTestTask {
        let handle = self.spawn_spiral(from_angle, to_angle, scale, shear, duration, mode, log_growth);
        let gd = handle.gd_handle.as_ref().unwrap().clone();
        let tracker = RcPtr::clone(&self.time_tracker);

        Box::pin(async move {
            tracker.wait_finished(&gd, duration).await;
        })
    }

    fn test_spiral_logarithmic(&mut self) -> PinnedTestTask {
        self.spiral_test(0.0, 16.0 * PI, Vector2::ONE, 0.0, 8.0, Spiral::Logarithmic, Vector2::new(0.05, 0.05))
    }
    fn test_spiral_logarithmic_sheared(&mut self) -> PinnedTestTask {
        self.spiral_test(0.0, 16.0 * PI, Vector2::ONE, 0.5, 8.0, Spiral::Logarithmic, Vector2::new(0.075, 0.075))
    }
    fn test_spiral_logarithmic_inverted(&mut self) -> PinnedTestTask {
        self.spiral_test(16.0 * PI, 0.0, Vector2::ONE, 0.0, 8.0, Spiral::Logarithmic, Vector2::new(0.05, 0.05))
    }
    fn test_spiral_archimedean(&mut self) -> PinnedTestTask {
        self.spiral_test(0.0, 64.0 * PI, Vector2::ONE * 5.0, 0.0, 8.0, Spiral::Archimedean, Vector2::new(0.05, 0.05))
    }
    fn test_spiral_archimedean_sheared(&mut self) -> PinnedTestTask {
        self.spiral_test(0.0, 64.0 * PI, Vector2::ONE * 5.0, -0.3, 8.0, Spiral::Archimedean, Vector2::new(0.05, 0.05))
    }
    fn test_spiral_archimedean_inverted(&mut self) -> PinnedTestTask {
        self.spiral_test(64.0 * PI, 0.0, Vector2::ONE * 5.0, 0.0, 8.0, Spiral::Archimedean, Vector2::new(0.05, 0.05))
    }
    fn test_spiral_hyperbolic(&mut self) -> PinnedTestTask {
        self.spiral_test(0.3, 16.0 * PI, Vector2::new(512.0, 512.0), 0.0, 8.0, Spiral::Hyperbolic, Vector2::new(0.05, 0.05))
    }
    fn test_spiral_hyperbolic_sheared(&mut self) -> PinnedTestTask {
        self.spiral_test(0.3, 16.0 * PI, Vector2::new(512.0, 512.0), 0.7, 8.0, Spiral::Hyperbolic, Vector2::new(0.05, 0.05))
    }
    fn test_spiral_hyperbolic_inverted(&mut self) -> PinnedTestTask {
        self.spiral_test(16.0 * PI, 0.0, Vector2::new(256.0, 256.0), 0.0, 8.0, Spiral::Hyperbolic, Vector2::new(0.05, 0.05))
    }
    fn test_spiral_fermat(&mut self) -> PinnedTestTask {
        self.spiral_test(0.0, 24.0 * PI, Vector2::new(64.0, 64.0), 0.0, 8.0, Spiral::Fermat, Vector2::new(0.05, 0.05))
    }
    fn test_spiral_fermat_sheared(&mut self) -> PinnedTestTask {
        self.spiral_test(0.0, 24.0 * PI, Vector2::new(64.0, 64.0), -0.5, 8.0, Spiral::Fermat, Vector2::new(0.05, 0.05))
    }
    fn test_spiral_fermat_inverted(&mut self) -> PinnedTestTask {
        self.spiral_test(24.0 * PI, 0.0, Vector2::new(64.0, 64.0), 0.0, 8.0, Spiral::Fermat, Vector2::new(0.05, 0.05))
    }
}
