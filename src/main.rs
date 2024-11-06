use crate::search::BFSState;
use macroquad::prelude::*;

mod button;
mod constants;
mod draw;
mod inputs;
mod search;
mod tile;

use button::Button;
use constants::{buttons, grid};

#[macroquad::main("Pathfinder")]
async fn main() {
    let buttons = [
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

    let mode = WHITE;
    let grid = [[tile::Tile::new(0.0, 0.0, 50.0, WHITE); grid::NUM_TILES]; grid::NUM_TILES];

    let mut visual_handler = draw::VisualHandler::new(30, grid, buttons);
    let mut window_size = (screen_width(), screen_height());
    let start_flag = (grid::NUM_TILES + 1, grid::NUM_TILES + 1);
    let end_flag = (grid::NUM_TILES + 1, grid::NUM_TILES + 1);
    let mut input_handler = inputs::InputHandler::new(mode, start_flag, end_flag);

    let mut bfs_state: Option<BFSState> = None;

    loop {
        clear_background(DARKGRAY);

        visual_handler.zoom_grid();
        visual_handler.draw_grid(input_handler.mode);
        visual_handler.draw_buttons(input_handler.start_flag, input_handler.end_flag, input_handler.mode);
        input_handler.handle_inputs(&mut visual_handler);

        if input_handler.mode == RED {
            if bfs_state.is_none() {
                bfs_state = Some(BFSState::new(input_handler.start_flag, input_handler.end_flag));
            }

            if let Some(state) = &mut bfs_state {
                let finished = state.step(&mut visual_handler).await;
                if finished {
                    bfs_state = None;
                    input_handler.mode = YELLOW;
                }
            }
        } else {
            bfs_state = None;
        }

        let current_window_size = (screen_width(), screen_height());
        if window_size != current_window_size {
            inputs::resize_buttons(&mut visual_handler.buttons);
            window_size = current_window_size;
        }

        next_frame().await;
    }
}
