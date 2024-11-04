use macroquad::prelude::*;

mod button;
mod constants;
mod draw;
mod inputs;
mod tile;
mod search;

use button::Button;
use constants::{buttons, grid};

#[macroquad::main("Pathfinder")]
async fn main() {
    let mut buttons = [
        Button::new(buttons::right_buttons_x(), grid::y_pos(), "CLEAR", true),
        Button::new(
            buttons::right_buttons_x(),
            grid::y_pos() + buttons::button_distance_y(),
            "PLACE WALL",
            true,
        ),
        Button::new(
            buttons::right_buttons_x(),
            grid::y_pos() + buttons::button_distance_y() * 2.0,
            "START FLAG",
            true,
        ),
        Button::new(
            buttons::right_buttons_x(),
            grid::y_pos() + buttons::button_distance_y() * 3.0,
            "END FLAG",
            true,
        ),
        Button::new(
            buttons::left_buttons_x(),
            grid::y_pos() + buttons::button_distance_y(),
            "SEARCH",
            false,
        ),
    ];

    let mut mode = WHITE;
    let mut zoom_level: u16 = 30;
    let mut grid = [[tile::Tile::new(0.0, 0.0, 50.0, WHITE); grid::NUM_TILES]; grid::NUM_TILES];
    let mut input_handler = inputs::InputHandler::new();
    let mut window_size = (screen_width(), screen_height());
    let mut start_flag = (grid::NUM_TILES + 1, grid::NUM_TILES + 1);
    let mut end_flag = (grid::NUM_TILES + 1, grid::NUM_TILES + 1);

    loop {
        clear_background(DARKGRAY);

        draw::zoom_grid(zoom_level, &mut grid);
        draw::draw_objects(zoom_level, grid, &buttons, mode);
        input_handler.handle_inputs(&mut zoom_level, &mut grid, &mut buttons, &mut mode, &mut start_flag, &mut end_flag);

        let current_window_size = (screen_width(), screen_height());
        if window_size != current_window_size {
            input_handler.resize(&mut buttons);
            window_size = current_window_size;
        }

        next_frame().await;
    }
}
