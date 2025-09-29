use super::*;

object_properties_enum! {
    GdBridge = Node2DTweener;
    Class = Node2D;
    Trait = DoNode2DGlobalMove;

    @f32 {
        Data = PropertyF32Node2DData;
        Enum = PropertyF32Node2DKind;

        [PositionX, "position:x", do_pos_x]
        fn get(n) { n.get_position().x }
        fn set(n, v) {
            let mut pos = n.get_position();
            pos.x = v;
            n.set_position(pos);
        }
        [PositionY, "position:y", do_pos_y]
        fn get(n) { n.get_position().y }
        fn set(n, v) {
            let mut pos = n.get_position();
            pos.y = v;
            n.set_position(pos);
        }
        [MoveX, "position:x", do_move_x]
        fn get(n) { n.get_position().x }
        fn set(n, v) {
            let mut pos = n.get_position();
            pos.x = v;
            n.set_position(pos);
        }
        [MoveY, "position:y", do_move_y]
        fn get(n) { n.get_position().y }
        fn set(n, v) {
            let mut pos = n.get_position();
            pos.y = v;
            n.set_position(pos);
        }

        [GlobalPositionX, "global_position:x", do_global_pos_x]
        fn get(n) { n.get_global_position().x }
        fn set(n, v) {
            let mut pos = n.get_global_position();
            pos.x = v;
            n.set_global_position(pos);
        }
        [GlobalPositionY, "global_position:y", do_global_pos_y]
        fn get(n) { n.get_global_position().y }
        fn set(n, v) {
            let mut pos = n.get_global_position();
            pos.y = v;
            n.set_global_position(pos);
        }

        [Rotation, "rotation", do_rotation]
        fn get(n) { n.get_rotation() }
        fn set(n, v) { n.set_rotation(v); }
        [RotationDegrees, "rotation_degrees", do_rotation_deg]
        fn get(n) { n.get_rotation_degrees() }
        fn set(n, v) { n.set_rotation_degrees(v); }
        [GlobalRotation, "global_rotation", do_global_rotation]
        fn get(n) { n.get_global_rotation() }
        fn set(n, v) { n.set_global_rotation(v); }
        [GlobalRotationDegrees, "global_rotation_degrees", do_global_rotation_deg]
        fn get(n) { n.get_global_rotation_degrees() }
        fn set(n, v) { n.set_global_rotation_degrees(v); }

        [Skew, "skew", do_skew]
        fn get(n) { n.get_skew() }
        fn set(n, v) { n.set_skew(v); }
        [GlobalSkew, "global_skew", do_global_skew]
        fn get(n) { n.get_global_skew() }
        fn set(n, v) { n.set_global_skew(v); }

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

        [GlobalScaleX, "global_scale:x", do_global_scale_x]
        fn get(n) { n.get_global_scale().x }
        fn set(n, v) {
            let mut scale = n.get_global_scale();
            scale.x = v;
            n.set_global_scale(scale);
        }
        [GlobalScaleY, "global_scale:y", do_global_scale_y]
        fn get(n) { n.get_global_scale().y }
        fn set(n, v) {
            let mut scale = n.get_global_scale();
            scale.y = v;
            n.set_global_scale(scale);
        }
    }

    @vector2 {
        Data = PropertyVector2Node2DData;
        Enum = PropertyVector2Node2DKind;

        [Position, "position", do_pos]
        fn get(n) { n.get_position() }
        fn set(n, v) { n.set_position(v); }
        [Move, "position", do_move]
        fn get(n) { n.get_position() }
        fn set(n, v) { n.set_position(v); }
        [GlobalPosition, "global_position", do_global_pos]
        fn get(n) { n.get_global_position() }
        fn set(n, v) { n.set_global_position(v); }
        [Scale, "scale", do_scale]
        fn get(n) { n.get_scale() }
        fn set(n, v) { n.set_scale(v); }
        [GlobalScale, "global_scale", do_global_scale]
        fn get(n) { n.get_global_scale() }
        fn set(n, v) { n.set_global_scale(v); }
    }
}

#[godot_api(secondary)]
impl Node2DTweener {
    #[func]
    pub fn do_follow(
        node: Gd<Node2D>,
        follow_this: Gd<Node2D>,
        speed: f64,
    ) -> Gd<InternalTweenPropertyVector2> {
        register_gd_handle! { node.do_follow(follow_this, speed), InternalTweenPropertyVector2::new_gd() }
    }
}
