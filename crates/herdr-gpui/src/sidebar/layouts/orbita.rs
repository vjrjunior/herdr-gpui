//! Orbita's rows: Herdr's rows on the rounded comfortable look, plus tree
//! guides for worktrees, a child highlight that starts where the guide's tick
//! ends, the `[sidebar_worktrees]` font on worktree rows, and a badge line for
//! values plugins report through workspace metadata. Rows the daemon's sidebar
//! config lays out name those values themselves, so they take no badge line.

use super::super::{
    ARROW_RESERVE,
    agents::agent_labels,
    cell::{AgentRow, RowContext, RowLayout, RowState, WorkspaceRow},
    layout::{SidebarLook, SidebarMetrics},
    line_height,
    row::{RowBadge, RowIcon, RowKind, RowLift, RowTree},
};
use crate::config::Theme;
use gpui::{prelude::*, *};

pub(in super::super) struct Orbita;

impl RowLayout for Orbita {
    fn workspace(&self, row: WorkspaceRow<'_>, state: RowState, cx: &RowContext<'_>) -> Div {
        let density = cx.look.density;
        let badge_lines = row.badge.as_ref().map_or(0, |badge| badge.lines(&density));
        let text_lines = if row.lines.is_empty() {
            if density.workspace_details() { 2 } else { 1 }
        } else {
            row.lines.len().max(badge_lines).max(1)
        };
        let (branch, status, upstream) = (row.branch().unwrap_or(""), row.status(), row.upstream());
        let WorkspaceRow {
            workspace,
            label,
            tree,
            icon,
            fold,
            grouped,
            badge,
            removing,
            lines,
            ..
        } = row;
        let font = if tree == RowTree::None {
            cx.font
        } else {
            cx.worktree_font
        };
        let tokens: &[(String, String)] = if lines.is_empty() {
            &workspace.tokens
        } else {
            &[]
        };
        let badge = if tokens.iter().any(|(name, _)| name == "pr") {
            badge.and_then(RowBadge::without_pr)
        } else {
            badge
        };
        let arrow = fold.map(|fold| {
            fold.element(cx.theme)
                .w(px(ARROW_RESERVE - density.gap()))
                .h(px(line_height(cx.font) * text_lines as f32))
                .text_size(px(16.))
        });
        super::super::row::row(
            label,
            &[(label, true)],
            branch,
            RowKind::Workspace,
            status,
            cx.indicators,
            removing,
            state,
            tree,
            grouped,
            icon,
            arrow,
            badge,
            upstream,
            None,
            &lines,
            tokens,
            &RowContext {
                indicators: cx.indicators,
                font,
                worktree_font: cx.worktree_font,
                theme: cx.theme,
                look: cx.look,
                width: cx.width,
                nest: cx.nest,
                mark: cx.mark,
                host: cx.host,
            },
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
            cx.indicators,
            false,
            state,
            RowTree::None,
            false,
            RowIcon::None,
            None,
            None,
            None,
            agent.status_text.as_deref(),
            &agent.lines,
            &[],
            cx,
        )
    }
}

const GUIDE_ALPHA: u32 = 0x40;

impl SidebarMetrics {
    pub(in super::super) fn orbita(self) -> Self {
        Self {
            child_indent: self.child_indent + self.gap,
            tree_lines: true,
            orbita: true,
            ..self
        }
    }
}

impl SidebarLook {
    pub(in super::super) fn tree_color(&self, theme: &Theme) -> Rgba {
        if self.density.orbita {
            rgba((theme.foreground << 8) | GUIDE_ALPHA)
        } else {
            rgb(theme.muted)
        }
    }

    pub(in super::super) fn tree_padding(&self) -> f32 {
        if self.density.orbita {
            0.
        } else {
            self.density.padding()
        }
    }

    pub(in super::super) fn mark_nested(
        &self,
        row: Div,
        key: &str,
        state: RowState,
        indent: f32,
        theme: &Theme,
    ) -> Div {
        if !self.density.orbita || state.lift == RowLift::Lifted {
            return self.mark(row, key, state, theme);
        }
        let layer = self
            .highlight(key, state, theme)
            .left(px(self.inset() + indent));
        match state.lift {
            RowLift::Resting => self.hover_group(row).child(layer),
            _ => row.child(layer),
        }
    }
}
