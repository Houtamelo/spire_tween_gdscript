//! Per-character animation state shared by ordinary tweens and a RichTextEffect.

mod render;
mod tween;

use godot::classes::IRichTextLabel;

use super::{effect::*, punch::clean_vec3, *};

/// Native animation handle for a [`SpireRichTextLabel`]. Create with [`Self::new`].
/// Handles share the label's character state without adding helper nodes.
/// The label's content must be supplied through its `text` property;
/// manual `push_*`, `add_text`, and `append_text` stack edits are not tracked.
///
/// Text/BBCode/translation changes rebuild character data and reset deformations.
/// Existing tweens retain their clocks and indices; missing indices cancel on
/// their next processing step or forced completion. Resize/reflow preserves state.
/// Rotation is a YXZ Euler vector in radians, projected orthographically.
/// Rotation/scale use the cluster's glyph pen origin unless a pixel pivot is set.
/// For initial BBCode colors and shaped-cluster identity, wait for the label's
/// first draw before constructing animations which need those initial values.
/// All tween constructors return unregistered method-progress builders.
#[derive(Clone)]
pub struct TextAnimator {
    state: RcPtr<AnimatorState>,
}

#[derive(Clone, Copy)]
struct Value<T: Copy> {
    value: T,
    stamp: u64,
}
impl<T: Copy> Value<T> {
    fn new(value: T) -> Self { Self { value, stamp: 0 } }
    fn merge(&mut self, other: Self) {
        if other.stamp > self.stamp {
            *self = other;
        }
    }
}

#[derive(Clone, Copy)]
struct Character {
    cluster: usize,
    offset: Value<Vector2>,
    rotation: Value<Vector3>,
    scale: Value<Vector2>,
    color: Value<Option<Color>>,
    alpha: Value<Option<f64>>,
    pivot: Value<Vector2>,
    baseline_color: Color,
    pen_origin: Option<(u64, Vector2)>,
}
impl Character {
    fn new(cluster: usize, baseline_color: Color) -> Self {
        Self {
            cluster,
            offset: Value::new(Vector2::ZERO),
            rotation: Value::new(Vector3::ZERO),
            scale: Value::new(Vector2::ONE),
            color: Value::new(None),
            alpha: Value::new(None),
            pivot: Value::new(Vector2::ZERO),
            baseline_color,
            pen_origin: None,
        }
    }
    fn effective_color(&self) -> Color {
        let mut value = self.color.value.unwrap_or(self.baseline_color);
        if let Some(alpha) = self.alpha.value {
            value.a = alpha as f32;
        }
        value
    }
    fn set_color(&mut self, value: Color, stamp: u64) {
        self.color = Value {
            value: Some(value),
            stamp,
        };
        self.alpha = Value {
            value: Some(value.a as f64),
            stamp,
        };
    }
    fn merge(&mut self, other: Self) {
        self.offset.merge(other.offset);
        self.rotation.merge(other.rotation);
        self.scale.merge(other.scale);
        self.color.merge(other.color);
        self.alpha.merge(other.alpha);
        self.pivot.merge(other.pivot);
    }
}

pub(crate) struct AnimatorState {
    label: Option<Gd<RichTextLabel>>,
    effect: Option<Gd<render::SpireCharacterEffect>>,
    source: Option<GString>,
    bbcode: bool,
    characters: Vec<Character>,
    stamp: u64,
    alive: bool,
    epoch: u64,
    #[cfg(feature = "integration-tests")]
    samples: HashMap<usize, (Transform2D, Color)>,
}

impl AnimatorState {
    fn refresh(&mut self, force: bool) -> bool {
        if !self.alive {
            return false;
        }

        let Some(label) = self.label.as_mut() else { return false };
        if !label.is_instance_valid() {
            return false;
        }

        let source = label.get_text();
        let bbcode = label.is_using_bbcode();
        if !force && self.source.as_ref() == Some(&source) && self.bbcode == bbcode {
            return true;
        }

        let Some(effect) = &self.effect else { return false };
        // clear/add/append edit the rendering stack, not the source text property.
        // atr preserves the node's auto-translation policy and translation domain.
        let translated = label.atr(&source);
        label.clear();
        label.push_customfx(effect, &VarDictionary::new());

        if bbcode {
            label.append_text(&translated);
        } else {
            label.add_text(&translated);
        }

        label.pop();
        let count = label.get_total_character_count().max(0) as usize;
        let color = label.get_theme_color("default_color");
        self.characters.clear();
        self.epoch = self.epoch.wrapping_add(1);
        self.characters.extend((0..count).map(|i| Character::new(i, color)));
        self.source = Some(source);
        self.bbcode = bbcode;
        #[cfg(feature = "integration-tests")]
        self.samples.clear();
        label.queue_redraw();
        true
    }
    fn resolve(&mut self, index: i64) -> Option<usize> {
        if !self.refresh(false) {
            return None;
        }
        let index = usize::try_from(index).ok()?;
        self.characters.get(index).map(|c| c.cluster)
    }
}

impl TextAnimator {
    fn uninitialized() -> Self {
        Self {
            state: RcPtr::new(AnimatorState {
                label: None,
                effect: None,
                source: None,
                bbcode: false,
                characters: Vec::new(),
                stamp: 0,
                alive: false,
                epoch: 0,
                #[cfg(feature = "integration-tests")]
                samples: HashMap::new(),
            }),
        }
    }

    fn bind_label(&self, label: Gd<RichTextLabel>) {
        let state = self.state.to_mut();
        state.label = Some(label);
        state.alive = true;
        state.effect = Some(Gd::from_init_fn(|base| render::SpireCharacterEffect::new(base, self.state.downgrade())));
        state.refresh(true);
    }

    /// Returns the label's shared animation state, initializing it on first use.
    /// Works before entering the tree. Dropping this handle preserves deformations;
    /// freeing the label invalidates all of its character tweens.
    pub fn new(label: Gd<SpireRichTextLabel>) -> Self {
        let animator = { label.bind().animator.clone() };
        if animator.state.label.is_none() {
            animator.bind_label(label.upcast());
        }
        animator
    }

    /// Rebuilds the rendering stack and resets all character state. Does not
    /// cancel/restart tweens. Source-property edits are also detected automatically.
    pub fn refresh(&self) { self.state.to_mut().refresh(true); }

    /// Resets all deformations without rebuilding the text or changing playback.
    pub fn reset(&self) {
        let state = self.state.to_mut();
        if !state.refresh(false) {
            return;
        }
        for c in &mut state.characters {
            *c = Character::new(c.cluster, c.baseline_color);
        }
        state.epoch = state.epoch.wrapping_add(1);
        if let Some(label) = &mut state.label {
            label.queue_redraw();
        }
    }

    /// Number of logical characters, including spaces/newlines, excluding BBCode
    /// syntax. Indices use Godot's parsed text order, not visual/bidirectional order.
    pub fn character_count(&self) -> i64 {
        let state = self.state.to_mut();
        if state.refresh(false) { state.characters.len() as i64 } else { 0 }
    }

    /// Whether this numeric logical character index still exists. Visibility,
    /// clipping and `visible_characters` do not affect index lifetime.
    pub fn is_valid_char(&self, index: i64) -> bool { self.state.to_mut().resolve(index).is_some() }

    fn read(&self, index: i64) -> Option<Character> {
        let state = self.state.to_mut();
        let index = state.resolve(index)?;
        Some(state.characters[index])
    }
    fn write(&self, index: i64, write: impl FnOnce(&mut Character, u64)) {
        let state = self.state.to_mut();
        let Some(index) = state.resolve(index) else { return };
        state.stamp = state.stamp.wrapping_add(1);
        write(&mut state.characters[index], state.stamp);
        if let Some(label) = &mut state.label {
            label.queue_redraw();
        }
    }

    /// Current additional pixel offset. Invalid indices return zero.
    pub fn get_char_offset(&self, index: i64) -> Vector2 { self.read(index).map_or(Vector2::ZERO, |c| c.offset.value) }
    /// Sets the additional pixel offset. Invalid indices warn and do nothing.
    pub fn set_char_offset(&self, index: i64, value: Vector2) {
        if self.check(index) {
            let v = clean_vec3(Vector3::new(value.x, value.y, 0.0), false);
            self.write(index, |c, stamp| {
                c.offset = Value {
                    value: Vector2::new(v.x, v.y),
                    stamp,
                }
            });
        }
    }
    /// Current YXZ Euler rotation in radians. Invalid indices return zero.
    pub fn get_char_rotation(&self, index: i64) -> Vector3 {
        self.read(index).map_or(Vector3::ZERO, |c| c.rotation.value)
    }
    /// Sets the YXZ Euler rotation in radians, with orthographic projection.
    pub fn set_char_rotation(&self, index: i64, value: Vector3) {
        if self.check(index) {
            self.write(index, |c, stamp| {
                c.rotation = Value {
                    value: clean_vec3(value, false),
                    stamp,
                }
            });
        }
    }
    /// Current per-axis scale multiplier. Invalid indices return one.
    pub fn get_char_scale(&self, index: i64) -> Vector2 { self.read(index).map_or(Vector2::ONE, |c| c.scale.value) }
    /// Sets per-axis scale multipliers. Negative values mirror the glyph.
    pub fn set_char_scale(&self, index: i64, value: Vector2) {
        if self.check(index) {
            let v = clean_vec3(Vector3::new(value.x, value.y, 0.0), false);
            self.write(index, |c, stamp| {
                c.scale = Value {
                    value: Vector2::new(v.x, v.y),
                    stamp,
                }
            });
        }
    }
    /// Current fill color. Before the first draw, the theme default is used;
    /// drawing captures the effective BBCode/preceding-effect color. Invalid
    /// indices return white. Alpha is independent of the RGB animation channel.
    pub fn get_char_color(&self, index: i64) -> Color { self.read(index).map_or(Color::WHITE, |c| c.effective_color()) }
    /// Overrides the fill color and alpha. Outlines retain their configured RGB.
    pub fn set_char_color(&self, index: i64, value: Color) {
        if self.check(index) {
            let v = Color::from_rgba(
                finite(value.r as f64, 1.0, "red") as f32,
                finite(value.g as f64, 1.0, "green") as f32,
                finite(value.b as f64, 1.0, "blue") as f32,
                unit(value.a as f64, 1.0, "alpha") as f32,
            );
            self.write(index, |c, stamp| c.set_color(v, stamp));
        }
    }
    /// Current fill alpha. Invalid indices return one.
    pub fn get_char_alpha(&self, index: i64) -> f64 { self.get_char_color(index).a as f64 }
    /// Overrides alpha without overwriting RGB. Invalid indices warn.
    pub fn set_char_alpha(&self, index: i64, value: f64) {
        if self.check(index) {
            let value = unit(value, 1.0, "alpha");
            self.write(index, |c, stamp| {
                c.alpha = Value {
                    value: Some(value),
                    stamp,
                }
            });
        }
    }
    /// Pixel offset of the transform pivot from the glyph pen origin.
    pub fn get_char_pivot(&self, index: i64) -> Vector2 { self.read(index).map_or(Vector2::ZERO, |c| c.pivot.value) }
    /// Sets a pixel pivot offset. Does not move the text's layout.
    pub fn set_char_pivot(&self, index: i64, value: Vector2) {
        if self.check(index) {
            let v = clean_vec3(Vector3::new(value.x, value.y, 0.0), false);
            self.write(index, |c, stamp| {
                c.pivot = Value {
                    value: Vector2::new(v.x, v.y),
                    stamp,
                }
            });
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
    fn validation(&self, index: i64) -> Callable {
        let animator = self.clone();
        Callable::from_fn("spire_character_valid", move |_| animator.is_valid_char(index).to_variant())
    }

    /// Last real rendering callback's transform and color for this character.
    /// Test instrumentation only; absent until the engine draws the glyph.
    #[cfg(feature = "integration-tests")]
    #[doc(hidden)]
    pub fn render_sample_for_test(&self, index: i64) -> Option<(Transform2D, Color)> {
        let state = self.state.to_mut();
        let index = state.resolve(index)?;
        state.samples.get(&index).copied()
    }
}

/// A RichTextLabel with native per-character animation methods and state.
/// Create this node directly or use the editor's Change Type action on an
/// existing RichTextLabel. Rust callers use [`TextAnimator::new`] for builders.
/// Supply content through [member RichTextLabel.text]. Processing respects
/// [member Node.process_mode]; the addon never overrides it.
///
/// [b]Initialization:[/b] Wait for the label's first [signal CanvasItem.draw]
/// before starting color/fade tweens which must capture BBCode or other effect
/// colors. Before drawing, the theme default is used. Shaping clusters are also
/// learned during drawing.
///
/// [b]Transforms:[/b] Rotation is a three-axis YXZ Euler vector in radians with
/// orthographic projection. Rotation/scale default to the glyph pen origin;
/// use [method set_char_pivot] for an explicit pixel offset. Exact automatic
/// center pivots and perspective are not provided by this renderer.
#[derive(GodotClass)]
#[class(init, tool, base = RichTextLabel)]
pub struct SpireRichTextLabel {
    base: Base<RichTextLabel>,
    #[init(val = TextAnimator::uninitialized())]
    pub(crate) animator: TextAnimator,
}

#[godot_api]
impl IRichTextLabel for SpireRichTextLabel {
    #[func(gd_self)]
    fn ready(this: Gd<Self>) {
        if !Engine::singleton().is_editor_hint() {
            TextAnimator::new(this);
        }
    }

    #[func(gd_self)]
    fn process(this: Gd<Self>, _delta: f64) {
        if !Engine::singleton().is_editor_hint() {
            TextAnimator::new(this).state.to_mut().refresh(false);
        }
    }

    fn on_notification(&mut self, what: notify::ControlNotification) {
        if what == notify::ControlNotification::TRANSLATION_CHANGED {
            // Defer rebuilding until processing or the next character operation.
            // The notification may arrive during a native RichTextLabel call.
            self.animator.state.to_mut().source = None;
        } else if what == notify::ControlNotification::READY && Engine::singleton().is_editor_hint() {
            let callback = self.base().callable("update_configuration_warnings");
            if !self.base().is_connected("script_changed", &callback) {
                self.base_mut().connect("script_changed", &callback);
            }
            self.base_mut().update_configuration_warnings();
        }
    }

    fn get_configuration_warnings(&self) -> PackedStringArray {
        let mut warnings = PackedStringArray::new();
        if !Engine::singleton().is_editor_hint() {
            return warnings;
        }
        let mut script = self.base().get_script();
        while let Some(current) = script {
            if current
                .get_script_method_list()
                .iter_shared()
                .any(|method| method.get("name").is_some_and(|name| name.to_string() == "_process"))
            {
                warnings.push(
                    "The attached script overrides `_process()`, disabling SpireRichTextLabel's automatic character \
                     refresh. `super._process()` cannot invoke the Rust implementation. Use another node for custom \
                     per-frame logic.",
                );
                break;
            }
            script = current.get_base_script();
        }
        warnings
    }
}

impl Drop for SpireRichTextLabel {
    fn drop(&mut self) { self.animator.state.to_mut().alive = false; }
}

#[cfg(feature = "include_gdscript_bridge")]
mod gd;
