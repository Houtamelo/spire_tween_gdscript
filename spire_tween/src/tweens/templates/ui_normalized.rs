use godot::builtin::Vector2Axis;

use super::{effect::*, motion::owned_progress, *};

fn pivot(mut node: Gd<Control>, to: Vector2, duration: f64, axis: Option<usize>) -> SpireTween<LerpMethodData<f64>> {
    let to = Vector2::new(unit(to.x as f64, 0.0, "pivot X") as real, unit(to.y as f64, 0.0, "pivot Y") as real);
    let mut start = None;
    owned_progress(node.clone().upcast(), duration_or_default(duration), move |p| {
        let size = node.get_size();
        let initial = *start.get_or_insert_with(|| {
            let offset = node.get_pivot_offset();
            Vector2::new(
                if size.x == 0.0 { 0.0 } else { offset.x / size.x },
                if size.y == 0.0 { 0.0 } else { offset.y / size.y },
            )
        });
        let ratio = initial.lerp(to, p as real).clamp(Vector2::ZERO, Vector2::ONE);
        let mut offset = node.get_pivot_offset();
        match axis {
            Some(i) => {
                let i = if i == 0 { Vector2Axis::X } else { Vector2Axis::Y };
                offset[i] = ratio[i] * size[i];
            }
            None => offset = ratio * size,
        }
        node.set_pivot_offset(offset);
    })
}

/// Size-aware pivot ratios in `[0,1]`. Each write uses the Control's current size;
/// axis variants preserve the other pixel component. Zero-sized axes start at
/// ratio zero. Invalid targets warn and clamp. Constructors return builders.
pub trait DoNormalizedPivot {
    /// Tweens both pivot ratios, multiplying by live size on each write.
    fn do_pivot(&self, to: Vector2, duration: f64) -> SpireTween<LerpMethodData<f64>>;
    /// Tweens only the horizontal pivot ratio, preserving the vertical pixels.
    fn do_pivot_x(&self, to: f64, duration: f64) -> SpireTween<LerpMethodData<f64>>;
    /// Tweens only the vertical pivot ratio, preserving the horizontal pixels.
    fn do_pivot_y(&self, to: f64, duration: f64) -> SpireTween<LerpMethodData<f64>>;
}
impl<C: Inherits<Control> + Inherits<Object>> DoNormalizedPivot for Gd<C> {
    fn do_pivot(&self, to: Vector2, duration: f64) -> SpireTween<LerpMethodData<f64>> {
        pivot(self.clone().upcast(), to, duration, None)
    }
    fn do_pivot_x(&self, to: f64, duration: f64) -> SpireTween<LerpMethodData<f64>> {
        pivot(self.clone().upcast(), Vector2::new(to as real, 0.0), duration, Some(0))
    }
    fn do_pivot_y(&self, to: f64, duration: f64) -> SpireTween<LerpMethodData<f64>> {
        pivot(self.clone().upcast(), Vector2::new(0.0, to as real), duration, Some(1))
    }
}

fn scroll_range(bar: &Range) -> (f64, f64) {
    let min = bar.get_min();
    (min, (bar.get_max() - bar.get_page() - min).max(0.0))
}
fn read_scroll(bar: &Range, vertical: bool) -> f64 {
    let (min, length) = scroll_range(bar);
    let ratio = if length > 0.0 { ((bar.get_value() - min) / length).clamp(0.0, 1.0) } else { 0.0 };
    if vertical { 1.0 - ratio } else { ratio }
}
fn write_scroll(bar: &mut Range, ratio: f64, vertical: bool) {
    let (min, length) = scroll_range(bar);
    let ratio = ratio.clamp(0.0, 1.0);
    bar.set_value(min + length * if vertical { 1.0 - ratio } else { ratio });
}
fn scroll(
    node: Gd<ScrollContainer>,
    to: Vector2,
    duration: f64,
    axis: Option<usize>,
) -> SpireTween<LerpMethodData<f64>> {
    let to = Vector2::new(unit(to.x as f64, 0.0, "scroll X") as real, unit(to.y as f64, 1.0, "scroll Y") as real);
    let mut start = None;
    owned_progress(node.clone().upcast(), duration_or_default(duration), move |p| {
        let (Some(horizontal), Some(vertical)) = (node.get_h_scroll_bar(), node.get_v_scroll_bar()) else {
            godot_warn!("[spire scroll] ScrollContainer has no scrollbars; skipping update.");
            return;
        };
        let mut horizontal = horizontal.upcast::<Range>();
        let mut vertical = vertical.upcast::<Range>();
        let initial = *start.get_or_insert_with(|| {
            Vector2::new(read_scroll(&horizontal, false) as real, read_scroll(&vertical, true) as real)
        });
        let ratio = initial.lerp(to, p as real);
        if axis != Some(1) {
            write_scroll(&mut horizontal, ratio.x as f64, false);
        }
        if axis != Some(0) {
            write_scroll(&mut vertical, ratio.y as f64, true);
        }
    })
}

/// Scroll ratios use the live scrollbar range minus page size. Horizontal zero
/// is left; vertical zero is bottom and one is top, matching ScrollRect. Axis
/// variants preserve the other scrollbar. Targets clamp to `[0,1]` with warnings;
/// empty ranges stay at minimum. Container layout may update ranges between steps.
pub trait DoNormalizedScroll {
    /// Tweens both live scroll ratios; vertical one is the top edge.
    fn do_normalized_position(&self, to: Vector2, duration: f64) -> SpireTween<LerpMethodData<f64>>;
    /// Tweens the left-to-right ratio without changing vertical scrolling.
    fn do_horizontal_normalized_position(&self, to: f64, duration: f64) -> SpireTween<LerpMethodData<f64>>;
    /// Tweens the bottom-to-top ratio without changing horizontal scrolling.
    fn do_vertical_normalized_position(&self, to: f64, duration: f64) -> SpireTween<LerpMethodData<f64>>;
}
impl<C: Inherits<ScrollContainer> + Inherits<Object>> DoNormalizedScroll for Gd<C> {
    fn do_normalized_position(&self, to: Vector2, duration: f64) -> SpireTween<LerpMethodData<f64>> {
        scroll(self.clone().upcast(), to, duration, None)
    }
    fn do_horizontal_normalized_position(&self, to: f64, duration: f64) -> SpireTween<LerpMethodData<f64>> {
        scroll(self.clone().upcast(), Vector2::new(to as real, 0.0), duration, Some(0))
    }
    fn do_vertical_normalized_position(&self, to: f64, duration: f64) -> SpireTween<LerpMethodData<f64>> {
        scroll(self.clone().upcast(), Vector2::new(0.0, to as real), duration, Some(1))
    }
}
