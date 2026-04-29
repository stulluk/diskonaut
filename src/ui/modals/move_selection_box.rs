use ::std::ffi::OsString;
use ::tui::buffer::Buffer;
use ::tui::layout::Rect;
use ::tui::style::{Color, Modifier, Style};
use ::tui::widgets::Widget;

use crate::ui::grid::draw_filled_rect;
use crate::ui::format::truncate_middle;

pub struct MoveSelectionBox<'a> {
    pub target_folders: &'a [OsString],
    pub selected_index: usize,
}

impl<'a> MoveSelectionBox<'a> {
    pub fn new(target_folders: &'a [OsString], selected_index: usize) -> Self {
        Self {
            target_folders,
            selected_index,
        }
    }
}

impl<'a> Widget for MoveSelectionBox<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let width = if area.width > 120 { 120 } else { area.width / 2 };
        let height = if area.height > 22 { 16 } else { 12 };
        let x = ((area.x + area.width) / 2) - width / 2;
        let y = ((area.y + area.height) / 2) - height / 2;
        let rect = Rect {
            x,
            y,
            width,
            height,
        };
        let fill_style = Style::default()
            .bg(Color::Black)
            .fg(Color::White)
            .add_modifier(Modifier::BOLD);
        draw_filled_rect(buf, fill_style, &rect);

        let title = "move-backup-to-subdir: choose destination";
        let title_x = ((rect.width - title.len() as u16) as f64 / 2.0).ceil() as u16 + rect.x;
        buf.set_string(title_x, rect.y + 2, title, fill_style);

        let hint = "Use up/down and Enter";
        let hint_x = ((rect.width - hint.len() as u16) as f64 / 2.0).ceil() as u16 + rect.x;
        buf.set_string(hint_x, rect.y + 3, hint, fill_style);

        let selected_style = Style::default()
            .bg(Color::White)
            .fg(Color::Black)
            .add_modifier(Modifier::BOLD);
        let normal_style = fill_style;

        let list_start_y = rect.y + 5;
        let list_end_y = rect.y + rect.height - 2;
        let visible_rows = if list_end_y > list_start_y {
            (list_end_y - list_start_y) as usize
        } else {
            1
        };
        let window_start = if self.selected_index >= visible_rows {
            self.selected_index - visible_rows + 1
        } else {
            0
        };

        for (row, i) in (window_start..self.target_folders.len())
            .take(visible_rows)
            .enumerate()
        {
            let folder = &self.target_folders[i];
            let y = list_start_y + row as u16;
            let max_label_len = if rect.width > 8 { rect.width - 8 } else { 1 };
            let folder_text = truncate_middle(&folder.to_string_lossy(), max_label_len);
            let label = format!(" {} ", folder_text);
            let style = if i == self.selected_index {
                selected_style
            } else {
                normal_style
            };
            buf.set_string(rect.x + 3, y, &label, style);
        }

        if self.target_folders.len() > visible_rows {
            let progress = format!(
                "{}/{}",
                self.selected_index + 1,
                self.target_folders.len()
            );
            let progress_x =
                ((rect.width - progress.len() as u16) as f64 / 2.0).ceil() as u16 + rect.x;
            buf.set_string(progress_x, rect.y + rect.height - 1, progress, fill_style);
        }
    }
}
