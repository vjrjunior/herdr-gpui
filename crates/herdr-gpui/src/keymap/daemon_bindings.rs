use super::{Binding, DaemonKeys, Keymap, PaneKeys};
use gpui::Keystroke;
use std::collections::BTreeMap;

#[derive(Debug, thiserror::Error)]
pub enum DaemonBindingError {
    #[error("daemon_keybindings: a daemon binding label must not be empty")]
    EmptyLabel,
    #[error("daemon_keybindings.{binding:?}: invalid keystroke {keystroke:?}")]
    InvalidKeystroke {
        binding: String,
        keystroke: String,
        #[source]
        source: gpui::InvalidKeystrokeError,
    },
    #[error(
        "daemon_keybindings.{binding:?}: {keystroke:?} needs a cmd, ctrl, alt, or fn modifier so typing still reaches the terminal"
    )]
    KeystrokeWithoutModifier { binding: String, keystroke: String },
    #[error("daemon_keybindings.{0:?} must list at most 8 keystrokes")]
    TooManyKeystrokes(String),
    #[error("daemon_keybindings.{binding:?}: {keystroke:?} is already bound to {owner}")]
    KeystrokeConflict {
        keystroke: String,
        binding: String,
        owner: String,
    },
    #[error("No Herdr command is bound to {0}.")]
    Unbound(String),
}

impl From<DaemonBindingError> for crate::Error {
    fn from(source: DaemonBindingError) -> Self {
        Self::ConfigFile {
            uri: None,
            source: Box::new(source),
        }
    }
}

pub(super) fn parse(binding: &str, keystroke: &str) -> Result<Keystroke, DaemonBindingError> {
    let parsed =
        Keystroke::parse(keystroke).map_err(|source| DaemonBindingError::InvalidKeystroke {
            binding: binding.to_owned(),
            keystroke: keystroke.to_owned(),
            source,
        })?;
    if !super::has_modifier(&parsed) {
        return Err(DaemonBindingError::KeystrokeWithoutModifier {
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
