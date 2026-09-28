#![allow(clippy::unwrap_used)]
use gpui::{KeyBinding, Keystroke, TestAppContext};
use herdr_client::protocol::{ClientShellCommand, ClientShellCommandAction};

#[gpui::test]
fn a_gui_shortcut_invokes_the_daemon_command_bound_to_its_label(cx: &mut TestAppContext) {
    let (view, cx) = cx.add_window_view(crate::sidebar::layout_tests::fixture_window);
    cx.update(|window, cx| {
        view.update(cx, |view, _| {
            view.reconnect();
            let client = herdr_client::connect(
                herdr_client::ConnectTarget::Socket("/unused-daemon-binding-test.sock".into()),
                view.options,
            )
            .unwrap();
            client.handle.disconnect();
            view.endpoints[0].connection.handle = Some(client.handle);
            let mut snapshot = crate::sidebar::layout_tests::snapshot(2);
            snapshot.commands = vec![ClientShellCommand {
                command_id: "cmd_2253-4_0".into(),
                action: ClientShellCommandAction::Shell,
                description: None,
                binding_label: "prefix+m".into(),
                binding_labels: vec!["prefix+m".into()],
            }];
            view.live.snapshot = Some(std::sync::Arc::new(snapshot));
            view.last_queued_options = Some(view.options);
            view.activation_deadline = None;
        });
        cx.bind_keys([KeyBinding::new(
            "cmd-shift-m",
            crate::actions::RunDaemonCommand {
                binding: "prefix+m".into(),
            },
            None,
        )]);
        window.focus(&view.read(cx).focus.clone(), cx);
        window.draw(cx).clear(cx);
        window.dispatch_keystroke(Keystroke::parse("cmd-shift-m").unwrap(), cx);
        let error = view.read(cx).local_error.clone();
        assert!(
            error
                .as_deref()
                .is_some_and(|error| error.starts_with("command.invoke")),
            "{error:?}"
        );
        view.update(cx, |view, cx| {
            view.local_error = None;
            view.run_daemon_binding("prefix+x", window, cx);
            assert_eq!(
                view.local_error.as_deref(),
                Some("No Herdr command is bound to prefix+x.")
            );
        });
    });
}
