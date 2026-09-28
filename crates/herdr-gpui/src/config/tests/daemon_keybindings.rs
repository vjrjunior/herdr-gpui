use super::*;
use crate::keymap::daemon_bindings;

fn refusal(result: Result<Config>) -> Option<daemon_bindings::Error> {
    match result {
        Err(crate::Error::DaemonBindings(error)) => Some(error),
        _ => None,
    }
}

#[test]
fn daemon_keybindings_bind_gui_keys_to_daemon_bindings() -> anyhow::Result<()> {
    let config = Config::parse("[daemon_keybindings]\n\"prefix+m\" = \"cmd-shift-m\"")?;
    assert_eq!(
        config.keybindings.daemon_bindings().collect::<Vec<_>>(),
        [("prefix+m", "cmd-shift-m")]
    );
    assert!(matches!(
        refusal(Config::parse(&format!(
            "[daemon_keybindings]\n\"prefix+m\" = \"{}\"",
            crate::controls::platform_default("cmd-t")
        ))),
        Some(daemon_bindings::Error::KeystrokeConflict { .. })
    ));
    assert!(matches!(
        refusal(Config::parse(
            "[keybindings]\nnew_tab = \"cmd-shift-m\"\n[daemon_keybindings]\n\"prefix+m\" = \"cmd-shift-m\""
        )),
        Some(daemon_bindings::Error::KeystrokeConflict { .. })
    ));
    Ok(())
}
