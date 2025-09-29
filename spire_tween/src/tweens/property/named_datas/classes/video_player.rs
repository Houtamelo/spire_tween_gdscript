use super::*;

object_properties_enum! {
    GdBridge = VideoStreamPlayerTweener;
    Class = VideoStreamPlayer;
    Trait = DoVideoPlayerVolumeDb;

    @f32 {
        Data = PropertyF32VideoPlayerData;
        Enum = PropertyF32VideoPlayerKind;

        [VolumeDb, "volume_db", do_volume_db]
        fn get(n) { n.get_volume_db() }
        fn set(n, v) { n.set_volume_db(v); }

        [Volume, "volume", do_volume]
        fn get(n) { n.get_volume() }
        fn set(n, v) { n.set_volume(v); }
    }
}
