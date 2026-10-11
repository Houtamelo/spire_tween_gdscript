//! Explicit binding regressions against real Godot nodes and manager ticks.
use std::panic::{AssertUnwindSafe, catch_unwind};

use godot::prelude::*;
use spire_tween::{prelude::*, unversioned_api::TM};

use super::{
    ownership::{CatchTestPanic, frames},
    util::PinnedTestTask,
};

type TestFn = fn(&mut NodeBindingTests) -> PinnedTestTask;
const CASES: &[(&str, TestFn)] = &[
    ("builder_bindings_and_clear", NodeBindingTests::builder_bindings_and_clear),
    ("clear_removes_manager_actions", NodeBindingTests::clear_removes_manager_actions),
    ("duplicate_bind_and_absent_unbind", NodeBindingTests::duplicate_bind_and_absent_unbind),
    ("unbind_preserves_remaining_node", NodeBindingTests::unbind_preserves_remaining_node),
    ("unregister_and_register_preserve_binding", NodeBindingTests::unregister_and_register_preserve_binding),
    ("completion_then_registration_replays", NodeBindingTests::completion_then_registration_replays),
    ("bound_children_append_join_insert", NodeBindingTests::bound_children_append_join_insert),
    ("clear_unregistered_binding", NodeBindingTests::clear_unregistered_binding),
    ("clear_freed_node_binding", NodeBindingTests::clear_freed_node_binding),
    ("automatic_property_and_method_owner", NodeBindingTests::automatic_property_and_method_owner),
    ("kill_then_register_restores_binding", NodeBindingTests::kill_then_register_restores_binding),
];

#[derive(GodotClass)]
#[class(init, base = Node)]
pub struct NodeBindingTests {
    base: Base<Node>,
}

fn sequence(duration: f64) -> TweenSequence {
    let mut tween = TweenSequence::new();
    tween.append_interval(duration);
    tween
}

impl NodeBindingTests {
    fn target(&mut self) -> Gd<Node> {
        let target = Node::new_alloc();
        self.base_mut().add_child(&target);
        target
    }

    fn clear_removes_manager_actions(&mut self) -> PinnedTestTask {
        let mut target = self.target();
        let calls = RcPtr::new(0usize);
        let counter = calls.clone();
        let mut cleared = sequence(10.0)
            .bound_to(target.clone())
            .on_finish(move || *counter.to_mut() += 1)
            .register();
        let bound = sequence(10.0).bound_to(target.clone()).register();
        cleared.to_mut().clear_bound_nodes();
        cleared.clear_bound_nodes();
        target.complete_bound_tweens();
        assert!(cleared.is_playing(), "cleared tween must not be force-completed by the node");
        assert!(bound.is_stopped(), "other tween remains in the manager binding index");
        target.kill_bound_tweens();
        assert!(cleared.is_registered(), "cleared tween must not be killed by the node");
        assert!(!bound.is_registered());
        target.queue_free();
        Box::pin(async move {
            frames(4).await;
            assert_eq!(*calls, 0);
            assert!(cleared.is_registered() && cleared.is_playing());
        })
    }

    fn duplicate_bind_and_absent_unbind(&mut self) -> PinnedTestTask {
        let mut target = self.target();
        let calls = RcPtr::new(0usize);
        let counter = calls.clone();
        let mut bound = sequence(10.0).on_finish(move || *counter.to_mut() += 1).register();
        let mut unrelated = sequence(10.0).register();
        bound.bind_node(target.clone());
        bound.bind_node(target.clone());
        bound.bind_node(target.clone());
        assert_eq!(bound.get_bound_nodes().len(), 1);
        unrelated.unbind_node(target.clone());
        target.complete_bound_tweens();
        assert!(unrelated.is_playing());
        bound.play();
        bound.unbind_node(target.clone());
        bound.unbind_node(target.clone());
        target.queue_free();
        Box::pin(async move {
            frames(4).await;
            assert_eq!(*calls, 1, "duplicate binds must not duplicate completion callbacks");
            assert!(bound.is_registered() && bound.is_playing());
            assert!(unrelated.is_registered() && unrelated.is_playing());
        })
    }

    fn unbind_preserves_remaining_node(&mut self) -> PinnedTestTask {
        let mut first = self.target();
        let mut second = self.target();
        let mut tween = sequence(10.0)
            .bound_to(first.clone())
            .bound_to(second.clone())
            .register();
        tween.unbind_node(first.clone());
        first.queue_free();
        Box::pin(async move {
            frames(4).await;
            assert!(tween.is_registered() && tween.is_playing());
            assert_eq!(tween.get_bound_nodes().len(), 1);
            second.queue_free();
            frames(4).await;
            assert!(!tween.is_registered() && tween.is_stopped());
        })
    }

    fn unregister_and_register_preserve_binding(&mut self) -> PinnedTestTask {
        let mut target = self.target();
        let mut tween = sequence(10.0).bound_to(target.clone()).register();
        tween.unregister();
        tween.re_register();
        tween.re_register();
        Box::pin(async move {
            frames(4).await;
            assert!(tween.is_registered());
            tween.unregister();
            frames(4).await;
            assert!(!tween.is_registered());
            tween.re_register();
            assert!(tween.is_registered());
            assert_eq!(tween.get_bound_nodes().len(), 1);
            target.queue_free();
            frames(4).await;
            assert!(!tween.is_registered() && tween.is_stopped());
        })
    }

    fn completion_then_registration_replays(&mut self) -> PinnedTestTask {
        let mut target = self.target();
        let calls = RcPtr::new(0usize);
        let counter = calls.clone();
        let mut tween = sequence(0.01)
            .bound_to(target.clone())
            .on_finish(move || *counter.to_mut() += 1)
            .register();
        Box::pin(async move {
            frames(4).await;
            assert_eq!(*calls, 1);
            tween.re_register();
            tween.play();
            frames(4).await;
            assert_eq!(*calls, 2);
            target.queue_free();
            frames(4).await;
            assert!(!tween.is_registered());
        })
    }

    fn bound_children_append_join_insert(&mut self) -> PinnedTestTask {
        let mut target = self.target();
        let children: Vec<_> = (0..3)
            .map(|_| sequence(0.01).bound_to(target.clone()).register())
            .collect();
        let mut parent = TweenSequence::new();
        parent.append_ptr(children[0].clone());
        parent.join_ptr(children[1].clone());
        parent.append_interval(0.03);
        parent.insert_ptr(0.0, children[2].clone());
        for child in &children {
            assert!(!child.is_registered());
        }
        let mut parent = parent.register();
        Box::pin(async move {
            frames(4).await;
            for child in &children {
                assert!(child.is_stopped());
            }
            parent.unregister();
            frames(4).await;
            parent.re_register();
            parent.play();
            frames(4).await;
            assert!(parent.is_registered() && parent.is_stopped());
            for child in &children {
                assert!(child.is_stopped() && !child.is_registered());
            }
            target.queue_free();
            frames(4).await;
            for child in &children {
                assert!(child.is_stopped());
            }
        })
    }

    fn clear_unregistered_binding(&mut self) -> PinnedTestTask {
        let mut target = self.target();
        let mut tween = sequence(10.0).bound_to(target.clone()).register();
        tween.unregister();
        Box::pin(async move {
            frames(4).await;
            tween.clear_bound_nodes();
            target.complete_bound_tweens();
            assert!(tween.is_playing(), "unregistered explicit bindings must also be removed");
            tween.re_register();
            target.queue_free();
            frames(4).await;
            assert!(tween.is_registered() && tween.is_playing());
        })
    }

    fn clear_freed_node_binding(&mut self) -> PinnedTestTask {
        let mut target = self.target();
        let mut tween = sequence(10.0).bound_to(target.clone()).register();
        target.queue_free();
        Box::pin(async move {
            frames(4).await;
            assert!(!tween.is_registered() && tween.is_stopped());
            tween.to_mut().clear_bound_nodes();
            tween.re_register();
            tween.play();
            frames(4).await;
            assert!(tween.is_registered() && tween.is_playing());
        })
    }

    fn builder_bindings_and_clear(&mut self) -> PinnedTestTask {
        let mut target = self.target();
        let mut value = sequence(10.0)
            .bound_to(target.clone())
            .maybe_bound(target.clone().upcast());
        assert_eq!(value.get_bound_nodes().len(), 1);
        let was_initialized = std::sync::LazyLock::get(&TM).is_some();
        value.clear_bound_nodes();
        assert_eq!(
            std::sync::LazyLock::get(&TM).is_some(),
            was_initialized,
            "clearing a builder must not initialize the manager"
        );
        let mut cleared = value.register();
        cleared.re_register();
        target.queue_free();
        Box::pin(async move {
            frames(4).await;
            assert!(cleared.is_registered() && cleared.is_playing());
        })
    }

    fn automatic_property_and_method_owner(&mut self) -> PinnedTestTask {
        let mut owner = Node2D::new_alloc();
        self.base_mut().add_child(&owner);
        let mut explicit = self.target();
        let mut property = owner
            .do_position(Vector2::ONE, 10.0)
            .bound_to(explicit.clone())
            .register();
        let mut method = owner
            .do_method("set_position", Vector2::ZERO, Vector2::ONE, 10.0)
            .bound_to(explicit.clone())
            .register();
        property.bind_node(owner.clone().upcast());
        property.unbind_node(owner.clone().upcast());
        property.clear_bound_nodes();
        method.clear_bound_nodes();
        explicit.queue_free();
        Box::pin(async move {
            frames(4).await;
            assert!(property.is_registered() && property.is_playing());
            assert!(method.is_registered() && method.is_playing());
            owner.queue_free();
            frames(4).await;
            assert!(!property.is_registered());
            assert!(!method.is_registered());
        })
    }

    fn kill_then_register_restores_binding(&mut self) -> PinnedTestTask {
        let mut target = self.target();
        let mut tween = sequence(10.0).bound_to(target.clone()).register();
        target.kill_bound_tweens();
        assert!(!tween.is_registered() && tween.is_stopped());
        Box::pin(async move {
            frames(4).await;
            tween.re_register();
            tween.play();
            target.kill_bound_tweens();
            assert!(!tween.is_registered() && tween.is_stopped());
            assert_eq!(tween.get_bound_nodes().len(), 1);
            target.queue_free();
            frames(4).await;
        })
    }
}

pub(super) async fn run_node_binding_tests(parent: &mut Gd<Node>) -> usize {
    let mut failures = 0;
    for &(name, test) in CASES {
        let mut node = NodeBindingTests::new_alloc();
        parent.add_child(&node);
        godot_print!("NATIVE_BINDING_START {name}");
        let task = catch_unwind(AssertUnwindSafe(|| test(&mut node.bind_mut())));
        let result = match task {
            Ok(task) => CatchTestPanic(task).await,
            Err(error) => Err(error),
        };
        if result.is_ok() {
            godot_print!("NATIVE_BINDING_PASS {name}");
        } else {
            failures += 1;
            godot_print!("NATIVE_BINDING_FAIL {name}");
        }
        node.queue_free();
        frames(4).await;
    }
    godot_print!("NATIVE_BINDING_RESULT total={} failed={failures}", CASES.len());
    failures
}

#[derive(GodotClass)]
#[class(init, base = Node)]
pub struct NodeBindingTestRunner {
    base: Base<Node>,
}

#[godot_api]
impl INode for NodeBindingTestRunner {
    fn ready(&mut self) {
        let mut parent = self.base().clone();
        godot::task::spawn(async move {
            let failed = run_node_binding_tests(&mut parent).await;
            parent
                .get_tree()
                .quit_ex()
                .exit_code(if failed == 0 { 0 } else { 1 })
                .done();
        });
    }
}
