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

    pub fn y_pos() -> f32 {
        window::height() / 5.0
    }

    pub fn x_pos() -> f32 {
        window::width() / 3.0
    }
}

pub mod buttons {
    use super::grid;

    pub const NUM_BUTTONS: usize = 1;
   
    pub fn width() -> f32 {
        grid::size() / 3.0
    }

    pub fn height() -> f32 {
        grid::size() / 6.0
    }

    pub fn right_buttons_x() -> f32 {
        grid::x_pos() as f32 + grid::size() + 100.0
    }
}
