use crate::button;
use crate::constants;
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
        zoom_level: &mut u16,
        grid: &mut [[[f32; 4]; constants::grid::NUM_TILES]; constants::grid::NUM_TILES],
        buttons: &mut [button::Button; constants::buttons::NUM_BUTTONS],
    ) {
        let (_, scroll) = mouse_wheel();
        if scroll > 0.0 && *zoom_level < (constants::grid::NUM_TILES as u16) - 1 {
            *zoom_level += 2;
        } else if scroll < 0.0 && *zoom_level > 10 {
            *zoom_level -= 2;
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            self.holding_leftclick = true;
            handle_button_click(buttons, grid);
        }

        if self.holding_leftclick {
            let (x, y) = mouse_position();
            handle_grid_click(x, y, *zoom_level, grid);
        }

        if is_mouse_button_released(MouseButton::Left) {
            self.holding_leftclick = false;
        }
    }

    pub fn resize(&mut self, buttons: &mut [button::Button; constants::buttons::NUM_BUTTONS]) {
        let mut y = constants::grid::y_pos();
        for button in buttons {
            button.x = constants::buttons::right_buttons_x();
            button.y = y;
            y += constants::buttons::button_distance_y();
        }
    }
}

fn click_inside_box(x: f32, y: f32, x_min: f32, y_min: f32, x_max: f32, y_max: f32) -> bool {
    x >= x_min && x <= x_max && y >= y_min && y <= y_max
}

pub fn handle_grid_click(
    x: f32,
    y: f32,
    zoom_level: u16,
    grid: &mut [[[f32; 4]; constants::grid::NUM_TILES]; constants::grid::NUM_TILES],
) {
    let center = constants::grid::NUM_TILES / 2;
    let i = center.saturating_sub(zoom_level as usize / 2);
    let j = (center + zoom_level as usize / 2).min(constants::grid::NUM_TILES);

    for row in i..j {
        for col in i..j {
            let grid_x = grid[row][col][0];
            let grid_y = grid[row][col][1];
            let grid_size = grid[row][col][2];

            let topleft = (grid_x, grid_y);
            let bottomright = (grid_x + grid_size, grid_y + grid_size);

            if click_inside_box(x, y, topleft.0, topleft.1, bottomright.0, bottomright.1) {
                grid[row][col][3] = 1.0;
            }
        }
    }
}

pub fn handle_button_click(
    buttons: &mut [button::Button; constants::buttons::NUM_BUTTONS],
    grid: &mut [[[f32; 4]; constants::grid::NUM_TILES]; constants::grid::NUM_TILES],
) {
    for button in buttons {
        if button.hovered() {
            match button.text.as_str() {
                "CLEAR" => clear_grid(grid),
                "PLACE WALL" => {},
                "START FLAG" => {},
                "END FLAGE" => {},
                _ => {}
            }
        }
    }
}

pub fn clear_grid(grid: &mut [[[f32; 4]; constants::grid::NUM_TILES]; constants::grid::NUM_TILES]) {
    for row in 0..constants::grid::NUM_TILES {
        for col in 0..constants::grid::NUM_TILES {
            grid[row][col][3] = 0.0;
        }
    }
}
