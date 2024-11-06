use crate::draw;
use crate::constants::grid;
use macroquad::prelude::*;
use std::collections::VecDeque;

pub struct BFSState {
    q: VecDeque<(usize, usize)>,
    end_flag: (usize, usize),
    start_flag: (usize, usize),
    travel_grid: [[u16; grid::NUM_TILES]; grid::NUM_TILES],
    level: u16,
    found_path: bool,
}

impl BFSState {
    pub fn new(start_flag: (usize, usize), end_flag: (usize, usize)) -> Self {
        let mut q = VecDeque::new();
        let travel_grid = [[(grid::NUM_TILES*grid::NUM_TILES) as u16; grid::NUM_TILES]; grid::NUM_TILES];
        let level = 0;
        let found_path = false;
        q.push_back(start_flag);
        BFSState { q, end_flag, start_flag, travel_grid, level, found_path }
    }

    pub async fn step(&mut self, vh: &mut draw::VisualHandler) -> bool {
        if let Some(tile) = self.q.pop_front() {
            if tile == self.end_flag || self.found_path {
                self.found_path = true;
                if self.create_path(vh) {
                    return true;
                }
            }


            let r: i16 = tile.0 as i16;
            let c: i16 = tile.1 as i16;
            let directions: [(i16, i16); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

            for (dr, dc) in directions.iter() {
                let nr = r + *dr;
                let nc = c + *dc;

                let lower = (grid::NUM_TILES as i16 - vh.zoom_level as i16) / 2;
                let upper = lower + vh.zoom_level as i16;

                if lower <= nr && nr < upper as i16 && lower <= nc && nc < upper as i16 {
                    if vh.grid[nr as usize][nc as usize].color != PINK && vh.grid[nr as usize][nc as usize].color != BLACK {
                        self.q.push_back((nr as usize, nc as usize));
                        vh.grid[nr as usize][nc as usize].color = PINK;
                        self.travel_grid[tile.0][tile.1] = self.level;
                        self.level += 1;
                    }
                }
            }
        }
        else {
            return true;      
        }

    false
    }


    pub fn create_path(&mut self, vh: &mut draw::VisualHandler) -> bool {
        if self.end_flag == self.start_flag {
            return true;
        }
    
        let r: i16 = self.end_flag.0 as i16;
        let c: i16 = self.end_flag.1 as i16;
        let mut next_tile = None;
        let mut min_distance = self.travel_grid[self.end_flag.0][self.end_flag.1];
        let directions: [(i16, i16); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
    
        for (dr, dc) in directions.iter() {
            let nr = r + *dr;
            let nc = c + *dc;
            let lower = (grid::NUM_TILES as i16 - vh.zoom_level as i16) / 2;
            let upper = lower + vh.zoom_level as i16;
    
            if lower <= nr && nr < upper && lower <= nc && nc < upper {
                let neighbor = (nr as usize, nc as usize);
                if self.travel_grid[neighbor.0][neighbor.1] < min_distance {
                    min_distance = self.travel_grid[neighbor.0][neighbor.1];
                    next_tile = Some(neighbor);
                }
            }
        }
    
        if let Some(next) = next_tile {
            self.end_flag = next;
            vh.grid[self.end_flag.0][self.end_flag.1].color = YELLOW;
        }
    
        false
    }
}
