use super::*;

object_properties_enum! {
    GdBridge = RangeTweener;
    Class = Range;
    Trait = DoRangeValue;

    @f64 {
        Data = PropertyF64RangeData;
        Enum = PropertyF64RangeKind;

        [Value, "value", do_value]
        fn get(n) { n.get_value() }
        fn set(n, v) { n.set_value(v); }

        [Ratio, "ratio", do_ratio]
        fn get(n) { n.get_as_ratio() }
        fn set(n, v) { n.set_as_ratio(v); }

        [MinValue, "min_value", do_min_value]
        fn get(n) { n.get_min() }
        fn set(n, v) { n.set_min(v); }

        [MaxValue, "max_value", do_max_value]
        fn get(n) { n.get_max() }
        fn set(n, v) { n.set_max(v); }
    }
}

object_properties_enum! {
    GdBridge = TextureProgressBarTweener;
    Class = TextureProgressBar;
    Trait = DoTextureProgressTintUnder;

    @f32 {
        Data = PropertyF32TextureProgressBarData;
        Enum = PropertyF32TextureProgressBarKind;

        [RadialInitialAngle, "radial_initial_angle", do_radial_initial_angle]
        fn get(n) { n.clone().get_radial_initial_angle() }
        fn set(n, v) { n.set_radial_initial_angle(v); }

        [RadialFillDegrees, "radial_fill_degrees", do_radial_fill_degrees]
        fn get(n) { n.clone().get_fill_degrees() }
        fn set(n, v) { n.set_fill_degrees(v); }

        [TintUnderA, "tint_under:a", do_tint_under_a]
        fn get(n) { n.get_tint_under().a }
        fn set(n, v) {
            let mut c = n.get_tint_under();
            c.a = v;
            n.set_tint_under(c);
        }

        [TintUnderR, "tint_under:r", do_tint_under_r]
        fn get(n) { n.get_tint_under().r }
        fn set(n, v) {
            let mut c = n.get_tint_under();
            c.r = v;
            n.set_tint_under(c);
        }

        [TintUnderG, "tint_under:g", do_tint_under_g]
        fn get(n) { n.get_tint_under().g }
        fn set(n, v) {
            let mut c = n.get_tint_under();
            c.g = v;
            n.set_tint_under(c);
        }

        [TintUnderB, "tint_under:b", do_tint_under_b]
        fn get(n) { n.get_tint_under().b }
        fn set(n, v) {
            let mut c = n.get_tint_under();
            c.b = v;
            n.set_tint_under(c);
        }

        [TintOverA, "tint_over:a", do_tint_over_a]
        fn get(n) { n.get_tint_over().a }
        fn set(n, v) {
            let mut c = n.get_tint_over();
            c.a = v;
            n.set_tint_over(c);
        }

        [TintOverR, "tint_over:r", do_tint_over_r]
        fn get(n) { n.get_tint_over().r }
        fn set(n, v) {
            let mut c = n.get_tint_over();
            c.r = v;
            n.set_tint_over(c);
        }

        [TintOverG, "tint_over:g", do_tint_over_g]
        fn get(n) { n.get_tint_over().g }
        fn set(n, v) {
            let mut c = n.get_tint_over();
            c.g = v;
            n.set_tint_over(c);
        }

        [TintOverB, "tint_over:b", do_tint_over_b]
        fn get(n) { n.get_tint_over().b }
        fn set(n, v) {
            let mut c = n.get_tint_over();
            c.b = v;
            n.set_tint_over(c);
        }

        [TintProgressA, "tint_progress:a", do_tint_progress_a]
        fn get(n) { n.get_tint_progress().a }
        fn set(n, v) {
            let mut c = n.get_tint_progress();
            c.a = v;
            n.set_tint_progress(c);
        }

        [TintProgressR, "tint_progress:r", do_tint_progress_r]
        fn get(n) { n.get_tint_progress().r }
        fn set(n, v) {
            let mut c = n.get_tint_progress();
            c.r = v;
            n.set_tint_progress(c);
        }

        [TintProgressG, "tint_progress:g", do_tint_progress_g]
        fn get(n) { n.get_tint_progress().g }
        fn set(n, v) {
            let mut c = n.get_tint_progress();
            c.g = v;
            n.set_tint_progress(c);
        }

        [TintProgressB, "tint_progress:b", do_tint_progress_b]
        fn get(n) { n.get_tint_progress().b }
        fn set(n, v) {
            let mut c = n.get_tint_progress();
            c.b = v;
            n.set_tint_progress(c);
        }

        [RadialCenterOffsetX, "radial_center_offset:x", do_radial_center_offset_x]
        fn get(n) { n.clone().get_radial_center_offset().x }
        fn set(n, v) {
            let mut o = n.get_radial_center_offset();
            o.x = v;
            n.set_radial_center_offset(o);
        }

        [RadialCenterOffsetY, "radial_center_offset:y", do_radial_center_offset_y]
        fn get(n) { n.clone().get_radial_center_offset().y }
        fn set(n, v) {
            let mut o = n.get_radial_center_offset();
            o.y = v;
            n.set_radial_center_offset(o);
        }
    }

     @color {
        Data = PropertyColorTextureProgressBarData;
        Enum = PropertyColorTextureProgressBarKind;

        [TintUnder, "tint_under", do_tint_under]
        fn get(n) { n.get_tint_under() }
        fn set(n, v) { n.set_tint_under(v); }

        [TintOver, "tint_over", do_tint_over]
        fn get(n) { n.get_tint_over() }
        fn set(n, v) { n.set_tint_over(v); }

        [TintProgress, "tint_progress", do_tint_progress]
        fn get(n) { n.get_tint_progress() }
        fn set(n, v) { n.set_tint_progress(v); }
    }

    @vector2 {
        Data = PropertyVector2TextureProgressBarData;
        Enum = PropertyVector2TextureProgressBarKind;

        [RadialCenterOffset, "radial_center_offset", do_radial_center_offset]
        fn get(n) { n.clone().get_radial_center_offset() }
        fn set(n, v) { n.set_radial_center_offset(v); }
    }
}
