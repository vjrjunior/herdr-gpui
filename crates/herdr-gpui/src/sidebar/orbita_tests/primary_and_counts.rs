use super::*;

#[gpui::test]
fn orbita_marks_a_repositorys_own_checkout_as_primary(cx: &mut TestAppContext) {
    let cx = draw(cx, LayoutMode::Orbita, 15., None);
    let chip = bounds(cx, "primary-agent-launcher");
    let name = bounds(cx, "name-agent-launcher");
    let column = bounds(cx, "column-agent-launcher");
    assert!(chip.size.width > px(0.));
    assert!(name.right() <= chip.left());
    assert_eq!(chip.right(), column.right());
    assert!(chip.right() <= bounds(cx, "collapse-3").left());
    assert!(chip.top() >= name.top());
    assert!(chip.bottom() <= name.bottom());
    assert!(cx.debug_bounds("primary-sidebar-child").is_none());
    assert!(cx.debug_bounds("primary-herdr").is_none());
}

#[gpui::test]
fn herdr_layouts_draw_no_primary_chip(cx: &mut TestAppContext) {
    let cx = draw(
        cx,
        LayoutMode::new(Density::Comfortable, Style::Rounded),
        15.,
        None,
    );
    assert!(cx.debug_bounds("primary-agent-launcher").is_none());
    assert_eq!(
        bounds(cx, "name-agent-launcher").right(),
        bounds(cx, "column-agent-launcher").right()
    );
}

#[gpui::test]
fn orbita_trails_upstream_counts_on_the_branch_line(cx: &mut TestAppContext) {
    let cx = draw_with(cx, LayoutMode::Orbita, 15., |data| {
        workspace(data, "develop").git_ahead_behind = Some((0, 3));
        workspace(data, "worktree/sidebar-child").git_ahead_behind = Some((4, 0));
    });
    for (column, name, detail, upstream) in [
        (
            "column-agent-launcher",
            "name-agent-launcher",
            "detail-agent-launcher",
            "upstream-agent-launcher",
        ),
        (
            "column-sidebar-child",
            "name-sidebar-child",
            "detail-sidebar-child",
            "upstream-sidebar-child",
        ),
    ] {
        let counts = bounds(cx, upstream);
        assert_eq!(counts.right(), bounds(cx, column).right(), "{upstream}");
        assert!(counts.top() >= bounds(cx, name).bottom(), "{upstream}");
        assert!(counts.left() >= bounds(cx, detail).right(), "{upstream}");
    }
    let branch = bounds(cx, "detail-agent-launcher");
    let counts = bounds(cx, "upstream-agent-launcher");
    assert!(counts.left() - branch.right() > px(20.));
}
