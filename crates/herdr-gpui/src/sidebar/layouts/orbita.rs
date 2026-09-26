//! Orbita's rows: Herdr's rows on the rounded comfortable look, plus tree
//! guides for worktrees, a child highlight that starts where the guide's tick
//! ends, the `[sidebar_worktrees]` font on worktree rows, and a badge line for
//! values plugins report through workspace metadata.

use super::super::{
    ARROW_RESERVE,
    agents::agent_labels,
    cell::{AgentRow, RowContext, RowLayout, RowState, WorkspaceRow},
    layout::{HeaderCase, Highlight, Rounded, SidebarDensity, SidebarStyle, outline_border},
    line_height,
    row::{RowBadge, RowIcon, RowKind, RowTree},
};
use crate::config::Theme;
use gpui::{prelude::*, *};

pub(in super::super) struct Orbita;

impl RowLayout for Orbita {
    fn workspace(&self, row: WorkspaceRow<'_>, state: RowState, cx: &RowContext<'_>) -> Div {
        let density = cx.look.density;
        let lines = if density.workspace_details() { 2. } else { 1. };
        let (branch, status) = (row.branch().unwrap_or(""), row.status());
        let WorkspaceRow {
            workspace,
            label,
            tree,
            icon,
            fold,
            grouped,
            badge,
            removing,
            ..
        } = row;
        let font = if tree == RowTree::None {
            cx.font
        } else {
            cx.worktree_font
        };
        let tokens = &workspace.tokens;
        let badge = if tokens.iter().any(|(name, _)| name == "pr") {
            badge.and_then(RowBadge::without_pr)
        } else {
            badge
        };
        let arrow = fold.map(|fold| {
            fold.element(cx.theme)
                .w(px(ARROW_RESERVE - density.gap()))
                .h(px(line_height(cx.font) * lines))
                .text_size(px(16.))
        });
        super::super::row::row(
            label,
            &[(label, true)],
            branch,
            RowKind::Workspace,
            status,
            removing,
            state,
            tree,
            grouped,
            cx.width,
            icon,
            arrow,
            badge,
            tokens,
            cx.look,
            (font, cx.theme),
        )
    }

    fn agent(&self, agent: AgentRow<'_>, state: RowState, cx: &RowContext<'_>) -> Div {
        let (name, detail) = agent_labels(agent.name, agent.place, cx.host);
        super::super::row::row(
            &agent.key,
            &name,
            detail,
            RowKind::Agent(agent.icon),
            agent.status,
            false,
            state,
            RowTree::None,
            false,
            cx.width,
            RowIcon::None,
            None,
            None,
            &[],
            cx.look,
            (cx.font, cx.theme),
        )
    }
}

pub(in super::super) struct OrbitaRounded;

impl SidebarStyle for OrbitaRounded {
    fn row_inset(&self, density: &dyn SidebarDensity) -> f32 {
        Rounded.row_inset(density)
    }
    fn row_spacing(&self, density: &dyn SidebarDensity) -> f32 {
        Rounded.row_spacing(density)
    }
    fn row_padding(&self, density: &dyn SidebarDensity) -> f32 {
        Rounded.row_padding(density)
    }
    fn radius(&self) -> f32 {
        Rounded.radius()
    }
    fn highlight(&self) -> Highlight {
        Rounded.highlight()
    }
    fn tree_lines(&self) -> bool {
        true
    }
    fn nests_children(&self) -> bool {
        true
    }
    fn tree_color(&self, theme: &Theme) -> Rgba {
        outline_border(theme)
    }
    fn header_case(&self) -> HeaderCase {
        Rounded.header_case()
    }
}
