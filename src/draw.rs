use piston_window::{rectangle, Context, G2d};
use piston_window::types::Color;

const BLOCK_SIZE: f64 = 25.0;
pub const WHITE: Color = [1.0,1.0,1.0,1.0];
pub const BLACK: Color = [0.0,0.0,0.0,1.0];

// takes integer coord and multiplies it by the block size to make the upscaled coord location
// after that it returns the value as a 64-bit float
pub fn to_coord(game_coord: i32) -> f64 {
    (game_coord as f64) * BLOCK_SIZE
}

pub fn to_coord_u32(game_coord: u32) -> u32 {
    game_coord*(BLOCK_SIZE as u32)
}


// we reference context because passing ownership kills our context upon exiting the function
// G2d is a referenced mutable reference because we want to borrow it for a bit while making changes
pub fn draw_block(x: i32, y: i32, color: Color, con: &Context, g: &mut G2d) {
    
    // ownership of x & y are pass off to the function that then hands ownership over to these variables
    let gui_x = to_coord(x);
    let gui_y = to_coord(y);

    rectangle(
        color,
        [gui_x, gui_y, BLOCK_SIZE, BLOCK_SIZE],
        con.transform,
        g
    );
}