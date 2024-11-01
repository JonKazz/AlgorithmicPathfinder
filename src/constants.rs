pub mod window {
    use macroquad::prelude::{screen_width, screen_height};

    pub fn width() -> f32 {
        screen_width()
    }

    pub fn height() -> f32 {
        screen_height()
    }
}

pub mod grid {
    use super::window;

    pub const NUM_TILES: usize = 40;
    pub const TILE_THICKNESS: f32 = 1.0;

    pub fn size() -> f32 {
        window::width() / 3.0
    }

    pub fn y_pos() -> u16 {
        (window::height() / 4.0) as u16
    }

    pub fn x_pos() -> u16 {
        (window::width() / 3.0) as u16
    }
}
