use super::{Binding, DaemonKeys, Keymap, PaneKeys};
use gpui::Keystroke;
use std::collections::BTreeMap;

mod error;

pub(crate) use error::Error;

pub(super) fn parse(binding: &str, keystroke: &str) -> Result<Keystroke, Error> {
    let parsed = Keystroke::parse(keystroke).map_err(|source| Error::InvalidKeystroke {
        binding: binding.to_owned(),
        keystroke: keystroke.to_owned(),
        source,
    })?;
    if !super::has_modifier(&parsed) {
        return Err(Error::KeystrokeWithoutModifier {
            binding: binding.to_owned(),
            keystroke: keystroke.to_owned(),
        });
    }
    Ok(parsed)
}

impl Keymap {
    pub(crate) fn with_edited_overrides(
        &self,
        overrides: &BTreeMap<String, Binding>,
        pane_keys: &PaneKeys,
        keys: &DaemonKeys,
    ) -> crate::Result<Self> {
        let bindings = self
            .daemon
            .iter()
            .map(|(label, keystrokes)| (label.clone(), Binding::Many(keystrokes.clone())))
            .collect();
        Self::with_daemon_bindings(overrides, &bindings, pane_keys, keys)
    }
}
