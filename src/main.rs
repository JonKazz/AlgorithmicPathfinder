use macroquad::prelude::*;
use macroquad::time::get_fps;

mod constants;
mod draw;
mod inputs;

#[macroquad::main("MyGame")]
async fn main() {
    set_fullscreen(true);

    let mut zoom_level: u16 = 30;
    let mut grid = [[[0.0; 4]; constants::grid::NUM_TILES]; constants::grid::NUM_TILES];
    let mut input_handler = inputs::InputHandler::new();

    loop {
        print!("FPS: {}\n", get_fps());
        clear_background(DARKGRAY);

        draw::adjust_grid(zoom_level, &mut grid);
        draw::draw_grid(zoom_level, grid);
        input_handler.handle_inputs(&mut zoom_level, &mut grid);
    
        next_frame().await;
    }
}
