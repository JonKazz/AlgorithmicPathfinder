use crate::constants;
use macroquad::prelude::*;

#[derive(Copy, Clone)]
pub struct Tile {
    pub x: f32,
    pub y: f32,
    pub size: f32,
    pub color: Color,
}

impl Tile {
    pub fn new(x: f32, y: f32, size: f32, color: Color) -> Self {
        Tile { x, y, size, color }
    }

    pub fn draw(&mut self) {
        if self.tile_hovered() {
            self.color = LIGHTGRAY;
        }

        draw_rectangle(self.x, self.y, self.size, self.size, self.color);
        draw_rectangle_lines(
            self.x,
            self.y,
            self.size,
            self.size,
            constants::grid::TILE_THICKNESS,
            BLACK,
        );
    }

    pub fn tile_hovered(&self) -> bool {
        let (x, y) = mouse_position();
        x >= self.x && x <= self.x + self.size && y >= self.y && y <= self.y + self.size
    }
}
