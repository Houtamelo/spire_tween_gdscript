# Manager bookkeeping follow-up — 2026-10-13

The author's new weak-entry pruning and connection-owned tracker fix the three previously failing runtime regressions. The bookkeeping suite now passes **11/11 methods in debug and release**, with no script errors or native panics.

## Verified behavior

- All observed collections return to baseline after 12,000 direct/queued node deletions per profile, covering nodes inside and outside the tree.
- Binding entries and expired entries stay at zero after each of three 1,000-tween batches, tested separately on a living in-tree node and a detached living node.
- Completed and unregistered tweens with retained handles keep their live bindings. Binding actions, re-registration, detached-node re-entry, and cancellation across idle/physics/manual and playing/paused/stopped modes still pass.
- The duplication regression no longer reads the removed tracker metadata. It now covers both deletion orders with sources inside and outside the tree: deleting the duplicate preserves the source tween; deleting the source cancels immediately and prunes its record independently of the duplicate.
- Dead-node records still wait until both controller loops observe them before pruning.

## Verification scope

Godot 4.7.2 (`ed1daf0bf`), godot-rust 0.5.5, default API 4.6. All processes were headless. Existing project caches were copied into isolated fixtures; no editor import or graphical session was launched.

Per profile, 11 bookkeeping methods, 45 existing focused native methods, and 91 focused GDScript methods pass. The 26 fixture processes exit zero with the expected result markers and counts. The native API compatibility suite deliberately catches two engine-rejected built-in tween calls and logs their expected Rust panics; no unexpected panic appears in any suite.

Workspace checking and fresh native/standalone addon debug and release builds pass. [runtime-results.json](runtime-results.json), [build-results.json](build-results.json), and the individual logs record exact commands and results. [source-verification.json](source-verification.json) confirms production and native test sources did not change during verification.

Fresh Linux addon libraries were installed into the actual GDScript project's ignored library paths. The native project's tracked library symlinks were preserved and resolve to the fresh builds. [installed-libraries.json](installed-libraries.json) records source/installed hashes and symlink targets. Actual installed-project runs also pass bookkeeping, GDScript node binding, and GDScript ownership checks.

## Editor caveat from source review

The tracker is captured by a Rust `tree_exiting` closure. In normal game execution, this connection owns the tracker and node duplication does not copy the connection. Under the editor, godot-rust 0.5.5's `signal_connections_registry.rs` clones Rust custom callables into a global registry; pruning happens when subsequent connections are stored or during reload cleanup. That extra holder can delay release of the captured tracker beyond source-node deletion.

These runtime checks do not validate manager operation inside editor tool scripts or hot reload. The registered method callable with a bound tracker proposed in the preceding discussion avoids that particular Rust-callable registry holder. No production cleanup behavior was changed by this review.

Earlier failing evidence remains in [bookkeeping_review_2026_10_13](../bookkeeping_review_2026_10_13/README.md). The broader release audit remains open.
