use super::Theme;
use serde::Deserialize;

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Eq)]
#[serde(default, deny_unknown_fields)]
pub struct ThemeOverrides {
    pub accent: Option<HexColor>,
    pub chrome: Option<HexColor>,
    pub active_tab: ActiveTab,
    pub sidebar_selection: SidebarSelection,
}

impl ThemeOverrides {
    pub(super) fn apply(self, mut theme: Theme) -> Theme {
        if let Some(HexColor(chrome)) = self.chrome {
            theme.surface = chrome;
            theme.chrome.titlebar = Titlebar::Flat;
        }
        theme.accent = self.accent.map(|HexColor(accent)| accent);
        theme.chrome.active_tab = self.active_tab;
        theme.chrome.sidebar_selection = self.sidebar_selection;
        theme
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HexColor(pub u32);

impl<'de> Deserialize<'de> for HexColor {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let text = String::deserialize(deserializer)?;
        let hex = text.strip_prefix('#').unwrap_or(&text);
        hex.bytes()
            .all(|byte| byte.is_ascii_hexdigit())
            .then_some(hex)
            .filter(|hex| hex.len() == 6)
            .and_then(|hex| u32::from_str_radix(hex, 16).ok())
            .map(Self)
            .ok_or_else(|| {
                serde::de::Error::invalid_value(
                    serde::de::Unexpected::Str(&text),
                    &"a #RRGGBB color",
                )
            })
    }
}

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ActiveTab {
    #[default]
    Wash,
    Solid,
}

#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum SidebarSelection {
    #[default]
    Fill,
    Bold,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Titlebar {
    #[default]
    Tinted,
    Flat,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ChromeStyle {
    pub titlebar: Titlebar,
    pub active_tab: ActiveTab,
    pub sidebar_selection: SidebarSelection,
}
