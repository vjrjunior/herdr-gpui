use super::*;

fn daemon(entries: &[(&str, Binding)]) -> Result<Keymap> {
    Keymap::with_daemon_bindings(
        &BTreeMap::new(),
        &overrides(entries),
        &PaneKeys::new(),
        &DaemonKeys::default(),
    )
}

#[test]
fn daemon_bindings_parse_and_reject_conflicts() {
    let keymap = daemon(&[(
        "prefix+m",
        Binding::Many(vec!["cmd-shift-m".into(), "ctrl-alt-m".into()]),
    )])
    .unwrap();
    assert_eq!(
        keymap.daemon_bindings().collect::<Vec<_>>(),
        [("prefix+m", "cmd-shift-m"), ("prefix+m", "ctrl-alt-m")]
    );
    assert!(Keymap::default().daemon_bindings().next().is_none());
    assert!(matches!(
        daemon(&[("prefix+m", one("m"))]),
        Err(Error::DaemonKeystrokeWithoutModifier { .. })
    ));
    assert!(matches!(
        daemon(&[("prefix+m", one("cmd-t"))]),
        Err(Error::DaemonKeystrokeConflict { .. })
    ));
    assert!(matches!(
        daemon(&[("prefix+m", one("cmd-k")), ("prefix+g", one("cmd-k"))]),
        Err(Error::DaemonKeystrokeConflict { .. })
    ));
    assert!(matches!(
        daemon(&[(" ", one("cmd-shift-m"))]),
        Err(Error::EmptyDaemonBinding)
    ));
    assert!(matches!(
        daemon(&[("prefix+m", Binding::Many(vec!["cmd-shift-m".into(); 9]))]),
        Err(Error::TooManyDaemonKeystrokes(_))
    ));
    assert!(matches!(
        daemon(&[("prefix+m", one("cmd-n-t"))]),
        Err(Error::InvalidDaemonKeystroke { .. })
    ));
}

#[test]
fn daemon_keys_and_their_prefix_yield_to_daemon_bindings() {
    let keys = DaemonKeys {
        bindings: vec![
            (Command::Tab, Trigger::Direct(keystroke("alt-m"))),
            (Command::SplitRight, Trigger::Prefixed(keystroke("v"))),
        ],
        ..no_keys()
    };
    let keymap = Keymap::with_daemon_bindings(
        &BTreeMap::new(),
        &overrides(&[(
            "prefix+m",
            Binding::Many(vec!["alt-m".into(), "ctrl-b".into()]),
        )]),
        &PaneKeys::new(),
        &keys,
    )
    .unwrap();
    assert_eq!(
        keymap.daemon_bindings().collect::<Vec<_>>(),
        [("prefix+m", "alt-m"), ("prefix+m", "ctrl-b")]
    );
    assert_eq!(list(&keymap, Command::Tab), ["cmd-t"]);
    assert_eq!(list(&keymap, Command::SplitRight), ["cmd-d"]);
    assert!(!keymap.is_prefix(&keystroke("ctrl-b")));
    assert_eq!(keymap.chord(&keystroke("v")), None);
}

#[test]
fn a_pane_key_keeps_its_keystroke_from_a_daemon_binding() {
    let result = Keymap::with_daemon_bindings(
        &BTreeMap::new(),
        &overrides(&[("prefix+m", one("cmd-j"))]),
        &pane_keys(&[("cmd-j", "ctrl-a")]),
        &no_keys(),
    );
    assert!(matches!(
        result,
        Err(Error::DaemonKeystrokeConflict { owner, .. }) if owner == "pane_keys"
    ));
}
