#![allow(clippy::unwrap_used)]
use crate::{
    git::{Git, Status},
    pull_request::Input,
    sidebar::layout_tests::REPO_KEY,
};
use gpui::{TestAppContext, px, size};

#[gpui::test]
fn orbita_marks_uncommitted_work_with_a_dot_on_the_branch_icon(cx: &mut TestAppContext) {
    let dirty = Status {
        additions: 12,
        deletions: 3,
        untracked: 1,
    };
    for (pull_request, status) in [
        (false, dirty),
        (true, dirty),
        (false, Status::default()),
        (true, Status::default()),
    ] {
        let (view, cx) = cx.add_window_view(crate::titlebar::tests::header_window);
        let input = Input {
            checkout: None,
            repo_key: Some(REPO_KEY.into()),
            branch: "develop".into(),
        };
        cx.simulate_resize(size(px(900.), px(600.)));
        cx.update(|_, cx| {
            view.update(cx, |view, cx| {
                view.config.layout.mode = crate::config::LayoutMode::Orbita;
                view.git = Git::fixture(input.clone(), status);
                if pull_request {
                    view.menu.github = crate::github::Auth::connected_fixture();
                    view.menu.pr_cache.seed(
                        input,
                        crate::pull_request::fixture().unwrap(),
                        std::time::Instant::now(),
                    );
                }
                cx.notify();
            })
        });
        cx.update(|window, cx| {
            window.refresh();
            let _ = window.draw(cx);
        });
        let case = format!("pull request {pull_request}, {status:?}");
        assert!(cx.debug_bounds("titlebar-git-dirty").is_none(), "{case}");
        assert_eq!(
            cx.debug_bounds("titlebar-git-pr").is_some(),
            pull_request,
            "{case}"
        );
        assert_eq!(
            cx.debug_bounds("titlebar-git-additions").is_some(),
            !pull_request && status.dirty(),
            "{case}"
        );
        let dot = cx.debug_bounds("titlebar-git-dirty-dot");
        assert_eq!(dot.is_some(), status.dirty(), "{case}");
        if let Some(dot) = dot {
            let button = cx.debug_bounds("titlebar-git").unwrap();
            assert_eq!(dot.size, size(px(6.), px(6.)), "{case}");
            assert!(button.contains(&dot.center()), "{case}");
            assert!(dot.center().y < button.center().y, "{case}");
        }
    }
}
