//! Observations from real glyph callbacks, kept outside the rendering Resource.
use std::cell::{LazyCell, RefCell};

use super::*;

thread_local! {
    static SAMPLES: RefCell<HashMap<InstanceId, HashMap<usize, (Transform2D, Color)>>> = RefCell::default();
}

pub(super) fn record(effect: InstanceId, index: usize, transform: Transform2D, color: Color) {
    SAMPLES.with_borrow_mut(|samples| {
        samples.entry(effect).or_default().insert(index, (transform, color));
    });
}

pub(super) fn clear(effect: InstanceId) {
    SAMPLES.with_borrow_mut(|samples| {
        samples.remove(&effect);
    });
}

impl SpireRichTextLabel {
    /// Last actual glyph rendering result. Available only in integration builds.
    #[doc(hidden)]
    pub fn sample_for_test(this: Gd<Self>, index: i64) -> Option<(Transform2D, Color)> {
        if !this.is_instance_valid() {
            return None;
        }
        let label = this.bind();
        let effect = LazyCell::get(&label.char_renderer)?;
        let index = effect.bind().resolve(index)?;
        SAMPLES.with_borrow(|samples| samples.get(&effect.instance_id())?.get(&index).copied())
    }
}
