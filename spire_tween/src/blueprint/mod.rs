mod characters;
mod inspector;
mod runtime;
mod triggers;
mod tweenable_property_validation;

use godot::signal::ConnectHandle;

use super::*;

/// Stores an editable animation and builds a fresh [SpireSequence] on every [method play].
/// Built-in kinds animate a property or compose direct TweenBlueprint children.
/// Attach GDScript to override [method _play_internal] and optionally [method _reset_state].
/// Synchronously calling play again on the same blueprint from its hooks is unsupported.
#[derive(GodotClass)]
#[class(base = Node, init, tool)]
pub struct TweenBlueprint {
    base: Base<Node>,
    #[export]
    #[var(set = set_target)]
    target: Option<Gd<Node>>,
    current_tween: Option<Gd<SpireSequence>>,
    trigger_connection: Option<triggers::TriggerConnection>,
    text_effect_connection: Option<ConnectHandle>,
    text_delay_curve_connection: Option<ConnectHandle>,
    enter_trigger_pending: bool,

    // General Configuration
    #[export]
    #[var(set = set_trigger_condition)]
    trigger_condition: TriggerCondition,
    #[export]
    #[var(set = set_trigger_source)]
    trigger_source: Option<Gd<Node>>,
    #[export_group(name = "Tween Configuration")]
    #[export(range = (0.0, 10.0, or_greater))]
    #[var]
    delay: f64,
    #[export(range = (0.0, 10.0, or_greater))]
    #[var]
    #[init(val = 1.0)]
    speed_scale: f64,
    #[export]
    #[var]
    ignore_time_scale: bool,
    #[export]
    #[var]
    tween_pause_mode: PauseMode,
    #[export]
    #[var]
    tween_process_mode: ProcessMode,
    #[export(range = (-1.0, 10.0, or_greater))]
    #[var]
    #[init(val = 1)]
    loops: i64,
    #[export]
    #[var]
    /// Forwarded to the native sequence, whose loops support Restart only.
    loop_mode: LoopMode,
    #[export]
    #[var(set = set_kind)]
    kind: BlueprintKind,

    // Tween Property Configuration
    #[export_group(name = "Property Animation", prefix = "property_")]
    #[export]
    #[var(set = set_property_to_animate)]
    property_to_animate: StringName,
    #[export]
    #[var]
    #[init(val = 1.0)]
    property_speed_or_duration: f64,
    #[export]
    #[var]
    /// Initialize before playback. Basic character effects use the resource's
    /// `from`; property and punch/shake steps use `property_start_value`.
    property_start_enabled: bool,
    #[export]
    #[var]
    property_start_value: Variant,
    #[export]
    #[var]
    property_end_value: Variant,
    #[export]
    #[var]
    property_lerp_mode: LerpModeKind,
    #[export_subgroup(name = "Ease", prefix = "property_ease_")]
    #[export]
    #[var]
    property_ease_source: EaseSource,
    #[export]
    #[var]
    property_ease_enum: Ease,
    #[export]
    #[var]
    property_ease_curve: Option<Gd<Curve>>,

    #[export_group(name = "Text Animation", prefix = "text_")]
    /// Effect parameters shared with looping rich-text presets. Each play builds
    /// a fresh Char range tween from the resource's current fields.
    #[export]
    #[var(set = set_text_effect, hint = RESOURCE_TYPE,
        hint_string = "SpireTextColorEffect,SpireTextFadeEffect,SpireTextOffsetEffect,SpireTextRotationEffect,SpireTextScaleEffect,SpireTextPunchOffsetEffect,SpireTextPunchRotationEffect,SpireTextPunchScaleEffect,SpireTextShakeOffsetEffect,SpireTextShakeRotationEffect,SpireTextShakeScaleEffect")]
    text_effect: Option<Gd<Resource>>,
    /// First logical char index, including spaces and excluding BBCode tags.
    #[export(range = (0.0, 100.0, or_greater))]
    #[var]
    text_start_index: i64,
    /// Inclusive last index; -1 selects through the end of the text.
    #[export(range = (-1.0, 100.0, or_greater))]
    #[var]
    #[init(val = -1)]
    text_end_index: i64,
    /// Start delay for each selected char as a multiple of the effect's
    /// per-char duration. Samples must be finite and non-negative. The
    /// range finishes when the last char completes its delay and loops.
    #[export_subgroup(name = "Delay Ease", prefix = "text_delay_ease_")]
    #[export]
    #[var(set = set_text_delay_ease_source)]
    text_delay_ease_source: EaseSource,
    #[export]
    #[var(set = set_text_delay_ease_enum)]
    #[init(val = Ease::Linear)]
    text_delay_ease_enum: Ease,
    #[export]
    #[var(set = set_text_delay_ease_curve)]
    text_delay_ease_curve: Option<Gd<Curve>>,
    /// Animation legs per character, independently of the containing sequence.
    #[export(range = (1.0, 10.0, or_greater))]
    #[var]
    #[init(val = 1)]
    text_loops: i64,
    #[export]
    #[var]
    #[init(val = true)]
    /// Preserve other additive contributions during punch/shake playback.
    text_blendable: bool,
}

#[godot_api]
impl INode for TweenBlueprint {
    #[func(gd_self)]
    fn enter_tree(mut this: Gd<Self>) {
        this.bind_mut().enter_trigger_pending = true;
        this.call_deferred("refresh_blueprint_triggers", &[]);
    }

    #[func(gd_self)]
    fn ready(this: Gd<Self>) {
        Self::refresh_blueprint_triggers(this.clone());
        let after_ready = {
            let state = this.bind();
            state.trigger_condition == TriggerCondition::AfterReady && !state.is_managed_by_parent()
        };
        if !Engine::singleton().is_editor_hint() && after_ready {
            Self::play(this);
        }
    }

    #[func(gd_self)]
    fn exit_tree(this: Gd<Self>) {
        Self::disconnect_triggers(this.clone());
        Self::stop(this);
    }

    fn on_validate_property(&self, property: &mut PropertyInfo) { inspector::on_validate_property(self, property); }

    fn get_configuration_warnings(&self) -> PackedStringArray { inspector::get_configuration_warnings(self) }
}

#[godot_api]
impl TweenBlueprint {
    #[signal]
    pub fn tween_stopped(tween: Gd<SpireSequence>);

    #[signal]
    pub fn tween_loop_finished(tween: Gd<SpireSequence>);

    #[signal]
    pub fn tween_finished(tween: Gd<SpireSequence>);

    #[signal]
    pub fn tween_spawned(tween: Gd<SpireSequence>);

    #[func(gd_self)]
    pub fn play(mut this: Gd<Self>) -> Gd<SpireSequence> {
        let old_seq = { this.bind_mut().current_tween.take() };
        if let Some(old_seq) = old_seq {
            old_seq.bind().to_mut().stop();
        }
        Self::reset_state(this.clone());
        Self::setup_sequence_and_play(this)
    }

    /// Stops the last run without forcing its final values or emitting tween_finished.
    #[func(gd_self)]
    pub fn stop(mut this: Gd<Self>) {
        let sequence = this.bind_mut().current_tween.take();
        if let Some(sequence) = sequence {
            sequence.bind().to_mut().stop();
        }
    }

    /// Returns this blueprint's configured target or first non-blueprint ancestor.
    #[func]
    pub fn get_resolved_target(&self) -> Option<Gd<Node>> { self.resolve_target() }

    #[func]
    fn set_property_to_animate(&mut self, name: StringName) {
        self.property_to_animate = name;
        self.configuration_changed();
    }

    #[func]
    fn set_target(&mut self, target: Option<Gd<Node>>) {
        self.target = target;
        self.configuration_changed();
    }

    #[func]
    fn set_kind(&mut self, kind: BlueprintKind) {
        self.kind = kind;
        self.initialize_text_start();
        self.configuration_changed();
    }

    #[func]
    fn set_text_effect(&mut self, effect: Option<Gd<Resource>>) {
        if self.kind == BlueprintKind::Text
            && let Ok(previous) = self.get_compiled_text_effect()
            && previous.impulse_start_default().as_ref() == Some(&self.property_start_value)
        {
            self.property_start_value = Variant::nil();
        }
        self.disconnect_text_effect();
        self.text_effect = effect;
        self.initialize_text_start();
        if let Some(resource) = &self.text_effect {
            self.text_effect_connection = Some(self.watch_text_configuration(resource));
        }
        self.configuration_changed();
    }

    #[func]
    fn set_text_delay_ease_source(&mut self, source: EaseSource) {
        self.text_delay_ease_source = source;
        self.configuration_changed();
    }

    #[func]
    fn set_text_delay_ease_enum(&mut self, ease: Ease) {
        self.text_delay_ease_enum = ease;
        self.configuration_changed();
    }

    #[func]
    fn set_text_delay_ease_curve(&mut self, curve: Option<Gd<Curve>>) {
        Self::disconnect_configuration_watch(&mut self.text_delay_curve_connection);
        self.text_delay_ease_curve = curve;
        if let Some(curve) = &self.text_delay_ease_curve {
            self.text_delay_curve_connection = Some(self.watch_text_configuration(&curve.clone().upcast()));
        }
        self.configuration_changed();
    }

    fn get_text_delay_ease(&self) -> EaseKind {
        self.text_delay_ease_source
            .resolve(self.text_delay_ease_enum, self.text_delay_ease_curve.as_ref())
    }

    fn watch_text_configuration(&self, resource: &Gd<Resource>) -> ConnectHandle {
        // Resource setters emit while their Rust state is borrowed.
        let warnings = self.base().callable("update_configuration_warnings");
        let inspector = self.base().callable("notify_property_list_changed");
        resource.signals().changed().connect(move || {
            warnings.call_deferred(&[]);
            inspector.call_deferred(&[]);
        })
    }

    fn initialize_text_start(&mut self) {
        if self.kind == BlueprintKind::Text
            && self.property_start_value.is_nil()
            && let Ok(effect) = self.get_compiled_text_effect()
            && let Some(value) = effect.impulse_start_default()
        {
            self.property_start_value = value;
        }
    }

    fn disconnect_text_effect(&mut self) { Self::disconnect_configuration_watch(&mut self.text_effect_connection); }

    fn disconnect_configuration_watch(connection: &mut Option<ConnectHandle>) {
        if let Some(handle) = connection.take()
            && handle.is_connected()
        {
            handle.disconnect();
        }
    }

    #[func]
    fn set_trigger_condition(&mut self, condition: TriggerCondition) {
        self.trigger_condition = condition;
        self.configuration_changed();
    }

    #[func]
    fn set_trigger_source(&mut self, source: Option<Gd<Node>>) {
        self.trigger_source = source;
        self.configuration_changed();
    }

    fn configuration_changed(&mut self) {
        let mut base = self.base_mut();
        base.notify_property_list_changed();
        base.update_configuration_warnings();
        base.call_deferred("initialize_blueprint_values", &[]);
        base.call_deferred("refresh_blueprint_configuration", &[]);
    }

    #[func(virtual, gd_self)]
    fn play_internal(this: Gd<Self>, seq: Gd<SpireSequence>) {
        if this.get_script().is_some() {
            godot_warn!("An attached script must override `_play_internal(sequence: SpireSequence)`.");
            return;
        }

        Self::build_native(this, seq);
    }

    #[func(virtual, gd_self)]
    fn reset_state(_this: Gd<Self>) {}
}
