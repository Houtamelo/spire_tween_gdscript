use super::*;

#[repr(transparent)]
pub struct WeakPtr<T: ?Sized>(pub(super) Weak<UnsafeCell<T>>);

impl<T: Sized> WeakPtr<T> {
    #[inline]
    pub fn upgrade(&self) -> Option<RcPtr<T>> { self.0.upgrade().map(RcPtr::<T>) }
}

impl<T: ?Sized> Address for WeakPtr<T> {
    #[inline]
    fn address(&self) -> *const () { Weak::as_ptr(&self.0) as *const () }
}

impl<T> From<&RcPtr<T>> for WeakPtr<T> {
    #[inline]
    fn from(ptr: &RcPtr<T>) -> Self { ptr.downgrade() }
}

impl<T> Clone for WeakPtr<T> {
    #[inline]
    fn clone(&self) -> Self { Self(self.0.clone()) }
}

impl<T: ?Sized> PartialEq for WeakPtr<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool { Weak::ptr_eq(&self.0, &other.0) }
}

impl<T: ?Sized> Eq for WeakPtr<T> {}

impl<T: ?Sized> Hash for WeakPtr<T> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) { std::ptr::hash(self.address(), state); }
}

impl<T: ?Sized> Equivalent<*const ()> for WeakPtr<T> {
    #[inline]
    fn equivalent(&self, key: &*const ()) -> bool { std::ptr::addr_eq(self.address(), *key) }
}

impl<T: ?Sized> Equivalent<WeakPtr<T>> for *const () {
    #[inline]
    fn equivalent(&self, key: &WeakPtr<T>) -> bool { std::ptr::addr_eq(*self, key.address()) }
}

impl<TStrong: ?Sized, TWeak: ?Sized> PartialEq<WeakPtr<TWeak>> for RcPtr<TStrong> {
    #[inline]
    fn eq(&self, other: &WeakPtr<TWeak>) -> bool {
        std::ptr::addr_eq(self.address(), other.address())
    }
}

impl<TStrong: ?Sized, TWeak: ?Sized> PartialEq<RcPtr<TStrong>> for WeakPtr<TWeak> {
    #[inline]
    fn eq(&self, other: &RcPtr<TStrong>) -> bool {
        std::ptr::addr_eq(self.address(), other.address())
    }
}
