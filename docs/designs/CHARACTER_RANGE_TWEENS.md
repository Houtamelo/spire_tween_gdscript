# Character ranges with a start-delay curve

Status: non-negative start-delay samples and continuous travelling loops agreed
on 2026-11-13. The curve replaces Blueprint's `character_stagger`. Implementation
has not started; the runtime representation below is a proposal.

Character animation constructors should target a range and update its characters
through one runtime tween. An optional scalar curve supplies each character's
start delay as a multiple of `duration`. **The user-provided `duration` is the
duration of one animation leg for each character. The total duration of the Char
tween is the largest completion time among its characters, including their start
delays and requested legs.** Characters animate concurrently, so their durations
are not added together.

## Start delays and local progress

For inclusive bounds `start` and `end`, character index `i`, elapsed animation
time `t`, and positive per-character duration `d`, calculate:

```text
x = (i - start) / (end - start)
delay_factor[i] = f(x)
start_time[i] = delay_factor[i] * d
```

Use floating-point division for `x`. For a single-character selection, use
`x = 0`. An absent curve supplies zero for every character. All sampled factors
must be finite and non-negative; factors greater than one are allowed and delay
the character by more than one leg's duration. Computed start times must also
remain finite.

Sample the delay curve once per selected character when constructing the tween.
The resulting delays belong to that playback instance, following existing
effect-configuration snapshot behavior. Curve edits affect subsequent runs.

| Curve | Result |
|---|---|
| `f(x) = 0` | All selected characters start immediately and advance together. |
| `f(x) = x` | The first character starts immediately; the last starts when the first finishes its first leg. |
| `f(x) = 1 - x` | The last character starts immediately; the first starts one leg's duration later. |
| A symmetric non-negative curve | Starts can spread from the middle toward both ends, or in the reverse direction. |

The character waits until `t >= start_time[i]`, then begins normal playback:

```text
local_elapsed[i] = t - start_time[i]
local_progress[i] = local_elapsed[i] / d
```

Here `t` starts when the range step begins, after containing delays and preceding
serial steps. With an implicit start, capture the character's initial value when
its playback begins. Existing explicit-start initialization can still prepare
the whole selection when the step begins, before these delays.

For one finite leg, clamp local progress to `[0, 1]`, then evaluate easing and
interpolate the character's value. Preserve legitimate easing overshoot in the
resulting value. The curve controls the start delay; normal easing controls the
animation after that start.

## Completion and duration

For one finite leg, a character reaches its endpoint at `start_time[i] + d`.
Characters that finish early hold their endpoint while the remaining characters
catch up. The range finishes once all selected characters have completed.

Measure each completion time from the beginning of the range step. The nominal
total playback duration for that single leg is:

```text
finish_time[i] = start_time[i] + d
total_duration = max(finish_time[i])
               = d * (1 + max(delay_factor[i]))
```

For example, with `duration = 1` second and `f(x) = x`:

| Position | Starts | Finishes |
|---|---|---|
| First (`x = 0`) | 0 seconds | 1 second |
| Middle (`x = 0.5`) | 0.5 seconds | 1.5 seconds |
| Last (`x = 1`) | 1 second | 2 seconds |

The runtime tween must account for this extended lifetime. Completing it after
only `d` seconds would truncate delayed characters. A subsequent serial step
starts after the whole range completes.

### User-facing documentation contract

Constructor documentation, the character-animation guide, and Inspector field
descriptions must distinguish per-character duration from total range duration.
Use this wording consistently:

> `duration` is the animation duration of each character for one leg. The Char
> tween finishes when the last character finishes, including its start delay and
> requested loops.

Include the example above: `duration = 1` second with `f(x) = x` gives every
character a one-second animation and the whole tween a two-second duration.
Public duration accessors must clearly identify whether they report the supplied
per-character duration or the computed total duration.

## Continuous travelling loops

Apply each character's start delay once. After it starts, advance its own local
timeline continuously through its requested legs. Each character starts its next
leg when its own current leg finishes, keeping the travelling pattern across
repetitions.

For example, with `duration = 1` second and `f(x) = x`, the first character
begins its second leg at 1 second, when the last character begins its first leg.
The first character does not wait for the last character before repeating.

Use the existing loop modes on those local timelines: Restart repeats the leg,
Yoyo alternates direction, and Incremental advances the endpoints by the original
delta. Resolve the local leg and its direction before applying temporal easing.

Finite loop counts apply per character. A character that completes its final
leg retains the existing effect's completion behavior, including punch/shake
cleanup, while the remaining characters finish. The range completes after the
last selected character completes its requested legs. Infinite looping has no
natural range completion.

For `L` finite legs, the nominal range duration is
`d * (L + max(delay_factor[i]))`. Actual playback retains the existing
one-loop-per-update policy described below.

Advancing these local timelines still follows the existing one-loop-per-update
rule. A large time step must not traverse multiple legs for a character in one
update. Continuous travelling describes the staggered relationship between
characters; it does not change that processing policy.

## Runtime and integration constraints

Use one update callback to borrow the renderer, update selected character states,
and request redraw. Per-character initial values, shake paths, and additive
contributions still need their own state where applicable.

Keep existing logical character indexing, including spaces and excluding BBCode
tags. A one-character range supplies the single-character case. Missing indices
continue to ignore writes while playback progresses.

Preserve the existing runtime rule that a tween completes at most one loop per
processing update. Curve offsets must not introduce catch-up loops.

Replace Blueprint's `character_stagger` with the optional start-delay curve.
Blueprint character steps build one range tween using their selected range,
effect resource, and delay curve. A linear delay curve covers ordinary staggered
starts.

The existing preset `character_phase_offset` belongs to the separate continuous
preset-rendering workflow. This range-tween design does not specify changes to
that preset field.

## Proposed runtime representation

Introduce character-range tween data under the existing `SpireTween<T>`
lifecycle, exposed as a new `AnyTween` category. Following existing handle storage:

```rust
pub enum AnyTween {
    // Existing variants...
    Char(TweenPtr<CharTweenData>),
}
```

`CharTweenData` implements `ITweenable` and supplies the `SpireTweener` hooks for
`SpireTween<CharTweenData>`. It owns the target, effect configuration, sampled
delays, and per-character playback state. Existing common lifecycle controls
continue to supply playback state, speed, pause/process policy, outer delay, and
stop/finish subscriptions. Character processing supplies local timing, easing,
loop completion, and effect cleanup.

| Representation | Trade-off |
|---|---|
| Existing method tween with a stateful callback | Can drive a batched range, but its normal easing applies to the driver and its normal loops repeat the driver. Additional plumbing must route public controls to the characters' local timelines. |
| Dedicated character tween data | Makes local easing, continuous character loops, range completion, and forced completion explicit while retaining the existing common tween lifecycle. Requires integration with strong/weak handles, GDScript wrappers, sequence default easing, and owner tracking. |

The dedicated category is the recommended direction. It still has one handle and
one batched update for the range; per-character state remains internal data.

## Range loop events

`get_loops_finished()` reports the minimum completed-leg count across the
selected characters. Emit `loop_finished` once whenever that count advances:
every selected character must have completed the numbered leg being reported.

Leading characters continue immediately into their next leg while the remaining
characters catch up. The range event reports collective progress without
introducing a wait between local legs.
