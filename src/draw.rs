extern crate rand;
use rand::Rng;
use crate::button;
use crate::constants;
use crate::tile;

use constants::{grid, buttons};
use macroquad::prelude::*;


pub struct VisualHandler {
    pub zoom_level : usize,
    pub grid : [[tile::Tile; grid::NUM_TILES]; grid::NUM_TILES],
    pub buttons : [button::Button; buttons::NUM_BUTTONS],
    view_range : std::ops::Range<usize>,
}


impl VisualHandler {
    pub fn new(zoom_level: usize, grid: [[tile::Tile; grid::NUM_TILES]; grid::NUM_TILES], buttons: [button::Button; buttons::NUM_BUTTONS]) -> Self {
        VisualHandler {
            zoom_level,
            grid,
            buttons,
            view_range: (grid::NUM_TILES / 2 - 15)..(zoom_level + (grid::NUM_TILES / 2 - 15)),
        }
    }

    pub fn zoom_grid(&mut self) {
        let tile_size = grid::size() / self.zoom_level as f32;
        self.view_range = (grid::NUM_TILES / 2 - self.zoom_level / 2)..(self.zoom_level + (grid::NUM_TILES / 2 - self.zoom_level / 2));

        let mut y_pos = grid::y_pos() as f32;
        for row in self.view_range.clone() {
            let mut x_pos: f32 = grid::x_pos() as f32;
            for col in self.view_range.clone() {
                self.grid[row][col].x = x_pos;
                self.grid[row][col].y = y_pos;
                self.grid[row][col].size = tile_size;
                x_pos += tile_size;
            }
            y_pos += tile_size;
        }
    }

    pub fn draw_grid(&self, mode: Color) {
        draw_rectangle(
            grid::x_pos() - grid::border_padding(),
            grid::y_pos() - grid::border_padding(),
            grid::size() + grid::border_padding() * 2.0,
            grid::size() + grid::border_padding() * 2.0,
            mode,
        );

        draw_rectangle_lines(
            grid::x_pos() - grid::border_padding(),
            grid::y_pos() - grid::border_padding(),
            grid::size() + grid::border_padding() * 2.0,
            grid::size() + grid::border_padding() * 2.0,
            buttons::border_size(),
            if mode == WHITE { DARKGRAY } else { mode },
        );
    
        for row in self.view_range.clone() {
            for col in self.view_range.clone() {
                let mut tile = self.grid[row][col];
                tile.draw();
            }
        }
    
        draw_rectangle_lines(grid::x_pos(), grid::y_pos(), grid::size(), grid::size(), 3.0, BLACK);
    }

    pub fn draw_buttons(&mut self, start_flag: (usize, usize), end_flag: (usize, usize), mode: Color) {

        for button in &mut self.buttons {
            button.check_if_valid_flags(start_flag, end_flag, mode);
            button.draw(mode);
        }
    }

    pub fn random_maze(&mut self) {
        self.blackout_grid();
        let rand_x = rand::thread_rng().gen_range(self.view_range.clone());
        let rand_y = rand::thread_rng().gen_range(self.view_range.clone());
        self.recursive_dfs((rand_x, rand_y));
    }

    fn recursive_dfs(&mut self, position: (usize, usize)) {
        let (original_row, original_col) = position;
        let mut moves = vec!["L", "R", "D", "U"];

        while !moves.is_empty() {
            let random_index = rand::thread_rng().gen_range(0..moves.len());
            let random_move = moves.remove(random_index);
            let mut row: isize = original_row as isize;
            let mut col: isize = original_col as isize;

            match random_move {
                "L" => col -= 2,
                "R" => col += 2,
                "U" => row += 2,
                "D" => row -= 2,
                _ => print!("WHAT"),
            } 

            if row > 0 && col > 0 && self.view_range.contains(&(row as usize)) && self.view_range.contains(&(col as usize)) {
                let row = row as usize;
                let col = col as usize;
                if self.grid[row][col].color != WHITE {
                    let mid_row = if row > original_row {
                        original_row + (row - original_row) / 2
                    } else {
                        original_row.saturating_sub((original_row - row) / 2)
                    };
                    
                    let mid_col = if col > original_col {
                        original_col + (col - original_col) / 2
                    } else {
                        original_col.saturating_sub((original_col - col) / 2)
                    };
                    
                    self.grid[row][col].color = WHITE;
                    self.grid[mid_row][mid_col].color = WHITE;
                    self.recursive_dfs((row, col));
                }
            }
        }

    }

    fn blackout_grid(&mut self) {    
        for row in self.view_range.clone() {
            for col in self.view_range.clone() {
                self.grid[row][col].color = BLACK;
            }
        }
    }
}
