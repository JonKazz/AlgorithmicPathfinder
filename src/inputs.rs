use crate::button;
use crate::constants;
use crate::tile;
use crate::draw;
use constants::grid;
use constants::buttons;
use macroquad::prelude::*;

pub struct InputHandler {
    holding_leftclick: bool,
}

impl InputHandler {
    pub fn new() -> Self {
        InputHandler {
            holding_leftclick: false,
        }
    }

    pub fn handle_inputs(
        &mut self,
        vh : &mut draw::VisualHandler,
        mode: &mut Color,
        start_flag: &mut (usize, usize),
        end_flag: &mut (usize, usize),
    ) {
        let (_, scroll) = mouse_wheel();
        if scroll > 0.0 && vh.zoom_level < (grid::NUM_TILES as u16) - 1 {
            vh.zoom_level += 2;
        } else if scroll < 0.0 && vh.zoom_level > 10 {
            vh.zoom_level -= 2;
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            self.holding_leftclick = true;
            handle_button_click(vh, mode);
            handle_grid_click(vh, mode, start_flag, end_flag);
        }

        if self.holding_leftclick && *mode == BLACK {
            handle_grid_click(vh, mode, start_flag, end_flag);
        }

        if is_mouse_button_released(MouseButton::Left) {
            self.holding_leftclick = false;
        }
    }

    pub fn resize(&mut self, buttons: &mut [button::Button; buttons::NUM_BUTTONS]) {
        let mut right_y = grid::y_pos();
        let mut left_y = grid::y_pos();
        for button in buttons {
            if button.rightside {
                button.x = buttons::right_buttons_x();
                button.y = right_y;
                right_y += buttons::button_distance_y();
            } else {
                button.x = buttons::left_buttons_x();
                button.y = left_y;
                left_y += buttons::button_distance_y();
            }
            button.width = buttons::width();
            button.height = buttons::height();
        }
    }
}

pub fn handle_grid_click(
    vh : &mut draw::VisualHandler,
    mode : &mut Color,
    start_flag : &mut (usize, usize),
    end_flag : &mut (usize, usize)
) {
    let center = grid::NUM_TILES / 2;
    let i = center.saturating_sub(vh.zoom_level as usize / 2);
    let j = (center + vh.zoom_level as usize / 2).min(grid::NUM_TILES);

    for row in i..j {
        for col in i..j {
            let tile = &mut vh.grid[row][col];
            if tile.tile_hovered() {
                if *mode == BLACK && tile.color == WHITE{
                    tile.color = BLACK;
                } else if *mode == GREEN {
                    tile.color = GREEN;
                    *mode = WHITE;
                    *start_flag = (row, col);
                } else if *mode == BLUE {
                    tile.color = BLUE;
                    *mode = WHITE;
                    *end_flag = (row, col);
                }
            }
        }
    }
}

pub fn handle_button_click(
    vh : &mut draw::VisualHandler,
    mode: &mut Color,
) {
    for button in &vh.buttons {
        if button.hovered() {
            match button.text.as_str() {
                "CLEAR" => clear_grid(mode, &mut vh.grid),
                "PLACE WALL" => *mode = BLACK,
                "START FLAG" => set_flag(mode, GREEN, &mut vh.grid),
                "END FLAG" => set_flag(mode, BLUE, &mut vh.grid),
                "SEARCH" => *mode = RED,
                _ => {}
            }
        }
    }
}

pub fn set_flag(
    mode: &mut Color,
    color: Color,
    grid: &mut [[tile::Tile; grid::NUM_TILES]; grid::NUM_TILES],
) {
    *mode = color;
    for row in 0..grid::NUM_TILES {
        for col in 0..grid::NUM_TILES {
            let tile = &mut grid[row][col];
            if tile.color == color {
                tile.color = WHITE;
            }
        }
    }
}
pub fn clear_grid(mode: &mut Color, grid: &mut [[tile::Tile; grid::NUM_TILES]; grid::NUM_TILES]) {
    *mode = WHITE;
    for row in 0..grid::NUM_TILES {
        for col in 0..grid::NUM_TILES {
            let tile = &mut grid[row][col];
            tile.color = WHITE;
        }
    }
}