use macroquad::prelude::*;
use crate::constants;

pub struct Button {
    pub x: f32,
    pub y: f32,
    pub text: String,
    pub color: Color,
    pub width: f32,
    pub height: f32,
}

impl Button {
    pub fn new(x: f32, y: f32, text: &str, color: Color) -> Self {
        Button {
            x,
            y,
            text: text.to_string(),
            color,
            width: constants::buttons::width(),
            height: constants::buttons::height(),
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.x, self.y, self.width, self.height, self.color);
        let font_size = 20.0;
        let text_x = self.x + (self.width - measure_text(&self.text, None, font_size as u16, 1.0).width) / 2.0;
        let text_y = self.y + (self.height + font_size) / 2.0;
        draw_text(&self.text, text_x, text_y, font_size, BLACK);
    }

    pub fn clicked(&mut self) {
        self.color = PINK;
    }
}