use godot::{
    classes::{
        AnimatedSprite3D,
        AudioStreamPlayer,
        AudioStreamPlayer2D,
        AudioStreamPlayer3D,
        Camera3D,
        CanvasItem,
        CollisionShape2D,
        DirectionalLight3D,
        Label3D,
        Light3D,
        OmniLight3D,
        SpotLight3D,
        Sprite3D,
    },
    prelude::*,
};
use spire_tween::prelude::*;

use super::util::*;
use crate::impl_test_base;

#[derive(GodotClass)]
#[class(init, base = Node2D)]
pub struct MiscTests {
    base: Base<Node2D>,
    #[init(node = "BouncyBall/CollisionShape2D")]
    shape: OnReady<Gd<CollisionShape2D>>,
    #[init(val = RcPtr::new(TimeTracker::new()))]
    time_tracker: RcPtr<TimeTracker>,
}

impl_test_base! { INode2D, MiscTests }

impl ITestClass for MiscTests {
    const PREFAB_PATH: &'static str = "res://examples/tests/misc.tscn";

    fn test_list() -> Vec<NamedTest<Self>> {
        vec![
            ("test_characters_translation", Self::test_characters_translation),
            ("test_characters_ranges", Self::test_characters_ranges),
            ("test_characters_ease_kind", Self::test_characters_ease_kind),
            ("test_characters_range_failures", Self::test_characters_range_failures),
            ("test_text_presets_rust_api", Self::test_text_presets_rust_api),
            ("test_characters_rotation", Self::test_characters_rotation),
            ("test_characters_effect_refresh", Self::test_characters_effect_refresh),
            ("test_characters_automatic", Self::test_characters_automatic),
            ("test_characters_label", Self::test_characters_label),
            ("test_characters_channels", Self::test_characters_channels),
            ("test_characters_refresh", Self::test_characters_refresh),
            ("test_characters_explicit_refresh", Self::test_characters_explicit_refresh),
            ("test_characters_lifecycle", Self::test_characters_lifecycle),
            ("test_characters_loops_sequence", Self::test_characters_loops_sequence),
            ("test_characters_effects", Self::test_characters_effects),
            ("test_characters_invalid", Self::test_characters_invalid),
            ("test_characters_render", Self::test_characters_render),
            ("test_characters_unicode_render", Self::test_characters_unicode_render),
            ("test_animatable_body_2d_translation", Self::test_animatable_body_2d_translation),
            ("test_animatable_body_3d_translation", Self::test_animatable_body_3d_translation),
            ("test_animatable_body_2d_rotation_sequence", Self::test_animatable_body_2d_rotation_sequence),
            ("test_animatable_body_3d_rotation_sequence", Self::test_animatable_body_3d_rotation_sequence),
            ("test_animatable_body_stop_and_free", Self::test_animatable_body_stop_and_free),
            ("test_group4_text_buffers", Self::test_group4_text_buffers),
            ("test_group4_text_relative", Self::test_group4_text_relative),
            ("test_group4_text_speed", Self::test_group4_text_speed),
            ("test_group4_text_options_api", Self::test_group4_text_options_api),
            ("test_group4_path_zero_scale_top_level", Self::test_group4_path_zero_scale_top_level),
            ("test_group4_text_markup_edges", Self::test_group4_text_markup_edges),
            ("test_group4_path_curve_edges", Self::test_group4_path_curve_edges),
            ("test_group4_path_modes", Self::test_group4_path_modes),
            ("test_group4_path_targets", Self::test_group4_path_targets),
            ("test_group4_margin_accessors", Self::test_group4_margin_accessors),
            ("test_group4_text_plain", Self::test_group4_text_plain),
            ("test_group4_text_bbcode", Self::test_group4_text_bbcode),
            ("test_group4_text_scramble", Self::test_group4_text_scramble),
            ("test_group4_text_lifecycle", Self::test_group4_text_lifecycle),
            ("test_group4_path_linear", Self::test_group4_path_linear),
            ("test_group4_path_curves", Self::test_group4_path_curves),
            ("test_group4_path_spaces", Self::test_group4_path_spaces),
            ("test_group4_path_3d", Self::test_group4_path_3d),
            ("test_group4_path_control", Self::test_group4_path_control),
            ("test_group4_path_closed_locks", Self::test_group4_path_closed_locks),
            ("test_group4_path_orientation", Self::test_group4_path_orientation),
            ("test_group4_path_lifecycle", Self::test_group4_path_lifecycle),
            ("test_group4_path_invalid", Self::test_group4_path_invalid),
            ("test_group4_path_cancellation", Self::test_group4_path_cancellation),
            ("test_group4_margins", Self::test_group4_margins),
            ("test_group4_integer_api", Self::test_group4_integer_api),
            ("test_typed_vec4_property", Self::test_typed_vec4_property),
            ("test_typed_rect2_property", Self::test_typed_rect2_property),
            ("test_typed_quaternion_property", Self::test_typed_quaternion_property),
            ("test_typed_methods_and_accessors", Self::test_typed_methods_and_accessors),
            ("test_typed_relative", Self::test_typed_relative),
            ("test_typed_speed", Self::test_typed_speed),
            ("test_typed_sequence_and_erasure", Self::test_typed_sequence_and_erasure),
            ("test_typed_loops", Self::test_typed_loops),
            ("test_typed_dynamic_targets", Self::test_typed_dynamic_targets),
            ("test_typed_cancellation", Self::test_typed_cancellation),
            ("test_typed_quaternion_edges", Self::test_typed_quaternion_edges),
            ("test_typed_quaternion_completion", Self::test_typed_quaternion_completion),
            ("test_typed_shader_vec4", Self::test_typed_shader_vec4),
            ("test_typed_variant_dispatch", Self::test_typed_variant_dispatch),
            ("test_variant_common_types", Self::test_variant_common_types),
            ("test_variant_conversion_and_override", Self::test_variant_conversion_and_override),
            ("test_group3_global_easing", Self::test_group3_global_easing),
            ("test_group3_punch", Self::test_group3_punch),
            ("test_group3_punch_3d", Self::test_group3_punch_3d),
            ("test_group3_effect_lifecycle", Self::test_group3_effect_lifecycle),
            ("test_group3_shake", Self::test_group3_shake),
            ("test_group3_shake_3d_camera", Self::test_group3_shake_3d_camera),
            ("test_group3_generic", Self::test_group3_generic),
            ("test_group3_quaternion", Self::test_group3_quaternion),
            ("test_group3_rotation_modes", Self::test_group3_rotation_modes),
            ("test_group3_blendable_rotation", Self::test_group3_blendable_rotation),
            ("test_group3_look_at", Self::test_group3_look_at),
            ("test_group3_pivot", Self::test_group3_pivot),
            ("test_group3_scroll", Self::test_group3_scroll),
            ("test_group3_spiral", Self::test_group3_spiral),
            ("test_group3_cancellation_and_invalid", Self::test_group3_cancellation_and_invalid),
            ("test_group2_typed_accessors", Self::test_group2_typed_accessors),
            ("test_group2_callable_validity", Self::test_group2_callable_validity),
            ("test_group2_motion", Self::test_group2_motion),
            ("test_group2_blendables", Self::test_group2_blendables),
            ("test_group2_resources", Self::test_group2_resources),
            ("test_group2_styles_counter", Self::test_group2_styles_counter),
            ("test_group2_arrays", Self::test_group2_arrays),
            ("test_group2_gradients_lines", Self::test_group2_gradients_lines),
            ("test_group2_shader_bus", Self::test_group2_shader_bus),
            ("test_group2_template_lifetimes", Self::test_group2_template_lifetimes),
            ("test_force_complete", Self::test_force_complete),
            ("test_dyn_target", Self::test_dyn_target),
            ("test_property_path", Self::test_property_path),
            ("test_fade_alias", Self::test_fade_alias),
            ("test_fade_alias_freed_owner", Self::test_fade_alias_freed_owner),
            ("test_camera_clip_shortcuts", Self::test_camera_clip_shortcuts),
            ("test_camera_clip_shortcuts_freed_owner", Self::test_camera_clip_shortcuts_freed_owner),
            ("test_light_shortcuts", Self::test_light_shortcuts),
            ("test_light_shortcuts_freed_owner", Self::test_light_shortcuts_freed_owner),
            ("test_audio_volume_linear", Self::test_audio_volume_linear),
            ("test_audio_volume_linear_freed_owner", Self::test_audio_volume_linear_freed_owner),
            ("test_visual_3d_fade_aliases", Self::test_visual_3d_fade_aliases),
            ("test_visual_3d_fade_aliases_freed_owner", Self::test_visual_3d_fade_aliases_freed_owner),
        ]
    }

    fn time_tracker(&self) -> &RcPtr<TimeTracker> { &self.time_tracker }
}

impl MiscTests {
    fn test_camera_clip_shortcuts(&mut self) -> PinnedTestTask {
        let camera = Camera3D::new_alloc();
        self.base_mut().add_child(&camera);
        assert_scalar_shortcut(camera.do_near(1.0, 1.0), camera.clone().upcast(), "near", 0.25, 1.0);
        assert_scalar_shortcut(camera.do_far(300.0, 1.0), camera.clone().upcast(), "far", 100.0, 300.0);
        assert_eq!(camera.get_near(), 1.0, "far tween must preserve near");
        camera.free();
        Box::pin(async {})
    }

    fn test_camera_clip_shortcuts_freed_owner(&mut self) -> PinnedTestTask {
        let camera = Camera3D::new_alloc();
        self.base_mut().add_child(&camera);
        assert_cancelled_shortcuts(camera.clone().upcast(), [camera.do_near(1.0, 1.0), camera.do_far(300.0, 1.0)]);
        Box::pin(async {})
    }

    fn test_light_shortcuts(&mut self) -> PinnedTestTask {
        let lights: [Gd<Light3D>; 3] = [
            DirectionalLight3D::new_alloc().upcast(),
            OmniLight3D::new_alloc().upcast(),
            SpotLight3D::new_alloc().upcast(),
        ];
        for light in lights {
            self.base_mut().add_child(&light);
            let color = light.get_color();
            assert_scalar_shortcut(light.do_light_energy(3.0, 1.0), light.clone().upcast(), "light_energy", 1.0, 3.0);
            assert_scalar_shortcut(
                light.do_shadow_opacity(0.0, 1.0),
                light.clone().upcast(),
                "shadow_opacity",
                1.0,
                0.0,
            );
            assert_eq!(light.get_param(godot::classes::light_3d::Param::ENERGY), 3.0);
            assert_eq!(light.get_color(), color);
            light.free();
        }
        Box::pin(async {})
    }

    fn test_light_shortcuts_freed_owner(&mut self) -> PinnedTestTask {
        let light = OmniLight3D::new_alloc();
        self.base_mut().add_child(&light);
        assert_cancelled_shortcuts(light.clone().upcast(), [
            light.do_light_energy(3.0, 1.0),
            light.do_shadow_opacity(0.0, 1.0),
        ]);
        Box::pin(async {})
    }

    fn test_audio_volume_linear(&mut self) -> PinnedTestTask {
        let player = AudioStreamPlayer::new_alloc();
        let player_2d = AudioStreamPlayer2D::new_alloc();
        let player_3d = AudioStreamPlayer3D::new_alloc();
        self.base_mut().add_child(&player);
        self.base_mut().add_child(&player_2d);
        self.base_mut().add_child(&player_3d);
        for (owner, volume, pitch) in [
            (player.clone().upcast::<Node>(), player.do_volume_linear(2.0, 1.0), player.do_pitch_scale(2.0, 1.0)),
            (player_2d.clone().upcast(), player_2d.do_volume_linear(2.0, 1.0), player_2d.do_pitch_scale(2.0, 1.0)),
            (player_3d.clone().upcast(), player_3d.do_volume_linear(2.0, 1.0), player_3d.do_pitch_scale(2.0, 1.0)),
        ] {
            assert_eq!(pitch.get_property_path(), NodePath::from("pitch_scale"));
            assert_scalar_shortcut(volume, owner.clone(), "volume_linear", 0.5, 2.0);
            assert_eq!(owner.get("pitch_scale").to::<f64>(), 1.0);
        }
        // With the audio alias removed, method syntax selects CanvasItem's visual fade.
        assert_eq!(player_2d.do_fade(0.0, 1.0).get_property_path(), NodePath::from("modulate:a"));
        let visual = player_2d.get_modulate();
        for (owner, volume) in [
            (player.clone().upcast::<Node>(), player.do_volume_linear(0.0, 1.0)),
            (player_2d.clone().upcast(), player_2d.do_volume_linear(0.0, 1.0)),
            (player_3d.clone().upcast(), player_3d.do_volume_linear(0.0, 1.0)),
        ] {
            assert_scalar_shortcut(volume, owner, "volume_linear", 2.0, 0.0);
        }
        assert_eq!(player_2d.get_modulate(), visual);
        player.free();
        player_2d.free();
        player_3d.free();
        Box::pin(async {})
    }

    fn test_audio_volume_linear_freed_owner(&mut self) -> PinnedTestTask {
        let player = AudioStreamPlayer::new_alloc();
        let player_2d = AudioStreamPlayer2D::new_alloc();
        let player_3d = AudioStreamPlayer3D::new_alloc();
        for (owner, volume) in [
            (player.clone().upcast::<Node>(), player.do_volume_linear(0.0, 1.0)),
            (player_2d.clone().upcast(), player_2d.do_volume_linear(0.0, 1.0)),
            (player_3d.clone().upcast(), player_3d.do_volume_linear(0.0, 1.0)),
        ] {
            self.base_mut().add_child(&owner);
            assert_cancelled_shortcuts(owner, [volume]);
        }
        Box::pin(async {})
    }

    fn test_visual_3d_fade_aliases(&mut self) -> PinnedTestTask {
        let sprite = Sprite3D::new_alloc();
        let animated = AnimatedSprite3D::new_alloc();
        let label = Label3D::new_alloc();
        let outline = label.get_outline_modulate();
        for (owner, fade, canonical, color_alias) in [
            (
                sprite.clone().upcast::<Node>(),
                sprite.do_fade(0.0, 1.0),
                sprite.do_modulate_a(0.0, 1.0),
                sprite.do_color_a(0.0, 1.0),
            ),
            (
                animated.clone().upcast(),
                animated.do_fade(0.0, 1.0),
                animated.do_modulate_a(0.0, 1.0),
                animated.do_color_a(0.0, 1.0),
            ),
            (
                label.clone().upcast(),
                label.do_fade(0.0, 1.0),
                label.do_modulate_a(0.0, 1.0),
                label.do_color_a(0.0, 1.0),
            ),
        ] {
            self.base_mut().add_child(&owner);
            assert_eq!(fade.get_property_path(), canonical.get_property_path());
            assert_eq!(color_alias.get_property_path(), canonical.get_property_path());
            owner
                .clone()
                .set("modulate", &Color::from_rgba(0.125, 0.25, 0.5, 1.0).to_variant());
            assert_scalar_shortcut(fade, owner.clone(), "modulate:a", 1.0, 0.0);
            assert_eq!(owner.get("modulate").to::<Color>(), Color::from_rgba(0.125, 0.25, 0.5, 0.0));
        }
        assert_eq!(label.get_outline_modulate(), outline);
        sprite.free();
        animated.free();
        label.free();
        Box::pin(async {})
    }

    fn test_visual_3d_fade_aliases_freed_owner(&mut self) -> PinnedTestTask {
        let sprite = Sprite3D::new_alloc();
        let label = Label3D::new_alloc();
        for (owner, fade) in [
            (sprite.clone().upcast::<Node>(), sprite.do_fade(0.0, 1.0)),
            (label.clone().upcast(), label.do_fade(0.0, 1.0)),
        ] {
            self.base_mut().add_child(&owner);
            assert_cancelled_shortcuts(owner, [fade]);
        }
        Box::pin(async {})
    }

    fn test_fade_alias(&mut self) -> PinnedTestTask {
        let initial = Color::from_rgba(0.125, 0.25, 0.5, 1.0);
        self.shape.set_modulate(initial);
        self.base_mut().set_modulate(initial);
        let self_modulate = self.shape.get_self_modulate();

        // Exercise both Gd<DerivedNode2D> and the native WithBaseField constructor.
        let candidates = [
            (self.shape.clone().upcast::<CanvasItem>(), self.shape.do_fade(0.0, 1.0)),
            (self.to_gd().upcast::<CanvasItem>(), self.do_fade(0.0, 1.0)),
        ];
        let canonical_path = self.shape.do_modulate_a(0.0, 1.0).get_property_path();
        assert_eq!(self.shape.do_color_a(0.0, 1.0).get_property_path(), canonical_path);
        for (node, tween) in candidates {
            assert_eq!(tween.get_property_path(), canonical_path);
            let mut tween = TweenPtr::new(tween);
            assert!(!tween.is_registered(), "native aliases must preserve builder semantics");
            tween.set_ease(EaseKind::Basic(Ease::Linear));
            assert!(matches!(tween.custom_step(0.5, false), TweenStepResult::Stepped(AdvanceTimeResult::Playing)));
            assert_eq!(node.get_modulate(), Color::from_rgba(initial.r, initial.g, initial.b, 0.5));
            assert!(matches!(
                tween.custom_step(0.75, false),
                TweenStepResult::Stepped(AdvanceTimeResult::Completed { .. })
            ));
            assert_eq!(node.get_modulate(), Color::from_rgba(initial.r, initial.g, initial.b, 0.0));
            assert!(tween.is_stopped());
        }
        assert_eq!(self.shape.get_self_modulate(), self_modulate);
        Box::pin(async {})
    }

    fn test_fade_alias_freed_owner(&mut self) -> PinnedTestTask {
        let node = Node2D::new_alloc();
        self.base_mut().add_child(&node);
        let mut tween = TweenPtr::new(node.do_fade(0.0, 1.0));
        node.free();
        assert!(matches!(tween.custom_step(0.5, false), TweenStepResult::Stepped(AdvanceTimeResult::ShouldDespawn)));
        assert!(tween.is_stopped());
        Box::pin(async {})
    }

    fn test_force_complete(&mut self) -> PinnedTestTask {
        let mut shape = self.shape.clone();
        let final_value = Vector2::ONE * 2.5;

        let mut handle = shape.do_scale(final_value, 5.0).register();
        handle.force_complete();

        assert!(handle.is_stopped());
        assert_eq!(shape.get_scale(), final_value);

        let tracker = RcPtr::clone(&self.time_tracker);

        Box::pin(async move {
            tracker.wait_seconds(2.0).await;

            shape.set_scale(Vector2::ONE);
            handle.play();

            wait_finished(&handle, &tracker, 7.0).await;
            assert_eq!(shape.get_scale(), final_value);
        })
    }

    fn test_dyn_target(&mut self) -> PinnedTestTask {
        let shape = self.shape.clone();
        let initial_scale = shape.get_scale();
        let speed = 0.5;

        let dyn_target = RcPtr::new(Vector2::ONE * 5.0);
        let dyn_target_for_callable = dyn_target.clone();

        let mut tween = shape.do_scale(Vector2::ZERO, speed).as_speed_based();
        tween.set_dynamic_target(Callable::from_fn("dyn_target_fn", move |_| {
            let val: Vector2 = *dyn_target_for_callable;
            val.to_variant()
        }));
        let handle = tween.register();
        let tracker = RcPtr::clone(&self.time_tracker);

        Box::pin(async move {
            let wait_time = 3.0;
            tracker.wait_seconds(wait_time).await;

            let expected_scale = initial_scale.move_toward(*dyn_target, (speed * wait_time) as f32);
            assert!(
                shape.get_scale().distance_to(expected_scale) <= 0.01,
                "Expected near {expected_scale}, got {}",
                shape.get_scale(),
            );

            *dyn_target.to_mut() = Vector2::ONE;
            let expected_duration = shape.get_scale().distance_to(*dyn_target) as f64 / speed;

            wait_finished(&handle, &tracker, wait_time + expected_duration).await;
            assert_eq!(shape.get_scale(), *dyn_target);
        })
    }

    fn test_property_path(&mut self) -> PinnedTestTask {
        let shape = self.shape.clone();

        let scale_tween = shape.do_scale_x(5.0, 1.0);
        assert_eq!(scale_tween.get_property_path(), NodePath::from("scale:x"));

        let pos_tween = shape.do_move(Vector2::new(200.0, 500.0), 2.0);
        assert_eq!(pos_tween.get_property_path(), NodePath::from("global_position"));

        Box::pin(async move {})
    }
}

fn assert_scalar_shortcut(
    tween: SpireTween<LerpPropertyData<f64>>,
    mut owner: Gd<Node>,
    path: &str,
    initial: f64,
    target: f64,
) {
    let path = NodePath::from(path);
    assert_eq!(tween.get_property_path(), path);
    // Set the start after construction to also exercise deferred start capture.
    owner.set_indexed(&path, &initial.to_variant());
    let mut tween = TweenPtr::new(tween);
    assert!(!tween.is_registered(), "native shortcuts return unregistered builders");
    tween.set_ease(EaseKind::Basic(Ease::Linear));
    let events = RcPtr::new([0, 0]);
    let stopped = events.clone();
    tween.stopped_connect(move || stopped.to_mut()[0] += 1, SpireFlags::empty());
    let finished = events.clone();
    tween.finished_connect(move || finished.to_mut()[1] += 1, SpireFlags::empty());
    assert!(matches!(tween.custom_step(0.5, false), TweenStepResult::Stepped(AdvanceTimeResult::Playing)));
    let midpoint = owner.get_indexed(&path).to::<f64>();
    assert!((midpoint - (initial + target) * 0.5).abs() < 0.00001, "unexpected midpoint: {midpoint}");
    assert_eq!(*events, [0, 0]);
    assert!(matches!(tween.custom_step(0.75, false), TweenStepResult::Stepped(AdvanceTimeResult::Completed { .. })));
    let actual = owner.get_indexed(&path).to::<f64>();
    assert!((actual - target).abs() < 0.00001, "unexpected final value: {actual}");
    assert!(tween.is_stopped());
    assert_eq!(*events, [1, 1]);
}

fn assert_cancelled_shortcuts(owner: Gd<Node>, tweens: impl IntoIterator<Item = SpireTween<LerpPropertyData<f64>>>) {
    let mut tweens = tweens.into_iter().map(TweenPtr::new).collect::<Vec<_>>();
    let events = RcPtr::new([0, 0]);
    for tween in &mut tweens {
        let stopped = events.clone();
        tween.stopped_connect(move || stopped.to_mut()[0] += 1, SpireFlags::empty());
        let finished = events.clone();
        tween.finished_connect(move || finished.to_mut()[1] += 1, SpireFlags::empty());
    }
    owner.free();
    for tween in &mut tweens {
        assert!(matches!(tween.custom_step(0.5, false), TweenStepResult::Stepped(AdvanceTimeResult::ShouldDespawn)));
        assert!(tween.is_stopped());
        tween.stop();
    }
    assert_eq!(*events, [tweens.len(), 0], "freed owners must stop once without completing");
}
