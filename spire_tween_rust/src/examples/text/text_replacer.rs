use godot::classes::{Button, Control, IControl, Label};
use godot::prelude::*;
use spire_tween::prelude::*;

/// Port of `examples/text/text_replacer.gd`.
///
/// Demonstrates animating a `Label`'s `text` property using `do_text` with
/// speed-based tweening. Each line's text is progressively replaced character
/// by character.
#[derive(GodotClass)]
#[class(init, base = Control)]
pub struct TextReplacer {
    base: Base<Control>,

    #[export]
    #[init(val = 30.0)]
    chars_per_second: f64,

    #[var]
    #[init(val = -1)]
    idx: i32,

    /// Handle to the current tween, used to cancel it when switching lines.
    #[init(val = None)]
    tween: Option<RcPtr<SpireTween<LerpPropertyData<GString>>>>,
}

const DIALOGUES: &[&str] = &[
    "Hi there.",
    "Thank you for trying out Spire Tween!",
    "I hope these examples give you inspiration to build wonderful games :)",
];

#[godot_api]
impl IControl for TextReplacer {
    fn ready(&mut self) {
        let mut button_next: Gd<Button> = self.base().get_node_as("Button_Next");
        let mut button_prev: Gd<Button> = self.base().get_node_as("Button_Previous");

        let self_gd = self.to_gd();

        // Connect "Next" button.
        let self_next = self_gd.clone();
        button_next.connect(
            "pressed",
            &Callable::from_fn("next_line", move |_| {
                if self_next.is_instance_valid() {
                    self_next.clone().bind_mut().next_line();
                }
                Variant::nil()
            }),
        );

        // Connect "Previous" button.
        let self_prev = self_gd.clone();
        button_prev.connect(
            "pressed",
            &Callable::from_fn("prev_line", move |_| {
                if self_prev.is_instance_valid() {
                    self_prev.clone().bind_mut().previous_line();
                }
                Variant::nil()
            }),
        );
    }
}

impl TextReplacer {
    fn next_line(&mut self) {
        let next_idx = self.idx + 1;
        if next_idx as usize >= DIALOGUES.len() {
            return;
        }

        // Kill previous tween.
        if let Some(handle) = self.tween.take() {
            handle.to_mut().stop();
        }

        self.idx = next_idx;
        let text = DIALOGUES[self.idx as usize];

        let label: Gd<Label> = self.base().get_node_as("Panel/Label");

        // DoLabel.text(label, text, chars_per_second).as_speed_based()
        self.tween = Some(
            label
                .do_text(GString::from(text), self.chars_per_second)
                .as_speed_based()
                .register(),
        );
    }

    fn previous_line(&mut self) {
        let prev_idx = self.idx - 1;
        if prev_idx < 0 {
            return;
        }

        // Kill previous tween.
        if let Some(handle) = self.tween.take() {
            handle.to_mut().stop();
        }

        self.idx = prev_idx;
        let text = DIALOGUES[self.idx as usize];

        let label: Gd<Label> = self.base().get_node_as("Panel/Label");

        // DoLabel.text(label, text, chars_per_second).as_speed_based()
        self.tween = Some(
            label
                .do_text(GString::from(text), self.chars_per_second)
                .as_speed_based()
                .register(),
        );
    }
}
