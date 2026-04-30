use godot::classes::{CollisionShape2D, Sprite2D};
use godot::prelude::*;
use spire_tween::prelude::*;

use super::util::*;

// Can't use impl_test_base! because we need custom _process and _physics_process.

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum ProcessPhase {
    None,
    Idle,
    Physics,
}

struct ProcessModeVerifier {
    expected_phase: ProcessPhase,
    pub failures: Vec<String>,
    active: bool,
    scale_at_idle_start: Option<Vector2>,
    scale_at_physics_start: Option<Vector2>,
    sprite: Option<Gd<Sprite2D>>,
    tween_is_stopped: Option<Box<dyn Fn() -> bool>>,
    tween_anim_pos: Option<Box<dyn Fn() -> f64>>,
}

impl ProcessModeVerifier {
    fn new() -> Self {
        Self {
            expected_phase: ProcessPhase::None,
            failures: Vec::new(),
            active: false,
            scale_at_idle_start: None,
            scale_at_physics_start: None,
            sprite: None,
            tween_is_stopped: None,
            tween_anim_pos: None,
        }
    }

    fn activate(
        &mut self,
        expected: ProcessPhase,
        sprite: Gd<Sprite2D>,
        is_stopped: impl Fn() -> bool + 'static,
        anim_pos: impl Fn() -> f64 + 'static,
    ) {
        self.expected_phase = expected;
        self.active = true;
        self.failures.clear();
        self.sprite = Some(sprite);
        self.tween_is_stopped = Some(Box::new(is_stopped));
        self.tween_anim_pos = Some(Box::new(anim_pos));
        self.scale_at_idle_start = None;
        self.scale_at_physics_start = None;
    }

    fn deactivate(&mut self) {
        self.active = false;
        self.sprite = None;
        self.tween_is_stopped = None;
        self.tween_anim_pos = None;
    }

    fn on_idle_start(&mut self) {
        if !self.active { return; }
        let sprite = self.sprite.as_ref().unwrap();
        self.scale_at_idle_start = Some(sprite.get_scale());
    }

    fn on_idle_end(&mut self) {
        if !self.active { return; }
        if (self.tween_is_stopped.as_ref().unwrap())() { return; }

        let sprite = self.sprite.as_ref().unwrap();
        let scale_now = sprite.get_scale();
        let scale_before = self.scale_at_idle_start.unwrap();

        if self.expected_phase == ProcessPhase::Idle {
            let anim_pos = (self.tween_anim_pos.as_ref().unwrap())();
            if anim_pos > 0.00001 && scale_before == scale_now {
                self.failures.push(format!(
                    "Expected scale to change during Idle (anim_pos={anim_pos}), but it didn't: {scale_before}"
                ));
            }
        } else {
            if scale_before != scale_now {
                self.failures.push(format!(
                    "Scale changed during Idle but expected_phase={:?}: {scale_before} -> {scale_now}",
                    self.expected_phase
                ));
            }
        }
    }

    fn on_physics_start(&mut self) {
        if !self.active { return; }
        let sprite = self.sprite.as_ref().unwrap();
        self.scale_at_physics_start = Some(sprite.get_scale());
    }

    fn on_physics_end(&mut self) {
        if !self.active { return; }
        if (self.tween_is_stopped.as_ref().unwrap())() { return; }

        let sprite = self.sprite.as_ref().unwrap();
        let scale_now = sprite.get_scale();
        let scale_before = self.scale_at_physics_start.unwrap();

        if self.expected_phase == ProcessPhase::Physics {
            let anim_pos = (self.tween_anim_pos.as_ref().unwrap())();
            if anim_pos > 0.00001 && scale_before == scale_now {
                self.failures.push(format!(
                    "Expected scale to change during Physics (anim_pos={anim_pos}), but it didn't: {scale_before}"
                ));
            }
        } else {
            if scale_before != scale_now {
                self.failures.push(format!(
                    "Scale changed during Physics but expected_phase={:?}: {scale_before} -> {scale_now}",
                    self.expected_phase
                ));
            }
        }
    }
}

#[derive(GodotClass)]
#[class(init, base = Node2D)]
pub struct PauseProcessModesTests {
    base: Base<Node2D>,
    #[init(node = "BouncyBall/CollisionShape2D")]
    shape: OnReady<Gd<CollisionShape2D>>,
    #[init(node = "BouncyBall/CollisionShape2D/Sprite2D")]
    sprite: OnReady<Gd<Sprite2D>>,
    #[init(val = RcPtr::new(TimeTracker::new()))]
    time_tracker: RcPtr<TimeTracker>,
    #[init(val = RcPtr::new(ProcessModeVerifier::new()))]
    verifier: RcPtr<ProcessModeVerifier>,
}

#[godot_api]
impl INode2D for PauseProcessModesTests {
    fn ready(&mut self) {
        self.base_mut().set_process_priority(-10);
        self.base_mut().set_physics_process_priority(-10);
    }

    fn process(&mut self, delta: f64) {
        self.time_tracker.timer += delta;

        self.verifier.to_mut().on_idle_start();

        if self.verifier.active {
            let verifier = self.verifier.clone();
            let callable = Callable::from_fn("idle_end_check", move |_| {
                verifier.to_mut().on_idle_end();
                Variant::nil()
            });
            callable.call_deferred(&[]);
        }
    }

    fn physics_process(&mut self, _delta: f64) {
        self.verifier.to_mut().on_physics_start();

        if self.verifier.active {
            let verifier = self.verifier.clone();
            let callable = Callable::from_fn("physics_end_check", move |_| {
                verifier.to_mut().on_physics_end();
                Variant::nil()
            });
            callable.call_deferred(&[]);
        }
    }
}

impl ITestClass for PauseProcessModesTests {
    const PREFAB_PATH: &'static str = "res://examples/tests/pause_process_modes.tscn";

    fn test_list() -> Vec<fn(&mut Self) -> PinnedTestTask> {
        vec![
            Self::test_pause_mode_process,
            Self::test_pause_mode_stop,
            Self::test_pause_mode_bound,
            Self::test_process_mode_idle,
            Self::test_process_mode_physics,
            Self::test_pause_mode_bound_process_mode_physics,
        ]
    }

    fn time_tracker(&self) -> &RcPtr<TimeTracker> { &self.time_tracker }
}

impl PauseProcessModesTests {
    fn test_pause_mode_process(&mut self) -> PinnedTestTask {
        let shape = self.shape.clone();
        let final_value = Vector2::ONE * 6.0;

        let handle = shape.do_scale(final_value, 3.0)
            .with_pause_mode(PauseMode::Process)
            .register();
        let tracker = RcPtr::clone(&self.time_tracker);

        let mut tree = self.base().get_tree();
        tree.set_pause(true);
        // Timer doesn't run while scene tree is paused, so we manually increase it here.
        self.time_tracker.to_mut().timer += 3.0;

        Box::pin(async move {
            wait_finished(&handle, &tracker, 3.0).await;
            assert_eq!(shape.get_scale(), final_value);
            tree.set_pause(false);
        })
    }

    fn test_pause_mode_stop(&mut self) -> PinnedTestTask {
        let shape = self.shape.clone();
        let initial_value = shape.get_scale();
        let final_value = Vector2::ONE * 6.0;

        let handle = shape.do_scale(final_value, 3.0)
            .with_pause_mode(PauseMode::Stop)
            .register();
        let tracker = RcPtr::clone(&self.time_tracker);

        let mut tree = self.base().get_tree();
        tree.set_pause(true);

        Box::pin(async move {
            // Tween with STOP mode should not process while paused.
            // Use create_timer with process_always=true to wait during pause.
            let timer = tree.create_timer_ex(5.0).process_always(true).process_in_physics(false).ignore_time_scale(true).done();
            Signal::from_object_signal(&timer, "timeout").to_future::<()>().await;
            assert_eq!(shape.get_scale(), initial_value);

            tree.set_pause(false);
            wait_finished(&handle, &tracker, 3.0).await;
            assert_eq!(shape.get_scale(), final_value);
        })
    }

    // Uses `register_with_gd_handle` to also exercise the Gd-handle `is_registered()` path.
    fn test_pause_mode_bound(&mut self) -> PinnedTestTask {
        let mut sprite = self.sprite.clone();
        let initial_value = sprite.get_scale();
        let final_value = Vector2::ONE * 4.0;

        let handle = sprite.do_scale(final_value, 3.0)
            .with_pause_mode(PauseMode::Bound)
            .register_with_gd_handle();
        let gd = handle.gd_handle.as_ref().unwrap().clone();
        let tracker = RcPtr::clone(&self.time_tracker);

        sprite.set_process_mode(godot::classes::node::ProcessMode::DISABLED);
        let mut tree = self.base().get_tree();
        tree.set_pause(true);

        Box::pin(async move {
            // Paused tree + disabled sprite: tween shouldn't process.
            let timer = tree.create_timer(4.0);
            Signal::from_object_signal(&timer, "timeout").to_future::<()>().await;
            // Timer does not update while tree is paused, so we do it manually here.
            tracker.to_mut().timer += 4.0;
            assert_eq!(sprite.get_scale(), initial_value);

            // Unpause tree, but sprite still disabled.
            tree.set_pause(false);
            let timer = tree.create_timer(2.0);
            Signal::from_object_signal(&timer, "timeout").to_future::<()>().await;
            assert_eq!(sprite.get_scale(), initial_value);

            // Enable sprite — tween should start processing.
            sprite.set_process_mode(godot::classes::node::ProcessMode::INHERIT);
            assert_eq!(handle.get_total_elapsed_time(), 0.0);
            assert!(handle.is_playing());
            assert!(sprite.can_process());
            assert!(gd.bind().is_registered());

            wait_finished(&handle, &tracker, 9.0).await;
            assert_eq!(sprite.get_scale(), final_value);
        })
    }

    fn test_process_mode_idle(&mut self) -> PinnedTestTask {
        let sprite = self.sprite.clone();
        let final_value = Vector2::ONE * 3.0;

        let handle = sprite.do_scale(final_value, 4.0)
            .with_process_mode(ProcessMode::Idle)
            .register();
        let tracker = RcPtr::clone(&self.time_tracker);

        assert_eq!(handle.get_process_mode(), ProcessMode::Idle);

        let h1 = handle.clone();
        let h2 = handle.clone();
        self.verifier.to_mut().activate(
            ProcessPhase::Idle, sprite.clone(),
            move || h1.is_stopped(),
            move || h2.get_animation_position(),
        );
        let verifier = self.verifier.clone();
        assert_finished_timing(&handle, &tracker, 4.0);

        Box::pin(async move {
            wait_finished(&handle, &tracker, 4.0).await;
            verifier.to_mut().deactivate();

            let failures = &verifier.failures;
            assert!(failures.is_empty(), "Process mode verification failures:\n{}", failures.join("\n"));
            assert_eq!(sprite.get_scale(), final_value);
        })
    }

    fn test_process_mode_physics(&mut self) -> PinnedTestTask {
        let sprite = self.sprite.clone();
        let final_value = Vector2::ONE * 3.0;

        let handle = sprite.do_scale(final_value, 4.0)
            .with_process_mode(ProcessMode::Physics)
            .register();
        let tracker = RcPtr::clone(&self.time_tracker);

        assert_eq!(handle.get_process_mode(), ProcessMode::Physics);

        let h1 = handle.clone();
        let h2 = handle.clone();
        self.verifier.to_mut().activate(
            ProcessPhase::Physics, sprite.clone(),
            move || h1.is_stopped(),
            move || h2.get_animation_position(),
        );
        let verifier = self.verifier.clone();
        assert_finished_timing(&handle, &tracker, 4.0);

        Box::pin(async move {
            wait_finished(&handle, &tracker, 4.0).await;
            verifier.to_mut().deactivate();

            let failures = &verifier.failures;
            assert!(failures.is_empty(), "Process mode verification failures:\n{}", failures.join("\n"));
            assert_eq!(sprite.get_scale(), final_value);
        })
    }

    // Uses `register_with_gd_handle` to also exercise the Gd-handle `is_registered()` path.
    fn test_pause_mode_bound_process_mode_physics(&mut self) -> PinnedTestTask {
        let mut sprite = self.sprite.clone();
        let initial_value = sprite.get_scale();
        let final_value = Vector2::ONE * 4.0;

        let handle = sprite.do_scale(final_value, 3.0)
            .with_pause_mode(PauseMode::Bound)
            .with_process_mode(ProcessMode::Physics)
            .register_with_gd_handle();
        let gd = handle.gd_handle.as_ref().unwrap().clone();
        let tracker = RcPtr::clone(&self.time_tracker);

        sprite.set_process_mode(godot::classes::node::ProcessMode::DISABLED);
        let mut tree = self.base().get_tree();
        tree.set_pause(true);

        // Activate verifier for the physics phase (will only check once sprite is enabled and tree unpaused)
        let h1 = handle.clone();
        let h2 = handle.clone();
        self.verifier.to_mut().activate(
            ProcessPhase::Physics, sprite.clone(),
            move || h1.is_stopped(),
            move || h2.get_animation_position(),
        );
        let verifier = self.verifier.clone();
        assert_finished_timing(&handle, &tracker, 9.0);

        Box::pin(async move {
            tracker.wait_seconds(4.0).await;
            // Timer doesn't update while tree is paused, so we do it manually here.
            tracker.to_mut().timer += 4.0;
            assert_eq!(sprite.get_scale(), initial_value);

            tree.set_pause(false);
            tracker.wait_seconds(2.0).await;
            assert_eq!(sprite.get_scale(), initial_value);

            sprite.set_process_mode(godot::classes::node::ProcessMode::INHERIT);
            assert_eq!(handle.get_total_elapsed_time(), 0.0);
            assert!(handle.is_playing());
            assert!(sprite.can_process());
            assert!(gd.bind().is_registered());

            wait_finished(&handle, &tracker, 9.0).await;
            verifier.to_mut().deactivate();

            let failures = &verifier.failures;
            assert!(failures.is_empty(), "Process mode verification failures:\n{}", failures.join("\n"));
            assert_eq!(sprite.get_scale(), final_value);
        })
    }
}
