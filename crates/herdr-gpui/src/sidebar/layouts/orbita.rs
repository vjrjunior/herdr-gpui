//! Orbita's rows: Herdr's rows on the rounded comfortable look, plus tree
//! guides for worktrees, a child highlight that starts where the guide's tick
//! ends, the `[sidebar_worktrees]` font on worktree rows, and a badge line for
//! values plugins report through workspace metadata.

use super::super::{
    ARROW_RESERVE,
    agents::agent_labels,
    cell::{AgentRow, Fold, RowContext, RowLayout, RowState, WorkspaceRow},
    layout::{HeaderCase, Highlight, Rounded, SidebarDensity, SidebarStyle, outline_border},
    line_height,
    row::{RowBadge, RowIcon, RowKind, RowTree},
};
use crate::{Command, HerdrWindow, config::Theme};
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
            chevron(fold, cx.theme)
                .w(px(ARROW_RESERVE - density.gap()))
                .h(px(line_height(cx.font) * lines))
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
            cx.width,
            icon,
            arrow,
            badge,
            None,
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
            cx.indicators,
            false,
            state,
            RowTree::None,
            false,
            cx.width,
            RowIcon::None,
            None,
            None,
            agent.status_text,
            &[],
            cx.look,
            (cx.font, cx.theme),
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
                        this.menu.anchor = menu_bounds.get().bottom_left();
                    }
                })),
        )
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
    fn uncommitted_on_branch(&self) -> bool {
        true
    }
    fn header_actions(&self) -> bool {
        true
    }
    fn header_case(&self) -> HeaderCase {
        Rounded.header_case()
    }
}
