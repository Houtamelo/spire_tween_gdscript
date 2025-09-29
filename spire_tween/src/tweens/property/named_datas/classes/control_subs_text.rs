use super::*;

object_properties_enum! {
    GdBridge = ButtonTweener;
    Class = Button;
    Trait = DoButton;
    @gstring {
        Data = PropertyGStringButtonData;
        Enum = PropertyGStringButtonKind;

        [Text, "text", do_text]
        fn get(n) { n.get_text() }
        fn set(n, v) { n.set_text(&v) }
    }
}

object_properties_enum! {
    GdBridge = LinkButtonTweener;
    Class = LinkButton;
    Trait = DoLinkButton;

    @gstring {
        Data = PropertyGStringLinkButtonData;
        Enum = PropertyGStringLinkButtonKind;

        [Text, "text", do_text]
        fn get(n) { n.get_text() }
        fn set(n, v) { n.set_text(&v) }
    }
}

object_properties_enum! {
    GdBridge = LabelTweener;
    Class = Label;
    Trait = DoLabel;

    @f32 {
        Data = PropertyF32LabelData;
        Enum = PropertyF32LabelKind;

        [VisibleRatio, "visible_ratio", do_visible_ratio]
        fn get(n) { n.get_visible_ratio() }
        fn set(n, v) { n.set_visible_ratio(v) }
    }

    @i32 {
        Data = PropertyI32LabelData;
        Enum = PropertyI32LabelKind;

        [MaxLinesVisible, "max_lines_visible", do_max_lines_visible]
        fn get(n) { n.get_max_lines_visible() }
        fn set(n, v) { n.set_max_lines_visible(v) }

        [VisibleCharacters, "visible_characters", do_visible_characters]
        fn get(n) { n.get_visible_characters() }
        fn set(n, v) { n.set_visible_characters(v) }
    }

    @gstring {
        Data = PropertyGStringLabelData;
        Enum = PropertyGStringLabelKind;

        [Text, "text", do_text]
        fn get(n) { n.get_text() }
        fn set(n, v) { n.set_text(&v) }
    }
}

object_properties_enum! {
    GdBridge = LineEditTweener;
    Class = LineEdit;
    Trait = DoLineEditText;

    @i32 {
        Data = PropertyI32LineEditData;
        Enum = PropertyI32LineEditKind;

        [MaxLength, "max_length", do_max_length]
        fn get(n) { n.get_max_length() }
        fn set(n, v) { n.set_max_length(v) }
    }

    @gstring {
        Data = PropertyGStringLineEditData;
        Enum = PropertyGStringLineEditKind;

        [Text, "text", do_text]
        fn get(n) { n.get_text() }
        fn set(n, v) { n.set_text(&v) }

        [PlaceholderText, "placeholder_text", do_placeholder_text]
        fn get(n) { n.get_placeholder() }
        fn set(n, v) { n.set_placeholder(&v) }
    }
}

object_properties_enum! {
    GdBridge = RichTextLabelTweener;
    Class = RichTextLabel;
    Trait = DoRichTextLabelText;

    @f32 {
        Data = PropertyF32RichTextLabelData;
        Enum = PropertyF32RichTextLabelKind;

        [VisibleRatio, "visible_ratio", do_visible_ratio]
        fn get(n) { n.get_visible_ratio() }
        fn set(n, v) { n.set_visible_ratio(v) }
    }

    @i32 {
        Data = PropertyI32RichTextLabelData;
        Enum = PropertyI32RichTextLabelKind;

        [VisibleCharacters, "visible_characters", do_visible_characters]
        fn get(n) { n.get_visible_characters() }
        fn set(n, v) { n.set_visible_characters(v) }
    }

    @gstring {
        Data = PropertyGStringRichTextLabelData;
        Enum = PropertyGStringRichTextLabelKind;

        [Text, "text", do_text]
        fn get(n) { n.get_text() }
        fn set(n, v) { n.set_text(&v) }
    }
}

object_properties_enum! {
    GdBridge = TextEditTweener;
    Class = TextEdit;
    Trait = DoTextEditText;

    @f64 {
        Data = PropertyF64TextEditData;
        Enum = PropertyF64TextEditKind;

        [ScrollVertical, "do_scroll_vertical", do_scroll_vertical]
        fn get(n) { n.get_v_scroll() }
        fn set(n, v) { n.set_v_scroll(v) }
    }

    @i32 {
        Data = PropertyI32TextEditData;
        Enum = PropertyI32TextEditKind;

        [MinimapWidth, "minimap_width", do_minimap_width]
        fn get(n) { n.get_minimap_width() }
        fn set(n, v) { n.set_minimap_width(v) }

        [ScrollHorizontal, "do_scroll_horizontal", do_scroll_horizontal]
        fn get(n) { n.get_h_scroll() }
        fn set(n, v) { n.set_h_scroll(v) }
    }

    @gstring {
        Data = PropertyGStringTextEditData;
        Enum = PropertyGStringTextEditKind;

        [Text, "text", do_text]
        fn get(n) { n.get_text() }
        fn set(n, v) { n.set_text(&v) }

        [PlaceholderText, "placeholder_text", do_placeholder_text]
        fn get(n) { n.get_placeholder() }
        fn set(n, v) { n.set_placeholder(&v) }
    }
}
