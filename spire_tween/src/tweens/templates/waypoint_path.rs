//! Independently authored curve evaluation and approximate arc-length inversion.
use super::{effect::duration_or_default, motion::owned_progress, *};

/// Copied when a path is constructed. In GDScript this is SpirePathOptions.new().
/// Waypoints and fixed look-at positions use the constructor's local/global space.
/// Target nodes are always resolved in world space. Axis masks: X=1, Y=2, Z=4.
#[derive(Clone)]
#[cfg_attr(feature = "include_gdscript_bridge", derive(GodotClass))]
#[cfg_attr(feature = "include_gdscript_bridge", class(base = RefCounted, rename = SpirePathOptions))]
pub struct PathOptions {
    #[cfg_attr(feature = "include_gdscript_bridge", var)]
    pub path_type: PathType,
    /// Samples per curved segment; clamps to 3..4096 with a warning. Three
    /// samples distinguish every nonconstant cubic from a stationary segment.
    #[cfg_attr(feature = "include_gdscript_bridge", var)]
    pub resolution: i32,
    #[cfg_attr(feature = "include_gdscript_bridge", var)]
    pub closed: bool,
    #[cfg_attr(feature = "include_gdscript_bridge", var)]
    pub lock_position: i32,
    #[cfg_attr(feature = "include_gdscript_bridge", var)]
    pub lock_rotation: i32,
    #[cfg_attr(feature = "include_gdscript_bridge", var)]
    pub orient: bool,
    #[cfg_attr(feature = "include_gdscript_bridge", var)]
    pub mode: PathMode,
    /// Fraction of total path length used by tangent orientation. Clamps to 0..1.
    #[cfg_attr(feature = "include_gdscript_bridge", var)]
    pub look_ahead: f64,
    #[cfg_attr(feature = "include_gdscript_bridge", var)]
    pub use_look_at_position: bool,
    /// Node2D/Control use X and Y; Z is ignored.
    #[cfg_attr(feature = "include_gdscript_bridge", var)]
    pub look_at_position: Vector3,
    /// Node2D/Control for 2D, Node3D for 3D. Takes priority over the fixed point.
    #[cfg_attr(feature = "include_gdscript_bridge", var)]
    pub look_at_target: Option<Gd<Node>>,
    /// 3D model forward direction. In 2D XY selects forward; zero XY means +X.
    #[cfg_attr(feature = "include_gdscript_bridge", var)]
    pub forward_direction: Vector3,
    #[cfg_attr(feature = "include_gdscript_bridge", var)]
    pub up: Vector3,
    #[cfg_attr(feature = "include_gdscript_bridge", var)]
    pub stable_z_rotation: bool,
}

impl Default for PathOptions {
    fn default() -> Self {
        Self {
            path_type: PathType::Linear,
            resolution: 10,
            closed: false,
            lock_position: 0,
            lock_rotation: 0,
            orient: false,
            mode: PathMode::Full3D,
            look_ahead: 0.01,
            use_look_at_position: false,
            look_at_position: Vector3::ZERO,
            look_at_target: None,
            forward_direction: Vector3::FORWARD,
            up: Vector3::UP,
            stable_z_rotation: false,
        }
    }
}

#[cfg(feature = "include_gdscript_bridge")]
#[godot_api]
impl IRefCounted for PathOptions {
    fn init(_base: Base<RefCounted>) -> Self { Self::default() }
}

impl PathOptions {
    fn clean(mut self, spatial: bool) -> Self {
        if !(3..=4096).contains(&self.resolution) {
            godot_warn!("[spire path] resolution must be in [3, 4096], got {}; clamping.", self.resolution);
            self.resolution = self.resolution.clamp(3, 4096);
        }
        if !self.look_ahead.is_finite() || !(0.0..=1.0).contains(&self.look_ahead) {
            godot_warn!("[spire path] look_ahead must be in [0, 1], got {}; normalizing.", self.look_ahead);
            self.look_ahead = if self.look_ahead.is_finite() { self.look_ahead.clamp(0.0, 1.0) } else { 0.01 };
        }
        for (name, vector, fallback) in [
            ("up", &mut self.up, Vector3::UP),
            ("forward_direction", &mut self.forward_direction, Vector3::FORWARD),
        ] {
            if !vector.is_finite() || vector.length_squared() < 0.00000001 {
                godot_warn!("[spire path] {name} must be finite and nonzero, got {vector}; using {fallback}.");
                *vector = fallback;
            } else {
                *vector = vector.normalized();
            }
        }
        if !self.look_at_position.is_finite() {
            godot_warn!("[spire path] look_at_position must be finite, got {}; using zero.", self.look_at_position);
            self.look_at_position = Vector3::ZERO;
        }
        for (name, mask) in [
            ("lock_position", &mut self.lock_position),
            ("lock_rotation", &mut self.lock_rotation),
        ] {
            if *mask < 0 || *mask > 7 {
                godot_warn!("[spire path] {name} mask must be in [0, 7], got {mask}; keeping XYZ bits.");
                *mask &= 7;
            }
        }
        if self.look_at_target.as_ref().is_some_and(|target| {
            if spatial { !target.is_class("Node3D") } else { !target.is_class("Node2D") && !target.is_class("Control") }
        }) {
            godot_warn!("[spire path] incompatible look_at_target {:?}; using path direction.", self.look_at_target);
            self.look_at_target = None;
        }
        self
    }
}

struct Segment {
    points: [Vector3; 4],
    kind:   PathType,
}
impl Segment {
    fn at(&self, t: f64) -> Vector3 {
        let [a, b, c, d] = self.points;
        let t = t as real;
        match self.kind {
            PathType::Linear => a.lerp(d, t),
            PathType::CubicBezier => {
                let u = 1.0 - t;
                a * (u * u * u) + b * (3.0 * u * u * t) + c * (3.0 * u * t * t) + d * (t * t * t)
            }
            PathType::CatmullRom => {
                // a,d are neighboring control points; the segment goes b -> c.
                (b * 2.0
                    + (c - a) * t
                    + (a * 2.0 - b * 5.0 + c * 4.0 - d) * (t * t)
                    + (-a + b * 3.0 - c * 3.0 + d) * (t * t * t))
                    * 0.5
            }
        }
    }
}

struct ArcSample {
    distance: f64,
    segment: usize,
    t: f64,
}
struct PathGeometry {
    segments: Vec<Segment>,
    samples: Vec<ArcSample>,
    length: f64,
    start: Vector3,
    end: Vector3,
}

impl PathGeometry {
    fn new(start: Vector3, raw: &[Vector3], options: &PathOptions) -> Self {
        let mut points = vec![start];
        for (index, point) in raw.iter().copied().enumerate() {
            let point = if point.is_finite() {
                point
            } else {
                godot_warn!("[spire path] waypoint {index} must be finite, got {point}; using the preceding point.");
                *points.last().unwrap_or(&start)
            };
            points.push(point);
        }
        // Locks constrain the curve before building its distance table, so speed
        // is measured along the curve that will actually be written.
        for point in &mut points {
            if options.lock_position & 1 != 0 {
                point.x = start.x;
            }
            if options.lock_position & 2 != 0 {
                point.y = start.y;
            }
            if options.lock_position & 4 != 0 {
                point.z = start.z;
            }
        }
        let mut segments = Vec::new();
        match options.path_type {
            PathType::Linear => {
                if options.closed && points.last() != Some(&start) {
                    points.push(start);
                }
                for pair in points.windows(2) {
                    segments.push(Segment {
                        points: [pair[0], pair[0], pair[1], pair[1]],
                        kind:   PathType::Linear,
                    });
                }
            }
            PathType::CatmullRom => {
                if options.closed && points.last() != Some(&start) {
                    points.push(start);
                }
                for i in 0..points.len().saturating_sub(1) {
                    let b = points[i];
                    let c = points[i + 1];
                    let a = if i > 0 {
                        points[i - 1]
                    } else if options.closed && points.len() > 2 {
                        points[points.len() - 2]
                    } else {
                        b * 2.0 - c
                    };
                    let d = if i + 2 < points.len() {
                        points[i + 2]
                    } else if options.closed && points.len() > 2 {
                        points[1]
                    } else {
                        c * 2.0 - b
                    };
                    segments.push(Segment {
                        points: [a, b, c, d],
                        kind:   PathType::CatmullRom,
                    });
                }
            }
            PathType::CubicBezier => {
                if raw.len() % 3 != 0 {
                    godot_warn!(
                        "[spire path] cubic Bezier requires complete triples, got {} values; discarding {} trailing \
                         values.",
                        raw.len(),
                        raw.len() % 3
                    );
                }
                let mut previous = start;
                for triple in points[1..].chunks_exact(3) {
                    let [destination, outgoing, incoming] = [triple[0], triple[1], triple[2]];
                    segments.push(Segment {
                        points: [previous, outgoing, incoming, destination],
                        kind:   PathType::CubicBezier,
                    });
                    previous = destination;
                }
                if options.closed && previous != start {
                    // The implicit closing edge has no user-supplied handles.
                    segments.push(Segment {
                        points: [previous, previous, start, start],
                        kind:   PathType::Linear,
                    });
                }
            }
        }
        let end = segments.last().map_or(start, |s| s.at(1.0));
        let mut samples = vec![ArcSample {
            distance: 0.0,
            segment: 0,
            t: 0.0,
        }];
        let mut length = 0.0;
        let mut previous = start;
        for (index, segment) in segments.iter().enumerate() {
            let resolution = if segment.kind == PathType::Linear { 1 } else { options.resolution };
            for i in 1..=resolution {
                let t = f64::from(i) / f64::from(resolution);
                let point = segment.at(t);
                length += previous.distance_to(point) as f64;
                samples.push(ArcSample {
                    distance: length,
                    segment: index,
                    t,
                });
                previous = point;
            }
        }
        Self {
            segments,
            samples,
            length,
            start,
            end,
        }
    }

    fn sample(&self, p: f64) -> Vector3 {
        if self.length <= 0.0 || self.segments.is_empty() {
            return self.start;
        }
        // Integer progress translates the complete path, making open incremental
        // loops continuous. Closed paths repeat in place. Negative progress works too.
        let cycle = p.floor();
        let fraction = p - cycle;
        let distance = fraction * self.length;
        let next = self
            .samples
            .partition_point(|s| s.distance <= distance)
            .min(self.samples.len() - 1);
        let upper = &self.samples[next];
        let (lower_distance, lower_t) = if next == 0 {
            (0.0, 0.0)
        } else {
            let lower = &self.samples[next - 1];
            (lower.distance, if lower.segment == upper.segment { lower.t } else { 0.0 })
        };
        let span = upper.distance - lower_distance;
        let t = if span > 0.0 { lower_t + (upper.t - lower_t) * ((distance - lower_distance) / span) } else { upper.t };
        self.segments[upper.segment].at(t) + (self.end - self.start) * cycle as real
    }

    fn locked_sample(&self, p: f64, mask: i32) -> Vector3 {
        let mut point = self.sample(p);
        if mask & 1 != 0 {
            point.x = self.start.x;
        }
        if mask & 2 != 0 {
            point.y = self.start.y;
        }
        if mask & 4 != 0 {
            point.z = self.start.z;
        }
        point
    }

    fn aim(&self, p: f64, sign: f64, options: &PathOptions) -> Vector3 {
        let epsilon = options.look_ahead.max(0.0001);
        let current = self.locked_sample(p, options.lock_position);
        let lower = if p > 0.0 && p.fract() == 0.0 { p - 1.0 } else { p.floor() };
        let next = (p + sign * epsilon).clamp(lower, lower + 1.0);
        let mut ahead = self.locked_sample(next, options.lock_position);
        if ahead.distance_squared_to(current) < 0.00000001 {
            let behind = (p - sign * epsilon).clamp(lower, lower + 1.0);
            ahead = current + (current - self.locked_sample(behind, options.lock_position));
        }
        ahead
    }
}

macro_rules! path_2d {
    ($trait:ident, $class:ty) => {
        /// Distance-parameterized waypoint paths. Start captured on first step.
        /// Native constructors are unregistered builders. See [`PathOptions`].
        pub trait $trait {
            /// Local-space waypoints, including cubic control points.
            fn do_path(
                &self,
                waypoints: Vec<Vector2>,
                duration: f64,
                options: PathOptions,
            ) -> SpireTween<LerpMethodData<f64>>;
            /// World-space counterpart of [`Self::do_path`].
            fn do_global_path(
                &self,
                waypoints: Vec<Vector2>,
                duration: f64,
                options: PathOptions,
            ) -> SpireTween<LerpMethodData<f64>>;
        }
        impl<C: Inherits<$class> + Inherits<Object>> $trait for Gd<C> {
            fn do_path(
                &self,
                waypoints: Vec<Vector2>,
                duration: f64,
                options: PathOptions,
            ) -> SpireTween<LerpMethodData<f64>> {
                path_2d_impl(self.clone().upcast::<$class>().upcast(), waypoints, duration, options, false)
            }
            fn do_global_path(
                &self,
                waypoints: Vec<Vector2>,
                duration: f64,
                options: PathOptions,
            ) -> SpireTween<LerpMethodData<f64>> {
                path_2d_impl(self.clone().upcast::<$class>().upcast(), waypoints, duration, options, true)
            }
        }
    };
}

// Node2D and Control share CanvasItem transforms but have different setters.
fn position_2d(node: &Gd<Object>, global: bool) -> Vector2 {
    node.get(if global { "global_position" } else { "position" })
        .to::<Vector2>()
}
fn parent_transform_2d(canvas: &CanvasItem) -> Transform2D {
    if canvas.is_set_as_top_level() {
        return Transform2D::IDENTITY;
    }
    canvas
        .get_parent()
        .and_then(|node| node.try_cast::<CanvasItem>().ok())
        .map_or(Transform2D::IDENTITY, |parent| parent.get_global_transform())
}
fn world_2d(node: &Gd<Object>, point: Vector3, global: bool) -> Vector2 {
    let point = Vector2::new(point.x, point.y);
    if global {
        return point;
    }
    let canvas = node.clone().cast::<CanvasItem>();
    // Waypoints are relative to the parent, not the moving node itself.
    parent_transform_2d(&canvas) * point
}
fn target_2d(target: &Gd<Node>) -> Vector2 { target.get("global_position").to::<Vector2>() }

fn rotation_2d(node: &Gd<Object>) -> f64 { node.clone().cast::<CanvasItem>().get_global_transform().rotation() as f64 }
fn set_rotation_2d(node: &mut Gd<Object>, value: f64) {
    if let Ok(mut spatial) = node.clone().try_cast::<Node2D>() {
        spatial.set_global_rotation(value as real);
    } else {
        let mut control = node.clone().cast::<Control>();
        let parent = parent_transform_2d(&control);
        if parent.determinant().abs() < 0.00000001 {
            return;
        }
        let local_direction = parent.affine_inverse().basis_xform(Vector2::from_angle(value as real));
        control.set_rotation(local_direction.angle());
    }
}

fn path_2d_impl(
    mut node: Gd<Object>,
    waypoints: Vec<Vector2>,
    duration: f64,
    options: PathOptions,
    global: bool,
) -> SpireTween<LerpMethodData<f64>> {
    let options = options.clean(false);
    let target_binding =
        if options.orient && options.mode != PathMode::Ignore { options.look_at_target.clone() } else { None };
    let waypoints = waypoints
        .into_iter()
        .map(|p| Vector3::new(p.x, p.y, 0.0))
        .collect::<Vec<_>>();
    let mut geometry: Option<PathGeometry> = None;
    let mut initial_rotation = None;
    let mut tween = owned_progress(node.clone(), duration_or_default(duration), move |p| {
        let geometry = geometry.get_or_insert_with(|| {
            let start = position_2d(&node, global);
            PathGeometry::new(Vector3::new(start.x, start.y, 0.0), &waypoints, &options)
        });
        let initial_rotation = *initial_rotation.get_or_insert_with(|| rotation_2d(&node));
        let position = geometry.locked_sample(p, options.lock_position);
        node.set(
            if global { "global_position" } else { "position" },
            &Vector2::new(position.x, position.y).to_variant(),
        );
        if options.orient
            && options.mode != PathMode::Ignore
            && options.lock_rotation & 4 == 0
            && !options.stable_z_rotation
        {
            let current = position_2d(&node, true);
            let aim = if let Some(target) = &options.look_at_target {
                target_2d(target)
            } else if options.use_look_at_position {
                world_2d(&node, options.look_at_position, global)
            } else {
                world_2d(&node, geometry.aim(p, 1.0, &options), global)
            };
            let direction = aim - current;
            if direction.length_squared() > 0.00000001 {
                let forward = Vector2::new(options.forward_direction.x, options.forward_direction.y);
                let rotation =
                    direction.angle() - if forward.length_squared() > 0.00000001 { forward.angle() } else { 0.0 };
                set_rotation_2d(&mut node, rotation as f64);
            }
        } else if options.orient && options.mode != PathMode::Ignore {
            set_rotation_2d(&mut node, initial_rotation);
        }
    });
    if let Some(target) = target_binding {
        tween = tween.bound_to(target);
    }
    tween
}

path_2d!(DoPath2D, Node2D);
path_2d!(DoPathControl, Control);

/// 3D local/global waypoint paths with scale-preserving orientation.
pub trait DoPath3D {
    fn do_path(&self, waypoints: Vec<Vector3>, duration: f64, options: PathOptions) -> SpireTween<LerpMethodData<f64>>;
    fn do_global_path(
        &self,
        waypoints: Vec<Vector3>,
        duration: f64,
        options: PathOptions,
    ) -> SpireTween<LerpMethodData<f64>>;
}
impl<C: Inherits<Node3D> + Inherits<Object>> DoPath3D for Gd<C> {
    fn do_path(&self, waypoints: Vec<Vector3>, duration: f64, options: PathOptions) -> SpireTween<LerpMethodData<f64>> {
        path_3d(self.clone().upcast(), waypoints, duration, options, false)
    }
    fn do_global_path(
        &self,
        waypoints: Vec<Vector3>,
        duration: f64,
        options: PathOptions,
    ) -> SpireTween<LerpMethodData<f64>> {
        path_3d(self.clone().upcast(), waypoints, duration, options, true)
    }
}

fn world_3d(node: &Node3D, point: Vector3, global: bool) -> Vector3 {
    if global || node.is_set_as_top_level() {
        point
    } else {
        node.get_parent_node_3d()
            .map_or(point, |parent| parent.get_global_transform() * point)
    }
}
fn path_3d(
    mut node: Gd<Node3D>,
    waypoints: Vec<Vector3>,
    duration: f64,
    options: PathOptions,
    global: bool,
) -> SpireTween<LerpMethodData<f64>> {
    let options = options.clean(true);
    let target_binding =
        if options.orient && options.mode != PathMode::Ignore { options.look_at_target.clone() } else { None };
    let mut geometry: Option<PathGeometry> = None;
    let mut initial_rotation = None;
    let mut tween = owned_progress(node.clone().upcast(), duration_or_default(duration), move |p| {
        let geometry = geometry.get_or_insert_with(|| {
            PathGeometry::new(
                if global { node.get_global_position() } else { node.get_position() },
                &waypoints,
                &options,
            )
        });
        let initial = *initial_rotation.get_or_insert_with(|| node.get_global_rotation());
        let position = geometry.locked_sample(p, options.lock_position);
        if global {
            node.set_global_position(position);
        } else {
            node.set_position(position);
        }
        if options.orient && options.mode != PathMode::Ignore {
            let current = node.get_global_position();
            let aim = if let Some(target) = &options.look_at_target {
                target.clone().cast::<Node3D>().get_global_position()
            } else if options.use_look_at_position {
                world_3d(&node, options.look_at_position, global)
            } else {
                world_3d(&node, geometry.aim(p, 1.0, &options), global)
            };
            let direction = aim - current;
            let candidate = match options.mode {
                PathMode::SideScroller2D if direction.x * direction.x + direction.y * direction.y > 0.00000001 => {
                    Some(Vector3::new(initial.x, initial.y, direction.y.atan2(direction.x)))
                }
                PathMode::TopDown2D if direction.x * direction.x + direction.z * direction.z > 0.00000001 => {
                    Some(Vector3::new(initial.x, (-direction.x).atan2(-direction.z), initial.z))
                }
                PathMode::Full3D => orientation::look_rotation(direction, options.up, false).and_then(|q| {
                    orientation::look_rotation(options.forward_direction, options.up, false)
                        .map(|correction| (q * correction.inverse()).get_euler())
                }),
                _ => None,
            };
            if let Some(mut euler) = candidate {
                let mask = options.lock_rotation | if options.stable_z_rotation { 4 } else { 0 };
                if mask & 1 != 0 {
                    euler.x = initial.x;
                }
                if mask & 2 != 0 {
                    euler.y = initial.y;
                }
                if mask & 4 != 0 {
                    euler.z = initial.z;
                }
                orientation::set_rotation(&mut node, Quaternion::from_euler(euler), true);
            }
        }
    });
    if let Some(target) = target_binding {
        tween = tween.bound_to(target);
    }
    tween
}
