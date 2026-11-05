use super::*;

// Weak ownership is essential: state owns this Resource, and the label's stack
// also retains it. Neither rendering nor tween closures own the processing node.
#[derive(GodotClass)]
#[class(base = RichTextEffect, no_init)]
pub(super) struct SpireCharacterEffect {
    base:  Base<RichTextEffect>,
    state: WeakPtr<AnimatorState>,
}
impl SpireCharacterEffect {
    pub(super) fn new(base: Base<RichTextEffect>, state: WeakPtr<AnimatorState>) -> Self { Self { base, state } }
}

#[godot_api]
impl IRichTextEffect for SpireCharacterEffect {
    fn process_custom_fx(&self, char_fx: Option<Gd<CharFxTransform>>) -> bool {
        let (Some(state), Some(mut fx)) = (self.state.upgrade(), char_fx) else { return true };
        let state = state.to_mut();
        if !state.alive {
            return true;
        }
        let range = fx.get_range();
        let Ok(start) = usize::try_from(range.x) else { return true };
        if start >= state.characters.len() {
            return true;
        }
        let end = (range.y.max(range.x + 1) as usize).min(state.characters.len());
        let mut character = state.characters[start];
        // A shaped glyph may represent multiple logical characters. Merge their
        // channels independently so writes before the first draw are preserved.
        for index in start..end {
            character.merge(state.characters[state.characters[index].cluster]);
            state.characters[index].cluster = start;
        }
        character.cluster = start;
        let incoming = fx.get_transform();
        let frame = Engine::singleton().get_process_frames();
        if fx.get_glyph_count() > 0 || character.pen_origin.is_none_or(|(last, _)| last != frame) {
            character.pen_origin = Some((frame, incoming.origin));
        }
        if !fx.is_outline() {
            character.baseline_color = fx.get_color();
        }
        state.characters[start] = character;

        let basis = Basis::from_quaternion(Quaternion::from_euler(character.rotation.value));
        // Orthographic XY projection of the rotated/scaled flat glyph: exact
        // affine projection, including X/Y tilts, skew, reflections and edge-on.
        let x = Vector2::new(basis.rows[0].x, basis.rows[1].x) * character.scale.value.x;
        let y = Vector2::new(basis.rows[0].y, basis.rows[1].y) * character.scale.value.y;
        let cluster_origin = character.pen_origin.map_or(incoming.origin, |(_, origin)| origin);
        let pivot = character.pivot.value + cluster_origin - incoming.origin;
        let projected = Transform2D::from_cols(x, y, character.offset.value + pivot - x * pivot.x - y * pivot.y);
        fx.set_transform(incoming * projected);

        let mut color = fx.get_color();
        if !fx.is_outline() {
            if let Some(override_color) = character.color.value {
                color = override_color;
            }
        }
        if let Some(alpha) = character.alpha.value {
            color.a = alpha as f32;
        }
        fx.set_color(color);
        #[cfg(feature = "integration-tests")]
        if !fx.is_outline() {
            state.samples.insert(start, (fx.get_transform(), color));
        }
        true
    }
}
