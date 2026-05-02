use super::*;

/// Moves a `Node2D` along a polyline traced by `vertices` (in global coordinates),
/// returning a [`Sequence`] whose blocks are one-segment `do_move` tweens chained
/// end-to-end.
///
/// The `duration_or_speed` parameter is interpreted as **seconds in both modes**;
/// the `is_speed_based` flag only changes how that duration is divided across
/// segments:
/// - `is_speed_based == false` — each segment runs for `duration_or_speed`
///   seconds. Total = `N * duration_or_speed`. Segment speeds vary by length.
/// - `is_speed_based == true` — `duration_or_speed` is the **total** path
///   duration. Each segment's share is `total_duration * (segment_length / total_perimeter)`,
///   so longer segments take proportionally longer. Sprite speed across the path
///   is constant: `total_perimeter / duration_or_speed` units per second.
///
/// Implemented for any `Gd<T: Inherits<Node2D>>`. Returns an unregistered
/// [`SpireTween<Sequence>`] — call [`register`](SpireTween::register).
///
/// Logs a `godot_error!` and returns an empty sequence when the polyline has zero
/// total length.
pub trait DoContourShape2D<Marker = ()> {
    fn do_contour_shape(
        &self,
        vertices: Array<Vector2>,
        duration_or_speed: f64,
        is_speed_based: bool,
    ) -> SpireTween<Sequence>;
}

impl<T: Inherits<Node2D> + Inherits<Object>> DoContourShape2D<()> for Gd<T> {
    fn do_contour_shape(
        &self,
        vertices: Array<Vector2>,
        duration_or_speed: f64,
        is_speed_based: bool,
    ) -> SpireTween<Sequence> {
        let owner: Gd<Node2D> = self.clone().upcast();
        let start_pos = owner.get_global_position();

        let (track_len, _) = vertices
            .iter_shared()
            .fold((0.0, start_pos), |(len, last_pos), next| (len + last_pos.distance_to(next), next));

        if track_len.approx_eq(&0.0) {
            godot_error!("Contour shape has zero length, returning empty sequence.");
            return SpireTween::<Sequence>::new();
        }

        let mut last_pos = start_pos;
        let mut seq = SpireTween::<Sequence>::new();

        for next in vertices.iter_shared() {
            let tween = if is_speed_based {
                let segment_len_ratio = (last_pos.distance_to(next) / track_len) as f64;
                let segment_duration = duration_or_speed * segment_len_ratio;
                owner.do_move(next, segment_duration)
            } else {
                owner.do_move(next, duration_or_speed)
            };

            seq.append(tween);
            last_pos = next;
        }

        seq
    }
}
