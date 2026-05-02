# Public Documentation Audit — `spire_tween_asset_plugin` (1.0 vs published 0.6.0)

> Generated 2026-05-01 from a comparison of the working tree against
> the latest crates.io release of `spire_tween` (v0.6.0, published 2025-08-31).
> The published code lives in a *separate* repository
> (`github.com/Houtamelo/spire_tween`) — this repo
> (`spire_tween_asset_plugin`, origin `Houtamelo/spire_tween_gdscript`) is
> the unreleased 1.0 rewrite.

---

## TL;DR

The repo has had a **paradigm shift**, not just an upgrade:

| Axis | v0.6.0 (published) | 1.0 (HEAD) |
|---|---|---|
| Audience | Rust gdext devs (`cargo add spire_tween`) | GDScript devs buying a Godot addon |
| Crate count | 1 (`spire_tween`) | 5 (`spire_tween`, `spire_tween_rust`, `spire_tween_plugin`, `spire_tween_gdscript`, `gdscript_bindgen`) |
| Source files in core | ~50 `.rs` | ~210 `.rs` |
| godot-rust version | `0.3.0` | `0.5.x` |
| GDScript-facing API | none | `Spire`, `Do{Class}`, `SpireGlobalSettings`, `SpirePropertyXxx`, `SpireMethodXxx`, `SpireDelayedCall`, `SpireSequence` |
| Setup ritual | autoload `tweens_controller` (panics if mis-named) | none — auto via `LazyLock` singleton |
| Tween templates (do_shake/spiral/follow/...) | none | 8 trait families, exposed to both Rust and GDScript |
| In-editor docs | none | yes, via `register-docs` + `util/docs/` extractor |
| Nightly Rust required | yes (5 features) | yes (4 features), but README does not mention it |

The current `README.md` was written for the addon audience and **does not match what crates.io still serves**. Both READMEs need work, in different directions.

---

## Inventory of public/quasi-public docs

| Path | Audience | Status |
|---|---|---|
| `README.md` (root) | GDScript users (asset plugin) | Stale in places (see Section A) |
| `LICENSE` | All | Fine (MIT) |
| `readme_images/*.png/.gif` | Embedded in README | All 12 referenced assets present |
| `spire_tween_gdscript/examples/{powerup,readme,...}/*.tscn` | Linked from README | Present; `example_powerup.gd` calls a stale signature (see A1) |
| `tweenable_properties.json` + `tweenable_properties_schema.json` | Generator input (semi-public — committed) | Schema looks current; not user-doc per se |
| `spire_tween/Cargo.toml` `description` / `repository` | crates.io listing | `repository` URL still points at the old standalone repo (see Section C) |
| `spire_tween/src/**/*.rs` doc comments | Rust API users + in-editor Godot docs (via `register-docs`) | Mostly comprehensive; some prelude exports (e.g. `DoBone`) are undocumented in the README |
| `test_review.md`, `spire_tween_rust/PLAN_TEST_REWRITE.md`, `examples/tests/results.json` | Internal QA artifacts | Not user-facing — out of scope but should probably be moved to `docs/internal/` or gitignored |
| `CLAUDE.md` | AI agent context | Just created; not user-facing |

The **published crates.io README** (still v0.6.0) is itself a "doc" for current users — it needs to be replaced or the repository link redirected. See Section C.

---

## Section A — `README.md` (current) audit

### A1. BROKEN: Showcase code sample (top of file, lines 13–24)

```gdscript
DoNode2D.spiral(ball, center, from_angle, to_angle, scale, duration, rotation, shear, Spire.SPIRAL_FERMAT)
```

Rust signature in `spire_tween/src/gdscript_bridge/generated/node_2_d.rs:294-305` is **10 parameters**:

```rust
fn spiral(
    node, center, from_angle, to_angle, scale, duration,
    rotation, shear, mode,
    log_growth: Vector2,   // <-- missing from README and from example_powerup.gd
)
```

`example_powerup.gd:38` makes the same call with 9 args. Either:
- The `log_growth` parameter was added after the example was written, or
- The wrapper should provide a default (gdext's official `#[func]` doesn't support defaults — see existing memory `feedback_fork_default_attr.md`)

**Action:** decide which: either add `log_growth` to the README + example call, or split `spiral` into a `spiral_logarithmic(...)` overload that takes `log_growth` and a `spiral_other(...)` that doesn't. The current state means the very first code on the README is uncompilable in GDScript.

### A2. WRONG NAME: Cheat-sheet API list (line 274)

```
do_property_vector2(owner: Object, property_path: NodePath, to: Vector2, duration: float) -> SpirePropertyVector2
```

Actual Rust function (in `gdscript_bridge/do_property.rs:45`) is registered as **`do_property_vec2`** (likewise `_vec2i`, `_vec3`, `_vec3i`). The `do_call_*` family on the next page uses `vector2/vector2i/vector3/vector3i` — which the README correctly documents at line 355.

**This is a code-side inconsistency too:**

| Family | Names in code |
|---|---|
| `do_property_*` | `_float`, `_int`, `_vec2`, `_vec2i`, `_vec3`, `_vec3i`, `_color`, `_string` |
| `do_call_*`     | `_float`, `_int`, `_vector2`, `_vector2i`, `_vector3`, `_vector3i`, `_color`, `_string` |

**Action:** either rename one family to match the other (recommended: change `do_property_vec*` → `do_property_vector*` for consistency with godot's own type names), then fix README. If you keep the inconsistency, README must list both spellings.

### A3. MISSING: Tween templates (do_shake / do_bone / do_follow / do_contour_shape / do_ellipsis)

The `tweens/templates/` module didn't exist in v0.6.0. It's now a major feature surface and all its traits are re-exported from `spire_tween::prelude`:

- `DoBone` (Skeleton3D)
- `DoContourShape2D` (Node2D)
- `DoEllipsis2D`, `DoEllipsis3D`
- `DoFollow2D`, `DoFollow3D`
- `DoShakeNode2D`, `DoShakeControl`
- `DoSpiral` (Node2D)

GDScript bindings exist for: `DoNode2D.{shake, ellipsis, circle, spiral, follow}`, `DoNode3D.{follow, ellipsis}`, `DoSkeleton3D.{bone_position, bone_scale}`. None of these appear in the README's cheat sheets, examples, or showcase narrative beyond the broken `spiral` call.

**Action:** add a "Templates / Higher-level animations" section. The powerup gif is already a `spiral` showcase — extend that block to cover the other templates with one-line call examples.

### A4. STALE/MISSING: Setup section vs. v0.6.0

v0.6.0 README had:
> "Step 2: Create an empty scene with the root named `tweens_controller` and type `TweensController`, then add that as your autoload. **Warning!: The node must be named "tweens_controller", otherwise Rust will panic**"

Current README has no such step (correctly — `TM` is now a `LazyLock` in `global/dash_map_impl.rs`, no autoload required).

**Action:** explicitly state in the Installation section that **no autoload setup is needed** (helpful for users migrating from 0.6.0) and that the GDScript classes register automatically once the addon is enabled.

### A5. MISSING: Nightly Rust requirement (compile-from-source path)

`rust-toolchain.toml` pins `channel = "nightly"` and `spire_tween/src/lib.rs:5-9` activates 4 unstable features:

```rust
#![feature(type_changing_struct_update)]
#![feature(unboxed_closures)]
#![feature(arbitrary_self_types)]
#![feature(stmt_expr_attributes)]
```

The README's "Compile from source" bullet (line 74) does not mention this. v0.6.0's README had a prominent "# Warning!" block listing the unstable features.

**Action:** restore a "Build requirements" subsection under Installation. Mention nightly + the 4 features so users compiling for unsupported platforms (macOS, etc.) aren't surprised.

### A6. STALE: Platform Support table

The table (lines 88–99) lists web/wasm as ❌ with "There is a temporary issue, will be fixed soon(TM)". The current `Cargo.toml` has the `experimental-wasm` feature enabled by default (`godot = { version = "0.5", features = ["experimental-wasm", ...] }`) and a `nothreads` feature for wasm-without-threads. The `.gdextension` shipped in `addons/spire_tween/spire_tween.gdextension` already wires `web.debug.wasm32`, `web.release.wasm32`, `web.debug.threads.wasm32`, `web.release.threads.wasm32`.

**Action:** verify whether wasm builds actually succeed now, and update the table accordingly. If still broken, update the note to describe the *current* blocker.

### A7. PLACEHOLDER: Itch.io purchase link (line 70)

```
Purchase it at [Itch.io](http://www.itch.io/placeholder_link)
```

**Action:** replace before any public release.

### A8. NICE-TO-HAVE: Configuration section is too thin

Section says "Currently, SpireTween supports a single global configuration option" (line 104). With the `standalone`, `dashmap`, `indexmap`, `double-precision`, `nothreads`, `verbose-stdout` Cargo features now present, source-build users have more knobs that aren't documented. (The `default = ["indexmap"]` choice between dashmap and indexmap, in particular, has perf implications worth a sentence.)

**Action:** add a "Cargo features" subsection for the source-build path. Also note that `SpireGlobalSettings` is the GDScript-facing config namespace and may grow over time.

### A9. CHEAT SHEETS — accurate but incomplete

I spot-checked the Properties / Methods / Signals tables against the current code:
- All `set_*` / `get_*` / `is_*` / `as_relative` / `as_speed_based` methods listed exist in `spire_tween/src/tweens/{property,method,sequence}/macros.rs` and `gd.rs`.
- `bind_node`, `unbind_node`, `clear_bound_nodes`, `register`, `unregister`, `get_bound_nodes`, `force_complete` all exist on the Gd handles.
- Sequence methods (`append`, `join`, `insert`, `remove`, `append_call`, `join_call`, `append_interval`, `join_interval`, `append_many`, `join_many`, `set_default_children_ease`) all match `tweens/sequence/gd.rs`.

What's **missing from the cheat sheets**:
- **Custom-easing entry points**: `set_ease_curve(Curve)` is documented; `set_ease_func(Callable)` is documented. Good.
- **Templates table**: no entry. (See A3.)
- **`Spire.is_registered(tween)` static** — listed nowhere despite being one of three `Spire` namespace methods (alongside `register`/`unregister`).
- **Per-class enums** (`Spire.SPIRAL_FERMAT/_LOGARITHMIC/_HYPERBOLIC/_ARCHIMEDEAN`, ease constants) — currently introduced only by example. A small "Enums" subsection in the cheat sheet would help.

### A10. Benchmarks section — current

Looks recent (Godot 4.5.1 stable, Linux Mint 22.1 listed in system info). No action needed unless you re-run for 1.0.

### A11. Examples — files exist, but `example_powerup.gd` will fail (see A1)

All linked example scenes (`example_common_property.tscn`, `example_custom_property.tscn`, `example_method.tscn`, `example_sequence.tscn`, `example_powerup.tscn`) and their `.gd` siblings exist on disk and use only documented APIs except for the spiral mismatch.

**Action:** re-run each example end-to-end after the spiral fix to confirm they still work post-1.0 refactor.

---

## Section B — `spire_tween/Cargo.toml` audit (crates.io metadata)

```toml
description = "A Slimy tweener library for Godot(4.4+), inspired on the design of DoTween."
repository  = "https://github.com/Houtamelo/spire_tween"
```

- `repository` URL points to the **published-version** repo, not this asset-plugin repo. If 1.0 is published as `spire_tween` on crates.io, downstream Rust users clicking "Repository" will land on a stale tree. Decide:
  - **Option A**: keep `Houtamelo/spire_tween` as the canonical home for the Rust crate; mirror the relevant `spire_tween/` subdirectory there before the next publish.
  - **Option B**: change the URL to `Houtamelo/spire_tween_gdscript` (this repo) and accept that the addon scaffolding lives alongside the crate.
- `description` — fine but consider mentioning "templates" or "GDScript bindings" since both are major 1.0 additions.
- `keywords`, `categories` — unchanged from 0.6.0; still valid.

---

## Section C — Crates.io README (the v0.6.0 one)

Even after you update `README.md` here, **the README displayed on crates.io is still the v0.6.0 one** until the next publish. That README:

- Documents the autoload requirement (`tweens_controller`) — no longer true.
- Lists `#![feature(inline_const_pat)]`, `#![feature(trait_alias)]`, `#![feature(hash_extract_if)]`, `#![feature(let_chains)]`, `#![feature(is_none_or)]` — none of these are used in 1.0 (the actual list is in A5).
- References functions like `do_var_property` and `do_var_method` — these no longer exist in the public surface. (Closest equivalents are `Spire.do_property_custom` / `Spire.do_call_custom` for runtime-typed paths.)
- Says the package is "largely undocumented" — no longer true; in-editor docs are extracted via `register-docs`.

When you publish 1.0:
- Decide whether the crates.io README should be the **GDScript-addon README** (this file, audience: addon users) or a **Rust-API README** (audience: gdext devs using `prelude::*`). They serve different audiences and pasting the addon README onto crates.io would mislead Rust users.
- **Recommendation**: ship a Rust-flavored `spire_tween/README.md` (which `Cargo.toml` `readme = "..."` points at by default) that:
  - Briefly describes the `prelude` API (`SpireTween<T>`, `DoProperty`, `DoMethod`, `DoDelayedCall`, `Sequence`, the `Do*` template traits)
  - Explains the `standalone` feature flag and its purpose
  - Links to the asset-plugin repo for the GDScript-facing usage

---

## Section D — Internal/QA docs

`test_review.md` (root, 589 lines) and `spire_tween_rust/PLAN_TEST_REWRITE.md` (116 lines) are tracked in git but read like working docs. `examples/tests/results.json` files capture last-run pass/fail snapshots.

**Action (low priority):** move these under `docs/internal/` or add to `.gitignore` if they're personal scratch space. They will appear in the published crate package unless excluded via `[package] exclude = [...]` or `include = [...]`.

---

## Recommended order of operations

Sorted by impact ÷ effort:

1. **Fix A1** (broken showcase) — 10 min. Either restore `log_growth` arg to the example & README call, or change the wrapper signature. Without this, the very first code in the README fails.
2. **Fix A2** (`do_property_vec2` vs `_vector2`) — pick a name, rename consistently, update README. ~30 min including tests.
3. **Add A3** (templates section) — 1–2 h to draft + screenshots. Biggest gap in user-facing docs.
4. **Patch A4 + A5 + A7** in one README pass (no autoload, nightly required, real Itch.io link) — 30 min.
5. **Decide on Section C** before any `cargo publish` — strategic, not just editorial. Ship a separate `spire_tween/README.md` for crates.io.
6. **Verify A6** (wasm) — depends on whether `cargo build --release --target wasm32-unknown-emscripten` actually works now.
7. **Update Section B** repository URL post-decision.
8. **Cleanup D** (move QA docs) — cosmetic.

---

## Files NOT audited (out of scope of "public documentation")

- `spire_tween/src/**/*.rs` doc-comment content (would need reading every `///` block; doc-style is consistent with 0.6.0 patterns)
- `gdscript_bindgen/`, `bin/`, `build_scripts/` internals
- The Godot editor plugin (`spire_tween_plugin/`) — has no user-facing prose
