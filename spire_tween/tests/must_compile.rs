#![allow(unused)]
extern crate godot;
extern crate spire_tween;

use godot::prelude::*;
use spire_tween::prelude::*;

macro_rules! new_tween_method {
    () => {
        TweenMethod::<f64>::new(Callable::from_fn("", |_| {}), 0.0, 5.0, 2.0)
    };
}

macro_rules! new_tween_property {
    () => {
        TweenProperty::<i64>::new(
            PropertyDataInt::Custom(PropertyDataCustom {
                path:  NodePath::from(""),
                owner: Node::new_alloc().into(),
            }),
            Evaluator::Static(5),
            5.0,
        )
    };
}

macro_rules! new_delayed_call {
    () => {
        TweenDelayedCall::new(new_callable!(), 5.0)
    };
}

macro_rules! new_tween_ptr {
    () => {{
        let tween = new_tween_property!();
        tween.register()
    }};
}

macro_rules! new_callable {
    () => {
        Callable::from_fn("", |_| {})
    };
}

macro_rules! full_builder_api {
    ($Tween:expr) => {
        ($Tween)
            .bound_to(Node::new_alloc())
            .maybe_bound(Object::new_alloc())
            .with_delay(5.0)
            .with_speed_scale(2.0)
            .with_pause_mode(PauseMode::Stop)
            .with_process_mode(ProcessMode::Manual)
    };
}

macro_rules! full_base_api {
    ($Tween:expr) => {
        let _: TweenState = $Tween.get_state();
        $Tween.set_state(TweenState::Paused);
        let _: bool = $Tween.is_playing();
        let _: bool = $Tween.is_paused();
        let _: bool = $Tween.is_stopped();
        $Tween.play();
        $Tween.pause();
        $Tween.stop();
        $Tween.force_complete();
        $Tween.get_bound_nodes();
        $Tween.clear_bound_nodes();
        let _: f64 = $Tween.get_delay();
        $Tween.set_delay(5.0);
        let _: f64 = $Tween.get_speed_scale();
        $Tween.set_speed_scale(2.0);
        let _: bool = $Tween.get_ignore_time_scale();
        $Tween.set_ignore_time_scale(true);
        let _: PauseMode = $Tween.get_pause_mode();
        $Tween.set_pause_mode(PauseMode::Stop);
        let _: ProcessMode = $Tween.get_process_mode();
        $Tween.set_process_mode(ProcessMode::Manual);
        let _: f64 = $Tween.get_animation_position();
        let _: f64 = $Tween.get_total_elapsed_time();
        let _: i64 = $Tween.get_loops_finished();
        let _: i64 = $Tween.get_loops();
        $Tween.set_loops(5, LoopMode::Restart);
        let _: LoopMode = $Tween.get_loop_mode();
        $Tween.set_loop_mode(LoopMode::Restart);
    };
}

macro_rules! full_ptr_api {
    ($Tween:expr) => {
        $Tween.bind_node(Node::new_alloc());
        $Tween.unbind_node(Node::new_alloc());
        let _: bool = $Tween.is_registered();
        $Tween.unregister();
        $Tween.re_register();
    };
}

fn compile() {
    {
        let ease = Ease::get_global_default();
        Ease::set_global_default(ease);
    }

    {
        let mut seq = TweenSequence::new();
        seq.append_fn(|| {});
        seq.append_call(new_callable!());
        seq.append_interval(5.0);
        seq.append(new_tween_method!());
        seq.append_ptr(new_tween_ptr!());

        seq.join_fn(|| {});
        seq.join_call(new_callable!());
        seq.join_interval(5.0);
        seq.join(new_tween_method!());
        seq.join_ptr(new_tween_ptr!());

        seq.insert_fn(2.0, || {});
        seq.insert_call(3.0, new_callable!());
        seq.insert(1.0, new_tween_property!());
        seq.insert_ptr(5.0, new_tween_ptr!());

        full_base_api!(seq);
    }

    {
        use spire_tween::prelude::{
            TweenDelayedCall,
            TweenMethod,
            TweenMethodPtr,
            TweenProperty,
            TweenPropertyPtr,
            TweenSequence,
            TweenSequencePtr,
        };
    }

    {
        let property: TweenProperty<i64> = TweenProperty::<i64>::new_typed("", Node::new_alloc(), 5, 2.0);
        let registered: TweenPropertyPtr<i64> = property.register();
    }

    {
        let property = new_tween_property!();
        let mut property = full_builder_api!(property);
        full_base_api!(property);
        let mut property = property.register();
        full_base_api!(property);
        full_ptr_api!(property);
    }

    {
        let method = new_tween_method!();
        let mut method = full_builder_api!(method);
        full_base_api!(method);
        let mut method = method.register();
        full_base_api!(method);
        full_ptr_api!(method);
    }

    {
        let delayed_call = new_delayed_call!();
        let mut delayed_call = full_builder_api!(delayed_call);
        full_base_api!(delayed_call);
        let mut delayed_call = delayed_call.register();
        full_base_api!(delayed_call);
        full_ptr_api!(delayed_call);
    }

    {
        let sequence = TweenSequence::new();
        let mut sequence = full_builder_api!(sequence);
        full_base_api!(sequence);
        let mut sequence = sequence.register();
        full_base_api!(sequence);
        full_ptr_api!(sequence);
    }

    {
        let mut node2d = Node2D::new_alloc();
        let tween = node2d
            .do_spiral(
                Vector2::ZERO,
                0.,
                std::f32::consts::PI,
                Vector2::ONE,
                5.0,
                0.0,
                0.0,
                Spiral::Archimedean,
                Vector2::ONE,
            )
            .register();
    }

    {
        let mut node = Node2D::new_alloc();
        node.complete_bound_tweens();
        node.kill_bound_tweens();
    }
}

fn unregister_rc_ptr<T: ITweenable>(mut tween: RcPtr<SpireTween<T>>) { tween.unregister(); }

fn unregister_delegated_method(mut tween: AnyMethodTween) { tween.unregister(); }
fn unregister_delegated_property(mut tween: AnyPropertyTween) { tween.unregister(); }
fn unregister_delegated_any(mut tween: AnyTween) { tween.unregister(); }

fn unregister_delayed_call(mut tween: RcPtr<SpireTween<Callable>>) { tween.unregister(); }
fn unregister_sequence(mut tween: RcPtr<SpireTween<Sequence>>) { tween.unregister(); }
