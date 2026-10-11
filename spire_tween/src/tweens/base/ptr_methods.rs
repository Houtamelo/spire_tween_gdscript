use super::*;

impl<T: ITweenable> TweenPtr<T> {
    /// Binds this tween to the given node.
    /// Unlike the built-in [Tween], Spire tweens can be bound to any number of nodes.
    ///
    /// This influences the tween in several ways:
    /// - It will automatically be deleted when any of the bound nodes is freed.
    /// - If [enum Spire.PauseMode] is set to [constant Spire.PAUSE_MODE_BOUND],
    ///   the tween will only process when all bound nodes are also processing
    ///   (this is checked by calling [method Node.can_process] on each).
    ///
    /// Repeatedly binding the same node is harmless. Explicit bindings persist
    /// across unregister/register and sequence insertion.
    ///
    /// [b]Note:[/b] Property/Method tweeners also track their animated node's
    /// lifetime automatically; explicit binding does not change that behavior.
    pub fn bind_node(&mut self, obj: Gd<Node>)
    where WeakAnyTween: From<WeakTweenPtr<T>> {
        self.bound_nodes_mut().insert(obj.clone());
        TM.node_bind(obj, self.downgrade());
    }

    /// Unbinds this tween from the given node.
    /// See [method bind_node] for details on what binding to a node does.
    ///
    /// Calling this method with a node that isn't bound to this tween is harmless.
    pub fn unbind_node(&mut self, obj: Gd<Node>) {
        self.bound_nodes_mut().remove(&obj);
        TM.node_unbind(obj, self);
    }

    pub fn unregister(&mut self) { TM.tween_unregister(self); }

    pub fn re_register(&mut self)
    where AnyTween: From<Self> {
        let ptr = TweenPtr::clone(self);
        TM.tween_register(ptr);
    }

    pub fn is_registered(&self) -> bool { TM.tween_is_registered(self) }
}
