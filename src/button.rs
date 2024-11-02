use crate::constants;
use macroquad::prelude::*;

pub struct Button {
    pub x: f32,
    pub y: f32,
    pub text: String,
    pub width: f32,
    pub height: f32,
    pub frozen: bool,
}

impl Button {
    pub fn new(x: f32, y: f32, text: &str) -> Self {
        Button {
            x,
            y,
            text: text.to_string(),
            width: constants::buttons::width(),
            height: constants::buttons::height(),
            frozen: false,
        }
    }

    pub fn draw(&self, border_color: Color) {
        draw_rectangle(self.x, self.y, self.width, self.height, self.get_color());
        draw_rectangle_lines(
            self.x,
            self.y,
            self.width,
            self.height,
            constants::buttons::BORDER_SIZE,
            border_color,
        );
        let font_size = 20.0;
        let text_x = self.x
            + (self.width - measure_text(&self.text, None, font_size as u16, 1.0).width) / 2.0;
        let text_y = self.y + (self.height + font_size) / 2.0;
        draw_text(&self.text, text_x, text_y, font_size, BLACK);
    }

    pub fn get_color(&self) -> Color {
        if self.frozen {
            DARKGRAY
        } else if self.hovered() {
            GRAY
        } else {
            WHITE
        }
    }

    pub fn hovered(&self) -> bool {
        let (x, y) = mouse_position();
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
    }
}
