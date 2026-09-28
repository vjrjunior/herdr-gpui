use crate::config::{FontConfig, Theme};
use gpui::*;
use herdr_client::protocol::ClientShellPopupSurface;

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

pub(crate) struct PopupLook {
    font: FontConfig,
    theme: Theme,
    cell_width: f32,
    cell_height: f32,
}

impl PopupLook {
    pub(crate) fn new(font: &FontConfig, theme: &Theme, cell_width: f32, cell_height: f32) -> Self {
        Self {
            font: font.clone(),
            theme: theme.clone(),
            cell_width,
            cell_height,
        }
    }

    pub(crate) fn paint(
        &self,
        popup: &ClientShellPopupSurface,
        terminal: Bounds<Pixels>,
        offset: Point<Pixels>,
        window: &mut Window,
        cx: &mut App,
    ) {
        let grid = Bounds::new(
            terminal.origin + offset,
            size(
                px(f32::from(popup.frame.width) * self.cell_width),
                px(f32::from(popup.frame.height) * self.cell_height),
            ),
        );
        paint_popup_chrome(
            &popup.title,
            grid,
            terminal,
            (&self.font, &self.theme),
            window,
            cx,
        );
    }
}

fn paint_popup_chrome(
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
mod tests;
