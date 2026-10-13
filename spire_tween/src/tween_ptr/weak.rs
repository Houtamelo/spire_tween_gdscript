use super::{handle::WeakHandleSlot, *};

/// Non-owning observer of a tween and its handle-control state. Upgrading a
/// live pointer restores strong Godot handle ownership when necessary.
pub struct WeakTweenPtr<T: ITweenable>(
    pub(super) Weak<UnsafeCell<SpireTween<T>>>,
    pub(super) WeakHandleSlot<T::GdHandle>,
);

impl<T: ITweenable> WeakTweenPtr<T> {
    /// Returns a strong pointer while the native tween is alive, promoting its
    /// Godot back-reference when required. Returns `None` after the final strong
    /// native owner is released; it never resurrects an expired tween.
    pub fn upgrade(&self) -> Option<TweenPtr<T>> {
        let native = self.0.upgrade()?;
        let ptr = TweenPtr(native, self.1.upgrade());
        if ptr.strong_count() == 2 {
            ptr.1.promote();
        }
        Some(ptr)
    }
}

impl<T: ITweenable> Clone for WeakTweenPtr<T> {
    fn clone(&self) -> Self { Self(self.0.clone(), self.1.clone()) }
}

impl<T: ITweenable> From<&TweenPtr<T>> for WeakTweenPtr<T> {
    fn from(ptr: &TweenPtr<T>) -> Self { ptr.downgrade() }
}

impl<T: ITweenable> Address for WeakTweenPtr<T> {
    fn address(&self) -> *const () { self.0.as_ptr().cast() }
}

impl<T: ITweenable> PartialEq for WeakTweenPtr<T> {
    fn eq(&self, other: &Self) -> bool { Weak::ptr_eq(&self.0, &other.0) }
}

impl<T: ITweenable> Eq for WeakTweenPtr<T> {}

impl<T: ITweenable> Hash for WeakTweenPtr<T> {
    fn hash<H: Hasher>(&self, state: &mut H) { self.address().hash(state); }
}

impl<T: ITweenable> Equivalent<*const ()> for WeakTweenPtr<T> {
    fn equivalent(&self, key: &*const ()) -> bool { std::ptr::addr_eq(self.address(), *key) }
}

impl<T: ITweenable> Equivalent<WeakTweenPtr<T>> for *const () {
    fn equivalent(&self, key: &WeakTweenPtr<T>) -> bool { std::ptr::addr_eq(*self, key.address()) }
}

impl<T: ITweenable> PartialEq<WeakTweenPtr<T>> for TweenPtr<T> {
    fn eq(&self, other: &WeakTweenPtr<T>) -> bool { std::ptr::addr_eq(self.address(), other.address()) }
}

impl<T: ITweenable> PartialEq<TweenPtr<T>> for WeakTweenPtr<T> {
    fn eq(&self, other: &TweenPtr<T>) -> bool { std::ptr::addr_eq(self.address(), other.address()) }
}
