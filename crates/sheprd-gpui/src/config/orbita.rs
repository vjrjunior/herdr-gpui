use super::{FontConfig, FontSettings, Settings};
use crate::{
    Result,
    keymap::{DaemonKeys, Keymap},
};

impl FontSettings {
    pub(super) fn worktree_font(self, sidebar: &FontConfig) -> Result<FontConfig> {
        let mut font = sidebar.clone();
        self.apply("sidebar_worktrees", &mut font)?;
        Ok(font)
    }
}

impl Settings {
    pub(super) fn keymap(&self, keys: &DaemonKeys) -> Result<Keymap> {
        Keymap::with_daemon_bindings(
            &self.keybindings,
            &self.daemon_keybindings,
            &self.pane_keys,
            keys,
        )
    }
}
