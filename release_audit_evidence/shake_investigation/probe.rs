use godot::{builtin::math::ApproxEq, classes::Control, prelude::*};
use spire_tween::prelude::*;

#[derive(GodotClass)]
#[class(init, base = Node)]
struct ShakeInvestigation { base: Base<Node> }

fn trace(tween: &mut SpireTween<LerpMethodData<f64>>, label: &'static str) {
    let original = tween.get_callable().clone();
    tween.t.callable = Callable::from_fn("trace_shake", move |args| {
        let time = args[0].to::<f64>();
        godot_print!("CALL {label} time={time:.12} approx_end={}", time.approx_eq(&1.0));
        original.call(&[args[0].clone()])
    });
}

fn observe(label: &str, node: &Gd<Node2D>, tween: &SpireTween<LerpMethodData<f64>>) {
    godot_print!("STATE {label} pos={} loop_time={:.12} state={:?} loops={}",
        node.get_position(), tween.get_animation_position(), tween.get_state(), tween.get_loops_finished());
}

#[godot_api]
impl INode for ShakeInvestigation {
    fn ready(&mut self) {
        for label in ["exact", "near", "several_near", "force_mid", "force_after_near", "restart_loop", "replay", "external_move"] {
            let mut node = Node2D::new_alloc();
            let mut tween = node.do_shake(20.0, 20.0, 0.5, 30.0, 1.0);
            if label == "restart_loop" { tween.set_loops(2, LoopMode::Restart); }
            trace(&mut tween, label);
            let _ = tween.process(0.5, false);
            let applied_offset = node.get_position();
            assert!((applied_offset.length() - 20.0).abs() < 0.001);
            observe(label, &node, &tween);
            let mut expected = Vector2::ZERO;
            match label {
                "exact" => { let _ = tween.process(0.5, false); }
                "near" => {
                    let _ = tween.process(0.499999999, false);
                    observe(label, &node, &tween);
                    assert!(node.get_position().length() < 0.001 && !tween.is_stopped());
                    let _ = tween.process(0.01, false);
                    expected = -applied_offset;
                }
                "several_near" => {
                    let _ = tween.process(0.499995, false);
                    observe(label, &node, &tween);
                    let _ = tween.process(0.000001, false);
                    observe(label, &node, &tween);
                    let _ = tween.process(0.01, false);
                    expected = -applied_offset * 2.0;
                }
                "force_mid" => tween.force_complete(),
                "force_after_near" => {
                    let _ = tween.process(0.499999999, false);
                    observe(label, &node, &tween);
                    tween.force_complete();
                    expected = -applied_offset;
                }
                "restart_loop" => {
                    let _ = tween.process(0.5, false);
                    observe(label, &node, &tween);
                    let _ = tween.process(0.5, false);
                    observe(label, &node, &tween);
                    let _ = tween.process(0.5, false);
                    expected = -applied_offset;
                }
                "replay" => {
                    let _ = tween.process(0.5, false);
                    observe(label, &node, &tween);
                    tween.play();
                    let _ = tween.process(0.5, false);
                    observe(label, &node, &tween);
                    let _ = tween.process(0.5, false);
                    expected = -applied_offset;
                }
                "external_move" => {
                    let _ = tween.process(0.499999999, false);
                    let movement = Vector2::new(100.0, 50.0);
                    let position = node.get_position();
                    node.set_position(position + movement);
                    let _ = tween.process(0.01, false);
                    expected = movement - applied_offset;
                }
                _ => unreachable!(),
            }
            observe(label, &node, &tween);
            let actual = node.get_position();
            assert!(actual.distance_to(expected) < 0.001, "{label}: actual={actual} expected={expected}");
            godot_print!("OBSERVATION {label} matches_traced_cause=true final_offset_length={:.6}", actual.length());
            drop(tween);
            node.free();
        }
        let node = Control::new_alloc();
        let mut tween = node.do_shake(20.0, 20.0, 0.5, 30.0, 1.0);
        trace(&mut tween, "control_near");
        let _ = tween.process(0.5, false);
        let offset = node.get_position();
        let _ = tween.process(0.499999999, false);
        assert!(node.get_position().length() < 0.001 && !tween.is_stopped());
        let _ = tween.process(0.01, false);
        assert!(node.get_position().distance_to(-offset) < 0.001);
        godot_print!("OBSERVATION control_near matches_traced_cause=true final_offset_length={:.6}", node.get_position().length());
        drop(tween);
        node.free();
        godot_print!("SHAKE_INVESTIGATION observations=9 mismatches=0");
        self.base().get_tree().quit();
    }
}
struct ProbeLibrary;
#[gdextension]
unsafe impl ExtensionLibrary for ProbeLibrary {}
