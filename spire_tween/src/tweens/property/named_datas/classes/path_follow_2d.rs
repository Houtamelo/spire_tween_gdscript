use super::*;

object_properties_enum! {
    GdBridge = PathFollow2DTweener;
    Class = PathFollow2D;
    Trait = DoPathFollowOffset;

    @f32 {
        Data = PropertyF32PathFollow2DData;
        Enum = PropertyF32PathFollow2DKind;

        [HOffset, "h_offset", do_h_offset]
        fn get(n) { n.get_h_offset() }
        fn set(n, v) { n.set_h_offset(v); }

        [VOffset, "v_offset", do_v_offset]
        fn get(n) { n.get_v_offset() }
        fn set(n, v) { n.set_v_offset(v); }

        [Progress, "progress", do_progress]
        fn get(n) { n.get_progress() }
        fn set(n, v) { n.set_progress(v); }

        [ProgressRatio, "progress_ratio", do_progress_ratio]
        fn get(n) { n.get_progress_ratio() }
        fn set(n, v) { n.set_progress_ratio(v); }
    }
}
