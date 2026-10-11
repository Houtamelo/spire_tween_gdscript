use std::{cell::Cell,rc::Rc};
use godot::{classes::Control,obj::bounds::DeclEngine,prelude::*};
use spire_tween::prelude::*;

#[derive(GodotClass)]
#[class(init,base=Node)]
struct ShakeReview {base: Base<Node>}
type Shake=SpireTween<LerpMethodData<f64>>;
#[derive(Default)]
struct Report {total:u32,failed:u32}
impl Report {
    fn check(&mut self,name:&str,pass:bool,detail:String) {
        self.total+=1;
        self.failed+=u32::from(!pass);
        godot_print!("SHAKE_REVIEW_{} name={name} {detail}",if pass{"PASS"}else{"FAIL"});
    }
}
fn make_node(node:&Gd<Node2D>,frequency:f64,duration:f64)->Shake {node.do_shake(20.0,20.0,1.0,frequency,duration)}
fn make_control(node:&Gd<Control>,frequency:f64,duration:f64)->Shake {node.do_shake(20.0,20.0,1.0,frequency,duration)}
fn inspect<T:GodotClass<Declarer=DeclEngine>+Inherits<Node>>(
    report:&mut Report,kind:&str,mut node:Gd<T>,get:fn(&T)->Vector2,set:fn(&mut T,Vector2),make:fn(&Gd<T>,f64,f64)->Shake,
) {
    let origin=Vector2::new(100.0,50.0);
    let steps:&[(&str,&[f64],i64,u8)]=&[
        ("exact",&[0.5,0.5],1,0),
        ("near",&[0.5,0.499995,0.01],1,0),
        ("two_loops",&[0.5,0.5,0.25,0.25,0.5],2,0),
        ("three_loops",&[0.5,0.5,0.25,0.25,0.5,0.5,0.5],3,0),
        ("force_playing",&[0.5],1,1),
        ("force_paused",&[0.5],1,2),
        ("external_movement",&[0.5,0.499995,0.01],1,3),
    ];
    for mode in [LoopMode::Restart,LoopMode::Yoyo,LoopMode::Incremental] {
        for &(case,deltas,loops,action) in steps {
            set(&mut node,origin);
            let mut tween=make(&node,30.0,1.0).with_loops(loops,mode);
            let movement=if action==3 {Vector2::new(7.0,-11.0)}else{Vector2::ZERO};
            for (i,delta) in deltas.iter().enumerate() {
                let _=tween.process(*delta,false);
                if i==0 && action==3 {let pos=get(&node);set(&mut node,pos+movement);}
            }
            if action==2 {tween.pause();}
            if action==1||action==2 {tween.force_complete();}
            let error=get(&node).distance_to(origin+movement);
            report.check(&format!("{kind}.{case}.{mode:?}"),tween.is_stopped()&&error<0.001,
                format!("error={error:.6} state={:?}",tween.get_state()));
            drop(tween);
        }
    }
    // Restoration on replay remains correct; its initial sample cadence is separate.
    set(&mut node,origin);
    let mut tween=make(&node,4.0,1.0);
    for delta in [0.25,0.25,0.25,0.25] {let _=tween.process(delta,false);}
    tween.play();
    for delta in [0.25,0.25,0.25,0.25] {let _=tween.process(delta,false);}
    let error=get(&node).distance_to(origin);
    report.check(&format!("{kind}.replay_restoration"),tween.is_stopped()&&error<0.001,format!("error={error:.6}"));
    drop(tween);

    for (case,mode,loops,replay) in [
        ("restart_sampling",LoopMode::Restart,2,false),
        ("replay_sampling",LoopMode::Restart,1,true),
        ("yoyo_sampling_control",LoopMode::Yoyo,2,false),
        ("incremental_sampling_control",LoopMode::Incremental,2,false),
    ] {
        set(&mut node,origin);
        let mut tween=make(&node,4.0,1.0).with_loops(loops,mode);
        for delta in [0.25,0.25,0.25,0.25] {let _=tween.process(delta,false);}
        if replay {tween.play();}
        let before=get(&node);
        let _=tween.process(0.01,false);
        let change=get(&node).distance_to(before);
        report.check(&format!("{kind}.{case}"),change<0.001,format!("elapsed_new_leg=0.01 sample_interval=0.25 position_change={change:.6}"));
        drop(tween);
    }
    set(&mut node,origin);
    let mut tween=make(&node,4.0,10.0);
    let _=tween.process(1.0,false);
    let before=get(&node);
    let _=tween.process(0.0,false);
    let change=get(&node).distance_to(before);
    report.check(&format!("{kind}.zero_delta_after_large_step"),change<0.001,
        format!("elapsed_new_tick=0 sample_interval=0.25 position_change={change:.6}"));
    drop(tween);

    for force in [false,true] {
        set(&mut node,origin);
        let ptr=TweenPtr::new(make(&node,1.0,1.0));
        let gd=LerpMethodData::<f64>::attach_gd_handle(&ptr);
        let godot_seen=Rc::new(Cell::new(None::<Vector2>));
        let native_seen=Rc::new(Cell::new(None::<Vector2>));
        let capture=Rc::clone(&godot_seen);
        let capture_node=node.clone();
        Signal::from_object_signal(&gd,"finished").connect(&Callable::from_fn("godot_observe",move |_| {
            capture.set(Some(get(&capture_node)));
            Variant::nil()
        }));
        let capture=Rc::clone(&native_seen);
        let capture_node=node.clone();
        ptr.to_mut().finished_connect(move||capture.set(Some(get(&capture_node))),SpireFlags::empty());
        if force {let _=ptr.to_mut().process(0.2,false);ptr.to_mut().force_complete();}
        else {let _=ptr.to_mut().process(1.0,false);}
        let observed=godot_seen.get().expect("Godot signal must fire").distance_to(origin);
        let native_error=native_seen.get().expect("native callback must fire").distance_to(origin);
        let final_error=get(&node).distance_to(origin);
        report.check(&format!("{kind}.godot_finished_restored.force={force}"),observed<0.001,
            format!("godot_observer_error={observed:.6} native_observer_error={native_error:.6} after_return_error={final_error:.6}"));
        assert!(native_error<0.001&&final_error<0.001,"native cleanup itself must work");
        drop(gd);
        drop(ptr);
    }
    set(&mut node,origin);
    let mut tween=make(&node,1.0,1.0);
    tween.finished_clear_connections();
    let _=tween.process(1.0,false);
    let error=get(&node).distance_to(origin);
    report.check(&format!("{kind}.clearing_user_connections_keeps_restoration"),error<0.001,format!("error={error:.6}"));
    drop(tween);
    node.upcast::<Node>().free();
}
#[godot_api]
impl INode for ShakeReview {
    fn ready(&mut self) {
        let mut report=Report::default();
        inspect(&mut report,"Node2D",Node2D::new_alloc(),Node2D::get_position,Node2D::set_position,make_node);
        inspect(&mut report,"Control",Control::new_alloc(),Control::get_position,Control::set_position,make_control);
        godot_print!("SHAKE_REVIEW_RESULT total={} failed={}",report.total,report.failed);
        self.base().get_tree().quit_ex().exit_code(if report.failed==0{0}else{1}).done();
    }
}
struct ReviewLibrary;
#[gdextension]
unsafe impl ExtensionLibrary for ReviewLibrary {}
