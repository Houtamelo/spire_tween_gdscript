use super::*;

object_properties_enum! {
    GdBridge = CanvasItemTweener;
    Class = CanvasItem;
    Trait = DoCanvasItem;

    @f32 {
        Data = PropertyF32CanvasItemData;
        Enum = PropertyF32CanvasItemKind;

        [ColorR, "modulate:r", do_color_r]
        fn get(n) { n.get_modulate().r }
        fn set(n, v) {
            let mut c = n.get_modulate();
            c.r = v;
            n.set_modulate(c);
        }

        [ColorG, "modulate:g", do_color_g]
        fn get(n) { n.get_modulate().g }
        fn set(n, v) {
            let mut c = n.get_modulate();
            c.g = v;
            n.set_modulate(c);
        }

        [ColorB, "modulate:b", do_color_b]
        fn get(n) { n.get_modulate().b }
        fn set(n, v) {
            let mut c = n.get_modulate();
            c.b = v;
            n.set_modulate(c);
        }

        [ColorA, "modulate:a", do_color_a]
        fn get(n) { n.get_modulate().a }
        fn set(n, v) {
            let mut c = n.get_modulate();
            c.a = v;
            n.set_modulate(c);
        }

        [ModulateR, "modulate:r", do_modulate_r]
        fn get(n) { n.get_modulate().r }
        fn set(n, v) {
            let mut c = n.get_modulate();
            c.r = v;
            n.set_modulate(c);
        }

        [ModulateG, "modulate:g", do_modulate_g]
        fn get(n) { n.get_modulate().g }
        fn set(n, v) {
            let mut c = n.get_modulate();
            c.g = v;
            n.set_modulate(c);
        }

        [ModulateB, "modulate:b", do_modulate_b]
        fn get(n) { n.get_modulate().b }
        fn set(n, v) {
            let mut c = n.get_modulate();
            c.b = v;
            n.set_modulate(c);
        }

        [ModulateA, "modulate:a", do_modulate_a]
        fn get(n) { n.get_modulate().a }
        fn set(n, v) {
            let mut c = n.get_modulate();
            c.a = v;
            n.set_modulate(c);
        }

        [Fade, "modulate:a", do_fade]
        fn get(n) { n.get_modulate().a }
        fn set(n, v) {
            let mut c = n.get_modulate();
            c.a = v;
            n.set_modulate(c);
        }

        [SelfColorR, "self_modulate:r", do_self_color_r]
        fn get(n) { n.get_self_modulate().r }
        fn set(n, v) {
            let mut c = n.get_self_modulate();
            c.r = v;
            n.set_self_modulate(c);
        }

        [SelfColorG, "self_modulate:g", do_self_color_g]
        fn get(n) { n.get_self_modulate().g }
        fn set(n, v) {
            let mut c = n.get_self_modulate();
            c.g = v;
            n.set_self_modulate(c);
        }

        [SelfColorB, "self_modulate:b", do_self_color_b]
        fn get(n) { n.get_self_modulate().b }
        fn set(n, v) {
            let mut c = n.get_self_modulate();
            c.b = v;
            n.set_self_modulate(c);
        }

        [SelfColorA, "self_modulate:a", do_self_color_a]
        fn get(n) { n.get_self_modulate().a }
        fn set(n, v) {
            let mut c = n.get_self_modulate();
            c.a = v;
            n.set_self_modulate(c);
        }

        [SelfFade, "self_modulate:a", do_self_fade]
        fn get(n) { n.get_self_modulate().a }
        fn set(n, v) {
            let mut c = n.get_self_modulate();
            c.a = v;
            n.set_self_modulate(c);
        }

        [SelfModulateR, "self_modulate:r", do_self_modulate_r]
        fn get(n) { n.get_self_modulate().r }
        fn set(n, v) {
            let mut c = n.get_self_modulate();
            c.r = v;
            n.set_self_modulate(c);
        }

        [SelfModulateG, "self_modulate:g", do_self_modulate_g]
        fn get(n) { n.get_self_modulate().g }
        fn set(n, v) {
            let mut c = n.get_self_modulate();
            c.g = v;
            n.set_self_modulate(c);
        }

        [SelfModulateB, "self_modulate:b", do_self_modulate_b]
        fn get(n) { n.get_self_modulate().b }
        fn set(n, v) {
            let mut c = n.get_self_modulate();
            c.b = v;
            n.set_self_modulate(c);
        }

        [SelfModulateA, "self_modulate:a", do_self_modulate_a]
        fn get(n) { n.get_self_modulate().a }
        fn set(n, v) {
            let mut c = n.get_self_modulate();
            c.a = v;
            n.set_self_modulate(c);
        }
    }

    @i32 {
        Data = PropertyI32CanvasItemData;
        Enum = PropertyI32CanvasItemKind;

        [ZIndex, "z_index", do_z_index]
        fn get(n) { n.get_z_index() }
        fn set(n, v) { n.set_z_index(v); }
    }


    @color {
        Data = PropertyColorCanvasItemData;
        Enum = PropertyColorCanvasItemKind;

        [Color, "modulate", do_color]
        fn get(n) { n.get_modulate() }
        fn set(n, v) { n.set_modulate(v) }

        [Modulate, "modulate", do_modulate]
        fn get(n) { n.get_modulate() }
        fn set(n, v) { n.set_modulate(v) }

        [SelfColor, "self_modulate", do_self_color]
        fn get(n) { n.get_self_modulate() }
        fn set(n, v) { n.set_self_modulate(v) }

        [SelfModulate, "self_modulate", do_self_modulate]
        fn get(n) { n.get_self_modulate() }
        fn set(n, v) { n.set_self_modulate(v) }
    }
}

// bellow just needs to compile
#[allow(unused)]
#[cfg(test)]
mod tests {
    use godot::{classes::*, prelude::*};

    use super::*;

    fn test(node_direct: &Gd<Sprite2D>, node_ref: Gd<CanvasItem>, node_tref: Gd<CpuParticles2D>) {
        node_direct.do_color_r(1., 5.);
        node_ref.do_color_r(1., 5.);
        node_tref.do_color_r(1., 5.);

        node_direct.do_color_g(1., 5.);
        node_ref.do_color_g(1., 5.);
        node_tref.do_color_g(1., 5.);

        node_direct.do_color_b(1., 5.);
        node_ref.do_color_b(1., 5.);
        node_tref.do_color_b(1., 5.);

        node_direct.do_color_a(1., 5.);
        node_ref.do_color_a(1., 5.);
        node_tref.do_color_a(1., 5.);

        node_direct.do_fade(1., 5.);
        node_ref.do_fade(1., 5.);
        node_tref.do_fade(1., 5.);

        let color = Color::from_rgb(1., 1., 1.);
        node_direct.do_color(color, 5.);
        node_ref.do_color(color, 5.);
        node_tref.do_color(color, 5.);
    }

    #[derive(GodotClass)]
    #[class(init, base = Node2D)]
    struct Test {
        base: Base<Node2D>,
    }

    #[godot_api]
    impl INode2D for Test {
        fn ready(&mut self) { self.do_color_r(1., 2.0); }
    }

    fn please_compile(
        node_direct: Gd<Sprite2D>,
        node_ref: &Gd<CanvasItem>,
        node_tref: Gd<CpuParticles2D>,
    ) {
        node_direct.do_self_color_r(1., 5.0);
        node_ref.do_self_color_r(1., 5.0);
        node_tref.do_self_color_r(1., 5.0);

        node_direct.do_self_color_g(1., 5.0);
        node_ref.do_self_color_g(1., 5.0);
        node_tref.do_self_color_g(1., 5.0);

        node_direct.do_self_color_b(1., 5.0);
        node_ref.do_self_color_b(1., 5.0);
        node_tref.do_self_color_b(1., 5.0);

        node_direct.do_self_color_a(1., 5.0);
        node_ref.do_self_color_a(1., 5.0);
        node_tref.do_self_color_a(1., 5.0);

        node_direct.do_self_fade(1., 5.0);
        node_ref.do_self_fade(1., 5.0);
        node_tref.do_self_fade(1., 5.0);

        let color = Color::from_rgb(1., 1., 1.);
        node_direct.do_self_color(color, 5.0);
        node_ref.do_self_color(color, 5.0);
        node_tref.do_self_color(color, 5.0);
    }
}
