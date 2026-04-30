use godot::classes::PackedScene;
use godot::prelude::*;

use super::util::*;
use super::delays::DelaysTests;
use super::ellipsis_test::EllipsisTests;
use super::error_handling::ErrorHandlingTests;
use super::gd_handle_path::GdHandlePathTests;
use super::lerp_callable::LerpCallableTests;
use super::lerp_modes::LerpModesTests;
use super::misc::MiscTests;
use super::pause_process_modes::PauseProcessModesTests;
use super::play_pause_stop::PlayPauseStopTests;
use super::register_unregister::RegisterUnregisterTests;
use super::sequences::SequencesTests;
use super::spiral_test::SpiralTests;

enum TestClass {
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
}

const ALL_TESTS: &[TestClass] = &[
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
];

#[derive(GodotClass)]
#[class(init, base = Node)]
pub struct TestRunner {
    base: Base<Node>,
}

#[godot_api]
impl INode for TestRunner {
    fn ready(&mut self) {
        godot_print!("=== SpireTween Rust Test Runner ===");

        let mut base = self.base().clone();
        godot::task::spawn(async move {
            for test_class in ALL_TESTS {
                match test_class {
                    TestClass::PlayPauseStop => run_tests_from_class::<PlayPauseStopTests>(&mut base).await,
                    TestClass::Delays => run_tests_from_class::<DelaysTests>(&mut base).await,
                    TestClass::Misc => run_tests_from_class::<MiscTests>(&mut base).await,
                    TestClass::RegisterUnregister => run_tests_from_class::<RegisterUnregisterTests>(&mut base).await,
                    TestClass::ErrorHandling => run_tests_from_class::<ErrorHandlingTests>(&mut base).await,
                    TestClass::LerpModes => run_tests_from_class::<LerpModesTests>(&mut base).await,
                    TestClass::LerpCallable => run_tests_from_class::<LerpCallableTests>(&mut base).await,
                    TestClass::PauseProcessModes => run_tests_from_class::<PauseProcessModesTests>(&mut base).await,
                    TestClass::Sequences => run_tests_from_class::<SequencesTests>(&mut base).await,
                    TestClass::Spiral => run_tests_from_class::<SpiralTests>(&mut base).await,
                    TestClass::Ellipsis => run_tests_from_class::<EllipsisTests>(&mut base).await,
                    TestClass::GdHandlePath => run_tests_from_class::<GdHandlePathTests>(&mut base).await,
                }
            }

            godot_print!("=== All tests finished! ===");
            base.get_tree().quit();
        });
    }
}

async fn run_tests_from_class<T: ITestClass>(parent: &mut Gd<Node>) {
    let prefab: Gd<PackedScene> = load(T::PREFAB_PATH);

    // Fresh scene instance per test, matching GDScript runner behavior.
    for test_fn in T::test_list() {
        let mut node = prefab.instantiate_as::<T>();
        parent.add_child(&node);

        let handle = node.bind_mut().run_test(test_fn);
        wait_for_task(handle).await;

        node.clone().upcast::<Node>().queue_free();
        next_frame().await;
    }
}
