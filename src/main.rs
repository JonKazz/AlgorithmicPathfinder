use crate::search::*;
use macroquad::prelude::*;

mod button;
mod constants;
mod draw;
mod inputs;
mod search;
mod tile;

use button::Button;
use constants::*;

enum SearchAlgorithm {
    None,
    DFS(DFSState),
    BFS(BFSState),
    ASTAR(ASTARState),
}

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
        Button::new(buttons::left_buttons_x(), grid::y_pos(), "DFS", false),
        Button::new(
            buttons::left_buttons_x(),
            grid::y_pos() + buttons::button_distance_y(),
            "BFS",
            false,
        ),
        Button::new(
            buttons::left_buttons_x(),
            grid::y_pos() + buttons::button_distance_y() * 2.0,
            "A*",
            false,
        ),
        Button::new(
            buttons::left_buttons_x(),
            grid::y_pos() + buttons::button_distance_y() * 3.0,
            "RANDOM MAZE",
            false,
        ),
    ];

    let grid = [[tile::Tile::new(0.0, 0.0, 50.0, WHITE); grid::NUM_TILES]; grid::NUM_TILES];
    let mut visual_handler = draw::VisualHandler::new(30, grid, buttons);
    let mut window_size = (screen_width(), screen_height());
    let start_flag = (grid::NUM_TILES + 1, grid::NUM_TILES + 1);
    let end_flag = (grid::NUM_TILES + 1, grid::NUM_TILES + 1);
    let mut input_handler = inputs::InputHandler::new(WHITE, start_flag, end_flag);
    let mut search_algo = SearchAlgorithm::None;
    visual_handler.zoom_grid();

    loop {
        clear_background(colors::BACKGROUND);

        visual_handler.draw_grid(input_handler.mode);
        visual_handler.draw_buttons(
            input_handler.start_flag,
            input_handler.end_flag,
            input_handler.mode,
        );
        input_handler.handle_inputs(&mut visual_handler);
        adjust_window(&mut window_size, &mut visual_handler);
        generate_search(&mut input_handler, &mut visual_handler, &mut search_algo).await;

        next_frame().await;
    }
}

fn adjust_window(window_size: &mut (f32, f32), visual_handler: &mut draw::VisualHandler) {
    let current_window_size = (screen_width(), screen_height());
    if *window_size != current_window_size {
        visual_handler.zoom_grid();
        inputs::resize_buttons(&mut visual_handler.buttons);
        *window_size = current_window_size;
    }
}

async fn generate_search(
    input_handler: &mut inputs::InputHandler,
    visual_handler: &mut draw::VisualHandler,
    search_algo: &mut SearchAlgorithm,
) {
    if input_handler.mode == YELLOW {
        if let SearchAlgorithm::None = search_algo {
            *search_algo = SearchAlgorithm::DFS(DFSState::new(input_handler.start_flag));
        } else if let SearchAlgorithm::DFS(state) = search_algo {
            let finished = state.step(visual_handler, input_handler.end_flag).await;
            if finished {
                *search_algo = SearchAlgorithm::None;
                input_handler.mode = PURPLE;
            }
        }
    } else if input_handler.mode == ORANGE {
        if let SearchAlgorithm::None = search_algo {
            *search_algo = SearchAlgorithm::BFS(BFSState::new(input_handler.start_flag));
        } else if let SearchAlgorithm::BFS(state) = search_algo {
            let finished = state.step(visual_handler, input_handler.end_flag).await;
            if finished {
                *search_algo = SearchAlgorithm::None;
                input_handler.mode = PURPLE;
            }
        }
    } else if input_handler.mode == RED {
        if let SearchAlgorithm::None = search_algo {
            *search_algo = SearchAlgorithm::ASTAR(ASTARState::new(
                input_handler.start_flag,
                input_handler.end_flag,
            ));
        } else if let SearchAlgorithm::ASTAR(state) = search_algo {
            let finished = state.step(visual_handler, input_handler.end_flag).await;
            if finished {
                *search_algo = SearchAlgorithm::None;
                input_handler.mode = PURPLE;
            }
        }
    } else {
        *search_algo = SearchAlgorithm::None;
    }
}
