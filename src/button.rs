use crate::constants::{buttons, colors};
use crate::grid;

use macroquad::prelude::*;

pub struct Button {
    pub x: f32,
    pub y: f32,
    pub text: String,
    pub width: f32,
    pub height: f32,
    pub frozen: bool,
    pub rightside: bool,
}

impl Button {
    pub fn new(x: f32, y: f32, text: &str, rightside: bool) -> Self {
        Button {
            x,
            y,
            text: text.to_string(),
            width: buttons::width(),
            height: buttons::height(),
            frozen: false,
            rightside,
        }
    }

    pub fn draw(&self, mode: Color) {
        draw_rectangle(self.x, self.y, self.width, self.height, self.get_color());
        draw_rectangle_lines(
            self.x,
            self.y,
            self.width,
            self.height,
            buttons::border_size(),
            self.get_border_color(mode),
        );
        let font_size = self.height / 3.0;
        let text_x = self.x
            + (self.width - measure_text(&self.text, None, font_size as u16, 1.0).width) / 2.0;
        let text_y = self.y + (self.height + font_size) / 2.0;
        draw_text(&self.text, text_x, text_y, font_size, BLACK);
    }

    pub fn get_color(&self) -> Color {
        if self.frozen {
            colors::FROZEN
        } else if self.hovered() {
            colors::HOVERED
        } else {
            WHITE
        }
    }

    pub fn get_border_color(&self, mode: Color) -> Color {
        if self.frozen || mode == WHITE {
            return DARKGRAY;
        } 
        mode
    }

    pub fn hovered(&self) -> bool {
        let (x, y) = mouse_position();
        if self.frozen {
            return false
        }
        x >= self.x && x <= self.x + self.width && y >= self.y && y <= self.y + self.height
    }

    pub fn check_if_valid_flags(
        &mut self, 
        start_flag: (usize, usize), 
        end_flag: (usize, usize),
        mode: Color,
    ) {
        if self.text != "CLEAR" {
            if colors::FROZEN_COLORS.contains(&mode) {
                self.frozen = true;
            } else {
                if buttons::SEARCH_BUTTONS.contains(&self.text.as_str()) {
                    self.frozen = start_flag.0 > grid::NUM_TILES || end_flag.0 > grid::NUM_TILES;
                } else {
                    self.frozen = false;
                }
            }
        }
    } 
}
