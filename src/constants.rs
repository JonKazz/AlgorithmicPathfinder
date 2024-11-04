pub mod window {
    use macroquad::prelude::{screen_height, screen_width};

    pub fn width() -> f32 {
        screen_width()
    }

    pub fn height() -> f32 {
        screen_height()
    }
}

pub mod grid {
    use super::window;

    pub const NUM_TILES: usize = 65;
    pub const TILE_THICKNESS: f32 = 1.0;

    pub fn border_thickness() -> f32 {
        size() / 8.0
    }

    pub fn border_padding() -> f32 {
        border_thickness() / 2.0
    }

    pub fn size() -> f32 {
        window::width() / 3.0
    }

    pub fn y_pos() -> f32 {
        (window::height() - size()) / 2.0
    }

    pub fn x_pos() -> f32 {
        window::width() / 3.0
    }
}

pub mod buttons {
    use super::grid;

    pub const NUM_BUTTONS: usize = 5;
    pub const BORDER_SIZE: f32 = 5.0;

    pub fn width() -> f32 {
        grid::size() / 3.0
    }

    pub fn height() -> f32 {
        grid::size() / 6.0
    }

    pub fn right_buttons_x() -> f32 {
        grid::x_pos() as f32 + grid::size() + 100.0
    }

    pub fn left_buttons_x() -> f32 {
        grid::x_pos() as f32 - width() - 100.0
    }

    pub fn button_distance_y() -> f32 {
        (grid::size() - (4.0 * height())) / (4.0 - 1.0) + height()
    }
}
