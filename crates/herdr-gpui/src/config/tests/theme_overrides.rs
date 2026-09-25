use super::*;
use crate::config::theme_overrides::HexColor;

#[test]
fn theme_overrides_default_to_the_derived_chrome() -> anyhow::Result<()> {
    let config = Config::parse("")?;
    assert_eq!(config.theme_overrides, ThemeOverrides::default());
    let theme = config.theme_with_directories(false, || Err(Error::MissingHome))?;
    assert_eq!(theme, Theme::default());
    assert_eq!(theme.primary(), theme.palette[5]);
    assert_eq!(theme.active_tab_fill(), theme.primary_wash());
    assert!(theme.fills_selected_row());
    assert_eq!(theme.chrome.titlebar, Titlebar::Tinted);
    Ok(())
}

#[test]
fn theme_overrides_pin_accent_chrome_tab_and_selection() -> anyhow::Result<()> {
    let config = Config::parse(
        "theme = \"Nord\"\n[theme_overrides]\naccent = \"#FFC799\"\nchrome = \"101010\"\nactive_tab = \"solid\"\nsidebar_selection = \"bold\"\n",
    )?;
    assert_eq!(
        config.theme_overrides,
        ThemeOverrides {
            accent: Some(HexColor(0xffc799)),
            chrome: Some(HexColor(0x101010)),
            active_tab: ActiveTab::Solid,
            sidebar_selection: SidebarSelection::Bold,
        }
    );
    let theme = config.theme_with_directories(false, || Err(Error::MissingHome))?;
    let nord = Theme::builtin("Nord").context("missing builtin")?;
    assert_eq!(theme.primary(), 0xffc799);
    assert_eq!(theme.surface, 0x101010);
    assert_eq!(theme.active_tab_fill(), 0xffc799);
    assert!(!theme.fills_selected_row());
    assert_eq!(theme.chrome.titlebar, Titlebar::Flat);
    assert_eq!(
        (theme.background, theme.foreground, theme.palette),
        (nord.background, nord.foreground, nord.palette)
    );
    Ok(())
}

#[test]
fn theme_overrides_apply_to_ghostty_themes_and_merge_by_key() -> anyhow::Result<()> {
    let temp = TempDirectory::new()?;
    fs::write(
        temp.0.join("VJR"),
        "background = #101010\npalette = 5=#FFCFA8\n",
    )?;
    let config = Config::parse_layers(
        [
            "theme = \"VJR\"\n[theme_overrides]\naccent = \"#ffc799\"\n",
            "[theme_overrides]\nactive_tab = \"solid\"\n",
        ],
        &Daemon::default(),
    )?;
    let theme = config.theme_with_directories(false, || Ok(vec![temp.0.clone()]))?;
    assert_eq!(theme.palette[5], 0xffcfa8);
    assert_eq!(theme.primary(), 0xffc799);
    assert_eq!(theme.active_tab_fill(), 0xffc799);
    assert!(theme.fills_selected_row());
    assert_eq!(theme.chrome.titlebar, Titlebar::Tinted);
    Ok(())
}

#[test]
fn theme_overrides_apply_to_each_side_of_a_system_pair() -> anyhow::Result<()> {
    let config = Config::parse(
        "theme = \"light:Catppuccin Latte,dark:Nord\"\n[theme_overrides]\naccent = \"#ffc799\"\n",
    )?;
    for light in [true, false] {
        let theme = config.theme_with_directories(light, || Err(Error::MissingHome))?;
        assert_eq!(theme.primary(), 0xffc799, "light={light}");
    }
    Ok(())
}

#[test]
fn theme_overrides_reject_bad_colors_values_and_keys() {
    for text in [
        "[theme_overrides]\naccent = \"#12345\"",
        "[theme_overrides]\naccent = \"#12345G\"",
        "[theme_overrides]\nchrome = \"red\"",
        "[theme_overrides]\nchrome = 1052688",
        "[theme_overrides]\nactive_tab = \"bright\"",
        "[theme_overrides]\nsidebar_selection = \"underline\"",
        "[theme_overrides]\ntitlebar = \"flat\"",
    ] {
        assert!(Config::parse(text).is_err(), "accepted {text}");
    }
}
