# Delayed-call lifetime recheck after ownership repair

On Godot 4.7.2 with the current installed API 4.6 debug and release libraries, both probes report `DELAY_LIFETIME_PROBE_RESULT failed=0`, exit 0, and no errors or warnings. Library paths and hashes are recorded in validation.json. The isolated editor-selected debug entry links to the intended installed profile; runtime extension lists are prewritten because this check concerns lifetime, not first import. No graphical runs or production code changes were made.

Both variants deliberately keep the GDScript tween handle alive while freeing their Node target. The target is verified invalid and the callable reports invalid after eight frames.

| Variant | Registered | Stopped |
| --- | --- | --- |
| Plain Spire.do_delayed_call | true | false |
| Same call with explicit bind_node(target) | false | true |

After explicit unregister and releasing the caller's handle, each wrapper's WeakRef becomes null. Thus the old unconditional wrapper ownership cycle is absent, while the unbound target-lifetime behavior remains. The manager legitimately owns scheduled playback until it completes or is cancelled; the GDScript constructor does not bind the callable owner, and delayed-call processing does not check validity before the delay elapses.

This specifically establishes the freed-Node case. It does not establish behavior for RefCounted targets, custom/anonymous callables, or invalid-call invocation at the deadline. The Rust DoDelayedCallable helper binds its receiver, which can differ from the callable target; that distinction must be preserved in later design work.
