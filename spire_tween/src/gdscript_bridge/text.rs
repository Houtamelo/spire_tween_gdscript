use super::*;

#[godot_api(secondary)]
impl SpirePropertyString {
    /// Configures BBCode-aware interpolation and scrambling on a String property
    /// tween. Defaults preserve plain text. BBCode must be enabled on the
    /// receiver separately. Characters are Unicode scalars; images and escaped
    /// brackets count as one unit each, while paired tags carry no distance.
    /// Relative mode appends/removes parsed characters. Speed-based mode counts
    /// character edits per second; formatting-only changes apply immediately.
    /// Scrambling is supported only in absolute mode; relative/speed-based modes
    /// warn once and ignore it without changing mode. Custom alphabets may contain
    /// one character; an empty custom alphabet warns and uses letters/digits.
    /// Returns this handle for chaining. Configure between processing updates.
    #[func(gd_self)]
    pub fn set_text_options(
        this: Gd<Self>,
        #[opt(default = false)] rich_text_enabled: bool,
        #[opt(default = ScrambleMode::None)] scramble_mode: ScrambleMode,
        #[opt(default = "")] scramble_chars: GString,
    ) -> Gd<Self> {
        this.bind().to_mut().set_text_options(TextOptions {
            rich_text_enabled,
            scramble_mode,
            scramble_chars,
        });
        this
    }
}
