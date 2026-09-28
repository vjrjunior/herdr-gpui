use super::*;
use crate::keymap::daemon_bindings::Error;

#[test]
fn every_daemon_binding_variant_has_a_message() {
    let binding = || "prefix+m".to_owned();
    assert_localized(&[
        Error::EmptyLabel,
        Error::InvalidKeystroke {
            binding: binding(),
            keystroke: "cmd-n-t".into(),
            source: gpui::Keystroke::parse("cmd-n-t").unwrap_err(),
        },
        Error::KeystrokeConflict {
            keystroke: "cmd-t".into(),
            binding: binding(),
            owner: "new_tab".into(),
        },
        Error::KeystrokeWithoutModifier {
            binding: binding(),
            keystroke: "n".into(),
        },
        Error::TooManyKeystrokes(binding()),
        Error::Unbound(binding()),
    ]);
}
