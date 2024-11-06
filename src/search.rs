use crate::constants::grid;
use crate::draw;
use macroquad::prelude::*;
use std::collections::VecDeque;

pub struct BFSState {
    q: VecDeque<(usize, usize)>,
    end_flag: (usize, usize),
    travel_grid: [[i16; grid::NUM_TILES]; grid::NUM_TILES],
    found_path: bool,
    color_scale: f32,
}

impl BFSState {
    pub fn new(start_flag: (usize, usize), end_flag: (usize, usize)) -> Self {
        let mut q = VecDeque::new();
        let mut travel_grid = [[-1; grid::NUM_TILES]; grid::NUM_TILES];
        travel_grid[start_flag.0][start_flag.1] = 0;
        let found_path = false;
        let color_scale = 0.2;
        q.push_back(start_flag);
        BFSState {
            q,
            end_flag,
            travel_grid,
            found_path,
            color_scale,
        }
    }

    pub async fn step(&mut self, vh: &mut draw::VisualHandler) -> bool {
        if self.found_path {
            return self.create_path(vh);
        } else if let Some(tile) = self.q.pop_front() {
            let (r, c) = (tile.0 as i16, tile.1 as i16);
            let directions = [(1, 0), (0, 1), (-1, 0), (0, -1)];

            let lower = (grid::NUM_TILES as i16 - vh.zoom_level as i16) / 2;
            let upper = lower + vh.zoom_level as i16;

            for &(dr, dc) in &directions {
                let (nr, nc) = (r + dr, c + dc);

                if (lower..upper).contains(&nr) && (lower..upper).contains(&nc) {
                    if self.end_flag == (nr as usize, nc as usize) {
                        self.found_path = true;
                        self.travel_grid[nr as usize][nc as usize] =
                            self.travel_grid[r as usize][c as usize] + 1;
                        break;
                    }

                    let grid_color = vh.grid[nr as usize][nc as usize].color;
                    if self.travel_grid[nr as usize][nc as usize] < 0 && grid_color != BLACK {
                        self.q.push_back((nr as usize, nc as usize));
                        vh.grid[nr as usize][nc as usize].color = Color::new(
                            self.color_scale * 2.0,
                            0.8 - self.color_scale,
                            0.3 - self.color_scale * 1.5,
                            1.0,
                        );

                        self.travel_grid[nr as usize][nc as usize] =
                            self.travel_grid[r as usize][c as usize] + 1;
                    }
                }
            }
            self.color_scale += 0.01 * (1.0 / vh.zoom_level as f32);
        } else {
            return true;
        }

        false
    }

    pub fn create_path(&mut self, vh: &mut draw::VisualHandler) -> bool {
        if self.travel_grid[self.end_flag.0][self.end_flag.1] == 1 {
            return true;
        }

        let (r, c) = (self.end_flag.0 as i16, self.end_flag.1 as i16);
        let current_distance = self.travel_grid[self.end_flag.0][self.end_flag.1];
        let directions: [(i16, i16); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

        for (dr, dc) in &directions {
            let (nr, nc) = (r + dr, c + dc);

            if 0 <= nr && nr < grid::NUM_TILES as i16 && 0 <= nc && nc < grid::NUM_TILES as i16 {
                let neighbor_distance = self.travel_grid[nr as usize][nc as usize];
                if neighbor_distance == current_distance - 1 {
                    self.end_flag = (nr as usize, nc as usize);
                    vh.grid[self.end_flag.0][self.end_flag.1].color = YELLOW;
                    break;
                }
            }
        }

        false
    }
}
