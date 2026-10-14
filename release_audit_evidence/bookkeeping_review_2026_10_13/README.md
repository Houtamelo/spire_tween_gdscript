# Manager bookkeeping review — 2026-10-13

The proposed changes fix ordinary dead-node record growth. Finding #14 remains open because expired weak bindings on living nodes still accumulate, and copied metadata can keep the lifetime tracker alive after its original node is freed.

## Confirmed findings

### Expired weak bindings are not pruned on living nodes

`node_bind` inserts into `NodeState::bound_tweens`. `prune_dead_nodes_seen_by_both` removes only dead node records; no new path removes expired weak entries from a living or detached node. The read-only snapshot checks native weak strong counts without upgrading them.

Both profiles produce these counts on one living node, repeated separately for a detached living node:

| Completed/released tweens | Tracked nodes | Binding entries | Expired bindings | Root tweens |
|---:|---:|---:|---:|---:|
| 0 | 1 | 0 | 0 | 0 |
| 1,000 | 1 | 1,000 | 1,000 | 0 |
| 2,000 | 1 | 2,000 | 2,000 | 0 |
| 3,000 | 1 | 3,000 | 3,000 | 0 |

These entries retain Rust allocation storage after the tween values have been dropped. Freeing the node clears them; long-lived nodes still accumulate them. Pruning must retain weak entries for live handles, including completed or unregistered handles that still participate in binding actions and replay.

### Node duplication delays the tracker notification

`node_ensure_tracked` stores a `Gd<NodeLifetimeTracker>` in node metadata. The tracker is RefCounted and its `PREDELETE` callback notifies the manager. Real Godot `Node.duplicate()` copies that metadata reference; the test confirms `shared_tracker=true`.

After freeing the original node while its duplicate remains alive:

- The original node is physically freed.
- Its cached record remains outside-tree rather than dead.
- One weak binding entry remains.
- Its retained manual tween remains playing and registered.
- Cleanup and cancellation occur only after freeing the duplicate, which releases the final tracker reference.

The tracker notification follows the shared tracker's deallocation. It does not independently observe the original node's physical deletion. The occupied-record instance-validity fallback in `node_get_status_fresh` is currently commented out, so it also cannot repair this stale cached record.

Source: [tweens_map.rs](../../spire_tween/src/global/tweens_map.rs). The review did not change cleanup or retention behavior; it added feature-gated read-only observations and integration tests.

## Integration coverage

[bookkeeping.rs](../../spire_tween_rust/src/tests/bookkeeping.rs) is registered in the ordinary native test runner and has a focused [bookkeeping_runner.tscn](../../spire_tween_rust/examples/tests/bookkeeping_runner.tscn). It catches failed assertions and exits nonzero. Each profile reports **11 methods, 8 passed, 3 failed**:

| Case | Debug | Release |
|---|---|---|
| Direct free inside tree: 3 × 1,000 nodes | Pass | Pass |
| Direct free outside tree: 3 × 1,000 nodes | Pass | Pass |
| Queued free inside tree: 3 × 1,000 nodes | Pass | Pass |
| Queued free outside tree: 3 × 1,000 nodes | Pass | Pass |
| Pruning waits for both process and physics; both orders | Pass | Pass |
| Expired bindings on one living node plateau | Fail | Fail |
| Expired bindings on one detached node plateau | Fail | Fail |
| Retained completed/unregistered bindings survive pruning | Pass | Pass |
| Detached node re-entry preserves binding and resumes playback | Pass | Pass |
| Direct deletion cancels all three clock and playback modes | Pass | Pass |
| Duplicated metadata does not delay source cleanup/cancellation | Fail | Fail |

The deletion batches return all observed collections to baseline after 12,000 ordinary node deletions per profile. The two-loop test temporarily disables one real controller callback and verifies the record survives until the other callback also runs. Existing native and GDScript suites continue covering broader binding semantics.

## Verification

Fresh debug/release libraries, Godot 4.7.2 (`ed1daf0bf`), godot-rust 0.5.5, default API 4.6. All runtime checks were headless, using copied existing caches and isolated writable user data. No editor import or graphical session was launched.

- New native bookkeeping suite: 8/11 passing, exit 1 in both profiles; three caught assertion failures, no script errors.
- Existing five focused native suites: 45 methods passing per profile. The API compatibility suite deliberately catches two rejected built-in tween calls and logs its expected engine panics.
- Existing seven focused GDScript suites: 91 methods passing per profile; no script errors or native panics.
- Workspace compilation and default addon compilation remain separate from these runtime claims.

The `integration-tests` feature exposes only read-only snapshots through the hidden, unstable native API. It is enabled by the non-published native demo/test crate and is disabled in ordinary standalone addon builds. It adds no customer-facing Godot methods and never upgrades weak pointers during observation.

The suite's failures are assertions of the intended steady-state behavior, not tests that bless continued growth. The production gaps remain for the author's next fix; no commit was made by this review.
