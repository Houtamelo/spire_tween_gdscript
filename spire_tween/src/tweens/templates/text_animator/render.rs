use super::*;

pub(super) fn cluster_pivot(incoming: Transform2D, pivot: Vector2, cluster_origin: Vector2) -> Vector2 {
    let determinant = incoming.determinant();
    // Earlier effects may scale/rotate the coordinate system. Convert the
    // cluster anchor back to that system before composing another transform.
    let local = if determinant != 0.0 && determinant.is_finite() {
        incoming.affine_inverse() * cluster_origin
    } else {
        Vector2::ZERO
    };
    pivot + local
}

pub(super) fn projected_transform(offset: Vector2, rotation: Vector3, scale: Vector2, pivot: Vector2) -> Transform2D {
    let basis = Basis::from_quaternion(Quaternion::from_euler(rotation));
    let x = Vector2::new(basis.rows[0].x, basis.rows[1].x) * scale.x;
    let y = Vector2::new(basis.rows[0].y, basis.rows[1].y) * scale.y;
    Transform2D::from_cols(x, y, offset + pivot - x * pivot.x - y * pivot.y)
}

#[derive(GodotClass)]
#[class(base = RichTextEffect, init, internal)]
pub struct SpireCharRenderer {
    base: Base<RichTextEffect>,
    pub(super) chars: Vec<CharState>,
    stamp: u64,
    pub epoch: u64,
}

impl SpireCharRenderer {
    /// Resets all deformations without rebuilding the text or changing playback.
    pub fn reset(&mut self) {
        for c in &mut self.chars {
            *c = CharState::new(c.cluster, c.baseline_color);
        }
        self.epoch = self.epoch.wrapping_add(1);
    }

    /// Number of logical characters, including spaces/newlines, excluding BBCode
    /// syntax. Indices use Godot's parsed text order, not visual/bidirectional order.
    pub fn character_count(&self) -> i64 { self.chars.len() as i64 }

    /// Whether this numeric logical character index still exists. Visibility,
    /// clipping and `visible_characters` do not affect index lifetime.
    pub fn is_valid_char(&self, index: i64) -> bool { self.resolve(index).is_some() }

    pub(super) fn resolve(&self, index: i64) -> Option<usize> {
        let index = usize::try_from(index).ok()?;
        self.chars.get(index).map(|c| c.cluster)
    }
    pub(super) fn read(&self, index: i64) -> Option<CharState> { Some(self.chars[self.resolve(index)?]) }
    pub(super) fn write(&mut self, index: i64, write: impl FnOnce(&mut CharState, u64)) {
        let Some(index) = self.resolve(index) else { return };
        self.stamp = self.stamp.wrapping_add(1);
        write(&mut self.chars[index], self.stamp);
    }

    /// Current additional pixel offset. Invalid indices return zero.
    pub fn get_char_offset(&self, index: i64) -> Vector2 { self.read(index).map_or(Vector2::ZERO, |c| c.offset.value) }
    /// Sets the additional pixel offset. Invalid indices warn and do nothing.
    pub fn set_char_offset(&mut self, index: i64, value: Vector2) {
        if self.check(index) {
            let v = clean_vec2(value, false);
            self.write(index, |c, stamp| c.offset.set(v, stamp));
        }
    }
    /// Current YXZ Euler rotation in radians. Invalid indices return zero.
    pub fn get_char_rotation(&self, index: i64) -> Vector3 {
        self.read(index).map_or(Vector3::ZERO, |c| c.rotation.value)
    }
    /// Sets the YXZ Euler rotation in radians, with orthographic projection.
    pub fn set_char_rotation(&mut self, index: i64, value: Vector3) {
        if self.check(index) {
            self.write(index, |c, stamp| c.rotation.set(clean_vec3(value, false), stamp));
        }
    }
    /// Current per-axis scale multiplier. Invalid indices return one.
    pub fn get_char_scale(&self, index: i64) -> Vector2 { self.read(index).map_or(Vector2::ONE, |c| c.scale.value) }
    /// Sets per-axis scale multipliers. Negative values mirror the glyph.
    pub fn set_char_scale(&mut self, index: i64, value: Vector2) {
        if self.check(index) {
            let v = clean_vec2(value, false);
            self.write(index, |c, stamp| c.scale.set(v, stamp));
        }
    }
    /// Current fill color. Before the first draw, the theme default is used;
    /// drawing captures the effective BBCode/preceding-effect color. Invalid
    /// indices return white. Alpha is independent of the RGB animation channel.
    pub fn get_char_color(&self, index: i64) -> Color { self.read(index).map_or(Color::WHITE, |c| c.effective_color()) }
    /// Overrides the fill color and alpha. Outlines retain their configured RGB.
    pub fn set_char_color(&mut self, index: i64, value: Color) {
        if self.check(index) {
            let v = clean_color(value);
            self.write(index, |c, stamp| c.set_color(v, stamp));
        }
    }
    /// Current fill alpha. Invalid indices return one.
    pub fn get_char_alpha(&self, index: i64) -> f64 { self.get_char_color(index).a as f64 }
    /// Overrides alpha without overwriting RGB. Invalid indices warn.
    pub fn set_char_alpha(&mut self, index: i64, value: f64) {
        if self.check(index) {
            let value = unit(value, 1.0, "alpha");
            self.write(index, |c, stamp| c.alpha.set(Some(value), stamp));
        }
    }
    /// Pixel offset of the transform pivot from the glyph pen origin.
    pub fn get_char_pivot(&self, index: i64) -> Vector2 { self.read(index).map_or(Vector2::ZERO, |c| c.pivot.value) }
    /// Sets a pixel pivot offset. Does not move the text's layout.
    pub fn set_char_pivot(&mut self, index: i64, value: Vector2) {
        if self.check(index) {
            let v = clean_vec2(value, false);
            self.write(index, |c, stamp| c.pivot.set(v, stamp));
        }
    }
    fn check(&self, index: i64) -> bool {
        let valid = self.is_valid_char(index);
        if !valid {
            godot_warn!(
                "[spire characters] invalid character index {index}; character count is {}.",
                self.character_count()
            );
        }
        valid
    }
}

#[godot_api]
impl IRichTextEffect for SpireCharRenderer {
    #[func(gd_self)]
    fn process_custom_fx(mut this: Gd<Self>, char_fx: Option<Gd<CharFxTransform>>) -> bool {
        let Some(mut fx) = char_fx else { return true };

        let state = &mut *this.bind_mut();
        let range = fx.get_range();
        let Ok(start) = usize::try_from(range.x) else { return true };
        if start >= state.chars.len() {
            return true;
        }

        let end = (range.y.max(range.x + 1) as usize).min(state.chars.len());
        let mut character = state.chars[start];
        // A shaped glyph may represent multiple logical characters. Merge their
        // channels independently so writes before the first draw are preserved.
        for index in start..end {
            character.merge(state.chars[state.chars[index].cluster]);
            state.chars[index].cluster = start;
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

        state.chars[start] = character;

        // Orthographic XY projection of the rotated/scaled flat glyph: exact
        // affine projection, including X/Y tilts, skew, reflections and edge-on.
        let cluster_origin = character.pen_origin.map_or(incoming.origin, |(_, origin)| origin);
        let pivot = cluster_pivot(incoming, character.pivot.value, cluster_origin);
        let projected =
            projected_transform(character.offset.value, character.rotation.value, character.scale.value, pivot);
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
            testing::record(state.base().instance_id(), start, fx.get_transform(), color);
        }

        true
    }
}
