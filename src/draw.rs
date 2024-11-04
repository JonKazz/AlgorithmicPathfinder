use crate::button;
use crate::constants;
use crate::tile;
use constants::grid;
use macroquad::prelude::*;

pub fn zoom_grid(
    zoom_level: u16, 
    grid: &mut [[tile::Tile; grid::NUM_TILES]; grid::NUM_TILES],
) {
    let tile_size = grid::size() / zoom_level as f32;
    let center = grid::NUM_TILES / 2;
    let i = center.saturating_sub(zoom_level as usize / 2);
    let j = (center + zoom_level as usize / 2).min(grid::NUM_TILES);

    let mut y_pos = grid::y_pos() as f32;
    for row in i..j {
        let mut x_pos: f32 = grid::x_pos() as f32;
        for col in i..j {
            grid[row][col].x = x_pos;
            grid[row][col].y = y_pos;
            grid[row][col].size = tile_size;
            x_pos += tile_size;
        }
        y_pos += tile_size;
    }
}

pub fn draw_objects(
    zoom_level: u16,
    grid: [[tile::Tile; grid::NUM_TILES]; grid::NUM_TILES],
    buttons: &[button::Button],
    mode: Color,
) {
    draw_grid(zoom_level, grid, mode);
    draw_buttons(buttons, mode);
}

pub fn draw_grid(
    zoom_level: u16,
    grid: [[tile::Tile; grid::NUM_TILES]; grid::NUM_TILES],
    mode: Color,
) {
    draw_rectangle(
        grid::x_pos() - grid::border_padding(),
        grid::y_pos() - grid::border_padding(),
        grid::size() + grid::border_padding() * 2.0,
        grid::size() + grid::border_padding() * 2.0,
        mode,
    );

    let center = grid::NUM_TILES / 2;
    let i = center.saturating_sub(zoom_level as usize / 2);
    let j = (center + zoom_level as usize / 2).min(grid::NUM_TILES);

    for row in i..j {
        for col in i..j {
            let mut tile = grid[row][col];
            tile.draw();
        }
    }


    draw_line(
        grid::x_pos(),
        grid::y_pos(),
        grid::x_pos() + grid::size(),
        grid::y_pos(),
        2.0,
        BLACK,
    );
    draw_line(
        grid::x_pos(),
        grid::y_pos(),
        grid::x_pos(),
        grid::y_pos() + grid::size(),
        2.0,
        BLACK,
    );
    draw_line(
        grid::x_pos() + grid::size(),
        grid::y_pos(),
        grid::x_pos() + grid::size(),
        grid::y_pos() + grid::size(),
        2.0,
        BLACK,
    );
    draw_line(
        grid::x_pos(),
        grid::y_pos() + grid::size(),
        grid::x_pos() + grid::size(),
        grid::y_pos() + grid::size(),
        2.0,
        BLACK,
    );
}

pub fn draw_buttons(buttons: &[button::Button], mode: Color) {
    for button in buttons {
        button.draw(mode);
    }
}
