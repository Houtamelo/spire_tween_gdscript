use std::{cell::RefCell, rc::Rc};
use godot::{classes::Control, obj::bounds::DeclEngine, prelude::*};
use spire_tween::prelude::*;

#[derive(GodotClass)]
#[class(init, base=Node)]
struct ShakeLoopModeProbe { base: Base<Node> }

type Shake = SpireTween<LerpMethodData<f64>>;

fn run<T: GodotClass<Declarer=DeclEngine> + Inherits<Node>>(
    kind: &str, case: &str, mode: LoopMode, loops: i64,
    steps: &[f64], force: bool,
    node: Gd<T>, get_pos: fn(&T)->Vector2, make_shake: fn(&Gd<T>)->Shake,
) {
    let mut tween=make_shake(&node).with_loops(loops,mode);
    let calls=Rc::new(RefCell::new(Vec::<f64>::new()));
    let recorded=Rc::clone(&calls);
    let original=tween.get_callable().clone();
    tween.t.callable=Callable::from_fn("trace_shake",move |args| {
        recorded.borrow_mut().push(args[0].to::<f64>());
        original.call(&[args[0].clone()])
    });
    let mut last=Vector2::ZERO;
    let mut forward_offset=Vector2::ZERO;
    let mut second_loop_changes=0;
    let mut second_loop_calls=Vec::new();
    godot_print!("START kind={kind} case={case} mode={mode:?} loops={loops} force={force}");
    for (i,delta) in steps.iter().enumerate() {
        let loop_before=tween.get_loops_finished();
        let _=tween.process(*delta,false);
        let pos=get_pos(&node);
        if i==0 { forward_offset=pos; }
        if loop_before==1 {
            second_loop_calls.push(*calls.borrow().last().unwrap());
            if pos.distance_to(last)>0.001 { second_loop_changes+=1; }
        }
        godot_print!("STEP index={i} callback={:.12} native_time={:.12} completed_loops={} state={:?} pos={pos}",
            calls.borrow().last().unwrap(),tween.get_animation_position(),tween.get_loops_finished(),tween.get_state());
        last=pos;
    }
    if force {
        tween.force_complete();
        godot_print!("FORCE callback={:.12} completed_loops={} state={:?} pos={}",
            calls.borrow().last().unwrap(),tween.get_loops_finished(),tween.get_state(),get_pos(&node));
    }
    assert!(tween.is_stopped(),"probe must reach a completed state");
    let final_pos=get_pos(&node);
    let final_len=final_pos.length();
    godot_print!("RESULT kind={kind} case={case} mode={mode:?} final_length={final_len:.6} restored={} second_loop_changes={second_loop_changes} second_loop_callbacks={second_loop_calls:?} initial_offset={forward_offset}", final_len<0.001);
    // Observations are printed for baseline/candidate comparison; cancellation,
    // object ownership and timestep math use the real Godot implementation.
    drop(tween);
    node.upcast::<Node>().free();
}

#[godot_api]
impl INode for ShakeLoopModeProbe {
    fn ready(&mut self) {
        let cases: &[(&str,i64,&[f64],bool)]=&[
            ("one_loop_control",1,&[0.5,0.5],false),
            ("two_loops_half_ticks",2,&[0.5,0.5,0.5,0.5],false),
            ("two_loops_reverse_activity",2,&[0.5,0.5,0.25,0.25,0.25,0.25],false),
            ("two_loops_three_ticks",2,&[0.5,0.5,0.25,0.25,0.5],false),
            ("near_first_endpoint",2,&[0.5,0.499999999,0.01,0.25,0.25,0.5],false),
            ("three_loops",3,&[0.5,0.5,0.25,0.25,0.5,0.5,0.5],false),
            ("force_during_second",2,&[0.5,0.5,0.25],true),
        ];
        let mut count=0;
        for mode in [LoopMode::Yoyo,LoopMode::Incremental] {
            for &(case,loops,steps,force) in cases {
                run("Node2D",case,mode,loops,steps,force,Node2D::new_alloc(),Node2D::get_position,
                    |node|node.do_shake(20.0,20.0,1.0,30.0,1.0));
                run("Control",case,mode,loops,steps,force,Control::new_alloc(),Control::get_position,
                    |node|node.do_shake(20.0,20.0,1.0,30.0,1.0));
                count+=2;
            }
        }
        godot_print!("SHAKE_LOOP_MODES_COMPLETE observations={count}");
        self.base().get_tree().quit();
    }
}
struct ProbeLibrary;
#[gdextension]
unsafe impl ExtensionLibrary for ProbeLibrary {}
