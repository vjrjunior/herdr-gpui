#![allow(clippy::unwrap_used)]

use super::layout_tests::{REPO_KEY, fixture_window, full_draw, snapshot};
use crate::config::{Density, LayoutMode, Style, Theme};
use gpui::{Bounds, Pixels, TestAppContext, VisualTestContext, px, size};
use std::sync::Arc;

type Tokens = Vec<(String, String)>;

fn draw(
    cx: &mut TestAppContext,
    mode: LayoutMode,
    worktree_size: f32,
    tokens: Option<Tokens>,
) -> &mut VisualTestContext {
    let (_, cx) = cx.add_window_view(move |window, cx| {
        let mut view = fixture_window(window, cx);
        let mut data = snapshot(6);
        if let Some(tokens) = tokens {
            let child = data
                .workspaces
                .iter_mut()
                .find(|w| w.branch.as_deref() == Some("worktree/sidebar-child"))
                .unwrap();
            child.tokens = tokens;
        }
        view.live.snapshot = Some(Arc::new(data));
        view.menu.pr_cache.seed(
            crate::pull_request::Input {
                checkout: None,
                repo_key: REPO_KEY.into(),
                branch: "worktree/sidebar-child".into(),
            },
            crate::pull_request::fixture().unwrap(),
            std::time::Instant::now(),
        );
        view.git.seed_probe(
            crate::pull_request::Input {
                checkout: None,
                repo_key: REPO_KEY.into(),
                branch: "develop".into(),
            },
            true,
            std::time::Instant::now(),
        );
        view.config.layout.mode = mode;
        view.config.sidebar.size = 15.;
        view.config.sidebar_worktrees.size = worktree_size;
        view
    });
    cx.simulate_resize(size(px(800.), px(900.)));
    cx.run_until_parked();
    cx.update(|window, cx| full_draw(window, cx).clear(cx));
    cx
}

fn bounds(cx: &mut VisualTestContext, key: &'static str) -> Bounds<Pixels> {
    cx.debug_bounds(key)
        .unwrap_or_else(|| panic!("{key} was not drawn"))
}

fn line(size: f32) -> f32 {
    size * 4. / 3.
}

#[gpui::test]
fn orbita_nests_a_worktree_highlight_behind_its_tree_guide(cx: &mut TestAppContext) {
    let cx = draw(cx, LayoutMode::Orbita, 15., None);
    let child = bounds(cx, "row-sidebar-child");
    let tree = bounds(cx, "tree-sidebar-child");
    let child_highlight = bounds(cx, "highlight-sidebar-child");
    let parent_highlight = bounds(cx, "highlight-agent-launcher");
    let parent_column = bounds(cx, "column-agent-launcher");
    assert_eq!(tree.left(), parent_column.left());
    assert_eq!(child_highlight.left(), tree.right());
    assert!(child_highlight.left() > parent_highlight.left());
    assert_eq!(child_highlight.right(), parent_highlight.right());
    assert!(bounds(cx, "column-sidebar-child").left() > child_highlight.left());
    assert!(child_highlight.left() > child.left());
}

#[gpui::test]
fn comfortable_rounded_keeps_herdrs_worktree_rows(cx: &mut TestAppContext) {
    let cx = draw(
        cx,
        LayoutMode::new(Density::Comfortable, Style::Rounded),
        12.,
        Some(vec![("pr".into(), "#646 \u{b7} open".into())]),
    );
    let parent = bounds(cx, "highlight-agent-launcher");
    assert_eq!(bounds(cx, "highlight-sidebar-child").left(), parent.left());
    assert!(cx.debug_bounds("tree-sidebar-child").is_none());
    assert!(cx.debug_bounds("tokens-sidebar-child").is_none());
    assert!(cx.debug_bounds("pr-sidebar-child").is_some());
    let child = bounds(cx, "row-sidebar-child").size.height;
    let head = bounds(cx, "row-agent-launcher").size.height;
    assert_eq!(child, head);
}

#[gpui::test]
fn orbita_worktree_rows_use_their_own_font(cx: &mut TestAppContext) {
    let full = {
        let cx = draw(cx, LayoutMode::Orbita, 15., None);
        (
            bounds(cx, "row-agent-launcher").size.height,
            bounds(cx, "row-sidebar-child").size.height,
        )
    };
    let cx = draw(cx, LayoutMode::Orbita, 12., None);
    assert_eq!(bounds(cx, "row-agent-launcher").size.height, full.0);
    assert_eq!(
        full.1 - bounds(cx, "row-sidebar-child").size.height,
        px(2. * (line(15.) - line(12.)))
    );
}

#[gpui::test]
fn orbita_shows_reported_tokens_and_drops_the_native_pull_request(cx: &mut TestAppContext) {
    let plain = {
        let cx = draw(cx, LayoutMode::Orbita, 15., None);
        assert!(cx.debug_bounds("pr-sidebar-child").is_some());
        bounds(cx, "row-sidebar-child").size.height
    };
    let cx = draw(
        cx,
        LayoutMode::Orbita,
        15.,
        Some(vec![
            ("pr".into(), "#646 \u{b7} open".into()),
            ("pr_ci".into(), "\u{2713} CI".into()),
        ]),
    );
    let row = bounds(cx, "row-sidebar-child");
    assert_eq!(row.size.height, plain + px(line(15.)));
    let tokens = bounds(cx, "tokens-sidebar-child");
    assert!(tokens.top() >= bounds(cx, "name-sidebar-child").bottom());
    assert!(tokens.bottom() <= row.bottom());
    assert!(
        bounds(cx, "token-sidebar-child-pr").right()
            <= bounds(cx, "token-sidebar-child-pr_ci").left()
    );
    assert!(cx.debug_bounds("pr-sidebar-child").is_none());
}

#[test]
fn orbita_is_a_named_layout_after_herdrs() {
    assert_eq!(LayoutMode::try_from("orbita").unwrap(), LayoutMode::Orbita);
    assert_eq!(LayoutMode::Orbita.label(), "Orbita");
    assert_eq!(LayoutMode::ALL.last(), Some(&LayoutMode::Orbita));
    assert_eq!(LayoutMode::Orbita.density(), Density::Comfortable);
    assert_eq!(LayoutMode::Orbita.style(), Style::Rounded);
    assert!(REPO_KEY.ends_with(".git"));
}

#[test]
fn only_orbita_draws_guides_in_the_border_color() {
    let theme = Theme::builtin("Nord").unwrap();
    let border = gpui::rgba((theme.foreground << 8) | 0x40);
    let muted = gpui::rgb(theme.muted);
    assert_eq!(
        super::layout::for_mode(LayoutMode::Orbita).tree_color(&theme),
        border
    );
    for density in [Density::Compact, Density::Normal, Density::Comfortable] {
        for style in [Style::Flat, Style::Rounded] {
            let look = super::layout::for_mode(LayoutMode::new(density, style));
            assert_eq!(look.tree_color(&theme), muted);
        }
    }
}

#[test]
fn token_colors_follow_their_leading_mark() {
    let theme = Theme::builtin("Nord").unwrap();
    let color = |value| super::row::token_color(value, &theme);
    assert_eq!(color("\u{2713} CI"), theme.palette[2]);
    assert_eq!(color("\u{2717} changes"), theme.palette[1]);
    assert_eq!(color("\u{25cf} review"), theme.palette[3]);
    assert_eq!(color("#646 \u{b7} open"), theme.subtext());
    assert_eq!(color(""), theme.subtext());
}

#[gpui::test]
fn orbita_folds_a_repository_with_a_chevron(cx: &mut TestAppContext) {
    let cx = draw(cx, LayoutMode::Orbita, 15., None);
    let fold = bounds(cx, "collapse-3");
    let chevron = bounds(cx, "chevron-3");
    assert_eq!(chevron.size.width, px(12.));
    assert!(fold.contains(&chevron.center()));
    cx.simulate_click(fold.center(), Default::default());
    cx.update(|window, cx| full_draw(window, cx).clear(cx));
    let view = cx.update(|window, _| window.root::<crate::HerdrWindow>().flatten().unwrap());
    view.read_with(cx, |view, _| {
        assert!(view.collapsed_repos.contains(REPO_KEY))
    });
}

#[gpui::test]
fn herdr_layouts_keep_their_fold_triangle(cx: &mut TestAppContext) {
    let cx = draw(
        cx,
        LayoutMode::new(Density::Comfortable, Style::Rounded),
        15.,
        None,
    );
    assert!(cx.debug_bounds("collapse-3").is_some());
    assert!(cx.debug_bounds("chevron-3").is_none());
}

#[gpui::test]
fn the_fold_chevrons_load_and_render(cx: &mut TestAppContext) {
    use gpui::{AssetSource, DevicePixels, Image, ImageFormat};
    let renderer = cx.update(|cx| cx.svg_renderer());
    for path in ["icons/chevron-right.svg", "icons/chevron-down.svg"] {
        let bytes = crate::icons::Icons.load(path).unwrap().unwrap();
        let image = Image::from_bytes(ImageFormat::Svg, bytes.into_owned())
            .to_image_data(renderer.clone())
            .unwrap();
        let rendered = image.size(0);
        assert_eq!(rendered.width, rendered.height, "{path}");
        assert!(rendered.width >= DevicePixels(16), "{path}");
        let pixels = image.as_bytes(0).unwrap();
        assert!(pixels.chunks_exact(4).any(|pixel| pixel[3] > 0), "{path}");
    }
}

#[gpui::test]
fn orbita_marks_uncommitted_work_with_a_dot(cx: &mut TestAppContext) {
    let cx = draw(cx, LayoutMode::Orbita, 15., None);
    let mark = bounds(cx, "dirty-agent-launcher");
    let dot = bounds(cx, "dirty-dot");
    assert_eq!(dot.size.width, px(8.));
    assert_eq!(dot.size.height, px(8.));
    assert!(mark.contains(&dot.center()));
}

#[gpui::test]
fn herdr_layouts_keep_the_uncommitted_pencil(cx: &mut TestAppContext) {
    let cx = draw(
        cx,
        LayoutMode::new(Density::Comfortable, Style::Rounded),
        15.,
        None,
    );
    assert!(cx.debug_bounds("dirty-agent-launcher").is_some());
    assert!(cx.debug_bounds("dirty-dot").is_none());
}
