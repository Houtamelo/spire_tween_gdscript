use super::*;

object_properties_enum! {
    GdBridge = Camera2DTweener;
    Class = Camera2D;
    Trait = DoCamera2D;

    @f32 {
        Data = PropertyF32Camera2DData;
        Enum = PropertyF32Camera2DKind;
        [ZoomX, "zoom:x", do_zoom_x]
        fn get(n) { n.get_zoom().x }
        fn set(n, v) {
            let mut zoom = n.get_zoom();
            zoom.x = v;
            n.set_zoom(zoom);
        }
        [ZoomY, "zoom:y", do_zoom_y]
        fn get(n) { n.get_zoom().y }
        fn set(n, v) {
            let mut zoom = n.get_zoom();
            zoom.y = v;
            n.set_zoom(zoom);
        }
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
        [DragHorizontalOffset, "drag_horizontal_offset", do_drag_horizontal_offset]
        fn get(n) { n.get_drag_horizontal_offset() }
        fn set(n, v) { n.set_drag_horizontal_offset(v) }
        [DragVerticalOffset, "drag_vertical_offset", do_drag_vertical_offset]
        fn get(n) { n.get_drag_vertical_offset() }
        fn set(n, v) { n.set_drag_vertical_offset(v) }
        [DragLeftMargin, "drag_left_margin", do_drag_left_margin]
        fn get(n) { n.get_drag_margin(Side::LEFT) }
        fn set(n, v) { n.set_drag_margin(Side::LEFT, v) }
        [DragRightMargin, "drag_right_margin", do_drag_right_margin]
        fn get(n) { n.get_drag_margin(Side::RIGHT) }
        fn set(n, v) { n.set_drag_margin(Side::RIGHT, v) }
        [DragTopMargin, "drag_top_margin", do_drag_top_margin]
        fn get(n) { n.get_drag_margin(Side::TOP) }
        fn set(n, v) { n.set_drag_margin(Side::TOP, v) }
        [DragBottomMargin, "drag_bottom_margin", do_drag_bottom_margin]
        fn get(n) { n.get_drag_margin(Side::BOTTOM) }
        fn set(n, v) { n.set_drag_margin(Side::BOTTOM, v) }
    }

    @vector2 {
        Data = PropertyVector2Camera2DData;
        Enum = PropertyVector2Camera2DKind;

        [Zoom, "zoom", do_zoom]
        fn get(n) { n.get_zoom() }
        fn set(n, v) { n.set_zoom(v); }
        [Offset, "offset", do_offset]
        fn get(n) { n.get_offset() }
        fn set(n, v) { n.set_offset(v); }
    }
}
