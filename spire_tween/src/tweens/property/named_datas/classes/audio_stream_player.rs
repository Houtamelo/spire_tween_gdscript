use super::*;

object_properties_enum! {
    GdBridge = AudioStreamPlayerTweener;
    Class = AudioStreamPlayer;
    Trait = DoAudioStreamPlayer;
    @f32 {
        Data = PropertyF32AudioStreamPlayerData;
        Enum = PropertyF32AudioStreamPlayerKind;

        [VolumeDb, "volume_db", do_volume_db]
        fn get(n) { n.get_volume_db() }
        fn set(n, v) { n.set_volume_db(v) }

        [VolumeLinear, "volume_linear", do_volume_linear]
        fn get(n) { n.get_volume_linear() }
        fn set(n, v) { n.set_volume_linear(v) }

        [PitchScale, "pitch_scale", do_pitch_scale]
        fn get(n) { n.get_pitch_scale() }
        fn set(n, v) { n.set_pitch_scale(v) }
    }
}

object_properties_enum! {
    GdBridge = AudioStreamPlayer2DTweener;
    Class = AudioStreamPlayer2D;
    Trait = DoAudioStreamPlayer2D;
    @f32 {
        Data = PropertyF32AudioStreamPlayer2DData;
        Enum = PropertyF32AudioStreamPlayer2DKind;

        [VolumeDb, "volume_db", do_volume_db]
        fn get(n) { n.get_volume_db() }
        fn set(n, v) { n.set_volume_db(v) }

        [VolumeLinear, "volume_linear", do_volume_linear]
        fn get(n) { n.get_volume_linear() }
        fn set(n, v) { n.set_volume_linear(v) }

        [PitchScale, "pitch_scale", do_pitch_scale]
        fn get(n) { n.get_pitch_scale() }
        fn set(n, v) { n.set_pitch_scale(v) }
    }
}

object_properties_enum! {
    GdBridge = AudioStreamPlayer3DTweener;
    Class = AudioStreamPlayer3D;
    Trait = DoAudioStreamPlayer3D;
    @f32 {
        Data = PropertyF32AudioStreamPlayer3DData;
        Enum = PropertyF32AudioStreamPlayer3DKind;

        [VolumeDb, "volume_db", do_volume_db]
        fn get(n) { n.get_volume_db() }
        fn set(n, v) { n.set_volume_db(v) }

        [VolumeLinear, "volume_linear", do_volume_linear]
        fn get(n) { n.get_volume_linear() }
        fn set(n, v) { n.set_volume_linear(v) }

        [PitchScale, "pitch_scale", do_pitch_scale]
        fn get(n) { n.get_pitch_scale() }
        fn set(n, v) { n.set_pitch_scale(v) }
    }
}
