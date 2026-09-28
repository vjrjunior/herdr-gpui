#![allow(clippy::unwrap_used)]
use super::{MAX_TITLE_CHARS, PADDING, clean_title, popup_chrome};
use gpui::{Bounds, Pixels, point, px, size};

fn bounds(x: f32, y: f32, width: f32, height: f32) -> Bounds<Pixels> {
    Bounds::new(point(px(x), px(y)), size(px(width), px(height)))
}

#[test]
fn the_panel_frames_the_grid_with_the_header_above_it() {
    let terminal = bounds(0., 0., 800., 600.);
    let grid = bounds(200., 200., 400., 200.);
    let chrome = popup_chrome(grid, terminal, Some(40.));
    assert_eq!(chrome.panel, bounds(188., 148., 424., 264.));
    assert_eq!(chrome.header, Some(bounds(188., 148., 424., 40.)));
    assert!(chrome.panel.contains(&grid.origin));
    assert_eq!(chrome.header.unwrap().bottom() + px(PADDING), grid.top());
}

#[test]
fn the_panel_stays_inside_the_terminal_and_never_moves_the_grid() {
    let terminal = bounds(0., 0., 800., 600.);
    let grid = bounds(4., 10., 792., 580.);
    let chrome = popup_chrome(grid, terminal, Some(40.));
    assert_eq!(chrome.panel, terminal);
    assert_eq!(chrome.header, Some(bounds(0., 0., 800., 0.)));
}

#[test]
fn an_untitled_popup_has_no_header() {
    let grid = bounds(200., 200., 400., 200.);
    let chrome = popup_chrome(grid, bounds(0., 0., 800., 600.), None);
    assert_eq!(chrome.header, None);
    assert_eq!(chrome.panel, bounds(188., 188., 424., 224.));
}

#[test]
fn titles_are_one_bounded_printable_line() {
    assert_eq!(
        clean_title("  Move to worktree \n"),
        Some("Move to worktree".into())
    );
    assert_eq!(clean_title("a\u{202e}b\u{7}c").as_deref(), Some("abc"));
    assert_eq!(clean_title(" \u{1b} "), None);
    assert_eq!(
        clean_title(&"x".repeat(500)).map(|title| title.chars().count()),
        Some(MAX_TITLE_CHARS)
    );
}
