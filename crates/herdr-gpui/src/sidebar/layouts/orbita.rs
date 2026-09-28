//! Orbita's rows: Herdr's rows on the rounded comfortable look, plus tree
//! guides for worktrees, a child highlight that starts where the guide's tick
//! ends, the `[sidebar_worktrees]` font on worktree rows, and a badge line for
//! values plugins report through workspace metadata. Rows the daemon's sidebar
//! config lays out name those values themselves, so they take no badge line.

use super::super::{
    ARROW_RESERVE,
    agents::agent_labels,
    cell::{AgentRow, Fold, RowContext, RowLayout, RowState, WorkspaceRow},
    layout::{SidebarLook, SidebarMetrics},
    line_height,
    row::{RowBadge, RowIcon, RowKind, RowLift, RowTree},
};
use crate::{Command, HerdrWindow, config::Theme};
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
            chevron(fold, cx.theme)
                .w(px(ARROW_RESERVE - density.gap()))
                .h(px(line_height(cx.font) * text_lines as f32))
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

const CHEVRON_GROUP: &str = "orbita-fold";
const CHEVRON_SIZE: f32 = 12.;
const HEADER_BUTTON: f32 = 20.;
const HEADER_ICON: f32 = 14.;

fn chevron(fold: Fold, theme: &Theme) -> Stateful<Div> {
    let Fold {
        id,
        index,
        collapsed,
        toggle,
    } = fold;
    let foreground = theme.foreground;
    div()
        .id(id)
        .debug_selector(move || format!("collapse-{index}"))
        .group(CHEVRON_GROUP)
        .flex_none()
        .flex()
        .items_center()
        .justify_center()
        .cursor_pointer()
        .child(
            svg()
                .debug_selector(move || format!("chevron-{index}"))
                .path(if collapsed {
                    "icons/chevron-right.svg"
                } else {
                    "icons/chevron-down.svg"
                })
                .size(px(CHEVRON_SIZE))
                .flex_none()
                .text_color(rgb(theme.muted))
                .group_hover(CHEVRON_GROUP, move |style| {
                    style.text_color(rgb(foreground))
                }),
        )
        .on_click(move |event, window, cx| {
            cx.stop_propagation();
            toggle(event, window, cx);
        })
}

pub(in super::super) fn spaces_actions(theme: &Theme, cx: &mut Context<HerdrWindow>) -> Div {
    let (muted, foreground) = (theme.muted, theme.foreground);
    let hover = rgba((foreground << 8) | 0x14);
    let button = |id: &'static str, icon: &'static str| {
        div()
            .id(id)
            .debug_selector(move || id.into())
            .group(id)
            .size(px(HEADER_BUTTON))
            .flex_none()
            .flex()
            .items_center()
            .justify_center()
            .rounded(px(crate::config::corners::CONTROL))
            .cursor_pointer()
            .hover(move |style| style.bg(hover))
            .child(
                svg()
                    .path(icon)
                    .size(px(HEADER_ICON))
                    .flex_none()
                    .text_color(rgb(muted))
                    .group_hover(id, move |style| style.text_color(rgb(foreground))),
            )
    };
    let menu_bounds = std::rc::Rc::new(std::cell::Cell::new(Bounds::<Pixels>::default()));
    let painted_menu = menu_bounds.clone();
    div()
        .flex()
        .flex_none()
        .items_center()
        .gap(px(2.))
        .child(button("spaces-new", "icons/plus.svg").on_click(
            cx.listener(|this, _, window, cx| this.command(Command::Workspace, window, cx)),
        ))
        .child(
            button("spaces-menu", "icons/menu.svg")
                .child(
                    canvas(
                        |_, _, _| (),
                        move |bounds, _, _, _| painted_menu.set(bounds),
                    )
                    .absolute()
                    .inset_0()
                    .size_full(),
                )
                .on_click(cx.listener(move |this, _: &ClickEvent, window, cx| {
                    if this.open_menu(window, cx) {
                        let bounds = menu_bounds.get();
                        this.menu.anchor = bounds.bottom_right();
                        this.menu.right_edge = Some(bounds.right());
                    }
                })),
        )
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
    pub(in super::super) fn header_actions(&self) -> bool {
        self.density.orbita
    }

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
