use macroquad::prelude::*;

mod constants;
mod draw;
mod inputs;
mod button;

use constants::{buttons, grid};
use button::Button;

#[macroquad::main("MyGame")]
async fn main() {
    set_fullscreen(true);

    let mut buttons = [
        Button::new(buttons::right_buttons_x(), grid::y_pos(), "CLEAR", WHITE),
    ];

    let mut zoom_level: u16 = 30;
    let mut grid = [[[0.0; 4]; grid::NUM_TILES]; grid::NUM_TILES];
    let mut input_handler = inputs::InputHandler::new();

    loop {
        clear_background(DARKGRAY);

        draw::adjust_grid(zoom_level, &mut grid);
        draw::draw_grid(zoom_level, grid);
        draw::draw_buttons(&buttons);
        input_handler.handle_inputs(&mut zoom_level, &mut grid, &mut buttons);
    
        next_frame().await;
    }
}
