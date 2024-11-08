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

    pub const NUM_BUTTONS: usize = 8;
    pub const SEARCH_BUTTONS: [&str; 3] = ["BFS SEARCH", "DFS SEARCH", "A* SEARCH"];

    pub fn border_size() -> f32 {
        height() / 10.0
    }

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

pub mod colors {
    use macroquad::{
        color::{BLUE, GREEN, ORANGE, PURPLE, RED, YELLOW},
        prelude::Color,
    };

    pub const BACKGROUND: Color = Color::new(200.0 / 255.0, 220.0 / 255.0, 250.0 / 255.0, 1.0);
    pub const FROZEN: Color = Color::new(170.0 / 255.0, 170.0 / 255.0, 170.0 / 255.0, 1.0);
    pub const HOVERED: Color = Color::new(215.0 / 255.0, 215.0 / 255.0, 215.0 / 255.0, 1.0);
    pub const HOVERED_BLUE: Color = Color::new(140.0 / 255.0, 180.0 / 255.0, 1.0, 1.0);
    pub const HOVERED_GREEN: Color = Color::new(160.0 / 255.0, 1.0, 160.0 / 255.0, 1.0);
    pub const INVALID: Color = Color::new(160.0 / 255.0, 70.0 / 255.0, 70.0 / 255.0, 1.0);
    pub const FROZEN_COLORS: [Color; 6] = [YELLOW, ORANGE, RED, PURPLE, GREEN, BLUE];
}
