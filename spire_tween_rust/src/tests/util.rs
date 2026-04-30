use std::pin::Pin;

use godot::{
    classes::{Engine, object::ConnectFlags},
    obj::{Bounds, bounds},
    prelude::*,
};
use spire_tween::prelude::*;

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

    pub fn frames_since_start(&self) -> u64 {
        Engine::singleton().get_process_frames() - self.start_frame
    }

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
}

/// Internal state shared between a closure connected to a tween event and the
/// future waiting on it.
struct WaitState {
    fired: bool,
    timer_at_fire: f64,
    waker: Option<std::task::Waker>,
}

impl WaitState {
    fn new() -> Self {
        Self {
            fired: false,
            timer_at_fire: 0.0,
            waker: None,
        }
    }
}

/// Awaits the tween's `finished` event using the closure-based connection API.
/// No Godot `Gd` handle is required.
///
/// The captured timer value is taken at the *exact moment* of emission (synchronous
/// closure invocation), so the timing assertion is independent of when the async
/// task is later resumed by the gdext deferred-wake.
pub fn wait_finished<T>(
    tween: &RcPtr<SpireTween<T>>,
    tracker: &RcPtr<TimeTracker>,
    expected_end_time: f64,
) -> impl Future<Output = ()>
where T: ITweenable
{
    let state = RcPtr::new(WaitState::new());
    let state_for_closure = state.clone();
    let tracker_for_closure = tracker.clone();

    tween.to_mut().finished_connect(
        move || {
            let mut s = state_for_closure.to_mut();
            s.timer_at_fire = tracker_for_closure.timer;
            s.fired = true;
            if let Some(w) = s.waker.take() {
                w.wake();
            }
        },
        SpireFlags::ONE_SHOT,
    );

    let tracker_for_log = tracker.clone();

    async move {
        let timer = std::future::poll_fn(move |cx| {
            let mut s = state.to_mut();
            if s.fired {
                std::task::Poll::Ready(s.timer_at_fire)
            } else {
                s.waker = Some(cx.waker().clone());
                std::task::Poll::Pending
            }
        })
        .await;

        assert_within_tolerance(timer - expected_end_time, TIME_TOLERANCE);
        tracker_for_log.debug_msg("Finished");
    }
}

/// Awaits the tween's `loop_finished` event using the closure-based connection API.
/// No Godot `Gd` handle is required.
pub fn wait_loop_finished<T>(
    tween: &RcPtr<SpireTween<T>>,
    tracker: &RcPtr<TimeTracker>,
    expected_end_time: f64,
) -> impl Future<Output = ()>
where T: ITweenable
{
    let state = RcPtr::new(WaitState::new());
    let state_for_closure = state.clone();
    let tracker_for_closure = tracker.clone();

    tween.to_mut().loop_finished_connect(
        move || {
            let mut s = state_for_closure.to_mut();
            s.timer_at_fire = tracker_for_closure.timer;
            s.fired = true;
            if let Some(w) = s.waker.take() {
                w.wake();
            }
        },
        SpireFlags::ONE_SHOT,
    );

    let tracker_for_log = tracker.clone();

    async move {
        let timer = std::future::poll_fn(move |cx| {
            let mut s = state.to_mut();
            if s.fired {
                std::task::Poll::Ready(s.timer_at_fire)
            } else {
                s.waker = Some(cx.waker().clone());
                std::task::Poll::Pending
            }
        })
        .await;

        assert_within_tolerance(timer - expected_end_time, TIME_TOLERANCE);
        tracker_for_log.debug_msg("Loop completed");
    }
}

/// Registers a synchronous timing assertion on the tween's `finished` event.
/// Fires at emission time (no deferred wake), so timing is checked precisely.
pub fn assert_finished_timing<T>(
    tween: &RcPtr<SpireTween<T>>,
    tracker: &RcPtr<TimeTracker>,
    expected_end_time: f64,
) where T: ITweenable
{
    let tracker_c = tracker.clone();
    tween.to_mut().finished_connect(
        move || assert_within_tolerance(tracker_c.timer - expected_end_time, TIME_TOLERANCE),
        SpireFlags::ONE_SHOT,
    );
}

/// Registers a closure to run whenever the tween's `finished` event fires.
/// Persistent (does NOT auto-disconnect after first call). Use to detect
/// unwanted re-emissions on stopped tweens.
pub fn assert_upon_finished<T, F>(tween: &RcPtr<SpireTween<T>>, assert_fn: F)
where
    T: ITweenable,
    F: FnMut() + 'static,
{
    tween.to_mut().finished_connect(assert_fn, SpireFlags::empty());
}

/// Registers a closure to run whenever the tween's `loop_finished` event fires.
/// Persistent. Use to detect unwanted re-emissions.
pub fn assert_upon_loop_finished<T, F>(tween: &RcPtr<SpireTween<T>>, assert_fn: F)
where
    T: ITweenable,
    F: FnMut() + 'static,
{
    tween.to_mut().loop_finished_connect(assert_fn, SpireFlags::empty());
}

/// Captures a value at the exact moment the tween's `finished` event fires.
pub fn snapshot_on_finished<T, V>(
    tween: &RcPtr<SpireTween<T>>,
    capture_fn: impl Fn() -> V + 'static,
) -> RcPtr<Option<V>>
where
    T: ITweenable,
    V: 'static,
{
    let slot = RcPtr::new(None);
    let slot_clone = slot.clone();
    tween.to_mut().finished_connect(
        move || *slot_clone.to_mut() = Some(capture_fn()),
        SpireFlags::ONE_SHOT,
    );
    slot
}

/// Generic Signal-based snapshot (kept for tree-signal scenarios such as
/// `process_frame` or `SceneTreeTimer.timeout` where there is no tween-side
/// equivalent).
pub fn snapshot_on_signal<T: 'static>(
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

/// Generic Signal-based one-shot assertion (kept for tree-signal scenarios).
pub fn assert_upon_emission(signal: Signal, assert_fn: impl Fn() + 'static) {
    signal.connect_flags(
        &Callable::from_fn("assert_upon_emission", move |_| {
            assert_fn();
            Variant::nil()
        }),
        ConnectFlags::ONE_SHOT,
    );
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
