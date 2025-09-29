use super::*;

object_properties_enum! {
    GdBridge = CanvasModulateTweener;
    Class = CanvasModulate;
    Trait = DoCanvasModulate;

    @f32 {
        Data = PropertyF32CanvasModulateData;
        Enum = PropertyF32CanvasModulateKind;

        [ColorR, "color:r", do_canvas_color_r]
        fn get(n) { n.get_color().r }
        fn set(n, v) {
            let mut c = n.get_color();
            c.r = v;
            n.set_color(c);
        }

        [ColorG, "color:g", do_canvas_color_g]
        fn get(n) { n.get_color().g }
        fn set(n, v) {
            let mut c = n.get_color();
            c.g = v;
            n.set_color(c);
        }

        [ColorB, "color:b", do_canvas_color_b]
        fn get(n) { n.get_color().b }
        fn set(n, v) {
            let mut c = n.get_color();
            c.b = v;
            n.set_color(c);
        }

        [ColorA, "color:a", do_canvas_color_a]
        fn get(n) { n.get_color().a }
        fn set(n, v) {
            let mut c = n.get_color();
            c.a = v;
            n.set_color(c);
        }

        [Fade, "color:a", do_canvas_fade]
        fn get(n) { n.get_color().a }
        fn set(n, v) {
            let mut c = n.get_color();
            c.a = v;
            n.set_color(c);
        }
    }

    @color {
        Data = PropertyColorCanvasModulateData;
        Enum = PropertyColorCanvasModulateKind;

        [Color, "color", do_canvas_color]
        fn get(n) { n.get_color() }
        fn set(n, v) { n.set_color(v); }
    }
}
