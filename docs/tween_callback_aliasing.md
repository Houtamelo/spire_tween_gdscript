# Reading a tween through a weak pointer during its callback aliases an active mutable borrow

Source review on 2026-10-25 found that upgrading a weak tween pointer is safe for
ownership, but dereferencing it from a synchronous callback conflicts with the
processing path's active `&mut SpireTween<_>`. This applies even when the callback
only reads state. An existing signal test exercises the same access pattern.

## Evidence

- `spire_tween/src/tweens/base/spire_tween.rs::TweenPtr::custom_step` invokes
  processing through `self.to_mut().process(...)`.
- `spire_tween/src/tweens/method/data.rs::process_internal` takes `&mut self`,
  invokes the method callable, then accesses `self` again to finish processing.
- `spire_tween/src/tween_ptr/strong.rs::Deref` creates `&SpireTween<T>` directly
  from the allocation's `UnsafeCell` pointer. An upgraded weak pointer reaches
  the same allocation through this implementation, rather than reborrowing the
  processor's existing mutable reference.
- `spire_tween_rust/src/tests/stop_events.rs::events` synchronously upgrades a weak
  pointer and calls `is_stopped()` while `stop(&mut self)` emits its signal.

Rust's [UnsafeCell aliasing rules](https://doc.rust-lang.org/std/cell/struct.UnsafeCell.html#aliasing-rules)
require mutable references to remain exclusive. Wrapping the allocation in
`UnsafeCell` does not permit an independently obtained shared reference to alias
the active mutable reference to its contents. Weak ownership prevents ownership
cycles; it does not change this borrowing requirement.

This is a source-based finding. Miri is not installed in the current toolchain;
no Miri reproduction or runtime failure was established. Passing Godot integration
tests does not establish that the unsafe aliasing is valid.

## Design options

To support synchronous weak-pointer state reads, prepare the invocation under a
mutable borrow, end that borrow, invoke the callback, and finish processing under
a fresh borrow. No outer processing or signal-dispatch function may retain the
original mutable reference across the callback. Forced completion and synchronous
signals need the same treatment.

Alternatively, pass the required values directly to the callback, or supply a
shared reference reborrowed from the active mutable reference. Those approaches
avoid independently dereferencing a pointer to the borrowed allocation.

Returning a `TweenPtr` from a template supplies a stable allocation; registration
is optional because `TweenPtr::new(builder)` already supports `downgrade()`. That
return-type decision is independent of the callback borrowing issue. It also does
not expose whether a method invocation came from forced completion, which still
needs an explicit mechanism or an agreed change to restoration timing.

No processing or pointer implementation was changed as part of this report.
