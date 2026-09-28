use super::Localize;
use crate::{i18n::fl, keymap::daemon_bindings::Error};

fn quoted(text: &str) -> String {
    format!("{text:?}")
}

impl Localize for Error {
    fn localize(&self) -> String {
        match self {
            Self::EmptyLabel => fl!("orbita-error-daemon-bindings-empty-label"),
            Self::InvalidKeystroke {
                binding, keystroke, ..
            } => fl!(
                "orbita-error-daemon-bindings-invalid-keystroke",
                binding = quoted(binding),
                keystroke = quoted(keystroke)
            ),
            Self::KeystrokeConflict {
                keystroke,
                binding,
                owner,
            } => fl!(
                "orbita-error-daemon-bindings-keystroke-conflict",
                binding = quoted(binding),
                keystroke = quoted(keystroke),
                owner = owner.as_str()
            ),
            Self::KeystrokeWithoutModifier { binding, keystroke } => fl!(
                "orbita-error-daemon-bindings-keystroke-without-modifier",
                binding = quoted(binding),
                keystroke = quoted(keystroke)
            ),
            Self::TooManyKeystrokes(binding) => fl!(
                "orbita-error-daemon-bindings-too-many-keystrokes",
                binding = quoted(binding)
            ),
            Self::Unbound(binding) => fl!(
                "orbita-error-daemon-bindings-unbound",
                binding = binding.as_str()
            ),
        }
    }
}
