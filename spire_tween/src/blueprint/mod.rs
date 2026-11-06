mod inspector;
mod runtime;
mod triggers;
mod tweenable_property_validation;

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
    enter_trigger_pending: bool,

    // <-------------------------- General Configuration
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

    // General Configuration -------------------------->

    // <-------------------------- Tween Property Configuration
    #[export_group(name = "Property Animation")]
    #[export]
    #[var(set = set_animated_property)]
    animated_property: StringName,
    #[export]
    #[var]
    #[init(val = 1.0)]
    speed_or_duration: f64,
    #[export]
    #[var]
    start_enabled: bool,
    #[export]
    #[var]
    start_value: Variant,
    #[export]
    #[var]
    end_value: Variant,
    #[export]
    #[var]
    lerp_mode: LerpModeKind,
    #[export]
    #[var]
    ease_basic: Ease,
    #[export]
    #[var]
    ease_curve: Option<Gd<Curve>>,
    // Tween Property Data -------------------------->
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

    fn on_validate_property(&self, property: &mut PropertyInfo) { self.validate_inspector_property(property); }

    fn get_configuration_warnings(&self) -> PackedStringArray { self.configuration_warnings() }
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
    fn set_animated_property(&mut self, name: StringName) {
        self.animated_property = name;
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
        self.configuration_changed();
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
