use crate::button;
use crate::constants;
use crate::tile;
use crate::draw;
use constants::grid;
use constants::buttons;
use macroquad::prelude::*;

pub struct InputHandler {
    holding_leftclick: bool,
    pub mode: Color,
    pub start_flag: (usize, usize),
    pub end_flag: (usize, usize)
}

impl InputHandler {
    pub fn new(mode: Color, start_flag: (usize, usize), end_flag: (usize, usize)) -> Self {
        InputHandler {
            holding_leftclick: false,
            mode,
            start_flag,
            end_flag
        }
    }

    pub fn handle_inputs(
        &mut self,
        vh : &mut draw::VisualHandler,
    ) {
        if self.mode != RED {
            let (_, scroll) = mouse_wheel();
            if scroll > 0.0 && vh.zoom_level < (grid::NUM_TILES) - 1 {
                vh.zoom_level += 2;
                vh.zoom_grid();
            } else if scroll < 0.0 && vh.zoom_level > 10 {
                vh.zoom_level -= 2;
                vh.zoom_grid();
            }
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            self.holding_leftclick = true;
            self.handle_button_click(vh);
            self.handle_grid_click(vh);
        }

        if self.holding_leftclick && self.mode == BLACK {
            self.handle_grid_click(vh);
        }

        if is_mouse_button_released(MouseButton::Left) {
            self.holding_leftclick = false;
        }
    }

    pub fn handle_grid_click(
        &mut self,
        vh : &mut draw::VisualHandler
    ) {
        let center = grid::NUM_TILES / 2;
        let i = center.saturating_sub(vh.zoom_level as usize / 2);
        let j = (center + vh.zoom_level as usize / 2).min(grid::NUM_TILES);
    
        for row in i..j {
            for col in i..j {
                let tile = &mut vh.grid[row][col];
                if tile.tile_hovered() {
                    if self.mode == BLACK && tile.color == WHITE{
                        tile.color = BLACK;
                    } else if self.mode == GREEN {
                        tile.color = GREEN;
                        self.mode = WHITE;
                        self.start_flag = (row, col);
                    } else if self.mode == BLUE {
                        tile.color = BLUE;
                        self.mode = WHITE;
                        self.end_flag = (row, col);
                    }
                }
            }
        }
    }

    fn handle_button_click(
        &mut self,
        vh : &mut draw::VisualHandler,
    ) {
    
        let clicked_button = vh.buttons.iter()
            .find(|button| button.hovered())
            .map (|button| button.text.as_str());
    
        if let Some(clicked_button) = clicked_button {
            match clicked_button {
                "CLEAR" => self.clear_grid(vh),
                "PLACE WALL" => self.mode = BLACK,
                "START FLAG" => set_flag(&mut self.mode, GREEN, &mut vh.grid),
                "END FLAG" => set_flag(&mut self.mode, BLUE, &mut vh.grid),
                "SEARCH" => {
                    self.mode = RED;
                    vh.toggle_freeze_buttons(); 
                },
                _ => {}
            }
        }
    }

    fn clear_grid(
        &mut self,
        vh: &mut draw::VisualHandler,
    ) {
        if self.mode == RED || self.mode == YELLOW {
            vh.toggle_freeze_buttons();
        }
        self.mode = WHITE;
        
        self.start_flag = (grid::NUM_TILES + 1, grid::NUM_TILES + 1);
        self.end_flag = (grid::NUM_TILES + 1, grid::NUM_TILES + 1);
        vh.reset_tiles();
    }
    
}


fn set_flag(
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

pub fn resize_buttons(buttons: &mut [button::Button; buttons::NUM_BUTTONS]) {
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