use super::*;

/// Moves a `Node2D` along a polyline. `duration_or_speed` is distance/second if
/// `is_speed_based`, otherwise total duration distributed across segments.
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
