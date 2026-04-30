use std::pin::Pin;

use godot::{
    classes::{Engine, object::ConnectFlags},
    obj::{Bounds, bounds},
    prelude::*,
};
use spire_tween::prelude::RcPtr;

pub const TIME_TOLERANCE: f64 = 0.017;

pub type PinnedTestTask = Pin<Box<dyn Future<Output = ()> + 'static>>;

pub trait ITestClass: GodotClass + Bounds<Declarer = bounds::DeclUser> + Inherits<Node> {
    const PREFAB_PATH: &'static str;
    fn test_list() -> Vec<fn(&mut Self) -> PinnedTestTask>;
    fn time_tracker(&self) -> &RcPtr<TimeTracker>;

    fn run_test(&mut self, test_fn: fn(&mut Self) -> PinnedTestTask) -> godot::task::TaskHandle {
        self.time_tracker().to_mut().reset();
        let test_task = test_fn(self);
        let test_name = std::any::type_name_of_val(&test_fn);

        godot::task::spawn(async move {
            godot_print!("---------------------------------");
            godot_print!("Test Started: {test_name}");
            test_task.await;
            godot_print!("Test Finished: {test_name}");
        })
    }
}

pub struct TimeTracker {
    pub timer: f64,
    pub start_frame: u64,
}

impl TimeTracker {
    pub fn new() -> Self {
        Self {
            timer: 0.0,
            start_frame: Engine::singleton().get_process_frames(),
        }
    }

    pub fn reset(&mut self) {
        self.timer = 0.0;
        self.start_frame = Engine::singleton().get_process_frames();
    }

    pub fn frames_since_start(&self) -> u64 { Engine::singleton().get_process_frames() - self.start_frame }

    pub fn debug_msg(&self, message: impl AsRef<str>) {
        let frame = format!("{:06.0}", self.frames_since_start());
        let time = format!("{:06.4}", self.timer);
        println!("[{frame}f, {time}s] {}", message.as_ref());
    }

    pub fn wait_seconds(&self, seconds: f64) -> impl Future<Output = ()> {
        let mut tree = Engine::singleton().get_main_loop().unwrap().cast::<SceneTree>();
        let timer = tree.create_timer(seconds);
        let signal = Signal::from_object_signal(&timer, "timeout");

        async move {
            signal.to_future::<()>().await;
        }
    }

    pub fn wait_loop_finished<Handle: Inherits<RefCounted>>(
        &self,
        node: &Gd<Handle>,
        expected_end_time: f64,
    ) -> impl Future<Output = ()> {
        let signal = Signal::from_object_signal(node, "loop_finished");
        let tween_name = node.upcast_ref().get_class().to_string();

        async move {
            signal.to_future::<()>().await;

            assert_within_tolerance(self.timer - expected_end_time, TIME_TOLERANCE);
            self.debug_msg(format!("Tween {tween_name}: Loop completed"));
        }
    }

    pub fn wait_finished<Handle: Inherits<RefCounted>>(
        &self,
        node: &Gd<Handle>,
        expected_end_time: f64,
    ) -> impl Future<Output = ()> {
        let signal = Signal::from_object_signal(node, "finished");
        let tween_name = node.upcast_ref().get_class().to_string();
        async move {
            signal.to_future::<()>().await;

            assert_within_tolerance(self.timer - expected_end_time, TIME_TOLERANCE);
            self.debug_msg(format!("Tween {tween_name}: Finished"));
        }
    }
}

/// Captures a value at the exact moment a signal fires (inline via Callable),
/// rather than at async-resume time. Use when a property might change between
/// signal emission and async polling (e.g. sequence children transitioning
/// within the same frame).
pub fn snapshot_on_signal<T: Copy + 'static>(
    signal: &Signal,
    capture_fn: impl Fn() -> T + 'static,
) -> RcPtr<Option<T>> {
    let slot = RcPtr::new(None);
    let slot_clone = slot.clone();
    signal.connect_flags(
        &Callable::from_fn("snapshot_capture", move |_| {
            *slot_clone.to_mut() = Some(capture_fn());
            Variant::nil()
        }),
        ConnectFlags::ONE_SHOT,
    );
    slot
}

pub async fn wait_for_task(handle: godot::task::TaskHandle) {
    while handle.is_pending() {
        next_frame().await;
    }
}

pub async fn next_frame() {
    let tree = Engine::singleton().get_main_loop().unwrap().cast::<SceneTree>();
    tree.signals().process_frame().to_untyped().to_future::<()>().await;
}

pub fn assert_upon_emission(signal: Signal, assert_fn: impl Fn() + 'static) {
    signal.connect_flags(
        &Callable::from_fn("assert_upon_emission", move |_| {
            assert_fn();
            Variant::nil()
        }),
        ConnectFlags::ONE_SHOT,
    );
}

#[macro_export]
macro_rules! impl_test_base {
    ($INode:ty, $GdTy:ty) => {
        #[godot_api]
        impl $INode for $GdTy {
            fn ready(&mut self) {
                self.base_mut().set_process_priority(-10);
                self.base_mut().set_physics_process_priority(-10);
            }

            fn process(&mut self, delta: f64) { self.time_tracker.timer += delta; }
        }
    };
}

pub fn assert_within_tolerance(num: f64, tolerance: f64) {
    assert!(num.abs() <= tolerance, "Abs({num}) is bigger than tolerance `{tolerance}`");
}

/// Registers a synchronous assertion that checks `finished` signal timing at emission time.
/// This runs inside the signal callback (not deferred), so the timing check happens
/// regardless of whether the async task awaits `finished` or skips it via the
/// `if !handle.is_stopped()` guard.
pub fn assert_finished_timing<Handle: Inherits<RefCounted>>(
    node: &Gd<Handle>,
    tracker: &RcPtr<TimeTracker>,
    expected_end_time: f64,
) {
    let tracker_c = tracker.clone();
    assert_upon_emission(Signal::from_object_signal(node, "finished"), move || {
        assert_within_tolerance(tracker_c.timer - expected_end_time, TIME_TOLERANCE)
    });
}
