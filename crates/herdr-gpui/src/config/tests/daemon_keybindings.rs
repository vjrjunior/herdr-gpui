use super::*;

#[test]
fn daemon_keybindings_bind_gui_keys_to_daemon_bindings() -> anyhow::Result<()> {
    let config = Config::parse("[daemon_keybindings]\n\"prefix+m\" = \"cmd-shift-m\"")?;
    assert_eq!(
        config.keybindings.daemon_bindings().collect::<Vec<_>>(),
        [("prefix+m", "cmd-shift-m")]
    );
    assert!(matches!(
        Config::parse("[daemon_keybindings]\n\"prefix+m\" = \"cmd-t\""),
        Err(Error::DaemonKeystrokeConflict { .. })
    ));
    assert!(matches!(
        Config::parse(
            "[keybindings]\nnew_tab = \"cmd-shift-m\"\n[daemon_keybindings]\n\"prefix+m\" = \"cmd-shift-m\""
        ),
        Err(Error::DaemonKeystrokeConflict { .. })
    ));
    Ok(())
}
