use godot::prelude::*;
use spire_tween::prelude::*;
use std::{cell::Cell,rc::Rc};

struct ReviewExtension;
#[gdextension]
unsafe impl ExtensionLibrary for ReviewExtension {}

#[derive(GodotClass)]
#[class(init,base=Node)]
struct SequenceReview {
    base:Base<Node>,
    failed:i32,
    total:i32,
    ticks:usize,
    pending_bound:Option<(TweenSequence,Rc<Cell<usize>>,TweenPtr<Callable>)>,
}

fn sequence(duration:f64)->TweenSequence {
    let mut value=TweenSequence::new();value.append_interval(duration);value
}
fn child(duration:f64)->TweenPtr<Sequence> {TweenPtr::new(sequence(duration))}
fn close(a:f64,b:f64)->bool {(a-b).abs()<1e-8}

impl SequenceReview {
    fn check(&mut self,name:&str,passed:bool) {
        self.total+=1;
        if !passed {self.failed+=1;}
        godot_print!("CHECK {} {}",if passed {"PASS"} else {"FAIL"},name);
    }
}

#[godot_api]
impl INode for SequenceReview {
    fn ready(&mut self) {
        // Lazy block activation and one-time materialization across replay.
        let calls=Rc::new(Cell::new(0));let counter=calls.clone();
        let mut parent=TweenSequence::new();parent.append_interval(1.0);
        parent.append_ctor(move || {counter.set(counter.get()+1);child(10.0).into()});
        let _=parent.process(0.1,false);self.check("queue_ctor_waits_for_block",calls.get()==0);
        let _=parent.process(1.0,false);self.check("queue_ctor_constructs_once",calls.get()==1);
        parent.force_complete();parent.play();let _=parent.process(2.0,false);
        self.check("replay_reuses_constructed_child",calls.get()==1);

        // A future insert survives an empty queue and starts at its offset.
        let calls=Rc::new(Cell::new(0));let counter=calls.clone();
        let mut parent=TweenSequence::new();
        parent.insert_ctor(1.0,move || {counter.set(counter.get()+1);child(10.0).into()});
        let _=parent.process(0.1,false);
        self.check("future_ctor_keeps_empty_parent_playing",parent.is_playing()&&calls.get()==0);
        let _=parent.process(1.0,false);
        self.check("future_ctor_runs_at_offset",calls.get()==1&&parent.is_playing());

        // A long insert keeps a completed queue alive.
        let inserted=child(10.0);let returned=inserted.clone();
        let mut parent=sequence(0.01);parent.insert_ctor(0.0,move || returned.into());
        let _=parent.process(0.1,false);
        self.check("long_insert_outlives_queue",parent.is_playing()&&inserted.is_playing());

        // Delta correction and clipping when the first update crosses the offset.
        let inserted=child(10.0);let returned=inserted.clone();
        let mut parent=sequence(100.0);parent.insert_ctor(0.0,move || returned.into());
        let _=parent.process(0.1,false);let _=parent.process(0.1,false);
        godot_print!("OBS two_steps parent={} child={}",parent.loop_time,inserted.loop_time);
        self.check("insert_advances_by_delta",close(inserted.loop_time,0.2));
        let inserted=child(10.0);let returned=inserted.clone();
        let mut parent=sequence(100.0);parent.insert_ctor(0.25,move || returned.into());
        let _=parent.process(0.3,false);
        self.check("first_insert_delta_clipped",close(inserted.loop_time,0.05));

        // Child adoption: registration, default easing, duplicate rejection.
        let mut inserted=child(10.0);inserted.to_mut().set_process_mode(ProcessMode::Manual);inserted.re_register();
        let returned=inserted.clone();let mut parent=TweenSequence::new();parent.set_default_ease(Ease::InSine);
        parent.append_ctor(move || returned.into());let _=parent.process(0.1,false);
        self.check("rust_ctor_unregisters_child",!inserted.is_registered());
        self.check("rust_ctor_inherits_ease",inserted.t.default_ease==Some(Ease::InSine));
        let inserted=child(10.0);let returned=inserted.clone();let mut parent=TweenSequence::new();
        parent.append_ptr(inserted.clone());parent.join_ctor(move || returned.into());let _=parent.process(0.1,false);
        self.check("duplicate_child_rejected",parent.iter_inner_tweens_non_recursive().count()==1&&close(inserted.loop_time,0.1));

        // Prevent recursion if a broken implementation accepts a self-returning ctor.
        let parent=TweenPtr::new(sequence(10.0));parent.to_mut().stop();let returned=parent.clone();
        parent.to_mut().insert_ctor(0.0,move ||returned.into());
        let _=parent.to_mut().process(0.1,false);let accepted=parent.to_mut().remove(&parent);
        self.check("self_cycle_rejected",!accepted);

        // A valid callable factory follows the same adoption path.
        let (mut inserted,handle)=sequence(10.0).with_process_mode(ProcessMode::Manual).register_with_gd_handle();
        let mut parent=TweenSequence::new();parent.append_ctor_callable(Callable::from_fn("ctor",move |_|handle.to_variant()));
        let _=parent.process(0.1,false);
        self.check("callable_ctor_materializes_and_unregisters",close(inserted.loop_time,0.1)&&!inserted.is_registered());inserted.unregister();

        // Forced completion also adopts and finishes the constructed child.
        for inserted_mode in [false,true] {
            let mut inserted=child(10.0);inserted.to_mut().set_process_mode(ProcessMode::Manual);inserted.re_register();
            let returned=inserted.clone();let mut parent=TweenSequence::new();parent.set_default_ease(Ease::InSine);
            if inserted_mode {parent.insert_ctor(5.0,move ||returned.into());} else {parent.append_ctor(move ||returned.into());}
            parent.force_complete();
            self.check(if inserted_mode {"force_insert_ctor"} else {"force_block_ctor"},!inserted.is_registered()&&inserted.is_stopped()&&inserted.t.default_ease==Some(Ease::InSine));
        }

        // Invalid factory results must not skip the next valid parallel child.
        let calls=Rc::new(Cell::new(0));let counter=calls.clone();let mut parent=TweenSequence::new();
        parent.append_ctor_callable(Callable::from_fn("bad_ctor", |_|Variant::nil()));
        parent.join_ctor(move || {counter.set(counter.get()+1);child(10.0).into()});let _=parent.process(0.1,false);
        self.check("failure_then_valid_ctor",calls.get()==1&&parent.iter_inner_tweens_non_recursive().count()==1);

        // Insert completion must contribute correct excess time to its parent.
        let inserted=child(0.25);let mut parent=TweenSequence::new();parent.insert_ctor(0.25,move ||inserted.into());
        let result=parent.process(0.6,false);
        godot_print!("OBS insert_completion result={:?} loop_time={}",result,parent.loop_time);
        self.check("insert_completion_keeps_duration",close(parent.loop_time,0.5));
        let inserted=child(0.25);let mut inner=TweenSequence::new();inner.insert_ctor(0.25,move ||inserted.into());
        let tail=child(1.0);let mut outer=TweenSequence::new();outer.append(inner);outer.append_ptr(tail.clone());
        let _=outer.process(0.6,false);godot_print!("OBS nested_insert_successor tail_elapsed={}",tail.loop_time);
        self.check("nested_insert_passes_correct_excess",close(tail.loop_time,0.1));

        // Restart loops must carry only overshoot, not the whole frame.
        let calls=Rc::new(Cell::new(0));let counter=calls.clone();let mut parent=TweenSequence::new().with_loops(2,LoopMode::Restart);
        parent.insert_fn(0.5,move ||counter.set(counter.get()+1));let _=parent.process(0.6,false);
        godot_print!("OBS insert_loop_first calls={} loop_time={}",calls.get(),parent.loop_time);
        self.check("insert_loop_carries_correct_excess",close(parent.loop_time,0.1));
        let _=parent.process(0.01,false);godot_print!("OBS insert_loop_next calls={} elapsed={}",calls.get(),parent.total_elapsed_time);
        self.check("insert_callback_not_repeated_early",calls.get()==1&&parent.is_playing());

        // A queue does not mask the need to account for callable insertion time.
        let calls=Rc::new(Cell::new(0));let counter=calls.clone();
        let mut parent=sequence(0.3);parent.insert_fn(0.5,move ||counter.set(counter.get()+1));
        let result=parent.process(0.6,false);
        godot_print!("OBS inserted_callback_with_queue result={:?} loop_time={}",result,parent.loop_time);
        self.check("inserted_callback_contributes_excess",calls.get()==1&&close(parent.loop_time,0.5));

        // Godot callable callbacks must contribute the same excess as Rust callbacks.
        let calls=Rc::new(Cell::new(0));let counter=calls.clone();
        let mut parent=sequence(0.3);
        parent.insert_call(0.5,Callable::from_fn("inserted_callback",move |_|{counter.set(counter.get()+1);Variant::nil()}));
        let result=parent.process(0.6,false);
        godot_print!("OBS inserted_callable_with_queue result={:?} loop_time={}",result,parent.loop_time);
        self.check("inserted_callable_contributes_excess",calls.get()==1&&close(parent.loop_time,0.5));

        // A subsequent full step reaches the second callback at its proper loop time.
        let calls=Rc::new(Cell::new(0));let counter=calls.clone();
        let mut parent=TweenSequence::new().with_loops(2,LoopMode::Restart);
        parent.insert_fn(0.5,move ||counter.set(counter.get()+1));
        let _=parent.process(0.6,false);let _=parent.process(0.01,false);let _=parent.process(0.41,false);
        self.check("insert_restart_finishes_at_second_offset",calls.get()==2&&parent.is_stopped()&&close(parent.loop_time,0.5));

        // Interleaved removals preserve order and survivors after SmallVec spills.
        for forced in [false,true] {
            let calls=Rc::new(Cell::new(0));let mut parent=TweenSequence::new();
            let expected=18usize;
            for _ in 0..expected {
                parent.join_ctor_callable(Callable::from_fn("bad_bulk_ctor", |_|Variant::nil()));
                let counter=calls.clone();
                parent.join_ctor(move || {counter.set(counter.get()+1);child(10.0).into()});
            }
            if forced {parent.force_complete();} else {let _=parent.process(0.1,false);}
            self.check(if forced {"bulk_force_pruning_after_spill"} else {"bulk_pruning_after_spill"},
                calls.get()==expected&&parent.iter_inner_tweens_non_recursive().count()==expected);
        }

        // Stop means skip for the current loop, including future inserted children.
        let inserted=child(10.0);let mut parent=TweenSequence::new();parent.insert_ptr(10.0,inserted.clone());inserted.to_mut().stop();
        let result=parent.process(0.1,false);godot_print!("OBS stopped_future_insert result={:?}",result);
        self.check("stopped_future_insert_skipped",parent.is_stopped());

        // Explicit bindings and actual queued deletion of a constructor-created child.
        let mut target=Node::new_alloc();self.base_mut().add_child(&target);
        let calls=Rc::new(Cell::new(0));let counter=calls.clone();
        let inserted=TweenPtr::new(TweenDelayedCall::new(Callable::from_fn("bound_complete",move |_|{counter.set(counter.get()+1);Variant::nil()}),10.0).bound_to(target.clone()));
        let returned=inserted.clone();let mut parent=TweenSequence::new();parent.append_ctor(move ||returned.into());let _=parent.process(0.1,false);
        target.complete_bound_tweens();self.check("bound_complete_reaches_ctor_child",calls.get()==1);target.free();
        let mut target=Node::new_alloc();self.base_mut().add_child(&target);
        let calls=Rc::new(Cell::new(0));let counter=calls.clone();
        let inserted=TweenPtr::new(TweenDelayedCall::new(Callable::from_fn("bound_delete",move |_|{counter.set(counter.get()+1);Variant::nil()}),10.0).bound_to(target.clone()));
        let returned=inserted.clone();let mut parent=TweenSequence::new();parent.append_ctor(move ||returned.into());let _=parent.process(0.1,false);
        target.queue_free();self.pending_bound=Some((parent,calls,inserted));
    }
    fn process(&mut self,_delta:f64) {
        self.ticks+=1;if self.ticks<3{return;}
        let (mut parent,calls,inserted)=self.pending_bound.take().unwrap();let _=parent.process(11.0,false);
        self.check("freed_bound_target_cancels_ctor_child",calls.get()==0&&inserted.is_stopped());
        godot_print!("REVIEW_RESULT total={} failed={}",self.total,self.failed);
        self.base().get_tree().quit_ex().exit_code(self.failed).done();
    }
}
