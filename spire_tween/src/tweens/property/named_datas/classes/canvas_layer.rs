use super::*;

object_properties_enum! {
    GdBridge = CanvasLayerTweener;
    Class = CanvasLayer;
    Trait = DoCanvasLayer;

    @f32 {
        Data = PropertyF32CanvasLayerData;
        Enum = PropertyF32CanvasLayerKind;

        [OffsetX, "offset:x", do_offset_x]
        fn get(n) { n.get_offset().x }
        fn set(n, v) {
            let mut offset = n.get_offset();
            offset.x = v;
            n.set_offset(offset);
        }

        [OffsetY, "offset:y", do_offset_y]
        fn get(n) { n.get_offset().y }
        fn set(n, v) {
            let mut offset = n.get_offset();
            offset.y = v;
            n.set_offset(offset);
        }

        [Rotation, "rotation", do_rotation]
        fn get(n) { n.get_rotation() }
        fn set(n, v) { n.set_rotation(v); }

        [ScaleX, "scale:x", do_scale_x]
        fn get(n) { n.get_scale().x }
        fn set(n, v) {
            let mut scale = n.get_scale();
            scale.x = v;
            n.set_scale(scale);
        }

        [ScaleY, "scale:y", do_scale_y]
        fn get(n) { n.get_scale().y }
        fn set(n, v) {
            let mut scale = n.get_scale();
            scale.y = v;
            n.set_scale(scale);
        }

        [FollowViewportScale, "follow_viewport_scale", do_follow_viewport_scale]
        fn get(n) { n.get_follow_viewport_scale() }
        fn set(n, v) { n.set_follow_viewport_scale(v); }
    }

    @vector2 {
        Data = PropertyVector2CanvasLayerData;
        Enum = PropertyVector2CanvasLayerKind;

        [Offset, "offset", do_offset]
        fn get(n) { n.get_offset() }
        fn set(n, v) { n.set_offset(v); }

        [Scale, "scale", do_scale]
        fn get(n) { n.get_scale() }
        fn set(n, v) { n.set_scale(v); }
    }
}
