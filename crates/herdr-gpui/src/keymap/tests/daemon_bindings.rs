use super::*;

fn refusal(result: Result<Keymap>) -> Option<DaemonBindingError> {
    match result {
        Err(Error::ConfigFile { source, .. }) => source.downcast().ok().map(|error| *error),
        _ => None,
    }
}

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
        refusal(daemon(&[("prefix+m", one("m"))])),
        Some(DaemonBindingError::KeystrokeWithoutModifier { .. })
    ));
    assert!(matches!(
        refusal(daemon(&[("prefix+m", one(default("cmd-t")))])),
        Some(DaemonBindingError::KeystrokeConflict { .. })
    ));
    assert!(matches!(
        refusal(daemon(&[
            ("prefix+m", one("cmd-k")),
            ("prefix+g", one("cmd-k"))
        ])),
        Some(DaemonBindingError::KeystrokeConflict { .. })
    ));
    assert!(matches!(
        refusal(daemon(&[(" ", one("cmd-shift-m"))])),
        Some(DaemonBindingError::EmptyLabel)
    ));
    assert!(matches!(
        refusal(daemon(&[(
            "prefix+m",
            Binding::Many(vec!["cmd-shift-m".into(); 9])
        )])),
        Some(DaemonBindingError::TooManyKeystrokes(_))
    ));
    assert!(matches!(
        refusal(daemon(&[("prefix+m", one("cmd-n-t"))])),
        Some(DaemonBindingError::InvalidKeystroke { .. })
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
    assert_eq!(list(&keymap, Command::Tab), [default("cmd-t")]);
    assert_eq!(list(&keymap, Command::SplitRight), [default("cmd-d")]);
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
    assert!(
        matches!(refusal(result), Some(DaemonBindingError::KeystrokeConflict { owner, .. }) if owner == "pane_keys")
    );
}

#[test]
fn a_refused_daemon_binding_keeps_its_message_and_cause() {
    let Err(error) = daemon(&[("prefix+m", one("cmd-n-t"))]) else {
        panic!("an invalid keystroke must be refused");
    };
    let refusal = std::error::Error::source(&error)
        .and_then(|source| source.downcast_ref::<DaemonBindingError>());
    assert!(matches!(
        refusal,
        Some(DaemonBindingError::InvalidKeystroke { .. })
    ));
    assert_eq!(Some(error.to_string()), refusal.map(ToString::to_string));
    assert!(refusal.and_then(std::error::Error::source).is_some());
}
