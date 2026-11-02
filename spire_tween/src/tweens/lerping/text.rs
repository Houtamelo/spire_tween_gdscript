//! String property interpolation with optional BBCode handling and scrambling.
//! Parsed glyphs and styles borrow character ranges from cached Godot strings.
use std::ops::Range;

use rand::{Rng, SeedableRng};
use rand_xoshiro::Xoshiro256PlusPlus;

use super::*;

/// Options for string property tweens, including the `do_text` shortcuts.
/// Defaults preserve ordinary string interpolation. Markup is Godot BBCode, not HTML.
/// Scrambling uses a revealed target prefix and a random unrevealed suffix.
/// Custom alphabets can contain any Unicode scalar, including a single character.
/// Relative and speed-based modes support BBCode but warn and ignore scrambling.
#[derive(Clone, Debug, Default)]
pub struct TextOptions {
    /// Count parsed text units and preserve balanced Godot BBCode tags.
    /// Does not enable BBCode rendering on the receiver.
    pub rich_text_enabled: bool,
    /// Random alphabet for unrevealed characters in absolute mode only.
    pub scramble_mode: ScrambleMode,
    /// Unicode alphabet for [`ScrambleMode::Custom`]. Empty warns and uses All.
    pub scramble_chars: GString,
}

macro_rules! is_control_tag {
    ($name:expr) => {
        matches!(
            $name,
            ['l', 'b']
                | ['r', 'b']
                | ['l', 'r', 'm']
                | ['r', 'l', 'm']
                | ['l', 'r', 'e']
                | ['r', 'l', 'e']
                | ['l', 'r', 'o']
                | ['r', 'l', 'o']
                | ['p', 'd', 'f']
                | ['a', 'l', 'm']
                | ['l', 'r', 'i']
                | ['r', 'l', 'i']
                | ['f', 's', 'i']
                | ['p', 'd', 'i']
                | ['z', 'w', 'j']
                | ['z', 'w', 'n', 'j']
                | ['w', 'j']
                | ['s', 'h', 'y']
        )
    };
}

struct Style {
    open:   Range<usize>,
    name:   Range<usize>,
    parent: Option<usize>,
}

struct Glyph {
    text:  Range<usize>,
    style: Option<usize>,
}

/// Storage is reused when an endpoint, current value, or relative sample changes.
/// Glyphs own no strings or style stacks: ranges refer to `source.chars()`.
#[derive(Default)]
struct ParsedText {
    source: GString,
    rich:   bool,
    glyphs: Vec<Glyph>,
    styles: Vec<Style>,
}

impl ParsedText {
    fn update(&mut self, source: &GString, rich: bool) {
        if self.source == *source && self.rich == rich {
            return;
        }

        self.source.clone_from(source);
        self.rich = rich;
        self.glyphs.clear();
        self.styles.clear();
        let text = source.chars();
        let mut cursor = 0;
        let mut style = None;
        while cursor < text.len() {
            if rich
                && text[cursor] == '['
                && let Some(end) = tag_end(text, cursor)
            {
                let name_start = cursor + 1;
                let name_end = (name_start..end - 1)
                    .find(|&i| matches!(text[i], '=' | ' ' | '\t'))
                    .unwrap_or(end - 1);
                let name = &text[name_start..name_end];
                if let Some(close) = name.strip_prefix(&['/']) {
                    if let Some(index) = style {
                        let active: &Style = &self.styles[index];
                        if &text[active.name.clone()] == close {
                            style = active.parent;
                            cursor = end;
                            continue;
                        }
                    }
                } else if is_control_tag!(name) {
                    self.glyphs.push(Glyph {
                        text: cursor..end,
                        style,
                    });
                    cursor = end;
                    continue;
                } else if !name.is_empty()
                    && name.iter().all(|c| c.is_ascii_alphanumeric() || *c == '_')
                    && let Some(close_end) = closing_tag_end(text, end, name)
                {
                    if name == ['i', 'm', 'g'] {
                        self.glyphs.push(Glyph {
                            text: cursor..close_end,
                            style,
                        });
                        cursor = close_end;
                    } else {
                        let index = self.styles.len();
                        self.styles.push(Style {
                            open:   cursor..end,
                            name:   name_start..name_end,
                            parent: style,
                        });
                        style = Some(index);
                        cursor = end;
                    }
                    continue;
                }
            }
            self.glyphs.push(Glyph {
                text: cursor..cursor + 1,
                style,
            });
            cursor += 1;
        }
    }
}

// Locate a tag's end without cutting a quoted attribute containing ']'.
fn tag_end(text: &[char], start: usize) -> Option<usize> {
    let mut quote = None;
    for (i, &c) in text.iter().enumerate().skip(start + 1) {
        match (quote, c) {
            (Some(q), c) if q == c => quote = None,
            (None, '\'' | '"') => quote = Some(c),
            (None, ']') => return Some(i + 1),
            _ => {}
        }
    }
    None
}

fn closing_tag_end(text: &[char], start: usize, name: &[char]) -> Option<usize> {
    let length = name.len() + 3;
    text[start..]
        .windows(length)
        .position(|candidate| {
            candidate.starts_with(&['[', '/']) && candidate[length - 1] == ']' && &candidate[2..length - 1] == name
        })
        .map(|offset| start + offset + length)
}

/// Identifies a cached glyph; scrambling overrides only its displayed character.
struct RenderGlyph {
    document: usize,
    index: usize,
    replacement: Option<char>,
}

impl RenderGlyph {
    fn original(document: usize, index: usize) -> Self {
        Self {
            document,
            index,
            replacement: None,
        }
    }
}

#[derive(Default)]
struct RenderBuffer {
    output: Vec<char>,
    active: SmallVec<[(usize, usize); 8]>,
    next:   SmallVec<[(usize, usize); 8]>,
}

impl RenderBuffer {
    fn close_style(&mut self, text: &[char], style: &Style) {
        self.output.extend_from_slice(&['[', '/']);
        self.output.extend_from_slice(&text[style.name.clone()]);
        self.output.push(']');
    }

    fn render(
        &mut self,
        documents: [&ParsedText; 2],
        glyphs: impl IntoIterator<Item = RenderGlyph>,
        rich: bool,
    ) -> GString {
        self.output.clear();
        self.active.clear();
        let text = [documents[0].source.chars(), documents[1].source.chars()];
        let mut previous_style = None;
        for glyph in glyphs {
            let document = documents[glyph.document];
            let style_key = (glyph.document, document.glyphs[glyph.index].style);
            if previous_style != Some(style_key) {
                self.next.clear();
                let mut style = style_key.1;
                while let Some(index) = style {
                    self.next.push((glyph.document, index));
                    style = document.styles[index].parent;
                }
                self.next.reverse();
                let common = self
                    .active
                    .iter()
                    .zip(&self.next)
                    .take_while(|((a_doc, a), (b_doc, b))| {
                        text[*a_doc][documents[*a_doc].styles[*a].open.clone()]
                            == text[*b_doc][documents[*b_doc].styles[*b].open.clone()]
                    })
                    .count();
                for index in (common..self.active.len()).rev() {
                    let (document, style) = self.active[index];
                    self.close_style(text[document], &documents[document].styles[style]);
                }
                for &(document, style) in &self.next[common..] {
                    self.output
                        .extend_from_slice(&text[document][documents[document].styles[style].open.clone()]);
                }
                self.active.clear();
                self.active.extend_from_slice(&self.next);
                previous_style = Some(style_key);
            }
            match glyph.replacement {
                Some('[') if rich => self.output.extend_from_slice(&['[', 'l', 'b', ']']),
                Some(']') if rich => self.output.extend_from_slice(&['[', 'r', 'b', ']']),
                Some(c) => self.output.push(c),
                None => self
                    .output
                    .extend_from_slice(&text[glyph.document][document.glyphs[glyph.index].text.clone()]),
            }
        }
        for index in (0..self.active.len()).rev() {
            let (document, style) = self.active[index];
            self.close_style(text[document], &documents[document].styles[style]);
        }
        GString::from(self.output.as_slice())
    }
}

#[derive(Default)]
struct TextCache {
    from: ParsedText,
    to: ParsedText,
    current: ParsedText,
    relative_sample: ParsedText,
    render: RenderBuffer,
    alphabet: SmallVec<[char; 64]>,
    alphabet_ready: bool,
    rng: Option<Xoshiro256PlusPlus>,
}

impl TextCache {
    fn configure_alphabet(&mut self, options: &TextOptions, ignore_scramble: bool) {
        if self.alphabet_ready {
            return;
        }
        self.alphabet.clear();
        let mode = if ignore_scramble { ScrambleMode::None } else { options.scramble_mode };
        let alphabet = match mode {
            ScrambleMode::None | ScrambleMode::Custom => "",
            ScrambleMode::All => "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",
            ScrambleMode::Uppercase => "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
            ScrambleMode::Lowercase => "abcdefghijklmnopqrstuvwxyz",
            ScrambleMode::Numerals => "0123456789",
        };
        if mode == ScrambleMode::Custom {
            self.alphabet.extend_from_slice(options.scramble_chars.chars());
            if self.alphabet.is_empty() {
                godot_warn!(
                    "[spire text] custom scramble alphabet is empty, got {:?}; using ASCII letters and digits.",
                    options.scramble_chars
                );
                self.alphabet
                    .extend("ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789".chars());
            }
        } else {
            self.alphabet.extend(alphabet.chars());
        }
        self.alphabet_ready = true;
    }

    fn sample(&mut self, from: &GString, to: &GString, progress: f64, rich: bool) -> GString {
        let p = progress.clamp(0.0, 1.0);
        if p >= 1.0 {
            return to.clone();
        }
        if p <= 0.0 {
            return from.clone();
        }
        self.from.update(from, rich);
        self.to.update(to, rich);
        let from_len = self.from.glyphs.len();
        let to_len = self.to.glyphs.len();
        let revealed = (to_len as f64 * p).round() as usize;
        if self.alphabet.is_empty() {
            // Preserve the plain lerper's rounding for shrinking half-steps.
            let length = (from_len as i64 + ((to_len as f64 - from_len as f64) * p).round() as i64) as usize;
            let glyphs = (0..revealed)
                .map(|i| RenderGlyph::original(1, i))
                .chain((revealed..from_len).map(|i| RenderGlyph::original(0, i)))
                .take(length);
            self.render.render([&self.from, &self.to], glyphs, rich)
        } else {
            let rng = self.rng.get_or_insert_with(Xoshiro256PlusPlus::from_os_rng);
            let alphabet = &self.alphabet;
            let glyphs = (0..to_len).map(|index| RenderGlyph {
                document: 1,
                index,
                replacement: (index >= revealed).then(|| alphabet[rng.random_range(0..alphabet.len())]),
            });
            self.render.render([&self.from, &self.to], glyphs, rich)
        }
    }
}

/// Default string property lerper. Plain text uses the existing `()` operations;
/// opt-in BBCode and scrambling share the same property-tween processing path.
/// Parsed strings, glyph/style tables, and rendering buffers are cached and reused.
#[derive(Default)]
pub struct StringLerper {
    options: TextOptions,
    cache: Option<Box<TextCache>>,
    ignore_scramble: bool,
    warned_unsupported: bool,
}

impl StringLerper {
    /// Returns the configured options, including scrambling ignored by the active mode.
    pub fn options(&self) -> &TextOptions { &self.options }

    /// Replaces the options while retaining reusable text/rendering storage.
    /// Unsupported scrambling warns once on the next relative/speed-based update
    /// or forced completion.
    pub fn set_options(&mut self, options: TextOptions) {
        self.options = options;
        if let Some(cache) = &mut self.cache {
            cache.alphabet_ready = false;
        }
        self.warned_unsupported = false;
    }
}

impl BasicLerp<GString> for StringLerper {
    fn spire_lerp(&mut self, from: &GString, to: &GString, weight: f64) -> GString {
        if !self.options.rich_text_enabled && (self.ignore_scramble || self.options.scramble_mode == ScrambleMode::None)
        {
            return ().spire_lerp(from, to, weight);
        }
        let cache = self.cache.get_or_insert_with(Box::default);
        cache.configure_alphabet(&self.options, self.ignore_scramble);
        cache.sample(from, to, weight, self.options.rich_text_enabled)
    }
}

impl SpireLerp<GString> for StringLerper {
    fn prepare_mode(&mut self, mode: &LerpMode<GString>) {
        let ignore_scramble = !matches!(mode, LerpMode::Absolute { .. });
        if self.ignore_scramble != ignore_scramble {
            if let Some(cache) = &mut self.cache {
                cache.alphabet_ready = false;
            }
            self.ignore_scramble = ignore_scramble;
        }
        if ignore_scramble && self.options.scramble_mode != ScrambleMode::None && !self.warned_unsupported {
            let mode_name = if matches!(mode, LerpMode::Relative { .. }) { "relative" } else { "speed-based" };
            godot_warn!(
                "[spire text] scrambling ({:?}) is unsupported in {mode_name} mode; ignoring scrambling.",
                self.options.scramble_mode
            );
            self.warned_unsupported = true;
        }
    }

    fn add_relative(&mut self, current: &GString, previous: &GString, next: &GString) -> GString {
        if !self.options.rich_text_enabled {
            return ().add_relative(current, previous, next);
        }
        let cache = self.cache.get_or_insert_with(Box::default);
        cache.relative_sample.update(previous, true);
        let previous_len = cache.relative_sample.glyphs.len();
        cache.relative_sample.update(next, true);
        let next_len = cache.relative_sample.glyphs.len();
        cache.current.update(current, true);
        let current_len = cache.current.glyphs.len();
        let keep = current_len.saturating_sub(previous_len.saturating_sub(next_len));
        let glyphs = (0..keep)
            .map(|i| RenderGlyph::original(0, i))
            .chain((previous_len..next_len).map(|i| RenderGlyph::original(1, i)));
        cache
            .render
            .render([&cache.current, &cache.relative_sample], glyphs, true)
    }

    fn spire_step(&mut self, from: &GString, to: &GString, speed: f64, step: f64) -> (GString, StepResult) {
        if !self.options.rich_text_enabled {
            return ().spire_step(from, to, speed, step);
        }
        let cache = self.cache.get_or_insert_with(Box::default);
        cache.from.update(from, true);
        cache.to.update(to, true);
        let distance = glyph_distance(&cache.from, &cache.to);
        if distance == 0 {
            return (to.clone(), StepResult::Finished { excess_time: step });
        }
        if speed <= 0.0 || step <= 0.0 {
            return (from.clone(), StepResult::Unfinished { accumulated_time: 0.0 });
        }
        let edits = (speed * step).floor().min(distance as f64) as usize;
        let mut remaining = edits;
        let mut prefix = 0;
        let from_len = cache.from.glyphs.len();
        let to_len = cache.to.glyphs.len();
        let from_chars = cache.from.source.chars();
        let to_chars = cache.to.source.chars();
        for index in 0..to_len {
            if index >= from_len
                || from_chars[cache.from.glyphs[index].text.clone()] != to_chars[cache.to.glyphs[index].text.clone()]
            {
                if remaining == 0 {
                    break;
                }
                remaining -= 1;
            }
            prefix = index + 1;
        }
        let removed = from_len.saturating_sub(to_len).min(remaining);
        let length = from_len.max(prefix) - removed;
        let unused = (step - edits as f64 / speed).max(0.0);
        if edits == distance {
            (to.clone(), StepResult::Finished { excess_time: unused })
        } else {
            let glyphs = (0..prefix)
                .map(|i| RenderGlyph::original(1, i))
                .chain((prefix..length).map(|i| RenderGlyph::original(0, i)));
            (cache.render.render([&cache.from, &cache.to], glyphs, true), StepResult::Unfinished {
                accumulated_time: unused,
            })
        }
    }

    fn spire_distance(&mut self, from: &GString, to: &GString) -> f64 {
        if !self.options.rich_text_enabled {
            return ().spire_distance(from, to);
        }
        let cache = self.cache.get_or_insert_with(Box::default);
        cache.from.update(from, true);
        cache.to.update(to, true);
        glyph_distance(&cache.from, &cache.to) as f64
    }
}

fn glyph_distance(from: &ParsedText, to: &ParsedText) -> usize {
    let from_chars = from.source.chars();
    let to_chars = to.source.chars();
    (0..from.glyphs.len().min(to.glyphs.len()))
        .filter(|&i| from_chars[from.glyphs[i].text.clone()] != to_chars[to.glyphs[i].text.clone()])
        .count()
        + from.glyphs.len().abs_diff(to.glyphs.len())
}

impl SpireTween<LerpPropertyData<GString>> {
    /// Configured string options. Scrambling is honored only in absolute mode.
    pub fn get_text_options(&self) -> &TextOptions { self.t.lerper.options() }

    /// Sets BBCode/scrambling options without changing the target, easing, or mode.
    /// Configure between updates; editing a tween during processing is unsupported.
    pub fn set_text_options(&mut self, options: TextOptions) { self.t.lerper.set_options(options); }

    /// Chainable equivalent of [`set_text_options`](Self::set_text_options).
    pub fn with_text_options(mut self, options: TextOptions) -> Self {
        self.set_text_options(options);
        self
    }
}
