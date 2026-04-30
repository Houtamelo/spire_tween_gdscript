use godot::classes::{Engine, Time};
use godot::prelude::*;

use super::util;
use std::collections::HashMap;

/// Standard benchmark node amounts.
pub const STD_AMOUNTS: &[i64] = &[1_000, 10_000, 50_000];

/// Standard benchmark duration in seconds.
pub const STD_DURATION: f64 = 10.0;

/// Callback type for setting up tweens in the benchmark.
///
/// Arguments: `(root: &mut Gd<Node>, is_builtin: bool, duration: f64, amount: i64)`
pub type SetupFn = fn(&mut Gd<Node>, bool, f64, i64);

/// State machine phases for the benchmark runner.
enum RunnerPhase {
    /// Waiting for initial delay before starting setup.
    WaitingPreDelay { remaining: f64 },
    /// Running: collecting measurements for `remaining` seconds.
    Running { remaining: f64 },
    /// Finished: results are ready to be collected.
    Finished,
}

/// Benchmark runner node.
///
/// Port of `runner.gd`. Attaches to the scene tree, calls a setup function,
/// then measures FPS and delta times for the configured duration.
#[derive(GodotClass)]
#[class(base = Node)]
pub struct BenchmarkRunner {
    base: Base<Node>,

    setup_fn: Option<SetupFn>,
    is_builtin: bool,
    duration: f64,
    amount: i64,

    // Measurements
    setup_time: f64,
    delta_times: Vec<f64>,
    engine_fps: Vec<f64>,
    begin_sum: f64,
    skip_frames: i32,

    phase: RunnerPhase,
}

#[godot_api]
impl INode for BenchmarkRunner {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            setup_fn: None,
            is_builtin: false,
            duration: STD_DURATION,
            amount: 0,
            setup_time: 0.0,
            delta_times: Vec::new(),
            engine_fps: Vec::new(),
            begin_sum: 0.0,
            skip_frames: 2,
            phase: RunnerPhase::WaitingPreDelay { remaining: 2.0 },
        }
    }

    fn process(&mut self, delta: f64) {
        match &mut self.phase {
            RunnerPhase::WaitingPreDelay { remaining } => {
                *remaining -= delta;
                if *remaining <= 0.0 {
                    // Start the benchmark
                    self.run_setup();
                    self.phase = RunnerPhase::Running {
                        remaining: self.duration,
                    };
                }
            }
            RunnerPhase::Running { remaining } => {
                self.begin_sum += delta;
                self.skip_frames -= 1;
                if self.skip_frames < 0 {
                    self.delta_times.push(delta);
                }

                *remaining -= delta;
                if *remaining <= 0.0 {
                    self.phase = RunnerPhase::Finished;
                }
            }
            RunnerPhase::Finished => {}
        }
    }

    fn physics_process(&mut self, _delta: f64) {
        if let RunnerPhase::Running { .. } = &self.phase {
            // Engine FPS is inaccurate at the start because it reports the
            // average over the last second.
            if self.begin_sum > 2.0 {
                self.engine_fps
                    .push(Engine::singleton().get_frames_per_second() as f64);
            }
        }
    }
}

impl BenchmarkRunner {
    /// Configure the runner before adding it to the tree.
    pub fn configure(&mut self, setup_fn: SetupFn, is_builtin: bool, duration: f64, amount: i64) {
        self.setup_fn = Some(setup_fn);
        self.is_builtin = is_builtin;
        self.duration = duration;
        self.amount = amount;
    }

    /// Called when the pre-delay expires; runs the setup function and records
    /// the setup time.
    fn run_setup(&mut self) {
        let start = Time::singleton().get_ticks_usec();

        if let Some(setup_fn) = self.setup_fn {
            let mut root = self.base().get_parent().unwrap();
            (setup_fn)(&mut root, self.is_builtin, self.duration, self.amount);
        }

        let end = Time::singleton().get_ticks_usec();
        self.setup_time = (end - start) as f64 / 1_000_000.0;
    }

    /// Check whether the runner has finished collecting data.
    pub fn is_finished(&self) -> bool {
        matches!(self.phase, RunnerPhase::Finished)
    }

    /// Extract the collected results.
    pub fn take_results(&self) -> HashMap<&'static str, Vec<f64>> {
        util::make_results(
            self.setup_time,
            self.engine_fps.clone(),
            self.delta_times.clone(),
        )
    }
}

/// Orchestrator node that manages sequential benchmark runs.
///
/// Since Rust/gdext does not have native `await`, this uses a state machine
/// in `_process` to sequence: pre-delay -> builtin run -> post-delay ->
/// spire run -> print results -> next amount -> quit.
#[derive(GodotClass)]
#[class(base = Node)]
pub struct BenchmarkOrchestrator {
    base: Base<Node>,

    test_name: String,
    setup_fn: SetupFn,
    duration: f64,
    amounts: Vec<i64>,

    /// Index into `amounts` for the current benchmark round.
    current_amount_idx: usize,

    /// Current orchestrator state.
    state: OrchestratorState,

    /// Stored results for the current amount.
    results_builtin: Option<HashMap<&'static str, Vec<f64>>>,
    results_spire: Option<HashMap<&'static str, Vec<f64>>>,

    /// Reference to the active runner node.
    active_runner: Option<Gd<BenchmarkRunner>>,
}

enum OrchestratorState {
    /// Idle/finished.
    Idle,
    /// Waiting before starting the builtin test case.
    PreDelayBuiltin { remaining: f64 },
    /// Running the builtin test case (runner is active).
    RunningBuiltin,
    /// Waiting before starting the spire test case.
    PreDelaySpire { remaining: f64 },
    /// Running the spire test case (runner is active).
    RunningSpire,
    /// Post-test delay before quitting.
    PostDelay { remaining: f64 },
}

#[godot_api]
impl INode for BenchmarkOrchestrator {
    fn init(base: Base<Node>) -> Self {
        Self {
            base,
            test_name: String::new(),
            setup_fn: dummy_setup,
            duration: STD_DURATION,
            amounts: STD_AMOUNTS.to_vec(),
            current_amount_idx: 0,
            state: OrchestratorState::Idle,
            results_builtin: None,
            results_spire: None,
            active_runner: None,
        }
    }

    fn process(&mut self, delta: f64) {
        match &mut self.state {
            OrchestratorState::Idle => {}

            OrchestratorState::PreDelayBuiltin { remaining } => {
                *remaining -= delta;
                if *remaining <= 0.0 {
                    self.spawn_runner(true);
                    self.state = OrchestratorState::RunningBuiltin;
                }
            }

            OrchestratorState::RunningBuiltin => {
                if let Some(runner) = &self.active_runner {
                    if runner.bind().is_finished() {
                        self.results_builtin = Some(runner.bind().take_results());
                        self.cleanup_runner();
                        self.state = OrchestratorState::PreDelaySpire { remaining: 2.0 };
                    }
                }
            }

            OrchestratorState::PreDelaySpire { remaining } => {
                *remaining -= delta;
                if *remaining <= 0.0 {
                    self.spawn_runner(false);
                    self.state = OrchestratorState::RunningSpire;
                }
            }

            OrchestratorState::RunningSpire => {
                if let Some(runner) = &self.active_runner {
                    if runner.bind().is_finished() {
                        self.results_spire = Some(runner.bind().take_results());
                        self.cleanup_runner();

                        // Print results for this amount
                        let amount = self.amounts[self.current_amount_idx];
                        let label = format!("{} | {} Nodes", self.test_name, amount);
                        if let (Some(builtin), Some(spire)) =
                            (&self.results_builtin, &self.results_spire)
                        {
                            util::print_results(&label, builtin, spire);
                        }

                        self.results_builtin = None;
                        self.results_spire = None;

                        // Move to the next amount or finish
                        self.current_amount_idx += 1;
                        if self.current_amount_idx < self.amounts.len() {
                            self.state =
                                OrchestratorState::PreDelayBuiltin { remaining: 2.0 };
                        } else {
                            self.state = OrchestratorState::PostDelay { remaining: 0.5 };
                        }
                    }
                }
            }

            OrchestratorState::PostDelay { remaining } => {
                *remaining -= delta;
                if *remaining <= 0.0 {
                    self.state = OrchestratorState::Idle;
                    self.base_mut()
                        .get_tree()
                        .quit();
                }
            }
        }
    }
}

impl BenchmarkOrchestrator {
    /// Configure and start the orchestrator.
    pub fn start(
        &mut self,
        test_name: String,
        setup_fn: SetupFn,
        duration: f64,
        amounts: Vec<i64>,
    ) {
        self.test_name = test_name;
        self.setup_fn = setup_fn;
        self.duration = duration;
        self.amounts = amounts;
        self.current_amount_idx = 0;
        self.state = OrchestratorState::PreDelayBuiltin { remaining: 2.0 };
    }

    fn spawn_runner(&mut self, is_builtin: bool) {
        let amount = self.amounts[self.current_amount_idx];
        let mut runner = BenchmarkRunner::new_alloc();
        {
            let mut bind = runner.bind_mut();
            bind.configure(self.setup_fn, is_builtin, self.duration, amount);
        }
        self.base_mut().add_child(&runner);
        self.active_runner = Some(runner);
    }

    fn cleanup_runner(&mut self) {
        if let Some(mut runner) = self.active_runner.take() {
            // Free all children that the setup function may have added to the
            // runner's parent (the orchestrator itself or the root).
            runner.queue_free();
        }
    }
}

fn dummy_setup(_root: &mut Gd<Node>, _is_builtin: bool, _duration: f64, _amount: i64) {}
