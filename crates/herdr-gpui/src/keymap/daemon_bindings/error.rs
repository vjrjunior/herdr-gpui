#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("daemon_keybindings: a daemon binding label must not be empty")]
    EmptyLabel,
    #[error("daemon_keybindings.{binding:?}: invalid keystroke {keystroke:?}")]
    InvalidKeystroke {
        binding: String,
        keystroke: String,
        #[source]
        source: gpui::InvalidKeystrokeError,
    },
    #[error("daemon_keybindings.{binding:?}: {keystroke:?} is already bound to {owner}")]
    KeystrokeConflict {
        keystroke: String,
        binding: String,
        owner: String,
    },
    #[error(
        "daemon_keybindings.{binding:?}: {keystroke:?} needs a cmd, ctrl, alt, or fn modifier so typing still reaches the terminal"
    )]
    KeystrokeWithoutModifier { binding: String, keystroke: String },
    #[error("daemon_keybindings.{0:?} must list at most 8 keystrokes")]
    TooManyKeystrokes(String),
    #[error("No Herdr command is bound to {0}.")]
    Unbound(String),
}
