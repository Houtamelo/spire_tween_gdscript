//! Minimum reproducible project for a Callable dispatch bug observed with
//! gdext 0.5.2 / Godot 4.6.
//!
//! ## Hypothesis under test
//!
//! For built-in C++ methods registered via ClassDB that take **2 or more**
//! positional arguments, splitting the arguments between `Callable::bind()`
//! and `Callable::call()` causes the call to silently no-op: the method is
//! never invoked, no error is raised, and the bound callable returns nil.
//!
//! Specifically:
//!   - `callable.call([all_args])`              → works
//!   - `callable.bind([all_args]).call([])`     → works
//!   - `callable.bind([prefix]).call([rest])`   → SILENTLY NO-OPS (bug)
//!
//! Both `Gd::callable("name")` and `Callable::from_object_method(&gd, "name")`
//! exhibit the same failure on the split case.
//!
//! User-defined Rust `#[func]` methods do NOT exhibit this bug; they handle
//! split bind/call correctly.
//!
//! ## How the test runs
//!
//! `MrEntry::ready` is invoked by Godot once the scene loads. It probes
//! every relevant pattern, prints a markdown table to stdout, then quits.
//!
//! Run from the project directory:
//!     cargo build --release -p callable_bind_mre
//!     godot4 --path . main.tscn

use godot::classes::animation::TrackType;
use godot::classes::curve::TangentMode;
use godot::classes::image::Format;
use godot::classes::{Animation, Curve, INode, Image, Node, Skeleton3D};
use godot::prelude::*;

struct MreExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MreExtension {}

#[derive(Clone, Copy, PartialEq)]
enum Outcome {
    Ok,
    Fail,
}

impl Outcome {
    fn label(self) -> &'static str {
        match self {
            Outcome::Ok => "OK",
            Outcome::Fail => "FAIL",
        }
    }
}

struct Row {
    method: String,
    args: u8,
    pattern: String,
    constructor: &'static str,
    outcome: Outcome,
    detail: String,
}

#[derive(GodotClass)]
#[class(init, base = Node)]
struct DiagFuncTarget {
    base: Base<Node>,
    state: Vector2,
}

#[godot_api]
impl DiagFuncTarget {
    #[func]
    fn set_pos(&mut self, pos: Vector2) { self.state = pos; }

    #[func]
    fn get_pos(&self) -> Vector2 { self.state }
}

#[derive(GodotClass)]
#[class(init, base = Node)]
struct MrEntry {
    base: Base<Node>,
}

#[godot_api]
impl INode for MrEntry {
    fn ready(&mut self) {
        let mut tree = self.base().get_tree();

        let mut rows = Vec::new();
        run_all_probes(self.base().clone(), &mut rows);
        print_table("Original deep-dive: Skeleton3D 2-arg patterns + #[func] sanity", &rows);
        let (ok, total) = summary(&rows);
        godot_print!("\nSummary: {ok}/{total} scenarios worked\n");

        let mut arity_rows = Vec::new();
        run_arity_sweep(self.base().clone(), &mut arity_rows);
        print_table("Cross-arity sweep: 2/3/4/5 mandatory positional args", &arity_rows);
        let (a_ok, a_total) = summary(&arity_rows);
        godot_print!("\nSummary: {a_ok}/{a_total} scenarios worked");

        let exit_code = if ok == total && a_ok == a_total { 0 } else { 1 };
        tree.quit_ex().exit_code(exit_code).done();
    }
}

fn run_all_probes(parent: Gd<Node>, rows: &mut Vec<Row>) {
    let mut parent = parent;

    // -- Build-in target: a fresh Skeleton3D with one configured bone.
    let mut skeleton = Skeleton3D::new_alloc();
    parent.add_child(&skeleton);
    let bone_idx = skeleton.add_bone("diag_bone");
    skeleton.set_bone_rest(bone_idx, Transform3D::IDENTITY);
    skeleton.set_bone_enabled_ex(bone_idx).enabled(true).done();

    // -- Custom Rust target as a sibling child node.
    let func_target = DiagFuncTarget::new_alloc();
    parent.add_child(&func_target);

    probe_skeleton_set(rows, &mut skeleton, bone_idx);
    probe_skeleton_get(rows, &mut skeleton, bone_idx);
    probe_func_set(rows, func_target.clone());
    probe_func_get(rows, func_target);
}

fn probe_skeleton_set(rows: &mut Vec<Row>, skeleton: &mut Gd<Skeleton3D>, bone_idx: i32) {
    let probe = Vector3::new(1.0, 2.0, 3.0);
    let reset = |s: &mut Gd<Skeleton3D>| s.set_bone_pose_position(bone_idx, Vector3::ZERO);

    // A. Direct .call with both args  (expect: OK)
    reset(skeleton);
    skeleton
        .callable("set_bone_pose_position")
        .call(&[bone_idx.to_variant(), probe.to_variant()]);
    let after = skeleton.get_bone_pose_position(bone_idx);
    rows.push(Row {
        method: "Skeleton3D::set_bone_pose_position".to_string(),
        args: 2,
        pattern: "direct .call([idx, vec])".to_string(),
        constructor: "Gd::callable",
        outcome: outcome(after == probe),
        detail: format!("after: {after}"),
    });

    // B. .bind(idx).call([vec])  (expect: BUG → FAIL)
    reset(skeleton);
    skeleton
        .callable("set_bone_pose_position")
        .bind(&[bone_idx.to_variant()])
        .call(&[probe.to_variant()]);
    let after = skeleton.get_bone_pose_position(bone_idx);
    rows.push(Row {
        method: "Skeleton3D::set_bone_pose_position".to_string(),
        args: 2,
        pattern: "bind([idx]) .call([vec])".to_string(),
        constructor: "Gd::callable",
        outcome: outcome(after == probe),
        detail: format!("after: {after}"),
    });

    // C. .bind(idx, vec).call([])  (expect: OK)
    reset(skeleton);
    skeleton
        .callable("set_bone_pose_position")
        .bind(&[bone_idx.to_variant(), probe.to_variant()])
        .call(&[]);
    let after = skeleton.get_bone_pose_position(bone_idx);
    rows.push(Row {
        method: "Skeleton3D::set_bone_pose_position".to_string(),
        args: 2,
        pattern: "bind([idx, vec]) .call([])".to_string(),
        constructor: "Gd::callable",
        outcome: outcome(after == probe),
        detail: format!("after: {after}"),
    });

    // D. Callable::from_object_method, direct .call  (expect: OK)
    reset(skeleton);
    Callable::from_object_method(skeleton, "set_bone_pose_position")
        .call(&[bone_idx.to_variant(), probe.to_variant()]);
    let after = skeleton.get_bone_pose_position(bone_idx);
    rows.push(Row {
        method: "Skeleton3D::set_bone_pose_position".to_string(),
        args: 2,
        pattern: "direct .call([idx, vec])".to_string(),
        constructor: "Callable::from_object_method",
        outcome: outcome(after == probe),
        detail: format!("after: {after}"),
    });

    // E. Callable::from_object_method, .bind(idx).call([vec])  (expect: BUG → FAIL)
    reset(skeleton);
    Callable::from_object_method(skeleton, "set_bone_pose_position")
        .bind(&[bone_idx.to_variant()])
        .call(&[probe.to_variant()]);
    let after = skeleton.get_bone_pose_position(bone_idx);
    rows.push(Row {
        method: "Skeleton3D::set_bone_pose_position".to_string(),
        args: 2,
        pattern: "bind([idx]) .call([vec])".to_string(),
        constructor: "Callable::from_object_method",
        outcome: outcome(after == probe),
        detail: format!("after: {after}"),
    });

    // F. Closure control via Callable::from_fn  (expect: OK)
    reset(skeleton);
    let mut sk_clone = skeleton.clone();
    let closure = Callable::from_fn("ctrl_set_bone", move |args| {
        let v: Vector3 = args[0].to();
        sk_clone.set_bone_pose_position(bone_idx, v);
        Variant::nil()
    });
    closure.call(&[probe.to_variant()]);
    let after = skeleton.get_bone_pose_position(bone_idx);
    rows.push(Row {
        method: "Skeleton3D::set_bone_pose_position".to_string(),
        args: 2,
        pattern: "direct .call([vec]) (idx captured)".to_string(),
        constructor: "Callable::from_fn (closure)",
        outcome: outcome(after == probe),
        detail: format!("after: {after}"),
    });
}

fn probe_skeleton_get(rows: &mut Vec<Row>, skeleton: &mut Gd<Skeleton3D>, bone_idx: i32) {
    let known = Vector3::new(4.0, 5.0, 6.0);
    skeleton.set_bone_pose_position(bone_idx, known);

    // A. Direct .call([idx])
    let v = skeleton
        .callable("get_bone_pose_position")
        .call(&[bone_idx.to_variant()]);
    let got: Vector3 = v.try_to().unwrap_or_default();
    rows.push(Row {
        method: "Skeleton3D::get_bone_pose_position".to_string(),
        args: 1,
        pattern: "direct .call([idx])".to_string(),
        constructor: "Gd::callable",
        outcome: outcome(got == known),
        detail: format!("got: {got}"),
    });

    // B. .bind(idx).call([])  (works for 1-arg method, no split)
    let v = skeleton
        .callable("get_bone_pose_position")
        .bind(&[bone_idx.to_variant()])
        .call(&[]);
    let got: Vector3 = v.try_to().unwrap_or_default();
    rows.push(Row {
        method: "Skeleton3D::get_bone_pose_position".to_string(),
        args: 1,
        pattern: "bind([idx]) .call([])".to_string(),
        constructor: "Gd::callable",
        outcome: outcome(got == known),
        detail: format!("got: {got}"),
    });
}

fn probe_func_set(rows: &mut Vec<Row>, mut target: Gd<DiagFuncTarget>) {
    let probe = Vector2::new(13.0, 14.0);
    let reset = |t: &mut Gd<DiagFuncTarget>| t.bind_mut().state = Vector2::ZERO;
    let read = |t: &Gd<DiagFuncTarget>| t.bind().state;

    // A. Direct .call
    reset(&mut target);
    target.callable("set_pos").call(&[probe.to_variant()]);
    let res = read(&target);
    rows.push(Row {
        method: "#[func] set_pos".to_string(),
        args: 1,
        pattern: "direct .call([arg])".to_string(),
        constructor: "Gd::callable",
        outcome: outcome(res == probe),
        detail: format!("after: {res}"),
    });

    // B. .bind(arg).call([])
    reset(&mut target);
    target
        .callable("set_pos")
        .bind(&[probe.to_variant()])
        .call(&[]);
    let res = read(&target);
    rows.push(Row {
        method: "#[func] set_pos".to_string(),
        args: 1,
        pattern: "bind(all) .call([])".to_string(),
        constructor: "Gd::callable",
        outcome: outcome(res == probe),
        detail: format!("after: {res}"),
    });

    // C. from_object_method
    reset(&mut target);
    Callable::from_object_method(&target, "set_pos").call(&[probe.to_variant()]);
    let res = read(&target);
    rows.push(Row {
        method: "#[func] set_pos".to_string(),
        args: 1,
        pattern: "direct .call([arg])".to_string(),
        constructor: "Callable::from_object_method",
        outcome: outcome(res == probe),
        detail: format!("after: {res}"),
    });
}

fn probe_func_get(rows: &mut Vec<Row>, mut target: Gd<DiagFuncTarget>) {
    target.bind_mut().state = Vector2::new(99.0, 100.0);
    let known = target.bind().state;

    let v = target.callable("get_pos").call(&[]);
    let got: Vector2 = v.try_to().unwrap_or_default();
    rows.push(Row {
        method: "#[func] get_pos".to_string(),
        args: 0,
        pattern: "direct .call([])".to_string(),
        constructor: "Gd::callable",
        outcome: outcome(got == known),
        detail: format!("got: {got}"),
    });
}

fn outcome(b: bool) -> Outcome { if b { Outcome::Ok } else { Outcome::Fail } }

fn summary(rows: &[Row]) -> (usize, usize) {
    let total = rows.len();
    let ok = rows.iter().filter(|r| r.outcome == Outcome::Ok).count();
    (ok, total)
}

fn print_table(title: &str, rows: &[Row]) {
    godot_print!("\n=== {title} ===");
    godot_print!(
        "| {:<40} | {:<4} | {:<35} | {:<32} | {:<6} | detail",
        "method", "args", "call pattern", "constructor", "result"
    );
    godot_print!(
        "|{0:-<42}|{0:-<6}|{0:-<37}|{0:-<34}|{0:-<8}|{0:-<30}",
        ""
    );
    for r in rows {
        godot_print!(
            "| {:<40} | {:<4} | {:<35} | {:<32} | {:<6} | {}",
            r.method, r.args, r.pattern, r.constructor, r.outcome.label(), r.detail
        );
    }
}

// =====================================================================
// Cross-arity sweep: tests every bind/call split position for methods
// with 2/3/4/5 mandatory positional arguments, on different classes.
// =====================================================================

/// Describes one bind/call split: the first `bind_n` args go to `bind()`,
/// the remaining `call_n` go to `call()`. Returns a label like
/// `"bind([2 args]) .call([1 arg])"`.
fn split_label(bind_n: usize, call_n: usize) -> String {
    let bind_part = if bind_n == 0 {
        "(no bind)".to_string()
    } else {
        format!("bind([{bind_n} args])")
    };
    let call_part = format!(".call([{call_n} args])");
    format!("{bind_part} {call_part}")
}

/// Runs every bind/call split (K=0..=N) for the given method, using both
/// `Gd::callable` and `Callable::from_object_method` constructors. Pushes
/// one Row per (split, constructor) into `rows`. After each invocation,
/// `reset()` is called and `observe()` is checked against `expected`.
fn sweep_splits<T, F1, F2, F3, FObserve>(
    rows: &mut Vec<Row>,
    method_label: &str,
    method_name: &str,
    args: &[Variant],
    mut make_callable_a: F1,
    mut make_callable_b: F2,
    mut reset: F3,
    mut observe: FObserve,
    expected: &T,
)
where
    T: PartialEq + std::fmt::Debug,
    F1: FnMut() -> Callable,
    F2: FnMut() -> Callable,
    F3: FnMut(),
    FObserve: FnMut() -> T,
{
    let n = args.len();
    let arg_count = n as u8;
    for ctor_label in ["Gd::callable", "Callable::from_object_method"] {
        for k in 0..=n {
            reset();
            let cb = if ctor_label == "Gd::callable" {
                make_callable_a()
            } else {
                make_callable_b()
            };
            let cb = if k == 0 { cb } else { cb.bind(&args[..k]) };
            cb.call(&args[k..]);
            let after = observe();
            let ok = &after == expected;
            rows.push(Row {
                method: format!("{method_label}::{method_name}"),
                args: arg_count,
                pattern: split_label(k, n - k),
                constructor: if ctor_label == "Gd::callable" { "Gd::callable" } else { "Callable::from_object_method" },
                outcome: outcome(ok),
                detail: format!("after: {after:?}"),
            });
        }
    }
}

fn run_arity_sweep(parent: Gd<Node>, rows: &mut Vec<Row>) {
    let mut parent = parent;
    probe_arity_2(rows, &mut parent);
    probe_arity_3(rows);
    probe_arity_4(rows);
    probe_arity_5(rows);
}

// 2 args — Skeleton3D::set_bone_pose_position(bone_idx, position)
//   Node3D-derived. Observable via get_bone_pose_position.
fn probe_arity_2(rows: &mut Vec<Row>, parent: &mut Gd<Node>) {
    let mut sk = Skeleton3D::new_alloc();
    parent.add_child(&sk);
    let bone_idx = sk.add_bone("b0");
    sk.set_bone_rest(bone_idx, Transform3D::IDENTITY);
    sk.set_bone_enabled_ex(bone_idx).enabled(true).done();

    let probe_pos = Vector3::new(1.0, 2.0, 3.0);
    let args: Vec<Variant> = vec![bone_idx.to_variant(), probe_pos.to_variant()];

    let sk_a = sk.clone();
    let sk_b = sk.clone();
    let mut sk_reset = sk.clone();
    let sk_obs = sk.clone();

    sweep_splits(
        rows,
        "Skeleton3D",
        "set_bone_pose_position",
        &args,
        move || sk_a.callable("set_bone_pose_position"),
        move || Callable::from_object_method(&sk_b, "set_bone_pose_position"),
        move || sk_reset.set_bone_pose_position(bone_idx, Vector3::ZERO),
        move || sk_obs.get_bone_pose_position(bone_idx),
        &probe_pos,
    );
}

// 3 args — Image::set_pixel(x, y, color)
//   Resource. Observable via get_pixel(x, y).
//
// Note: RGBA8 storage truncates each channel to 8 bits, so the readback
// won't equal the original f32 probe color. We compute the expected value
// by doing a known-good round-trip first, then compare against that.
fn probe_arity_3(rows: &mut Vec<Row>) {
    let mut img = Image::create_empty(4, 4, false, Format::RGBA8)
        .expect("Image::create_empty");
    img.fill(Color::from_rgba(0.0, 0.0, 0.0, 0.0));

    let probe_x = 2i32;
    let probe_y = 1i32;
    let probe_input = Color::from_rgba(0.25, 0.5, 0.75, 1.0);

    // Round-trip once to capture the u8-quantized color we expect to read back.
    let mut img_calibrate = img.clone();
    img_calibrate.set_pixel(probe_x, probe_y, probe_input);
    let expected_color = img_calibrate.get_pixel(probe_x, probe_y);
    img_calibrate.set_pixel(probe_x, probe_y, Color::from_rgba(0.0, 0.0, 0.0, 0.0));

    let args: Vec<Variant> = vec![
        probe_x.to_variant(),
        probe_y.to_variant(),
        probe_input.to_variant(),
    ];

    let img_a = img.clone();
    let img_b = img.clone();
    let mut img_reset = img.clone();
    let img_obs = img.clone();

    sweep_splits(
        rows,
        "Image",
        "set_pixel",
        &args,
        move || img_a.callable("set_pixel"),
        move || Callable::from_object_method(&img_b, "set_pixel"),
        move || img_reset.set_pixel(probe_x, probe_y, Color::from_rgba(0.0, 0.0, 0.0, 0.0)),
        move || img_obs.get_pixel(probe_x, probe_y),
        &expected_color,
    );
}

// 4 args — Animation::track_insert_key(track_idx, time, key, transition)
//   Resource. Observable via track_get_key_value(track, key_idx).
fn probe_arity_4(rows: &mut Vec<Row>) {
    let mut anim = Animation::new_gd();
    let track_idx = anim.add_track(TrackType::VALUE);
    anim.track_set_path(track_idx, &NodePath::from("dummy"));

    let probe_time = 0.5_f64;
    let probe_value: f32 = 42.0;
    let probe_transition = 1.0_f32;
    let args: Vec<Variant> = vec![
        track_idx.to_variant(),
        probe_time.to_variant(),
        probe_value.to_variant(),
        probe_transition.to_variant(),
    ];

    let anim_a = anim.clone();
    let anim_b = anim.clone();
    let mut anim_reset = anim.clone();
    let anim_obs = anim.clone();

    sweep_splits(
        rows,
        "Animation",
        "track_insert_key",
        &args,
        move || anim_a.callable("track_insert_key"),
        move || Callable::from_object_method(&anim_b, "track_insert_key"),
        move || {
            // Wipe any keys inserted by previous probe iterations.
            while anim_reset.track_get_key_count(track_idx) > 0 {
                anim_reset.track_remove_key(track_idx, 0);
            }
        },
        move || {
            // After insert, the only key should be ours; read it back.
            if anim_obs.track_get_key_count(track_idx) == 0 {
                f32::NAN
            } else {
                anim_obs
                    .track_get_key_value(track_idx, 0)
                    .try_to::<f32>()
                    .unwrap_or(f32::NAN)
            }
        },
        &probe_value,
    );
}

// 5 args — Curve::add_point(position, left_tangent, right_tangent,
//                            left_mode, right_mode)
//   Resource. Observable via point_count + get_point_position(idx).
fn probe_arity_5(rows: &mut Vec<Row>) {
    let curve = Curve::new_gd();

    let probe_pos = Vector2::new(0.5, 0.7);
    let probe_lt: f32 = 1.5;
    let probe_rt: f32 = -1.5;
    let probe_lmode = TangentMode::FREE;
    let probe_rmode = TangentMode::LINEAR;
    let args: Vec<Variant> = vec![
        probe_pos.to_variant(),
        probe_lt.to_variant(),
        probe_rt.to_variant(),
        (probe_lmode.ord() as i32).to_variant(),
        (probe_rmode.ord() as i32).to_variant(),
    ];

    let curve_a = curve.clone();
    let curve_b = curve.clone();
    let mut curve_reset = curve.clone();
    let curve_obs = curve.clone();

    sweep_splits(
        rows,
        "Curve",
        "add_point",
        &args,
        move || curve_a.callable("add_point"),
        move || Callable::from_object_method(&curve_b, "add_point"),
        move || curve_reset.clear_points(),
        move || {
            // Read back the inserted point's position. If not inserted, NaN.
            if curve_obs.get_point_count() == 0 {
                Vector2::new(f32::NAN, f32::NAN)
            } else {
                curve_obs.get_point_position(0)
            }
        },
        &probe_pos,
    );
}
