use super::*;

#[test]
fn sidebar_worktrees_inherit_the_sidebar_font_and_override_by_key() -> anyhow::Result<()> {
    let config =
        Config::parse("[sidebar]\nfamily = \"JetBrains Mono\"\nsize = 14\nfallback = []\n")?;
    assert_eq!(config.sidebar_worktrees, config.sidebar);
    let config = Config::parse(
        "[sidebar]\nfamily = \"JetBrains Mono\"\nsize = 14\n[sidebar_worktrees]\nsize = 12\n",
    )?;
    assert_eq!(config.sidebar_worktrees.family, "JetBrains Mono");
    assert_eq!(config.sidebar_worktrees.size, 12.);
    assert_eq!(config.sidebar.size, 14.);
    for text in [
        "[sidebar_worktrees]\nsize = 4",
        "[sidebar_worktrees]\nfamily = \" \"",
        "[sidebar_worktrees]\nfallback = [\"\"]",
    ] {
        assert!(Config::parse(text).is_err(), "accepted {text}");
    }
    assert!(matches!(
        Config::parse("[sidebar_worktrees]\nsize = 4"),
        Err(crate::Error::Settings(Error::InvalidFontSize(
            "sidebar_worktrees"
        )))
    ));
    Ok(())
}

#[test]
fn sidebar_worktrees_take_no_terminal_only_keys() -> anyhow::Result<()> {
    let config = Config::parse("[sidebar_worktrees]\nweight = 'bold'\nline_height = 2.0\n")?;
    assert_eq!(
        config.unknown_keys,
        ["sidebar_worktrees.line_height", "sidebar_worktrees.weight"]
    );
    assert_eq!(config.sidebar_worktrees.weight, gpui::FontWeight::NORMAL);
    assert_eq!(config.sidebar_worktrees.line_height_multiple, None);
    Ok(())
}
