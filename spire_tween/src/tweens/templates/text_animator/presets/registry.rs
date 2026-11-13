//! Tracks authored resources and owns each label's generated tag adapters.

use std::{cell::Cell, rc::Rc};

use godot::{
    classes::{GDScript, Resource, RichTextEffect, Script},
    signal::ConnectHandle,
};

use super::{reserved_tags::RESERVED_TAGS, *};

struct PresetSnapshot {
    preset:  Option<Gd<SpireRichTextEffectPreset>>,
    bbcode:  GString,
    entries: Vec<Variant>,
}

impl PresetSnapshot {
    fn matches(&self, preset: &Option<Gd<SpireRichTextEffectPreset>>) -> bool {
        if &self.preset != preset {
            return false;
        }

        let Some(preset) = preset else { return true };

        let preset = preset.bind();
        preset.bbcode == self.bbcode
            && preset.effects.len() == self.entries.len()
            && preset.effects.iter_shared().zip(&self.entries).all(|(a, b)| &a == b)
    }
}

#[derive(Default)]
pub(in super::super) struct Registry {
    dirty: Rc<Cell<bool>>,
    snapshots: Vec<PresetSnapshot>,
    adapters: Vec<Gd<SpireRichTextEffect>>,
    authored_effects: Vec<Variant>,
    connections: Vec<(Gd<Resource>, ConnectHandle)>,
    pub(in super::super) warnings: Vec<String>,
}

impl Drop for Registry {
    fn drop(&mut self) { self.clear_connections(); }
}

fn tag_valid(tag: &str) -> bool {
    let mut chars = tag.chars();
    chars.next().is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
        && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
}

fn reserved(tag: &str) -> bool { tag.starts_with("img") || tag.starts_with("dropcap") || RESERVED_TAGS.contains(&tag) }

fn is_authored_effect(value: &Variant) -> bool { value.try_to::<Gd<SpireRichTextEffect>>().is_err() }

impl Registry {
    fn clear_connections(&mut self) {
        for (_, handle) in self.connections.drain(..) {
            if handle.is_connected() {
                handle.disconnect();
            }
        }
    }

    fn watch(&mut self, resource: Gd<Resource>) {
        if self.connections.iter().any(|(r, _)| r == &resource) {
            return;
        }
        let dirty = self.dirty.clone();
        let handle = resource.signals().changed().connect(move || {
            dirty.set(true);
        });
        self.connections.push((resource, handle));
    }

    pub(in super::super) fn sync(
        &mut self,
        effects: &VarArray,
        presets: &Array<Option<Gd<SpireRichTextEffectPreset>>>,
    ) -> bool {
        if !self.dirty.get()
            && effects
                .iter_shared()
                .filter(is_authored_effect)
                .eq(self.authored_effects.iter().cloned())
            && presets.len() == self.snapshots.len()
            && presets
                .iter_shared()
                .zip(&self.snapshots)
                .all(|(v, snapshot)| snapshot.matches(&v))
        {
            let authored = self.authored_effects.iter().cloned();
            let installed_matches = if Engine::singleton().is_editor_hint() {
                effects.iter_shared().eq(authored)
            } else {
                effects
                    .iter_shared()
                    .eq(authored.chain(self.adapters.iter().map(|adapter| adapter.to_variant())))
            };
            return !installed_matches;
        }

        // Generated adapters may already be installed, including ones copied from
        // another label. Only authored effects participate in tag precedence.
        self.authored_effects = effects.iter_shared().filter(is_authored_effect).collect();
        self.clear_connections();
        self.adapters.clear();
        self.snapshots.clear();
        self.warnings.clear();

        let mut tags = std::collections::HashSet::new();

        for value in self.authored_effects.clone() {
            let Ok(effect) = value.try_to::<Gd<RichTextEffect>>() else { continue };

            self.watch(effect.clone().upcast());
            if let Some(script) = effect.get_script() {
                let code = effect.get("bbcode");
                let tag = match code.get_type() {
                    VariantType::STRING => Some(code.to::<GString>().to_string()),
                    VariantType::NIL => {
                        let declared = script.get_script_property_list().iter_shared().any(|p| {
                            p.get("name")
                                .is_some_and(|name| name.to::<GString>() == GString::from("bbcode"))
                        });
                        (!declared).then(|| script.get_path().get_file().get_basename().to_string())
                    }
                    _ => None,
                };
                if let Some(tag) = tag {
                    tags.insert(tag);
                }
            }
        }
        for preset in presets.iter_shared() {
            let Some(preset) = preset else {
                self.snapshots.push(PresetSnapshot {
                    preset:  None,
                    bbcode:  GString::new(),
                    entries: Vec::new(),
                });
                continue;
            };
            let (bbcode, entries) = {
                let p = preset.bind();
                (p.bbcode.clone(), p.effects.iter_shared().collect::<Vec<_>>())
            };
            self.watch(preset.clone().upcast());
            let mut compiled = Vec::new();
            for entry in &entries {
                if let Ok(resource) = entry.try_to::<Gd<Resource>>() {
                    self.watch(resource.clone());
                    match CompiledTextEffect::compile(&resource, "effects") {
                        Ok(effect) => compiled.push(sample::Entry::new(effect)),
                        Err(warning) => self.warnings.push(warning.to_string()),
                    }
                } else if !entry.is_nil() {
                    self.warnings
                        .push("`effects` must contain supported effect resources; entry skipped.".into());
                }
            }
            self.snapshots.push(PresetSnapshot {
                preset: Some(preset),
                bbcode: bbcode.clone(),
                entries,
            });
            let tag = bbcode.to_string();
            if !tag_valid(&tag) || reserved(&tag) {
                self.warnings.push(format!(
                    "`bbcode` must be an identifier distinct from built-in tags; got `{tag}`. Preset skipped."
                ));
                continue;
            }
            if !tags.insert(tag.clone()) {
                self.warnings
                    .push(format!("`bbcode` `{tag}` is already registered; the first registration is used."));
                continue;
            }
            let mut script = GDScript::new_gd();
            script.set_source_code(include_str!("adapter.gd"));
            if script.reload() != godot::global::Error::OK {
                self.warnings
                    .push("`effect_presets` could not load the bundled tag adapter.".into());
                continue;
            }
            let mut adapter = SpireRichTextEffect::new_gd();
            adapter.bind_mut().entries = compiled;
            adapter.set_script(&script.upcast::<Script>());
            adapter.set("bbcode", &bbcode.to_variant());
            self.adapters.push(adapter);
        }
        self.dirty.set(false);
        if !Engine::singleton().is_editor_hint() {
            for warning in &self.warnings {
                godot_warn!("[spire text preset] {warning}");
            }
        }
        true
    }

    pub(in super::super) fn begin_parse(&mut self) -> (VarArray, VarArray) {
        for adapter in &mut self.adapters {
            for entry in adapter.bind_mut().entries.iter_mut() {
                entry.reset_origins();
            }
        }

        let authored: VarArray = self.authored_effects.iter().cloned().collect();
        let mut effects = authored.duplicate_shallow();

        for adapter in &self.adapters {
            effects.push(&adapter.to_variant());
        }

        (authored, effects)
    }

    #[cfg(feature = "integration-tests")]
    pub(in super::super) fn adapter_for_test(&self, index: usize) -> Option<Gd<SpireRichTextEffect>> {
        self.adapters.get(index).cloned()
    }
}
