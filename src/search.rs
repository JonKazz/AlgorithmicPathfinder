use crate::constants::grid;
use crate::tile::Tile;
use macroquad::prelude::*;
use std::collections::VecDeque;

pub fn bfs(start_flag: (usize, usize), end_flag: (usize, usize), grid: &mut [[Tile; grid::NUM_TILES]; grid::NUM_TILES]) {
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    q.push_back(start_flag);

    while let Some(tile) = q.pop_front() {
        if tile == end_flag {
            return;
        }

        let r: i16 = tile.0 as i16;
        let c: i16 = tile.1 as i16;
        let directions: [(i16, i16); 4]  = [(1,0),(0,1),(-1,0),(0,-1)];

        for (dr, dc) in directions.iter() {
            let nr = r + *dr;
            let nc = c + *dc;
            if 0 <= nr && nr < grid::NUM_TILES as i16 && 0 <= nc && nc < grid::NUM_TILES as i16 {
                if grid[nr as usize][nc as usize].color != PINK && grid[nr as usize][nc as usize].color != BLACK {
                    q.push_back((nr as usize, nc as usize));
                    grid[nr as usize][nc as usize].color = PINK;
                }    
            }
        }
    }
}