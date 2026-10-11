//! Exercise API-dependent timer and benchmark calls inside a real SceneTree.
use std::panic::{AssertUnwindSafe, catch_unwind};

use godot::{classes::Sprite2D, prelude::*};

use super::{
    ownership::{CatchTestPanic, frames},
    util::{PinnedTestTask, TimeTracker},
};
use crate::benchmarks::tween_setups::{Tweeners, tween_global_positions, tween_modulates};

type TestFn = fn(&mut Gd<Node>) -> PinnedTestTask;
const CASES: &[(&str, TestFn)] = &[
    ("timer", timer),
    ("builtin_position", builtin_position),
    ("builtin_color", builtin_color),
    ("started_position_tween_is_rejected", started_position_tween_is_rejected),
    ("started_color_tween_is_rejected", started_color_tween_is_rejected),
];

fn timer(_: &mut Gd<Node>) -> PinnedTestTask {
    Box::pin(async {
        let tracker = TimeTracker::new();
        tracker.wait_seconds(0.03).await;
    })
}

fn sprite(parent: &mut Gd<Node>) -> Gd<Sprite2D> {
    let node = Sprite2D::new_alloc();
    parent.add_child(&node);
    node
}

fn builtin_position(parent: &mut Gd<Node>) -> PinnedTestTask {
    let node = sprite(parent);
    let mut tweeners = Tweeners::Builtin(Vec::new());
    let to = Vector2::new(30.0, 40.0);
    tween_global_positions(
        0.03,
        true,
        &[node.clone().upcast()],
        &mut tweeners,
        false,
        Some(vec![Vector2::new(10.0, 20.0)]),
        Some(vec![to]),
    );
    let Tweeners::Builtin(values) = &mut tweeners else { unreachable!() };
    values[0].custom_step(0.015);
    assert!(node.get_global_position().distance_to(Vector2::new(20.0, 30.0)) < 0.0001);
    Box::pin(async move {
        frames(20).await;
        assert_eq!(node.get_global_position(), to);
        drop(tweeners);
    })
}

fn builtin_color(parent: &mut Gd<Node>) -> PinnedTestTask {
    let node = sprite(parent);
    let mut tweeners = Tweeners::Builtin(Vec::new());
    let to = Color::from_rgba(0.1, 0.2, 0.3, 0.4);
    tween_modulates(
        0.03,
        true,
        &[node.clone().upcast()],
        &mut tweeners,
        false,
        Some(vec![Color::from_rgba(0.9, 0.8, 0.7, 0.6)]),
        Some(vec![to]),
    );
    let Tweeners::Builtin(values) = &mut tweeners else { unreachable!() };
    values[0].custom_step(0.015);
    let midpoint = node.get_modulate();
    for component in [midpoint.r, midpoint.g, midpoint.b, midpoint.a] {
        assert!((component - 0.5).abs() < 0.0001);
    }
    Box::pin(async move {
        frames(20).await;
        assert_eq!(node.get_modulate(), to);
        drop(tweeners);
    })
}

fn started_position_tween_is_rejected(parent: &mut Gd<Node>) -> PinnedTestTask {
    started_tween_is_rejected(parent, false)
}

fn started_color_tween_is_rejected(parent: &mut Gd<Node>) -> PinnedTestTask { started_tween_is_rejected(parent, true) }

fn started_tween_is_rejected(parent: &mut Gd<Node>, color: bool) -> PinnedTestTask {
    let node = sprite(parent);
    let mut tweeners = Tweeners::Builtin(Vec::new());
    let schedule = |tweeners: &mut Tweeners| {
        if color {
            tween_modulates(
                10.0,
                true,
                &[node.clone().upcast()],
                tweeners,
                false,
                Some(vec![Color::WHITE]),
                Some(vec![Color::BLACK]),
            );
        } else {
            tween_global_positions(
                10.0,
                true,
                &[node.clone().upcast()],
                tweeners,
                false,
                Some(vec![Vector2::ZERO]),
                Some(vec![Vector2::ONE]),
            );
        }
    };
    schedule(&mut tweeners);
    let Tweeners::Builtin(values) = &mut tweeners else { unreachable!() };
    values[0].custom_step(0.01);
    // Godot refuses to append to a started Tween. The nullable 4.4 binding
    // returns None, and the guarded expect must fail instead of using it.
    godot_print!("EXPECTED_API_FAILURE_BEGIN");
    let result = catch_unwind(AssertUnwindSafe(|| schedule(&mut tweeners)));
    godot_print!("EXPECTED_API_FAILURE_END");
    assert!(result.is_err(), "a refused built-in tweener must not be accepted");
    let Tweeners::Builtin(values) = &mut tweeners else { unreachable!() };
    values[0].kill();
    Box::pin(async move {
        drop(tweeners);
    })
}

pub(super) async fn run_api_compatibility_tests(parent: &mut Gd<Node>) -> usize {
    let mut failures = 0;
    for &(name, test) in CASES {
        let mut fixture = Node::new_alloc();
        parent.add_child(&fixture);
        godot_print!("NATIVE_API_START {name}");
        let task = catch_unwind(AssertUnwindSafe(|| test(&mut fixture)));
        let result = match task {
            Ok(task) => CatchTestPanic(task).await,
            Err(error) => Err(error),
        };
        if result.is_ok() {
            godot_print!("NATIVE_API_PASS {name}");
        } else {
            failures += 1;
            godot_print!("NATIVE_API_FAIL {name}");
        }
        fixture.queue_free();
        frames(4).await;
    }
    godot_print!("NATIVE_API_RESULT total={} failed={failures}", CASES.len());
    failures
}

#[derive(GodotClass)]
#[class(init, base = Node)]
pub struct ApiCompatibilityTestRunner {
    base: Base<Node>,
}

#[godot_api]
impl INode for ApiCompatibilityTestRunner {
    fn ready(&mut self) {
        let mut parent = self.base().clone();
        godot::task::spawn(async move {
            let failed = run_api_compatibility_tests(&mut parent).await;
            parent
                .get_tree()
                .quit_ex()
                .exit_code(if failed == 0 { 0 } else { 1 })
                .done();
        });
    }
}
