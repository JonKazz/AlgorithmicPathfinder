use crate::constants::grid;
use crate::draw;
use crate::tile;
use macroquad::prelude::*;
use std::collections::{VecDeque, HashMap, HashSet, BinaryHeap};
use std::cmp::Reverse;

type Coordinate = (usize, usize);
pub struct BFSState {
    q: VecDeque<(usize, usize)>,
    end_flag: (usize, usize),
    path: HashMap<Coordinate, Coordinate>,
    current_path_tile: Coordinate,
    found_path: bool,
    color_scale: f32,
}

impl BFSState {
    pub fn new(start_flag: (usize, usize), end_flag: (usize, usize)) -> Self {
        let mut q = VecDeque::new();
        let path = HashMap::new();

        q.push_back(start_flag);
        BFSState {
            q,
            end_flag,
            path,
            current_path_tile: (0, 0),
            found_path: false,
            color_scale: 0.2,
        }
    }

    pub async fn step(&mut self, vh: &mut draw::VisualHandler) -> bool {
        if self.found_path {
            return reconstruct_path(&self.path, &mut self.current_path_tile, vh);
        } else if let Some(current) = self.q.pop_front() {
            for neighbor in get_neighbors(vh, current.0, current.1) {
                if self.end_flag == neighbor {
                    self.found_path = true;
                    break;
                }

                if !self.path.contains_key(&neighbor) {
                    self.q.push_back(neighbor);
                    self.path.insert(neighbor, current);
                    color_tile(&mut vh.grid[neighbor.0][neighbor.1], vh.zoom_level, &mut self.color_scale);
                }      
            }
        
        } else {
            return true;
        }
        return false;
    }
}





#[derive(Debug, Eq, PartialEq)]
struct Node {
    position: Coordinate,
    f_score: usize,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.f_score.cmp(&other.f_score)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn get_neighbors(vh: &mut draw::VisualHandler, row: usize, col: usize) -> Vec<Coordinate> {
    let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];
    let mut neighbors = Vec::new();
    
    let lower_bound = (grid::NUM_TILES - vh.zoom_level) / 2;
    let upper_bound = lower_bound + vh.zoom_level;

    for &(dr, dc) in &directions {
        let (nr, nc) = (row as i32 + dr, col as i32 + dc);
        if lower_bound as i32 <= nr && nr < upper_bound as i32 && lower_bound as i32<= nc && nc < upper_bound as i32 {
            if vh.grid[nr as usize][nc as usize].color == WHITE || vh.grid[nr as usize][nc as usize].color == BLUE {
                neighbors.push((nr as usize, nc as usize));
            }
        }
    }
    neighbors
}



pub struct ASTARState {
    node_candidates: BinaryHeap<Reverse<Node>>,
    g_score: HashMap<Coordinate, usize>,
    f_score: HashMap<Coordinate, usize>,
    path: HashMap<Coordinate, Coordinate>,
    found_path: bool,
    current_path_tile: Coordinate,
    color_scale: f32,
}

impl ASTARState {
    pub fn new(start_flag: (usize, usize), end_flag: (usize, usize)) -> Self {
        let mut node_candidates = BinaryHeap::new();
        node_candidates.push(Reverse(Node {position: start_flag, f_score: heuristic(start_flag, end_flag)}));
        
        let mut g_score = HashMap::new();
        g_score.insert(start_flag, 0);

        let mut f_score = HashMap::new();
        f_score.insert(start_flag, heuristic(start_flag, end_flag));
        
        let path = HashMap::new();

        ASTARState {
            node_candidates,
            g_score,
            f_score,
            path,
            found_path: false,
            current_path_tile: (0, 0),
            color_scale: 0.2,
        }
    }

    pub async fn step(&mut self, end_flag: Coordinate, vh: &mut draw::VisualHandler) -> bool{
        if self.found_path {
            return reconstruct_path(&self.path, &mut self.current_path_tile, vh);
        }
        
        
        if let Some(Reverse(Node {position: current, ..})) = self.node_candidates.pop() {
            for neighbor in get_neighbors(vh, current.0, current.1) {
                let neighbor_g_score = self.g_score[&current] + 1;

                if !self.g_score.contains_key(&neighbor) || neighbor_g_score < self.g_score[&neighbor] {
                    self.path.insert(neighbor, current);
                    self.g_score.insert(neighbor, neighbor_g_score);
                    self.f_score.insert(neighbor, neighbor_g_score + heuristic(neighbor, end_flag));
                    
                    self.node_candidates.push(Reverse(Node { position: neighbor, f_score: self.f_score[&neighbor] }));
                    
                    if neighbor == end_flag {
                        self.found_path = true;
                        self.current_path_tile = *self.path.get(&end_flag).unwrap();
                        break;
                    }

                    color_tile(&mut vh.grid[neighbor.0][neighbor.1], vh.zoom_level, &mut self.color_scale);
                }
            }
        }
        return false;
    }
}

fn heuristic(a: Coordinate, b: Coordinate) -> usize {
    (a.0 as isize - b.0 as isize).abs() as usize + (a.1 as isize - b.1 as isize).abs() as usize
}

fn color_tile(tile: &mut tile::Tile, zoom_level: usize, color_scale: &mut f32) {
    tile.color = Color::new(
        *color_scale * 2.0,
        0.8 - *color_scale,
        0.3 - *color_scale * 1.5,
        1.0,
    );

    *color_scale += 0.01 * (1.0 / zoom_level as f32);
}

fn reconstruct_path(
    path: &HashMap<Coordinate, Coordinate>,
    current: &mut Coordinate,
    vh: &mut draw::VisualHandler,
) -> bool {
    if let Some(&prev) = path.get(current) {
        vh.grid[current.0][current.1].color = YELLOW;
        *current = prev;
        return false;
    } else {
        return true;
    }
}