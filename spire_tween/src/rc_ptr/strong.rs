use super::*;

#[repr(transparent)]
pub struct RcPtr<T: ?Sized>(pub(super) Rc<UnsafeCell<T>>);

impl<T: Sized> RcPtr<T> {
    #[inline]
    pub fn new(inner: T) -> Self { Self(Rc::new(UnsafeCell::new(inner))) }
}

impl<T: ?Sized> RcPtr<T> {
    #[inline]
    pub fn downgrade(&self) -> WeakPtr<T> { WeakPtr(Rc::downgrade(&self.0)) }

    #[inline]
    pub fn strong_count(&self) -> usize { Rc::strong_count(&self.0) }

    #[allow(clippy::mut_from_ref)]
    #[allow(clippy::wrong_self_convention)]
    pub fn to_mut(&self) -> &mut T { unsafe { &mut *self.0.get() } }
}

impl<T: ?Sized> Address for RcPtr<T> {
    #[inline]
    fn address(&self) -> *const () { Rc::as_ptr(&self.0) as *const () }
}

impl<T: ?Sized> Clone for RcPtr<T> {
    #[inline]
    fn clone(&self) -> Self { Self(Rc::clone(&self.0)) }
}

impl<T: ?Sized> Deref for RcPtr<T> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &Self::Target { unsafe { &*self.0.get() } }
}

impl<T: ?Sized> DerefMut for RcPtr<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target { unsafe { &mut *self.0.get() } }
}

impl<T: ?Sized> PartialEq for RcPtr<T> {
    #[inline]
    fn eq(&self, other: &Self) -> bool { Rc::ptr_eq(&self.0, &other.0) }
}

impl<T: ?Sized> Eq for RcPtr<T> {}

impl<T: ?Sized> Hash for RcPtr<T> {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) { std::ptr::hash(self.address(), state); }
}

impl<T> Equivalent<*const ()> for RcPtr<T> {
    #[inline]
    fn equivalent(&self, key: &*const ()) -> bool { std::ptr::addr_eq(self.address(), *key) }
}

impl Equivalent<*const ()> for RcPtr<dyn Any> {
    #[inline]
    fn equivalent(&self, key: &*const ()) -> bool { std::ptr::addr_eq(self.address(), *key) }
}

impl<T> Equivalent<RcPtr<T>> for *const () {
    #[inline]
    fn equivalent(&self, key: &RcPtr<T>) -> bool { std::ptr::addr_eq(*self, key.address()) }
}

impl Equivalent<RcPtr<dyn Any>> for *const () {
    #[inline]
    fn equivalent(&self, key: &RcPtr<dyn Any>) -> bool { std::ptr::addr_eq(*self, key.address()) }
}
