use super::*;
use crate::keymap::DaemonBindingError;

fn refusal(result: Result<Config>) -> Option<DaemonBindingError> {
    match result {
        Err(Error::ConfigFile { source, .. }) => source.downcast().ok().map(|error| *error),
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
        Some(DaemonBindingError::KeystrokeConflict { .. })
    ));
    assert!(matches!(
        refusal(Config::parse(
            "[keybindings]\nnew_tab = \"cmd-shift-m\"\n[daemon_keybindings]\n\"prefix+m\" = \"cmd-shift-m\""
        )),
        Some(DaemonBindingError::KeystrokeConflict { .. })
    ));
    Ok(())
}
