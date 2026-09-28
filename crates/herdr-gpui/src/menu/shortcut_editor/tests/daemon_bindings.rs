use super::*;

#[gpui::test]
fn a_keystroke_a_daemon_binding_holds_never_starts_a_save(cx: &mut TestAppContext) {
    let (view, cx) = cx.add_window_view(crate::sidebar::layout_tests::fixture_window);
    view.update_in(cx, |view, window, cx| {
        view.config.keybindings = Keymap::with_daemon_bindings(
            &Default::default(),
            &[("prefix+m".to_owned(), Binding::One("cmd-shift-m".into()))].into(),
            &Default::default(),
            &DaemonKeys::default(),
        )
        .unwrap();
        view.open_keybinds(window, cx);
        view.edit_shortcut(info(Command::Tab), window, cx);
        view.menu
            .shortcut_editor
            .as_ref()
            .unwrap()
            .input
            .update(cx, |input, cx| input.set_text_selected("cmd-shift-m", cx));
        view.save_shortcut_with(
            Edit::Save,
            |_, _| panic!("a keystroke held by a daemon binding reached disk"),
            window,
            cx,
        );
        let editor = view.menu.shortcut_editor.as_ref().unwrap();
        assert_eq!(
            editor.error.as_deref(),
            Some("daemon_keybindings.\"prefix+m\": \"cmd-shift-m\" is already bound to new_tab")
        );
        assert!(!editor.saving);
    });
}
