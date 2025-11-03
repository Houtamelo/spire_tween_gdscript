use super::*;

pub(super) enum DeferredOp {
    Add(AnyTween),
    Remove,
}

/*
impl PartialEq for DeferredOp {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        let a_address = match self {
            DeferredOp::Add(tween) => tween.address(),
            DeferredOp::Remove(addr) => *addr,
        };

        let b_address = match other {
            DeferredOp::Add(tween) => tween.address(),
            DeferredOp::Remove(addr) => *addr,
        };

        std::ptr::addr_eq(a_address, b_address)
    }
}

impl Eq for DeferredOp {}

impl Hash for DeferredOp {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            DeferredOp::Add(tween) => tween.hash(state),
            DeferredOp::Remove(tween) => tween.hash(state),
        }
    }
}

impl<T> Equivalent<RcPtr<T>> for DeferredOp {
    #[inline]
    fn equivalent(&self, tween: &RcPtr<T>) -> bool {
        match self {
            DeferredOp::Add(to_add) => to_add == tween,
            DeferredOp::Remove(addr_to_rm) => std::ptr::addr_eq(*addr_to_rm, tween.address()),
        }
    }
}

impl<T> Equivalent<DeferredOp> for RcPtr<T> {
    #[inline]
    fn equivalent(&self, op: &DeferredOp) -> bool {
        <DeferredOp as Equivalent<RcPtr<T>>>::equivalent(op, self)
    }
}

impl<T> Equivalent<WeakPtr<T>> for DeferredOp {
    #[inline]
    fn equivalent(&self, weak: &WeakPtr<T>) -> bool {
        match self {
            DeferredOp::Add(to_add) => to_add == weak,
            DeferredOp::Remove(addr_to_rm) => std::ptr::addr_eq(*addr_to_rm, weak.address()),
        }
    }
}

impl<T> Equivalent<DeferredOp> for WeakPtr<T> {
    #[inline]
    fn equivalent(&self, op: &DeferredOp) -> bool {
        <DeferredOp as Equivalent<WeakPtr<T>>>::equivalent(op, self)
    }
}

impl Equivalent<*const ()> for DeferredOp {
    #[inline]
    fn equivalent(&self, addr: &*const ()) -> bool {
        match self {
            DeferredOp::Add(tween) => std::ptr::addr_eq(tween.address(), *addr),
            DeferredOp::Remove(addr_to_rm) => std::ptr::addr_eq(*addr_to_rm, *addr),
        }
    }
}

impl Equivalent<DeferredOp> for *const () {
    #[inline]
    fn equivalent(&self, op: &DeferredOp) -> bool {
        <DeferredOp as Equivalent<*const ()>>::equivalent(op, self)
    }
}
*/
