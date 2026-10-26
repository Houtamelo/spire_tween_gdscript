# Planar shake mathematical specification

This is the functional contract for the private sampler in
`spire_tween/src/tweens/templates/shake_path.rs`. It describes mathematics and
observable results. Engine ownership, clock integration, blending, and events
remain in the existing shake adapter. Inputs are normalized by that adapter and
are within ordinary supported ranges.

The implementation was authored in a fresh agent context supplied with this
specification and a private interface. That context was instructed to exclude
upstream code and the rejected generator. The specification's author had already
researched the upstream behavior; this records context separation, not legal
clearance or a claim that the entire development process was unexposed.

## Interface

The sampler uses the addon's shared `ShakeRandomnessMode::{Full, Harmonic}` enum
and accepts a mutable `rand::Rng`. The calculations make no engine runtime calls.
Provide crate-visible `ShakePath`:

* `ShakePath::new(segments: NonZeroUsize, strength: [f64; 2], randomness_degrees: f64,
  fade_out: bool, mode: ShakeRandomnessMode) -> Self`
* `regenerate(&mut self, rng: &mut impl rand::Rng)`
* `sample(&self, progress: f64) -> [f64; 2]`

Generation consumes uniform `f64` random draws in [0, 1) from the caller's RNG. It is
explicit. Sampling must be deterministic, consume no randomness, allocate nothing,
and support traversal in either direction. Allocate a waypoint buffer on first
generation and reuse it across later generations. One buffer is sufficient; keep
the implementation straightforward. Locate the containing segment in O(log N)
time or better, rather than scanning all earlier segments each frame. The type
`std::num::NonZeroUsize` excludes zero segments. One segment is an inert path
between zero endpoints; the public shake adapter clamps the generated count to
`[2, 65536]`. Nonpositive vibrato and counts above the upper bound emit warnings;
positive vibrato with a short duration uses the minimum without a warning. The
upper bound limits the waypoint buffer to approximately 1 MiB.
Strength components are finite and nonnegative. Randomness is within [0, 180]
degrees. Progress <= 0 and >= 1 evaluates to the exact zero offset. The integrating
driver is responsible for generating before sampling an interior progress value.

## Mathematical specification

There are N segments, an initial knot at offset zero/time zero, and N destination
knots. Index destinations by k=1..N. The last destination is exactly zero at
normalized time 1. For every k, the normalized knot time is:

* without fading: T(k) = k / N
* with fading: T(k) = k(k+1) / [N(N+1)]

Interior knots k=1..N-1 have magnitude A(k). Put S = hypot(strength_x, strength_y):

* without fading: A(k) = S
* with fading: A(k) = S * [1 - (k-1)/N]

For zero strength every knot and sample is zero. Direction angles are radians.
Draw the initial angle uniformly in [0, 2*pi). Set R to randomness in radians.
Let F(u) = (2*u - 1)*R for Full, or u*R for Harmonic. For each interior knot:

* the first uses the initial angle theta(1);
* each later angle satisfies theta(k) = theta(k-1) - pi + F(u);
* each interior knot also has an independent tilt phi(k) = F(v).

Random draw order: initial angle; first tilt; next turn and tilt; repeat for each
remaining interior knot. The final zero knot consumes no draws.

Strength is direction shaping, not independent component bounds. Define
alpha = strength_x / S and beta = strength_y / S. For theta and phi at a knot:

    x = alpha * cos(theta) * cos(phi)
    y = sin(theta)
    z = cos(theta) * sin(phi)
    d = sqrt(x*x + y*y + z*z)
    h = 1                         if d <= beta
        beta / d                  otherwise
    y_shaped = y * h
    L = hypot(x, y_shaped)

The knot is A(k) * (x, y_shaped) / L when L > 0, otherwise zero. The branch for h
also avoids dividing by zero when d is zero. Derive the implementation from these
equations in a suitable native representation; no 3D vector or rotation type is
needed. This shaping deliberately couples the axes and can produce components
larger than their input strengths.

For a progress value between two neighboring knots, return their ordinary linear
interpolation using that segment's own normalized start/end times. Position is
continuous at knots; continuous velocity is not required.

## Adapter playback contract

The adapter generates one path per playback. Restart and Incremental repeat the
same path without drift; Yoyo retraces it. It samples the method value modulo one,
so integer values produce zero offset and positive or negative easing overshoot
wraps around the path. The method's start and end values remain `0` and `1`.

An immediate native `stopped` connection invalidates the path for replay without
changing position or discarding the previous offset. Pause/resume retains the
path. An immediate native `finished` connection restores the tracked contribution
before later finish handlers; restoration is not guaranteed before stopped
handlers. This covers natural and forced completion even with a nonzero custom
eased endpoint. The finish connection skips access if the owner was freed during
stopped notification. Clearing these native connections removes their behavior.

Only template state is shared between callbacks. The adapter neither reads the
tween during processing nor adds clock metadata to method tweens.

## Examples for verification

For N=4, nonfading knot times are .25, .5, .75, 1. Fading times are .1, .3, .6, 1.
For strength=(3,4), the corresponding interior magnitudes are 5,5,5 without fading
or 5,3.75,2.5 with fading. Both end at zero.

For zero randomness, interior directions alternate sign. Thus at the midpoint
of the first constant-timing segment the output is half the first waypoint;
midway between equal-amplitude opposite waypoints the output is zero.

The sampler has equation, zero-strength, disabled-axis, boundary, regeneration,
and buffer-reuse tests. Real-engine tests pass an explicitly seeded
`Xoshiro256PlusPlus` to `do_shake_internal` and step the resulting tween on Godot
nodes. The public constructor creates its RNG and forwards to that same
implementation; there is no global random-input override. Seed `12345` produces
these first six unit inputs:

    [0.5530478066930038, 0.20495565689034478, 0.08512324022636453,
     0.17552997631905642, 0.2656135558172664, 0.7256094986680182]

For strength `(3,4)`, randomness `90` degrees, N=4 and fading, the three interior
knots computed from the equations above are:

| Mode | T=.1 | T=.3 | T=.6 |
|---|---|---|---|
| Full | (-3.7853454846, -3.2666740827) | (0.9489967272, -3.6279340142) | (0.1929966527, 2.4925393261) |
| Harmonic | (-4.2717486508, -2.5984925368) | (2.8244572807, 2.4667673319) | (-0.6085566292, -2.4248007813) |

These values were calculated from the specification, not recorded from the
discarded implementation. The helper stays in a private implementation module;
only the `integration-tests` feature re-exports it for the separate test crate.
It is absent from the ordinary public API and from the GDScript API.
