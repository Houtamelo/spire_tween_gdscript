//! Exercise API-dependent timers, benchmark calls and Blueprint warnings inside a real SceneTree.
use std::panic::{AssertUnwindSafe, catch_unwind};

use godot::{
    classes::{Area2D, Area3D, Control, Sprite2D, control::MouseFilter},
    prelude::*,
};
use spire_tween::prelude::{BlueprintKind, LerpModeKind, TriggerCondition, TweenBlueprint};

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
    ("blueprint_configuration_warnings", blueprint_configuration_warnings),
];

fn blueprint_configuration_warnings(parent: &mut Gd<Node>) -> PinnedTestTask {
    let target = Sprite2D::new_alloc();
    parent.add_child(&target);
    let mut blueprint = TweenBlueprint::new_alloc();
    blueprint.set("kind", &BlueprintKind::Property.to_variant());
    blueprint.set("target", &target.to_variant());
    blueprint.set("property_to_animate", &StringName::from("position").to_variant());
    blueprint.set("property_end_value", &Vector2::ONE.to_variant());
    parent.add_child(&blueprint);

    for mode in [LerpModeKind::Absolute, LerpModeKind::Relative, LerpModeKind::SpeedBased] {
        blueprint.set("property_lerp_mode", &mode.to_variant());
        for duration in [0.0_f64, 1.0, -1.0, f64::NAN, f64::INFINITY] {
            blueprint.set("property_speed_or_duration", &duration.to_variant());
            let warnings = INode::get_configuration_warnings(&*blueprint.bind());
            let valid = duration.is_finite() && duration > 0.0;
            assert_eq!(warnings.is_empty(), valid, "mode={mode:?}, duration={duration}, warnings={warnings}");
            if !valid {
                assert_warning_fields(&blueprint, &["property_speed_or_duration"]);
            }
        }
    }

    blueprint.set("property_speed_or_duration", &1.0.to_variant());
    blueprint.set("property_lerp_mode", &LerpModeKind::Absolute.to_variant());
    for (field, valid) in [("delay", 0.0), ("speed_scale", 1.0)] {
        blueprint.set(field, &(-1.0).to_variant());
        assert_warning_fields(&blueprint, &[field]);
        blueprint.set(field, &valid.to_variant());
        assert!(INode::get_configuration_warnings(&*blueprint.bind()).is_empty());
    }

    blueprint.set("property_start_enabled", &true.to_variant());
    blueprint.set("property_start_value", &Vector2::ZERO.to_variant());
    for field in ["property_start_value", "property_end_value"] {
        blueprint.set(field, &"incompatible".to_variant());
        assert_warning_fields(&blueprint, &[field, "property_to_animate"]);
        blueprint.set(field, &Vector2::ONE.to_variant());
        assert!(INode::get_configuration_warnings(&*blueprint.bind()).is_empty());
    }
    for (property, fields) in [
        ("", &["property_to_animate"][..]),
        ("missing_property", &["property_to_animate", "target"][..]),
        ("visible", &["property_to_animate"][..]),
    ] {
        blueprint.set("property_to_animate", &StringName::from(property).to_variant());
        assert_warning_fields(&blueprint, fields);
    }
    blueprint.set("property_to_animate", &StringName::from("position").to_variant());

    let mut control = Control::new_alloc();
    parent.add_child(&control);
    blueprint.set("trigger_condition", &TriggerCondition::MouseEnter.to_variant());
    blueprint.set("trigger_source", &control.to_variant());
    control.set_mouse_filter(MouseFilter::IGNORE);
    assert_warning_fields(&blueprint, &["trigger_source", "mouse_filter"]);
    control.set_mouse_filter(MouseFilter::STOP);
    assert!(INode::get_configuration_warnings(&*blueprint.bind()).is_empty());

    let mut area_2d = Area2D::new_alloc();
    parent.add_child(&area_2d);
    blueprint.set("trigger_source", &area_2d.to_variant());
    area_2d.set_pickable(false);
    area_2d.set_collision_layer(0);
    assert_warning_fields(&blueprint, &["trigger_source", "input_pickable", "collision_layer"]);
    area_2d.set_pickable(true);
    area_2d.set_collision_layer(1);
    assert!(INode::get_configuration_warnings(&*blueprint.bind()).is_empty());

    let mut area_3d = Area3D::new_alloc();
    parent.add_child(&area_3d);
    blueprint.set("trigger_source", &area_3d.to_variant());
    area_3d.set_ray_pickable(false);
    area_3d.set_collision_layer(0);
    assert_warning_fields(&blueprint, &["trigger_source", "input_ray_pickable", "collision_layer"]);
    area_3d.set_ray_pickable(true);
    area_3d.set_collision_layer(1);
    assert!(INode::get_configuration_warnings(&*blueprint.bind()).is_empty());
    blueprint.set("trigger_source", &target.to_variant());
    assert_warning_fields(&blueprint, &["trigger_source"]);

    let mut detached = TweenBlueprint::new_alloc();
    detached.set("kind", &BlueprintKind::Property.to_variant());
    assert_warning_fields(&detached, &["target"]);
    detached.set("kind", &BlueprintKind::ChainBlock.to_variant());
    detached.set("trigger_condition", &TriggerCondition::MouseEnter.to_variant());
    assert_warning_fields(&detached, &["trigger_source", "target"]);
    detached.free();
    blueprint.free();
    control.free();
    area_2d.free();
    area_3d.free();
    target.free();
    Box::pin(async {})
}

fn assert_warning_fields(blueprint: &Gd<TweenBlueprint>, fields: &[&str]) {
    let warnings = INode::get_configuration_warnings(&*blueprint.bind());
    assert!(
        warnings.to_vec().iter().any(|warning| {
            let text = warning.to_string();
            fields.iter().all(|field| text.contains(&format!("`{field}`")))
        }),
        "Expected a warning naming exported fields {fields:?}, got {warnings}"
    );
}

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
