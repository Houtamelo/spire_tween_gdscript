use super::*;
/**This class provides shortcut constructors to create tweens that animate a [TextEdit].

[b]Note:[/b] This class is not meant to be instantiated. To animate properties of a base class, use the methods in the "namespace" `Do[BaseClass]` instead (replace `[BaseClass]` with the base class' name).*/
#[derive(GodotClass)]
#[class(base = Object, no_init)]
pub struct DoTextEdit {}
#[godot_api]
impl DoTextEdit {
    /**[b]Behavior: [/b]Tweens the property [member TextEdit.minimap_width] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = minimap_width)]
    fn r#minimap_width(
        node: Gd<TextEdit>,
        to: i64,
        duration: f64,
    ) -> Gd<SpirePropertyInt> {
        let inner = UnsafeCell::new(node.do_minimap_width(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyInt { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the property [member TextEdit.scroll_horizontal] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = scroll_horizontal)]
    fn r#scroll_horizontal(
        node: Gd<TextEdit>,
        to: i64,
        duration: f64,
    ) -> Gd<SpirePropertyInt> {
        let inner = UnsafeCell::new(node.do_scroll_horizontal(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyInt { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the property [member TextEdit.scroll_vertical] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = scroll_vertical)]
    fn r#scroll_vertical(
        node: Gd<TextEdit>,
        to: f64,
        duration: f64,
    ) -> Gd<SpirePropertyFloat> {
        let inner = UnsafeCell::new(node.do_scroll_vertical(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyFloat { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the property [member TextEdit.text] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = text)]
    fn r#text(
        node: Gd<TextEdit>,
        to: GString,
        duration: f64,
    ) -> Gd<SpirePropertyString> {
        let inner = UnsafeCell::new(node.do_text(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyString { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
    /**[b]Behavior: [/b]Tweens the property [member TextEdit.placeholder_text] over [param duration] seconds.

[b]Returns:[/b] A handle that can be used to further customize the tween.*/
    #[func(rename = placeholder_text)]
    fn r#placeholder_text(
        node: Gd<TextEdit>,
        to: GString,
        duration: f64,
    ) -> Gd<SpirePropertyString> {
        let inner = UnsafeCell::new(node.do_placeholder_text(to, duration).register());
        let handle = Gd::from_init_fn(|base| SpirePropertyString { base, inner });
        let handle_clone = handle.clone();
        handle.bind().to_mut().gd_handle = Some(handle_clone);
        handle
    }
}
