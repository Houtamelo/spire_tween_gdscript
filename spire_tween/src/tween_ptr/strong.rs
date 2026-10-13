use super::*;

/// Shared ownership of a native tween. `T` is the tween data, e.g. `Sequence`
/// or `LerpPropertyData<f64>`, rather than `SpireTween<T>` itself.
///
/// The Godot wrapper owns one native pointer. Its back-reference is strong
/// while another native owner exists, and weak when the wrapper is the sole
/// owner. Clone, weak upgrade, and drop maintain that ownership boundary.
///
/// The generic parameter is the data type, not the outer tween:
/// ```compile_fail
/// use spire_tween::prelude::{Sequence, SpireTween, TweenPtr};
/// let _: Option<TweenPtr<SpireTween<Sequence>>> = None;
/// ```
pub struct TweenPtr<T: ITweenable>(pub(super) Rc<UnsafeCell<SpireTween<T>>>, pub(super) GdHandleSlot<T::GdHandle>);

impl<T: ITweenable> TweenPtr<T> {
    /// Moves a tween builder into shared ownership without registering it.
    /// Useful for returning `TweenPtr::new(builder).into()` from a sequence factory.
    pub fn new(tween: SpireTween<T>) -> Self {
        let slot = tween.gd_handle.clone();
        let ptr = Self(Rc::new(UnsafeCell::new(tween)), slot);
        ptr.1.demote();
        ptr
    }

    /// Creates a non-owning observer; it does not keep the tween or wrapper alive.
    pub fn downgrade(&self) -> WeakTweenPtr<T> { WeakTweenPtr(Rc::downgrade(&self.0), self.1.downgrade()) }

    /// Returns the native strong-owner count, including manager, sequence, and
    /// Godot-wrapper native pointers when present. This is not a Godot reference count.
    pub fn strong_count(&self) -> usize { Rc::strong_count(&self.0) }

    /// Callers must ensure no overlapping mutable references exist.
    #[allow(clippy::mut_from_ref)]
    pub fn to_mut(&self) -> &mut SpireTween<T> { unsafe { &mut *self.0.get() } }

    pub(crate) fn attach_gd_handle(&self, handle: T::GdHandle) { self.1.attach(handle, self.strong_count()); }

    pub(crate) fn has_external_owners(&self) -> bool { self.1.has_external_owners(self.strong_count()) }
}

impl<T: ITweenable> Clone for TweenPtr<T> {
    fn clone(&self) -> Self {
        let ptr = Self(self.0.clone(), self.1.clone());
        if ptr.strong_count() == 2 {
            ptr.1.promote();
        }
        ptr
    }
}

impl<T: ITweenable> Drop for TweenPtr<T> {
    fn drop(&mut self) {
        // Drop runs before the Rc field decrements its count. The last owner
        // outside the wrapper therefore observes 2, whose effective next count
        // is 1. Our Rc remains alive throughout recursive wrapper destruction.
        if self.strong_count() == 2 {
            self.1.demote();
        }
    }
}

impl<T: ITweenable> Deref for TweenPtr<T> {
    type Target = SpireTween<T>;
    fn deref(&self) -> &Self::Target { unsafe { &*self.0.get() } }
}

impl<T: ITweenable> DerefMut for TweenPtr<T> {
    fn deref_mut(&mut self) -> &mut Self::Target { self.to_mut() }
}

impl<T: ITweenable> Address for TweenPtr<T> {
    fn address(&self) -> *const () { Rc::as_ptr(&self.0).cast() }
}

impl<T: ITweenable> PartialEq for TweenPtr<T> {
    fn eq(&self, other: &Self) -> bool { Rc::ptr_eq(&self.0, &other.0) }
}

impl<T: ITweenable> Eq for TweenPtr<T> {}

impl<T: ITweenable> Hash for TweenPtr<T> {
    fn hash<H: Hasher>(&self, state: &mut H) { self.address().hash(state); }
}

impl<T: ITweenable> Equivalent<*const ()> for TweenPtr<T> {
    fn equivalent(&self, key: &*const ()) -> bool { std::ptr::addr_eq(self.address(), *key) }
}

impl<T: ITweenable> Equivalent<TweenPtr<T>> for *const () {
    fn equivalent(&self, key: &TweenPtr<T>) -> bool { std::ptr::addr_eq(*self, key.address()) }
}
