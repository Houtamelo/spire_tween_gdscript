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

use godot::classes::{INode, Node, Skeleton3D};
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
    method: &'static str,
    args: u8,
    pattern: &'static str,
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
        print_table(&rows);

        let total = rows.len();
        let ok = rows.iter().filter(|r| r.outcome == Outcome::Ok).count();
        godot_print!("\nSummary: {ok}/{total} scenarios worked");

        // Distinct exit code makes scripted runs easy to assert on.
        let exit_code = if ok == total { 0 } else { 1 };
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
        method: "Skeleton3D::set_bone_pose_position",
        args: 2,
        pattern: "direct .call([idx, vec])",
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
        method: "Skeleton3D::set_bone_pose_position",
        args: 2,
        pattern: "bind([idx]) .call([vec])",
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
        method: "Skeleton3D::set_bone_pose_position",
        args: 2,
        pattern: "bind([idx, vec]) .call([])",
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
        method: "Skeleton3D::set_bone_pose_position",
        args: 2,
        pattern: "direct .call([idx, vec])",
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
        method: "Skeleton3D::set_bone_pose_position",
        args: 2,
        pattern: "bind([idx]) .call([vec])",
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
        method: "Skeleton3D::set_bone_pose_position",
        args: 2,
        pattern: "direct .call([vec]) (idx captured)",
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
        method: "Skeleton3D::get_bone_pose_position",
        args: 1,
        pattern: "direct .call([idx])",
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
        method: "Skeleton3D::get_bone_pose_position",
        args: 1,
        pattern: "bind([idx]) .call([])",
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
        method: "#[func] set_pos",
        args: 1,
        pattern: "direct .call([arg])",
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
        method: "#[func] set_pos",
        args: 1,
        pattern: "bind(all) .call([])",
        constructor: "Gd::callable",
        outcome: outcome(res == probe),
        detail: format!("after: {res}"),
    });

    // C. from_object_method
    reset(&mut target);
    Callable::from_object_method(&target, "set_pos").call(&[probe.to_variant()]);
    let res = read(&target);
    rows.push(Row {
        method: "#[func] set_pos",
        args: 1,
        pattern: "direct .call([arg])",
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
        method: "#[func] get_pos",
        args: 0,
        pattern: "direct .call([])",
        constructor: "Gd::callable",
        outcome: outcome(got == known),
        detail: format!("got: {got}"),
    });
}

fn outcome(b: bool) -> Outcome { if b { Outcome::Ok } else { Outcome::Fail } }

fn print_table(rows: &[Row]) {
    godot_print!("\n=== Callable invocation diagnostic results ===");
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
