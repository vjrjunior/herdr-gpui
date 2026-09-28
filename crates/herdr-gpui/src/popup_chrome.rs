use crate::config::{FontConfig, Theme};
use gpui::*;

const PADDING: f32 = 12.;
const ACCENT_BAR_WIDTH: f32 = 3.;
const SCRIM: u32 = 0x0000_0066;
const MAX_TITLE_CHARS: usize = 80;

#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) struct PopupChrome {
    pub(crate) panel: Bounds<Pixels>,
    pub(crate) header: Option<Bounds<Pixels>>,
}

pub(crate) fn popup_chrome(
    grid: Bounds<Pixels>,
    terminal: Bounds<Pixels>,
    header_height: Option<f32>,
) -> PopupChrome {
    let padding = px(PADDING);
    let header_height = px(header_height.unwrap_or(0.));
    let top = grid.top() - padding - header_height;
    let panel = Bounds::from_corners(
        point(grid.left() - padding, top),
        point(grid.right() + padding, grid.bottom() + padding),
    )
    .intersect(&terminal);
    let header = (header_height > px(0.)).then(|| {
        Bounds::from_corners(
            panel.origin,
            point(panel.right(), (top + header_height).max(panel.top())),
        )
    });
    PopupChrome { panel, header }
}

pub(crate) fn clean_title(title: &str) -> Option<String> {
    let title: String = crate::notifications::safe_text(title, MAX_TITLE_CHARS * 4)
        .chars()
        .take(MAX_TITLE_CHARS)
        .collect();
    let title = title.trim();
    (!title.is_empty()).then(|| title.to_owned())
}

pub(crate) fn paint_popup_chrome(
    title: &str,
    grid: Bounds<Pixels>,
    terminal: Bounds<Pixels>,
    (font, theme): (&FontConfig, &Theme),
    window: &mut Window,
    cx: &mut App,
) {
    let title = clean_title(title);
    let line = px(font.line_height());
    let chrome = popup_chrome(
        grid,
        terminal,
        title.as_ref().map(|_| font.line_height() + 2. * PADDING),
    );
    window.with_content_mask(Some(ContentMask { bounds: terminal }), |window| {
        window.paint_quad(fill(terminal, rgba(SCRIM)));
        window.paint_quad(quad(
            chrome.panel,
            px(0.),
            rgb(theme.surface),
            px(1.),
            rgb(theme.active),
            BorderStyle::default(),
        ));
        let (Some(header), Some(title)) = (chrome.header, title) else {
            return;
        };
        window.paint_quad(fill(
            Bounds::new(
                point(header.left(), header.bottom() - px(1.)),
                size(header.size.width, px(1.)),
            ),
            rgb(theme.active),
        ));
        let bar = Bounds::new(
            point(header.left() + px(PADDING), header.center().y - line / 2.),
            size(px(ACCENT_BAR_WIDTH), line),
        );
        window.paint_quad(quad(
            bar,
            px(ACCENT_BAR_WIDTH / 2.),
            rgb(theme.primary()),
            px(0.),
            transparent_black(),
            BorderStyle::default(),
        ));
        let mut title_font = font.font();
        title_font.weight = FontWeight::SEMIBOLD;
        let shaped = window.text_system().shape_line(
            title.clone().into(),
            px(font.size),
            &[TextRun {
                len: title.len(),
                font: title_font,
                color: rgb(theme.foreground).into(),
                background_color: None,
                underline: None,
                strikethrough: None,
            }],
            None,
        );
        let origin = point(bar.right() + px(PADDING), header.center().y - line / 2.);
        window.with_content_mask(Some(ContentMask { bounds: header }), |window| {
            if shaped
                .paint(origin, line, TextAlign::Left, None, window, cx)
                .is_err()
            {
                tracing::warn!(category = "glyph_paint", "popup title paint failed");
            }
        });
    });
}

#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
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
}
