use ::tui::buffer::Buffer;
use ::tui::layout::Rect;
use ::tui::style::{Color, Modifier, Style};
use ::tui::widgets::Widget;

use crate::ui::grid::draw_filled_rect;

pub struct CreateDirBox<'a> {
    pub input: &'a str,
}

impl<'a> CreateDirBox<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input }
    }
}

impl<'a> Widget for CreateDirBox<'a> {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let width = if area.width > 100 { 100 } else { area.width / 2 };
        let height = 10;
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

        let title = "new directory (mkdir)";
        let title_x = ((rect.width - title.len() as u16) as f64 / 2.0).ceil() as u16 + rect.x;
        buf.set_string(title_x, rect.y + 2, title, fill_style);

        let input_prefix = "> ";
        let input_value = if self.input.is_empty() {
            "_"
        } else {
            self.input
        };
        let input_line = format!("{}{}", input_prefix, input_value);
        let input_x = ((rect.width - input_line.len() as u16) as f64 / 2.0).ceil() as u16 + rect.x;
        buf.set_string(input_x, rect.y + 5, input_line, fill_style);
    }
}
