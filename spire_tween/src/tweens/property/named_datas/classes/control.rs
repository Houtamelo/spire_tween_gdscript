use super::*;

object_properties_enum! {
    GdBridge = ControlTweener;
    Class = Control;
    Trait = DoControl;

    @f32 {
        Data = PropertyF32ControlData;
        Enum = PropertyF32ControlKind;

        [AnchorBottom, "anchor_bottom", do_anchor_bottom]
        fn get(n) { n.get_anchor(Side::BOTTOM) }
        fn set(n, v) { n.set_anchor(Side::BOTTOM, v); }

        [AnchorLeft, "anchor_left", do_anchor_left]
        fn get(n) { n.get_anchor(Side::LEFT) }
        fn set(n, v) { n.set_anchor(Side::LEFT, v); }

        [AnchorRight, "anchor_right", do_anchor_right]
        fn get(n) { n.get_anchor(Side::RIGHT) }
        fn set(n, v) { n.set_anchor(Side::RIGHT, v); }

        [AnchorTop, "anchor_top", do_anchor_top]
        fn get(n) { n.get_anchor(Side::TOP) }
        fn set(n, v) { n.set_anchor(Side::TOP, v); }

        [MinSizeX, "custom_minimum_size:x", do_min_size_x]
        fn get(n) { n.get_custom_minimum_size().x }
        fn set(n, v) {
            let mut s = n.get_custom_minimum_size();
            s.x = v;
            n.set_custom_minimum_size(s);
        }

        [MinSizeY, "custom_minimum_size:y", do_min_size_y]
        fn get(n) { n.get_custom_minimum_size().y }
        fn set(n, v) {
            let mut s = n.get_custom_minimum_size();
            s.y = v;
            n.set_custom_minimum_size(s);
        }

        [PosX, "position:x", do_pos_x]
        fn get(n) { n.get_position().x }
        fn set(n, v) {
            let mut p = n.get_position();
            p.x = v;
            n.set_position(p);
        }

        [PosY, "position:y", do_pos_y]
        fn get(n) { n.get_position().y }
        fn set(n, v) {
            let mut p = n.get_position();
            p.y = v;
            n.set_position(p);
        }

        [MoveX, "position:x", do_move_x]
        fn get(n) { n.get_position().x }
        fn set(n, v) {
            let mut p = n.get_position();
            p.x = v;
            n.set_position(p);
        }

        [MoveY, "position:y", do_move_y]
        fn get(n) { n.get_position().y }
        fn set(n, v) {
            let mut p = n.get_position();
            p.y = v;
            n.set_position(p);
        }

        [GlobalPosX, "global_position:x", do_global_pos_x]
        fn get(n) { n.get_global_position().x }
        fn set(n, v) {
            let mut p = n.get_global_position();
            p.x = v;
            n.set_global_position(p);
        }

        [GlobalPosY, "global_position:y", do_global_pos_y]
        fn get(n) { n.get_global_position().y }
        fn set(n, v) {
            let mut p = n.get_global_position();
            p.y = v;
            n.set_global_position(p);
        }

        [OffsetBottom, "offset_bottom", do_offset_bottom]
        fn get(n) { n.get_offset(Side::BOTTOM) }
        fn set(n, v) { n.set_offset(Side::BOTTOM, v); }

        [OffsetLeft, "offset_left", do_offset_left]
        fn get(n) { n.get_offset(Side::LEFT) }
        fn set(n, v) { n.set_offset(Side::LEFT, v); }

        [OffsetRight, "offset_right", do_offset_right]
        fn get(n) { n.get_offset(Side::RIGHT) }
        fn set(n, v) { n.set_offset(Side::RIGHT, v); }

        [OffsetTop, "offset_top", do_offset_top]
        fn get(n) { n.get_offset(Side::TOP) }
        fn set(n, v) { n.set_offset(Side::TOP, v); }

        [PivotOffsetX, "pivot_offset:x", do_pivot_offset_x]
        fn get(n) { n.get_pivot_offset().x }
        fn set(n, v) {
            let mut p = n.get_pivot_offset();
            p.x = v;
            n.set_pivot_offset(p);
        }

        [PivotOffsetY, "pivot_offset:y", do_pivot_offset_y]
        fn get(n) { n.get_pivot_offset().y }
        fn set(n, v) {
            let mut p = n.get_pivot_offset();
            p.y = v;
            n.set_pivot_offset(p);
        }

        [Rotation, "rotation", do_rotation]
        fn get(n) { n.get_rotation() }
        fn set(n, v) { n.set_rotation(v); }

        [RotationDeg, "rotation_degrees", do_rotation_deg]
        fn get(n) { n.get_rotation_degrees() }
        fn set(n, v) { n.set_rotation_degrees(v); }

        [ScaleX, "scale:x", do_scale_x]
        fn get(n) { n.get_scale().x }
        fn set(n, v) {
            let mut s = n.get_scale();
            s.x = v;
            n.set_scale(s);
        }

        [ScaleY, "scale:y", do_scale_y]
        fn get(n) { n.get_scale().y }
        fn set(n, v) {
            let mut s = n.get_scale();
            s.y = v;
            n.set_scale(s);
        }

        [SizeX, "size:x", do_size_x]
        fn get(n) { n.get_size().x }
        fn set(n, v) {
            let mut s = n.get_size();
            s.x = v;
            n.set_size(s);
        }

        [SizeY, "size:y", do_size_y]
        fn get(n) { n.get_size().y }
        fn set(n, v) {
            let mut s = n.get_size();
            s.y = v;
            n.set_size(s);
        }
    }

    @vector2 {
        Data = PropertyVector2ControlData;
        Enum = PropertyVector2ControlKind;

        [MinSize, "custom_minimum_size", do_min_size]
        fn get(n) { n.get_custom_minimum_size() }
        fn set(n, v) { n.set_custom_minimum_size(v); }

        [Pos, "position", do_pos]
        fn get(n) { n.get_position() }
        fn set(n, v) { n.set_position(v); }

        [Move, "position", do_move]
        fn get(n) { n.get_position() }
        fn set(n, v) { n.set_position(v); }

        [GlobalPos, "global_position", do_global_pos]
        fn get(n) { n.get_global_position() }
        fn set(n, v) { n.set_global_position(v); }

        [PivotOffset, "pivot_offset", do_pivot_offset]
        fn get(n) { n.get_pivot_offset() }
        fn set(n, v) { n.set_pivot_offset(v); }

        [Scale, "scale", do_scale]
        fn get(n) { n.get_scale() }
        fn set(n, v) { n.set_scale(v); }

        [Size, "size", do_size]
        fn get(n) { n.get_size() }
        fn set(n, v) { n.set_size(v); }
    }
}
