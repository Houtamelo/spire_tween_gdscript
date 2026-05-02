# Test Rewrite Plan

Port all GDScript integration tests to Rust using the async pattern from `sequences.rs` / `util.rs`.

## Architecture

Each test file:
- `#[derive(GodotClass)]` with `#[class(init, base = Node2D)]`
- `#[init(node = "...")]` for scene children
- `#[init(val = RcPtr::new(TimeTracker::new()))]` for shared timer
- `impl_test_base!(INode2D, ClassName)` macro for ready/process/run_test
- `get_test_list()` returning `Vec<fn(&mut Self) -> PinnedTestTask>`
- Each test returns `PinnedTestTask` (Pin<Box<dyn Future>>)
- Awaits use `tracker.wait_finished()`, `tracker.wait_seconds()`, `tracker.wait_loop_finished()`
- `gd_handle` is auto-attached by `register()`, accessed via `handle.gd_handle.as_ref().unwrap()`

## Key API Mappings (GDScript → Rust)

- `DoNode2D.scale(node, val, dur)` → `node.do_scale(val, dur).register()`
- `DoNode2D.move(node, val, dur)` → `node.do_move(val, dur).register()` or `node.do_global_position(val, dur).register()`
- `DoCanvasItem.color_g(node, val, dur)` → `node.do_color_g(val, dur).register()`
- `DoNode2D.spiral(...)` → `ball.do_spiral(...).register()`
- `DoNode2D.circle(...)` → `ball.do_circle(...).register()`
- `DoNode2D.ellipsis(...)` → `ball.do_ellipsis(...).register()`
- `Spire.sequence()` → `SpireTween::<Sequence>::new().register()`
- `Spire.do_delayed_call(fn, dur)` → `node.do_delayed_call(fn, dur).register()`
- `Spire.do_call_vec2(fn, from, to, dur)` → use do_method API
- `.set_delay(d)` → `.with_delay(d)`
- `.set_loops(n, mode)` → `.set_loops(n, mode)` (after register, via handle)
- `.set_pause_mode(m)` → `.with_pause_mode(m)`
- `.set_process_mode(m)` → `.with_process_mode(m)`
- `.as_relative()` → `.as_relative(initial_pos)`
- `.as_speed_based()` → `.as_speed_based()`
- `.from(val)` → `.set_begin_value(val)` (on SpireTween before register)
- `tween.pause()` → `handle.pause()`
- `tween.play()` → `handle.play()`
- `tween.stop()` → `handle.stop()`
- `tween.force_complete()` → `handle.force_complete()`
- `tween.unregister()` → `handle.unregister()`
- `tween.is_playing()` → `handle.is_playing()`
- `tween.is_stopped()` → `handle.is_stopped()`
- `await wait_finished(tween, time)` → `tracker.wait_finished(handle.gd_handle.as_ref().unwrap(), time).await`
- `await wait_loop_finished(tween, time)` → `tracker.wait_loop_finished(handle.gd_handle.as_ref().unwrap(), time).await`
- `await wait_seconds(s)` → `tracker.wait_seconds(s).await`

## Files to Write (in order)

### 1. play_pause_stop.rs
- Scene: `BouncyBall` (uses `$Ball` → Sprite2D or Node2D depending on scene)
- 3 tests: test_play_works, test_pause_play_works, test_play_stop_play_works
- Tests scale tweens, pause/resume, stop/restart

### 2. delays.rs
- Scene: `BouncyBall/CollisionShape2D`
- 4 tests: test_delay_is_respected, test_extra_loops_dont_have_delay, test_sequence_respects_delay, test_delayed_call
- Tests delay behavior, loop delay, sequence delay, delayed callables

### 3. misc.rs
- Scene: `BouncyBall/CollisionShape2D`
- 3 tests: test_force_complete, test_dyn_target, test_property_path
- Note: test_dyn_target uses `set_dynamic_target`; test_property_path only checks metadata

### 4. register_unregister.rs
- Scene: `BouncyBall/CollisionShape2D`
- 2 tests: test_unregistered_does_not_affect_game, test_unregistered_can_be_manually_stepped
- Tests unregister/re-register and manual stepping

### 5. error_handling.rs
- Scene: `BouncyBall/CollisionShape2D/Sprite2D`
- 4 tests: test_invalid_enum, test_invalid_tween, test_free_while_playing, test_invalid_sequence_adding
- Note: test_invalid_tween tests GDScript null/RefCounted handling — may need adaptation
- Note: test_free_while_playing must be LAST (frees sprite)

### 6. sequences.rs (ALREADY DONE — only needs remaining tests added)
- Scene: `BouncyBall/CollisionShape2D`
- 7 tests: test_proper_ordering, test_stopped_child_does_not_halt_sequence, test_append_many, test_remove, test_remove_midway, test_remove_call, test_default_children_ease
- test_remove_midway already implemented as reference

### 7. lerp_modes.rs
- Scene: Sprite2D (`$Sprite2D`)
- 4 tests: test_relative, test_two_relatives, test_speed_based, test_speed_based_plus_relative

### 8. lerp_callable.rs
- Scene: Sprite2D (`$Sprite2D`)
- 4 tests: test_lerp_call, test_lerp_call_float, test_lerp_call_with_ease, test_lerp_call_with_loop
- Uses `Spire.do_call_vec2` / `do_call_float` — need Rust equivalent

### 9. pause_process_modes.rs
- Scene: `BouncyBall/CollisionShape2D` + `BouncyBall/CollisionShape2D/Sprite2D`
- 6 tests: test_pause_mode_process, test_pause_mode_stop, test_pause_mode_bound, test_process_mode_idle, test_process_mode_physics, test_pause_mode_bound_process_mode_physics
- Complex: manipulates tree pause state, process modes, physics vs idle

### 10. spiral_test.rs
- Scene: Ball (Sprite2D/TrailSpawner)
- 12 tests: 4 types × 3 variants (normal, sheared, inverted)
- All identical pattern: spawn spiral, wait_finished(tween, 8.0)

### 11. ellipsis_test.rs
- Scene: Ball (Sprite2D/TrailSpawner)
- 5 tests: circle, circle_inverted, ellipsis, ellipsis_varying, ellipsis_varying_inverted
- All identical pattern: spawn shape, wait_finished

## Execution Order
1. play_pause_stop.rs (simplest, validates base pattern)
2. delays.rs
3. misc.rs
4. register_unregister.rs
5. error_handling.rs
6. sequences.rs (add remaining tests to existing file)
7. lerp_modes.rs
8. lerp_callable.rs
9. pause_process_modes.rs (most complex)
10. spiral_test.rs
11. ellipsis_test.rs
12. Update mod.rs
13. Build + run each individually to verify
