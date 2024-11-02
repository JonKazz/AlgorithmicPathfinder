use macroquad::prelude::*;

mod button;
mod constants;
mod draw;
mod inputs;

use button::Button;
use constants::{buttons, grid};

#[macroquad::main("MyGame")]
async fn main() {
    set_fullscreen(true);

    let mut buttons = [
        Button::new(buttons::right_buttons_x(), grid::y_pos(), "CLEAR"),
        Button::new(
            buttons::right_buttons_x(),
            grid::y_pos() + buttons::button_distance_y(),
            "PLACE WALL",
        ),
        Button::new(
            buttons::right_buttons_x(),
            grid::y_pos() + buttons::button_distance_y() * 2.0,
            "START FLAG",
        ),
        Button::new(
            buttons::right_buttons_x(),
            grid::y_pos() + buttons::button_distance_y() * 3.0,
            "END FLAG",
        ),
    ];

    let mut mode: Color = WHITE;
    let mut zoom_level: u16 = 30;
    let mut grid = [[[0.0; 4]; grid::NUM_TILES]; grid::NUM_TILES];
    let mut input_handler = inputs::InputHandler::new();
    let mut window_size = (screen_width(), screen_height());

    loop {
        clear_background(DARKGRAY);
        draw::adjust_grid(zoom_level, &mut grid);
        draw::draw_objects(zoom_level, grid, &buttons, mode);
        input_handler.handle_inputs(&mut zoom_level, &mut grid, &mut buttons, &mut mode);

        let current_window_size = (screen_width(), screen_height());
        if window_size != current_window_size {
            input_handler.resize(&mut buttons);
            window_size = current_window_size;
        }

        next_frame().await;
    }
}
