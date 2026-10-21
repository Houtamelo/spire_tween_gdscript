use std::time::{Duration, Instant};

use godot::{
    classes::{Os, PackedScene, Timer, node},
    prelude::*,
};

use super::{
    delays::DelaysTests,
    ellipsis_test::EllipsisTests,
    error_handling::ErrorHandlingTests,
    gd_handle_path::GdHandlePathTests,
    lerp_callable::LerpCallableTests,
    lerp_modes::LerpModesTests,
    misc::MiscTests,
    pause_process_modes::PauseProcessModesTests,
    play_pause_stop::PlayPauseStopTests,
    register_unregister::RegisterUnregisterTests,
    sequences::SequencesTests,
    spiral_test::SpiralTests,
    stop_events::StopEventsTests,
    templates_3d_test::Templates3DTests,
    templates_test::TemplatesTests,
    util::*,
};

enum TestClass {
    StopEvents,
    PlayPauseStop,
    Delays,
    Misc,
    RegisterUnregister,
    ErrorHandling,
    LerpModes,
    LerpCallable,
    PauseProcessModes,
    Sequences,
    Spiral,
    Ellipsis,
    GdHandlePath,
    Templates,
    Templates3D,
}

const ALL_TESTS: &[TestClass] = &[
    TestClass::StopEvents,
    TestClass::PlayPauseStop,
    TestClass::Delays,
    TestClass::Misc,
    TestClass::RegisterUnregister,
    TestClass::ErrorHandling,
    TestClass::LerpModes,
    TestClass::LerpCallable,
    TestClass::PauseProcessModes,
    TestClass::Sequences,
    TestClass::Spiral,
    TestClass::Ellipsis,
    TestClass::GdHandlePath,
    TestClass::Templates,
    TestClass::Templates3D,
];

#[derive(GodotClass)]
#[class(init, base = Node)]
pub struct TestRunner {
    base: Base<Node>,
    started: Option<Instant>,
    #[init(val = 900.0)]
    suite_timeout: f64,
}

#[godot_api]
impl INode for TestRunner {
    fn ready(&mut self) {
        godot_print!("=== SpireTween Rust Test Runner ===");

        let args = Os::singleton().get_cmdline_user_args().to_vec();
        let mut filter = String::new();
        let mut self_test = String::new();
        let mut test_timeout = 60.0;
        for arg in args {
            let arg = arg.to_string();
            if let Some(value) = arg.strip_prefix("--test=") {
                filter = value.to_owned();
            } else if let Some(value) = arg.strip_prefix("--runner-self-test=") {
                self_test = value.to_owned();
            } else if arg.starts_with("--test-timeout=") || arg.starts_with("--suite-timeout=") {
                let seconds = arg.split_once('=').unwrap().1.parse::<f64>().ok();
                let Some(seconds) = seconds.filter(|value| value.is_finite() && *value > 0.0 && *value <= 86400.0)
                else {
                    godot_error!("Invalid timeout: {arg}");
                    self.base().get_tree().quit_ex().exit_code(2).done();
                    return;
                };
                if arg.starts_with("--test-timeout=") {
                    test_timeout = seconds;
                } else {
                    self.suite_timeout = seconds;
                }
            }
        }
        self.started = Some(Instant::now());
        let started = self.started.unwrap();
        let suite_timeout = self.suite_timeout;
        let parent = self.base().clone();
        let mut watchdog = Timer::new_alloc();
        watchdog.set_process_mode(node::ProcessMode::ALWAYS);
        watchdog.set_ignore_time_scale(true);
        watchdog.set_wait_time(0.05);
        watchdog.signals().timeout().connect(move || {
            if started.elapsed().as_secs_f64() >= suite_timeout {
                godot_error!("TEST_SUITE_TIMEOUT");
                parent.get_tree().quit_ex().exit_code(1).done();
            }
        });
        self.base_mut().add_child(&watchdog);
        watchdog.start();

        let mut base = self.base().clone();
        godot::task::spawn(async move {
            let timeout = Duration::from_secs_f64(test_timeout);
            if !self_test.is_empty() {
                let failed = super::harness_checks::run_case(&mut base, &self_test, timeout).await;
                godot_print!("NATIVE_TEST_RESULT total=1 failed={failed}");
                base.get_tree().quit_ex().exit_code(i32::from(failed != 0)).done();
                return;
            }
            if filter.is_empty() {
                if super::bookkeeping::run_bookkeeping_tests(&mut base).await != 0 {
                    base.get_tree().quit_ex().exit_code(1).done();
                    return;
                }
                if super::delayed_call_validity::run_delayed_call_validity_tests(&mut base).await != 0 {
                    base.get_tree().quit_ex().exit_code(1).done();
                    return;
                }
                if super::method_validity::run_method_validity_tests(&mut base).await != 0 {
                    base.get_tree().quit_ex().exit_code(1).done();
                    return;
                }
                if super::api_compatibility::run_api_compatibility_tests(&mut base).await != 0 {
                    base.get_tree().quit_ex().exit_code(1).done();
                    return;
                }
                if super::ownership::run_ownership_tests(&mut base).await != 0 {
                    base.get_tree().quit_ex().exit_code(1).done();
                    return;
                }
                if super::node_binding::run_node_binding_tests(&mut base).await != 0 {
                    base.get_tree().quit_ex().exit_code(1).done();
                    return;
                }
                if super::sequence_cycles::run_sequence_cycle_tests(&mut base).await != 0 {
                    base.get_tree().quit_ex().exit_code(1).done();
                    return;
                }
            }
            let mut total = 0;
            let mut failed = 0;
            for test_class in ALL_TESTS {
                let result = match test_class {
                    TestClass::StopEvents => run_tests_from_class::<StopEventsTests>(&mut base, &filter, timeout).await,
                    TestClass::PlayPauseStop => {
                        run_tests_from_class::<PlayPauseStopTests>(&mut base, &filter, timeout).await
                    }
                    TestClass::Delays => run_tests_from_class::<DelaysTests>(&mut base, &filter, timeout).await,
                    TestClass::Misc => run_tests_from_class::<MiscTests>(&mut base, &filter, timeout).await,
                    TestClass::RegisterUnregister => {
                        run_tests_from_class::<RegisterUnregisterTests>(&mut base, &filter, timeout).await
                    }
                    TestClass::ErrorHandling => {
                        run_tests_from_class::<ErrorHandlingTests>(&mut base, &filter, timeout).await
                    }
                    TestClass::LerpModes => run_tests_from_class::<LerpModesTests>(&mut base, &filter, timeout).await,
                    TestClass::LerpCallable => {
                        run_tests_from_class::<LerpCallableTests>(&mut base, &filter, timeout).await
                    }
                    TestClass::PauseProcessModes => {
                        run_tests_from_class::<PauseProcessModesTests>(&mut base, &filter, timeout).await
                    }
                    TestClass::Sequences => run_tests_from_class::<SequencesTests>(&mut base, &filter, timeout).await,
                    TestClass::Spiral => run_tests_from_class::<SpiralTests>(&mut base, &filter, timeout).await,
                    TestClass::Ellipsis => run_tests_from_class::<EllipsisTests>(&mut base, &filter, timeout).await,
                    TestClass::GdHandlePath => {
                        run_tests_from_class::<GdHandlePathTests>(&mut base, &filter, timeout).await
                    }
                    TestClass::Templates => run_tests_from_class::<TemplatesTests>(&mut base, &filter, timeout).await,
                    TestClass::Templates3D => {
                        run_tests_from_class::<Templates3DTests>(&mut base, &filter, timeout).await
                    }
                };
                total += result.0;
                failed += result.1;
            }
            if total == 0 {
                failed += 1;
                godot_error!("No test matched: {filter}");
            }
            godot_print!("NATIVE_TEST_RESULT total={total} failed={failed}");
            for _ in 0..4 {
                next_frame().await;
            }
            base.get_tree().quit_ex().exit_code(i32::from(failed != 0)).done();
        });
    }
}

async fn run_tests_from_class<T: ITestClass>(parent: &mut Gd<Node>, filter: &str, timeout: Duration) -> (usize, usize) {
    let prefab: Gd<PackedScene> = load(T::PREFAB_PATH);
    let mut total = 0;
    let mut failed = 0;

    // Fresh scene instance per test, matching GDScript runner behavior.
    for (name, test_fn) in T::test_list() {
        if !filter.is_empty() && filter != name {
            continue;
        }
        total += 1;
        let mut node = prefab.instantiate_as::<T>();
        parent.add_child(&node);

        let handle = node.bind_mut().run_test(name, test_fn);
        failed += usize::from(!wait_for_task(handle, timeout).await);

        node.clone().upcast::<Node>().queue_free();
        next_frame().await;
    }
    (total, failed)
}
