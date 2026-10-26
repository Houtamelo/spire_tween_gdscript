# Group 3 mathematical contracts

These adapters use the existing float method clock. Formulas and behavior are
implemented independently; no DOTween source code is copied or translated.
The public API comparison is pinned in `dotween_api_inventory.csv`.

## Punch and shake

Let `N = clamp(floor(vibrato * duration), 2, 65536)`. Punch knot times are
`t(k) = k(k+1)/(N(N+1))`. Its scalar knots are zero at `k=0,N`, and otherwise
`(1-(k-1)/N) * (1 for odd k, -elasticity for even k)`. Interpolate neighboring
knots linearly. Multiply the result by the requested punch vector. This is the
alternating, linearly decaying waypoint construction, not a sinusoidal substitute.
Elasticity is clamped to `[0,1]`.

The 3D shake extends the existing planar specification. A random angle theta
and yaw phi produce `(cos(theta)*cos(phi), sin(theta), -cos(theta)*sin(phi))`.
For each axis in X/Y/Z order, multiply that component by
`min(1, strength_axis / (strength_length * current_vector_length))`.
Normalize the resulting vector to the current waypoint magnitude. All-zero
strength produces zero. Turning and fading use the existing planar knot rules.
The intermediate lengths are significant: component shaping is sequential.

Effects use the approved shake lifecycle: initial value captured on first step,
one path per playback, periodic phase for loops, replay regeneration, stop without
target mutation, and successful completion restoration. Linear default behavior
can be changed through ordinary easing. Generic Rust accessors require explicit
node binding when their closures access manually freed objects.

## Orientation

Quaternion targets use normalized shortest-arc spherical interpolation. Euler
Fast takes the per-component shortest signed angle; FastBeyond360 preserves
the requested winding. WorldAxisAdd premultiplies the initial world rotation;
LocalAxisAdd postmultiplies the initial local rotation. Angular arguments are
radians, following Godot. Blendable rotations apply successive quaternion deltas,
whose multiplication order is observable because 3D rotations do not commute.

Look-at uses Godot's -Z forward convention by default. Construct an orthonormal
frame from the direction and up vector, then follow shortest Euler routes from
the initially captured orientation. Static look-at captures the destination once; dynamic
look-at recomputes it as the source moves. A node-target variant also tracks a
moving target and binds its lifetime. Degenerate direction/up combinations retain
the previous valid orientation and emit a warning. Axis constraints preserve the
unselected initial Euler components throughout processing, not just at the endpoint.

## Normalized UI

Pivot ratios multiply the Control's current size on every write. Component
variants preserve the other pixel component. Scroll ratios map the scrollbar's
`min_value .. max(min_value, max_value-page)` range on every write. X increases
rightwards; normalized Y follows Unity's bottom=0, top=1 convention. Values are
clamped to `[0,1]`, and an empty range remains at its minimum.

## 3D spiral

The public Pro documentation describes axis, expanding/contracting motion,
rotation speed, frequency, depth, and snapping, but does not specify equations.
Our explicit contract is an Archimedean spiral in an orthonormal axis frame:
`travel = duration * speed * progress`, `angle = frequency * travel`, radius
`travel` for Expand, or a triangular out-and-back radius for ExpandThenContract.
Axial displacement is `depth * travel`; subtract the initial planar offset so
playback starts at the captured position. Frequency is positive, speed and depth
may be signed, and snapping rounds the resulting position. Pro source was not
inspected; numerical equivalence to its undocumented implementation is not asserted.

References: [DOTween API documentation](https://dotween.demigiant.com/documentation.php),
[Godot Quaternion](https://docs.godotengine.org/en/4.4/classes/class_quaternion.html),
[Godot Node3D](https://docs.godotengine.org/en/4.4/classes/class_node3d.html).
