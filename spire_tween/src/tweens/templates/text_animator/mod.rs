//! Per-character animation state shared by ordinary tweens and a RichTextEffect.
use super::{effect::*, punch::clean_vec3, *};

mod render;
mod tween;

/// Animates logical character indices of a RichTextLabel. Create with [`Self::new`].
/// The label owns a small processing child which maintains the custom-effect
/// scope without changing `text` or `bbcode_enabled`. Repeated attachment reuses
/// that child. The label's content must be supplied through its `text` property;
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
    fn unattached() -> Self {
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

    /// Attaches to a label, or reuses its existing animator. The label owns the
    /// processing child and frees it automatically. Keep this handle to construct
    /// animations; dropping the handle does not remove the rendered deformations.
    pub fn new(label: Gd<RichTextLabel>) -> Self { SpireTextAnimator::attach(label).bind().animator.clone() }

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

/// Godot-facing character animator. Create with `SpireTextAnimator.attach(label)`;
/// it is automatically owned by that RichTextLabel. Its Rust counterpart is
/// [`TextAnimator`]. Do not move this processing child to another node.
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
#[class(init, base = Node)]
pub struct SpireTextAnimator {
    base: Base<Node>,
    #[init(val = TextAnimator::unattached())]
    pub(crate) animator: TextAnimator,
}

#[godot_api]
impl INode for SpireTextAnimator {
    fn enter_tree(&mut self) {
        if self.animator.state.label.is_none()
            && let Some(label) = self
                .base()
                .get_parent()
                .and_then(|parent| parent.try_cast::<RichTextLabel>().ok())
        {
            self.animator.bind_label(label);
            self.base_mut()
                .set_process_mode(godot::classes::node::ProcessMode::ALWAYS);
        }
    }
    fn process(&mut self, _delta: f64) { self.animator.state.to_mut().refresh(false); }
    fn on_notification(&mut self, what: godot::classes::notify::NodeNotification) {
        if what == godot::classes::notify::NodeNotification::TRANSLATION_CHANGED {
            self.animator.refresh();
        }
    }
}
impl Drop for SpireTextAnimator {
    fn drop(&mut self) { self.animator.state.to_mut().alive = false; }
}
impl SpireTextAnimator {
    /// Attaches/reuses a processing child. Prefer [`TextAnimator::new`] in Rust.
    pub fn attach(mut label: Gd<RichTextLabel>) -> Gd<Self> {
        for child in label.get_children_ex().include_internal(true).done().iter_shared() {
            if let Ok(animator) = child.try_cast::<Self>() {
                let handle = animator.bind().animator.clone();
                if handle.state.label.is_none() {
                    handle.bind_label(label.clone());
                }
                return animator;
            }
        }
        let handle = TextAnimator::unattached();
        handle.bind_label(label.clone());
        let mut node = Gd::from_init_fn(|base| Self { base, animator: handle });
        node.set_name("SpireTextAnimator");
        node.set_process_mode(godot::classes::node::ProcessMode::ALWAYS);
        // Hide runtime machinery from ordinary child enumeration. Godot may
        // duplicate internal nodes; its inert constructor binds fresh state in enter_tree.
        label
            .add_child_ex(&node)
            .internal(godot::classes::node::InternalMode::BACK)
            .done();
        node
    }
}

#[cfg(feature = "include_gdscript_bridge")]
mod gd;
