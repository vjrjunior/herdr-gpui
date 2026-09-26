use super::{FontConfig, FontSettings};
use crate::Result;

impl FontSettings {
    pub(super) fn worktree_font(self, sidebar: &FontConfig) -> Result<FontConfig> {
        let mut font = sidebar.clone();
        self.apply("sidebar_worktrees", &mut font)?;
        Ok(font)
    }
}
