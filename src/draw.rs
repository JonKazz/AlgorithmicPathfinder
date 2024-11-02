use crate::button;
use crate::constants;
use constants::grid;
use macroquad::prelude::*;

pub fn adjust_grid(zoom_level: u16, grid: &mut [[[f32; 4]; grid::NUM_TILES]; grid::NUM_TILES]) {
    let tile_size = grid::size() / zoom_level as f32;

    let center = grid::NUM_TILES / 2;
    let i = center.saturating_sub(zoom_level as usize / 2);
    let j = (center + zoom_level as usize / 2).min(grid::NUM_TILES);

    let mut y_pos = grid::y_pos() as f32;
    for row in i..j {
        let mut x_pos: f32 = grid::x_pos() as f32;
        for col in i..j {
            grid[row][col][0] = x_pos;
            grid[row][col][1] = y_pos;
            grid[row][col][2] = tile_size;
            x_pos += tile_size;
        }
        y_pos += tile_size;
    }
}

pub fn draw_objects(
    zoom_level: u16,
    grid: [[[f32; 4]; grid::NUM_TILES]; grid::NUM_TILES],
    buttons: &[button::Button],
    mode: Color,
) {
    draw_grid(zoom_level, grid, mode);
    draw_buttons(buttons, mode);
}

pub fn draw_grid(
    zoom_level: u16,
    grid: [[[f32; 4]; grid::NUM_TILES]; grid::NUM_TILES],
    mode: Color,
) {
    let center = grid::NUM_TILES / 2;
    let i = center.saturating_sub(zoom_level as usize / 2);
    let j = (center + zoom_level as usize / 2).min(grid::NUM_TILES);

    for row in i..j {
        for col in i..j {
            let x = grid[row][col][0];
            let y = grid[row][col][1];
            let size = grid[row][col][2];

            let mut color = WHITE;
            if tile_hovered(x, y, size) {
                color = LIGHTGRAY;
            } else if grid[row][col][3] == 1.0 {
                color = BLACK;
            }

            draw_rectangle(x, y, size, size, color);
            draw_rectangle_lines(x, y, size, size, grid::TILE_THICKNESS, BLACK);
        }
    }
    draw_rectangle_lines(
        grid::x_pos(),
        grid::y_pos(),
        grid::size(),
        grid::size(),
        grid::border_thickness(),
        mode,
    );
}

pub fn tile_hovered(tile_x: f32, tile_y: f32, size: f32) -> bool {
    let (x, y) = mouse_position();
    x >= tile_x && x <= tile_x + size && y >= tile_y && y <= tile_y + size
}

pub fn draw_buttons(buttons: &[button::Button], mode: Color) {
    for button in buttons {
        button.draw(mode);
    }
}
