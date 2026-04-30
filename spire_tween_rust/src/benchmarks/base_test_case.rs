use godot::prelude::*;

use super::runner::{BenchmarkOrchestrator, SetupFn, STD_AMOUNTS, STD_DURATION};

/// Base class for benchmark test cases.
///
/// Port of `base_test_case.gd`. Subclasses should override `test_name()` and
/// `setup_fn()` to provide the benchmark name and setup callback.
///
/// On `_ready`, it creates a `BenchmarkOrchestrator` child that runs through
/// all standard amounts, prints results, and quits.
#[derive(GodotClass)]
#[class(base = Node)]
pub struct BaseTestCase {
    base: Base<Node>,
}

#[godot_api]
impl INode for BaseTestCase {
    fn init(base: Base<Node>) -> Self {
        Self { base }
    }
}

impl BaseTestCase {
    /// Launch benchmarks using the given test name and setup function.
    ///
    /// Call this from `_ready()` of concrete benchmark classes.
    pub fn launch(
        parent: &mut Gd<Node>,
        test_name: String,
        setup_fn: SetupFn,
        duration: f64,
        amounts: Vec<i64>,
    ) {
        let mut orchestrator = BenchmarkOrchestrator::new_alloc();
        {
            let mut bind = orchestrator.bind_mut();
            bind.start(test_name, setup_fn, duration, amounts);
        }
        parent.add_child(&orchestrator);
    }

    /// Launch with standard defaults.
    pub fn launch_default(parent: &mut Gd<Node>, test_name: String, setup_fn: SetupFn) {
        Self::launch(parent, test_name, setup_fn, STD_DURATION, STD_AMOUNTS.to_vec());
    }
}
