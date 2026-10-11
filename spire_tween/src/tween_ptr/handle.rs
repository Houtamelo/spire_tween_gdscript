use std::cell::RefCell;

use godot::obj::InstanceId;

use super::*;

enum Handle<H> {
    Empty,
    Strong { handle: H, owns_tween: bool },
    Weak(InstanceId),
}

/// Shared separately from the tween payload: pointer lifecycle hooks must not
/// borrow a payload that may already be mutably borrowed by a running callback.
pub(crate) struct GdHandleSlot<H: Signaler + Clone>(Rc<RefCell<Handle<H>>>);

pub(super) struct WeakHandleSlot<H: Signaler + Clone>(Weak<RefCell<Handle<H>>>);

impl<H: Signaler + Clone> Default for GdHandleSlot<H> {
    fn default() -> Self { Self(Rc::new(RefCell::new(Handle::Empty))) }
}

impl<H: Signaler + Clone> Clone for GdHandleSlot<H> {
    fn clone(&self) -> Self { Self(self.0.clone()) }
}

impl<H: Signaler + Clone> GdHandleSlot<H> {
    pub(super) fn downgrade(&self) -> WeakHandleSlot<H> { WeakHandleSlot(Rc::downgrade(&self.0)) }

    pub(crate) fn get(&self) -> Option<H> {
        let id = match &*self.0.borrow() {
            Handle::Empty => return None,
            Handle::Strong { handle, .. } => return Some(handle.clone()),
            Handle::Weak(id) => *id,
        };
        H::try_from_instance_id(id)
    }

    pub(crate) fn attach(&self, handle: H, native_count: usize) {
        let next = if native_count > 1 {
            Handle::Strong {
                handle,
                owns_tween: true,
            }
        } else {
            Handle::Weak(handle.instance_id())
        };
        let previous = self.0.replace(next);
        // Releasing a Godot wrapper can re-enter TweenPtr::drop. Never do so
        // while holding the RefCell borrow.
        drop(previous);
    }

    pub(crate) fn set_external(&self, handle: H) {
        // with_handle() can use a separate signal emitter that does not own
        // this tween. It contributes no internal native reference or cycle.
        let previous = self.0.replace(Handle::Strong {
            handle,
            owns_tween: false,
        });
        drop(previous);
    }

    pub(super) fn promote(&self) {
        let id = match &*self.0.borrow() {
            Handle::Weak(id) => *id,
            _ => return,
        };
        if let Some(handle) = H::try_from_instance_id(id) {
            let previous = self.0.replace(Handle::Strong {
                handle,
                owns_tween: true,
            });
            drop(previous);
        }
    }

    pub(super) fn demote(&self) {
        let previous = {
            let mut slot = self.0.borrow_mut();
            let Handle::Strong {
                handle,
                owns_tween: true,
            } = &*slot
            else {
                return
            };
            let id = handle.instance_id();
            std::mem::replace(&mut *slot, Handle::Weak(id))
        };
        // The weak state is visible before the strong reference is released.
        drop(previous);
    }

    pub(super) fn has_external_owners(&self, native_count: usize) -> bool {
        match &*self.0.borrow() {
            Handle::Strong {
                handle,
                owns_tween: true,
            } => native_count > 2 || handle.reference_count() > 1,
            _ => native_count > 1,
        }
    }
}

impl<H: Signaler + Clone> Clone for WeakHandleSlot<H> {
    fn clone(&self) -> Self { Self(self.0.clone()) }
}

impl<H: Signaler + Clone> WeakHandleSlot<H> {
    pub(super) fn upgrade(&self) -> GdHandleSlot<H> {
        GdHandleSlot(self.0.upgrade().expect("A live tween owns its handle-control slot"))
    }
}
