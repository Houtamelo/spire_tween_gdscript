//! Diagnostic test: empirically determines which `Callable` invocation patterns
//! actually propagate calls to underlying methods, across:
//!   - method kind (built-in C++ vs Rust `#[func]`)
//!   - argument count (0, 1, 2)
//!   - construction pattern (`Gd::callable`, `Callable::from_object_method`)
//!   - call pattern (direct, `bind(prefix).call(rest)`, `bind(all).call()`,
//!     `Callable::from_fn` closure)
//!
//! Each scenario:
//!   1. Resets the target state to a known value (zero / origin).
//!   2. Invokes the callable.
//!   3. Reads back via the strongly-typed Rust API to detect whether the call
//!      actually applied.
//!
//! Results are accumulated and printed as a markdown table at the end.

use godot::classes::{Node as NodeClass, Skeleton3D};
use godot::prelude::*;
use spire_tween::prelude::*;

use super::util::*;

/// Separate target class for `#[func]` callable probing.
///
/// Why a separate class: invoking a `Callable` that targets a `#[func]` method
/// requires gdext to `bind_mut()` the target's storage. If the target IS the
/// current test class, the test's own active borrow (or any concurrent
/// `process` tick) will trip "already bound; cannot borrow while accessible
/// mutable borrow exists". Using a child node sidesteps that entirely and
/// isolates the diagnostic to *Callable dispatch behavior*, not gdext borrow
/// semantics.
#[derive(GodotClass)]
#[class(init, base = Node)]
pub struct DiagFuncTarget {
    base: Base<NodeClass>,
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
#[class(init, base = Node2D)]
pub struct CallableDiagnosticTests {
    base: Base<Node2D>,
    #[init(node = "Skeleton3D")]
    skeleton: OnReady<Gd<Skeleton3D>>,
    #[init(val = RcPtr::new(TimeTracker::new()))]
    time_tracker: RcPtr<TimeTracker>,
}

#[godot_api]
impl INode2D for CallableDiagnosticTests {
    fn ready(&mut self) {
        self.base_mut().set_process_priority(-10);
        self.base_mut().set_physics_process_priority(-10);
    }

    fn process(&mut self, delta: f64) { self.time_tracker.timer += delta; }
}

impl ITestClass for CallableDiagnosticTests {
    const PREFAB_PATH: &'static str = "res://examples/tests/callable_diagnostic.tscn";

    fn test_list() -> Vec<fn(&mut Self) -> PinnedTestTask> {
        vec![Self::run_all_diagnostics]
    }

    fn time_tracker(&self) -> &RcPtr<TimeTracker> { &self.time_tracker }
}

#[derive(Clone, Copy, PartialEq)]
enum DiagResult {
    Worked,
    Failed,
}

impl DiagResult {
    fn as_str(self) -> &'static str {
        match self {
            DiagResult::Worked => "OK",
            DiagResult::Failed => "FAIL",
        }
    }
}

struct DiagRow {
    method: &'static str,
    args: u8,
    pattern: &'static str,
    constructor: &'static str,
    result: DiagResult,
    detail: String,
}

impl CallableDiagnosticTests {
    fn run_all_diagnostics(&mut self) -> PinnedTestTask {
        // Capture handles up-front. The async body runs after `&mut self` is
        // released by `run_test`, so callable invocations on these handles
        // don't conflict with any outer Rust borrow.
        let self_node2d: Gd<Node2D> = self.base().clone();
        let skeleton: Gd<Skeleton3D> = self.skeleton.clone();
        let bone_idx = skeleton.find_bone("diag_bone");
        assert!(bone_idx >= 0, "diag_bone must exist in callable_diagnostic.tscn");

        // Spawn a separate child node as the `#[func]` callable target — see
        // `DiagFuncTarget` doc-comment for the rationale.
        let mut func_target = DiagFuncTarget::new_alloc();
        self.base_mut().add_child(&func_target);
        func_target.set_name("DiagFuncTarget");

        Box::pin(async move {
            let mut rows: Vec<DiagRow> = Vec::new();
            let mut self_node2d = self_node2d;
            let mut skeleton = skeleton;

            // Helper: reset state for setter scenarios.
            let reset_pos = |n: &mut Gd<Node2D>| n.set_position(Vector2::ZERO);
            let reset_bone = |s: &mut Gd<Skeleton3D>, idx: i32| s.set_bone_pose_position(idx, Vector3::ZERO);
            let reset_func_target = |t: &mut Gd<DiagFuncTarget>| t.bind_mut().state = Vector2::ZERO;
            let read_func_target = |t: &Gd<DiagFuncTarget>| t.bind().state;

        // ============ Built-in 1-arg setter: Node2D::set_position(Vector2) ============
        let probe = Vector2::new(7.0, 8.0);

        // A. Gd::callable("name").call([arg])
        reset_pos(&mut self_node2d);
        self_node2d.callable("set_position").call(&[probe.to_variant()]);
        let res = self_node2d.get_position();
        rows.push(DiagRow {
            method: "Node2D::set_position",
            args: 1,
            pattern: "direct .call([arg])",
            constructor: "Gd::callable",
            result: if res == probe { DiagResult::Worked } else { DiagResult::Failed },
            detail: format!("after: {res}"),
        });

        // B. Gd::callable("name").bind([arg]).call([])
        reset_pos(&mut self_node2d);
        self_node2d
            .callable("set_position")
            .bind(&[probe.to_variant()])
            .call(&[]);
        let res = self_node2d.get_position();
        rows.push(DiagRow {
            method: "Node2D::set_position",
            args: 1,
            pattern: "bind(all) .call([])",
            constructor: "Gd::callable",
            result: if res == probe { DiagResult::Worked } else { DiagResult::Failed },
            detail: format!("after: {res}"),
        });

        // D. Callable::from_object_method(node, "name").call([arg])
        reset_pos(&mut self_node2d);
        Callable::from_object_method(&self_node2d, "set_position").call(&[probe.to_variant()]);
        let res = self_node2d.get_position();
        rows.push(DiagRow {
            method: "Node2D::set_position",
            args: 1,
            pattern: "direct .call([arg])",
            constructor: "Callable::from_object_method",
            result: if res == probe { DiagResult::Worked } else { DiagResult::Failed },
            detail: format!("after: {res}"),
        });

        // E. Callable::from_fn closure (control)
        reset_pos(&mut self_node2d);
        let n2d_clone = self_node2d.clone();
        let cb = Callable::from_fn("ctrl_set_pos", move |args| {
            let v: Vector2 = args[0].to();
            let mut n = n2d_clone.clone();
            n.set_position(v);
            Variant::nil()
        });
        cb.call(&[probe.to_variant()]);
        let res = self_node2d.get_position();
        rows.push(DiagRow {
            method: "Node2D::set_position",
            args: 1,
            pattern: "direct .call([arg])",
            constructor: "Callable::from_fn (closure)",
            result: if res == probe { DiagResult::Worked } else { DiagResult::Failed },
            detail: format!("after: {res}"),
        });

        // ============ Built-in 0-arg getter: Node2D::get_position() -> Vector2 ============
        // Set known value first, then verify the callable returns it.
        let known = Vector2::new(11.0, 22.0);
        self_node2d.set_position(known);

        // A. Gd::callable
        let v = self_node2d.callable("get_position").call(&[]);
        let extracted: Vector2 = v.try_to().unwrap_or_default();
        rows.push(DiagRow {
            method: "Node2D::get_position",
            args: 0,
            pattern: "direct .call([])",
            constructor: "Gd::callable",
            result: if extracted == known { DiagResult::Worked } else { DiagResult::Failed },
            detail: format!("got: {extracted}"),
        });

        // D. Callable::from_object_method
        let v = Callable::from_object_method(&self_node2d, "get_position").call(&[]);
        let extracted: Vector2 = v.try_to().unwrap_or_default();
        rows.push(DiagRow {
            method: "Node2D::get_position",
            args: 0,
            pattern: "direct .call([])",
            constructor: "Callable::from_object_method",
            result: if extracted == known { DiagResult::Worked } else { DiagResult::Failed },
            detail: format!("got: {extracted}"),
        });

        // ============ Built-in 2-arg setter: Skeleton3D::set_bone_pose_position(i32, Vector3) ============
        let probe3 = Vector3::new(1.0, 2.0, 3.0);

        // A. Direct call with all args
        reset_bone(&mut skeleton, bone_idx);
        skeleton
            .callable("set_bone_pose_position")
            .call(&[bone_idx.to_variant(), probe3.to_variant()]);
        let res = skeleton.get_bone_pose_position(bone_idx);
        rows.push(DiagRow {
            method: "Skeleton3D::set_bone_pose_position",
            args: 2,
            pattern: "direct .call([idx, vec])",
            constructor: "Gd::callable",
            result: if res == probe3 { DiagResult::Worked } else { DiagResult::Failed },
            detail: format!("after: {res}"),
        });

        // B. bind(idx).call([vec])
        reset_bone(&mut skeleton, bone_idx);
        skeleton
            .callable("set_bone_pose_position")
            .bind(&[bone_idx.to_variant()])
            .call(&[probe3.to_variant()]);
        let res = skeleton.get_bone_pose_position(bone_idx);
        rows.push(DiagRow {
            method: "Skeleton3D::set_bone_pose_position",
            args: 2,
            pattern: "bind([idx]) .call([vec])",
            constructor: "Gd::callable",
            result: if res == probe3 { DiagResult::Worked } else { DiagResult::Failed },
            detail: format!("after: {res}"),
        });

        // C. bind(idx, vec).call([])
        reset_bone(&mut skeleton, bone_idx);
        skeleton
            .callable("set_bone_pose_position")
            .bind(&[bone_idx.to_variant(), probe3.to_variant()])
            .call(&[]);
        let res = skeleton.get_bone_pose_position(bone_idx);
        rows.push(DiagRow {
            method: "Skeleton3D::set_bone_pose_position",
            args: 2,
            pattern: "bind([idx, vec]) .call([])",
            constructor: "Gd::callable",
            result: if res == probe3 { DiagResult::Worked } else { DiagResult::Failed },
            detail: format!("after: {res}"),
        });

        // D. from_object_method, direct call with both args
        reset_bone(&mut skeleton, bone_idx);
        Callable::from_object_method(&skeleton, "set_bone_pose_position")
            .call(&[bone_idx.to_variant(), probe3.to_variant()]);
        let res = skeleton.get_bone_pose_position(bone_idx);
        rows.push(DiagRow {
            method: "Skeleton3D::set_bone_pose_position",
            args: 2,
            pattern: "direct .call([idx, vec])",
            constructor: "Callable::from_object_method",
            result: if res == probe3 { DiagResult::Worked } else { DiagResult::Failed },
            detail: format!("after: {res}"),
        });

        // D-bind. from_object_method + bind prefix
        reset_bone(&mut skeleton, bone_idx);
        Callable::from_object_method(&skeleton, "set_bone_pose_position")
            .bind(&[bone_idx.to_variant()])
            .call(&[probe3.to_variant()]);
        let res = skeleton.get_bone_pose_position(bone_idx);
        rows.push(DiagRow {
            method: "Skeleton3D::set_bone_pose_position",
            args: 2,
            pattern: "bind([idx]) .call([vec])",
            constructor: "Callable::from_object_method",
            result: if res == probe3 { DiagResult::Worked } else { DiagResult::Failed },
            detail: format!("after: {res}"),
        });

        // E. from_fn closure (control)
        reset_bone(&mut skeleton, bone_idx);
        let mut sk_clone = skeleton.clone();
        let cb = Callable::from_fn("ctrl_set_bone", move |args| {
            let v: Vector3 = args[0].to();
            sk_clone.set_bone_pose_position(bone_idx, v);
            Variant::nil()
        });
        cb.call(&[probe3.to_variant()]);
        let res = skeleton.get_bone_pose_position(bone_idx);
        rows.push(DiagRow {
            method: "Skeleton3D::set_bone_pose_position",
            args: 2,
            pattern: "direct .call([vec]) (idx captured)",
            constructor: "Callable::from_fn (closure)",
            result: if res == probe3 { DiagResult::Worked } else { DiagResult::Failed },
            detail: format!("after: {res}"),
        });

        // ============ Built-in 1-arg getter: Skeleton3D::get_bone_pose_position(i32) -> Vector3 ============
        let known3 = Vector3::new(4.0, 5.0, 6.0);
        skeleton.set_bone_pose_position(bone_idx, known3);

        // A. Direct call with idx
        let v = skeleton
            .callable("get_bone_pose_position")
            .call(&[bone_idx.to_variant()]);
        let extracted: Vector3 = v.try_to().unwrap_or_default();
        rows.push(DiagRow {
            method: "Skeleton3D::get_bone_pose_position",
            args: 1,
            pattern: "direct .call([idx])",
            constructor: "Gd::callable",
            result: if extracted == known3 { DiagResult::Worked } else { DiagResult::Failed },
            detail: format!("got: {extracted}"),
        });

        // B. bind(idx).call([])
        let v = skeleton
            .callable("get_bone_pose_position")
            .bind(&[bone_idx.to_variant()])
            .call(&[]);
        let extracted: Vector3 = v.try_to().unwrap_or_default();
        rows.push(DiagRow {
            method: "Skeleton3D::get_bone_pose_position",
            args: 1,
            pattern: "bind([idx]) .call([])",
            constructor: "Gd::callable",
            result: if extracted == known3 { DiagResult::Worked } else { DiagResult::Failed },
            detail: format!("got: {extracted}"),
        });

        // ============ Custom Rust #[func] setter: DiagFuncTarget::set_pos(Vector2) ============
        // The target is a sibling child node, not the test class itself, so
        // gdext's `bind_mut()` during dispatch has no Rust-side borrow conflict.
        let mut func_target = func_target;
        let probe = Vector2::new(13.0, 14.0);

        // A. Gd::callable, direct
        reset_func_target(&mut func_target);
        func_target.callable("set_pos").call(&[probe.to_variant()]);
        let res = read_func_target(&func_target);
        rows.push(DiagRow {
            method: "#[func] set_pos",
            args: 1,
            pattern: "direct .call([arg])",
            constructor: "Gd::callable",
            result: if res == probe { DiagResult::Worked } else { DiagResult::Failed },
            detail: format!("after: {res}"),
        });

        // B. bind(arg).call([])
        reset_func_target(&mut func_target);
        func_target
            .callable("set_pos")
            .bind(&[probe.to_variant()])
            .call(&[]);
        let res = read_func_target(&func_target);
        rows.push(DiagRow {
            method: "#[func] set_pos",
            args: 1,
            pattern: "bind(all) .call([])",
            constructor: "Gd::callable",
            result: if res == probe { DiagResult::Worked } else { DiagResult::Failed },
            detail: format!("after: {res}"),
        });

        // D. from_object_method, direct
        reset_func_target(&mut func_target);
        Callable::from_object_method(&func_target, "set_pos").call(&[probe.to_variant()]);
        let res = read_func_target(&func_target);
        rows.push(DiagRow {
            method: "#[func] set_pos",
            args: 1,
            pattern: "direct .call([arg])",
            constructor: "Callable::from_object_method",
            result: if res == probe { DiagResult::Worked } else { DiagResult::Failed },
            detail: format!("after: {res}"),
        });

        // ============ Custom Rust #[func] getter: DiagFuncTarget::get_pos() -> Vector2 ============
        func_target.bind_mut().state = Vector2::new(99.0, 100.0);
        let known = read_func_target(&func_target);

        let v = func_target.callable("get_pos").call(&[]);
        let extracted: Vector2 = v.try_to().unwrap_or_default();
        rows.push(DiagRow {
            method: "#[func] get_pos",
            args: 0,
            pattern: "direct .call([])",
            constructor: "Gd::callable",
            result: if extracted == known { DiagResult::Worked } else { DiagResult::Failed },
            detail: format!("got: {extracted}"),
        });

        // ============ Print results table ============
        Self::print_results(&rows);
        })
    }

    fn print_results(rows: &[DiagRow]) {
        godot_print!("\n=== Callable invocation diagnostic results ===");
        godot_print!(
            "| {:<40} | {:<3} | {:<35} | {:<32} | {:<6} | detail",
            "method",
            "args",
            "call pattern",
            "constructor",
            "result"
        );
        godot_print!(
            "|{0:-<42}|{0:-<5}|{0:-<37}|{0:-<34}|{0:-<8}|{0:-<30}",
            ""
        );
        for r in rows {
            godot_print!(
                "| {:<40} | {:<3} | {:<35} | {:<32} | {:<6} | {}",
                r.method,
                r.args,
                r.pattern,
                r.constructor,
                r.result.as_str(),
                r.detail
            );
        }

        // Summary stats
        let total = rows.len();
        let ok = rows.iter().filter(|r| r.result == DiagResult::Worked).count();
        godot_print!("\nSummary: {ok}/{total} scenarios worked\n");
    }
}
