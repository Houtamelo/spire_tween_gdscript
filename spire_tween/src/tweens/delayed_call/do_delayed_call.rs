use super::*;

pub trait DoDelayedCall<Marker = ()> {
    fn do_delayed_call(&self, call: impl FnMut() + 'static, delay: f64) -> SpireTween<Callable>;
}

impl<T: Inherits<Object>> DoDelayedCall<()> for Gd<T> {
    fn do_delayed_call(
        &self,
        mut call: impl FnMut() + 'static,
        delay: f64,
    ) -> SpireTween<Callable> {
        SpireTween::<Callable>::new(
            Callable::from_local_fn("anonymous_closure", move |_| {
                call();
                Ok(Variant::nil())
            }),
            delay,
        )
        .maybe_bound(self.clone().upcast())
    }
}

impl<T: WithBaseField + Inherits<Object>> DoDelayedCall<BaseMarker> for T {
    fn do_delayed_call(
        &self,
        mut call: impl FnMut() + 'static,
        delay: f64,
    ) -> SpireTween<Callable> {
        SpireTween::<Callable>::new(
            Callable::from_local_fn("anonymous_closure", move |_| {
                call();
                Ok(Variant::nil())
            }),
            delay,
        )
        .maybe_bound(self.to_gd().upcast())
    }
}

pub trait DoDelayedCallable<Marker = ()> {
    fn do_delayed_callable(&self, callable: Callable, delay: f64) -> SpireTween<Callable>;
}

impl<T: Inherits<Object>> DoDelayedCallable<()> for Gd<T> {
    fn do_delayed_callable(&self, callable: Callable, delay: f64) -> SpireTween<Callable> {
        SpireTween::<Callable>::new(callable, delay).maybe_bound(self.clone().upcast()) // The callable might not belong to this object so we explicitly bind here.
    }
}

impl<T: WithBaseField + Inherits<Object>> DoDelayedCallable<BaseMarker> for T {
    fn do_delayed_callable(&self, callable: Callable, delay: f64) -> SpireTween<Callable> {
        SpireTween::<Callable>::new(callable, delay).maybe_bound(self.to_gd().upcast()) // The callable might not belong to this object so we explicitly bind here.
    }
}

#[allow(unused)]
#[cfg(test)]
mod must_compile {
    use godot::{classes::CanvasItem, prelude::*};

    use super::DoDelayedCall;
    use crate::internal_prelude::DoDelayedCallable;

    #[derive(GodotClass)]
    #[class(init, base = Node)]
    struct UserClass {
        base: Base<Node>,
    }

    #[godot_api]
    impl INode for UserClass {
        fn ready(&mut self) {
            self.do_delayed_call(|| (), 0.0).register();

            self.do_delayed_callable(Callable::from_local_fn("", |_| Ok(Variant::nil())), 1.0)
                .register();
        }
    }

    impl UserClass {
        fn self_gd(this: Gd<Self>) {
            this.do_delayed_call(|| (), 2.0).register();

            this.do_delayed_callable(Callable::from_local_fn("", |_| Ok(Variant::nil())), 1.0)
                .register();
        }
    }

    fn using_node(node: Gd<Node>) {
        node.do_delayed_call(|| godot_print!(""), 1.0).register();

        node.do_delayed_callable(Callable::from_local_fn("", |_| Ok(Variant::nil())), 1.0)
            .register();
    }

    fn using_node_ref(node: &Gd<Node2D>) {
        node.do_delayed_call(|| godot_print!(""), 1.0).register();

        node.do_delayed_callable(Callable::from_local_fn("", |_| Ok(Variant::nil())), 1.0)
            .register();
    }

    fn using_node_mut(node: &mut Gd<CanvasItem>) {
        node.do_delayed_call(|| godot_print!(""), 1.0).register();

        node.do_delayed_callable(Callable::from_local_fn("", |_| Ok(Variant::nil())), 1.0)
            .register();
    }
}
