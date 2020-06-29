use piston_window;
use piston_window::*;
use std::time::SystemTime;
use rand::{Rng, SeedableRng, rngs::StdRng};

mod draw;

// create map with basic rng
fn gen_map(width: u32, height: u32, rng: &mut StdRng) -> Vec<Vec<u8>>{
    let mut map = vec![vec![0u8; width as usize]; height as usize];
    for y in 0..height {
        for x in 0..width {

            // let gen_val = rng.gen_range(0,100);
            // if gen_val < 40 {
            //     map[y as usize][x as usize] = 1;
            // }else if gen_val < 60 {
            //     map[y as usize][x as usize] = 2;
            // }else {
            //     map[y as usize][x as usize] = 0;
            // }
            match rng.gen_bool(0.4){
                true => map[y as usize][x as usize] = 1,
                _ => map[y as usize][x as usize] = 0
            }
        }
    }
    map
}

// get neighbors of coord in a 3x3
fn get_wall_count(grid_x: i32, grid_y: i32, map: &Vec<Vec<u8>>) -> u32 {
    let width = map[0].len();
    let height = map.len();
    let mut wall_count = 0;

    for y in grid_y-1..grid_y+2 {
        for x in grid_x-1..grid_x+2 {
            if x == grid_x && y == grid_y {
                continue;
            }
            if x < width as i32 && x >= 0 && y >= 0 && y < height as i32 {
                wall_count += map[y as usize][x as usize] as u32;
            }else{
                wall_count += 1
            }
        }
    }

    wall_count
}

// deep clone our 2d vec
fn deep_clone_2d(map: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let width = map[0].len();
    let height = map.len();
    let mut clone = vec![vec![0u8; width as usize]; height as usize];
    for (y, row) in map.iter().enumerate() {
        for (x,col) in row.iter().enumerate() {
            clone[y][x] = *col as u8;
        }
    }

    clone
}

// Smooth out the map based off rules provided
fn smooth_map(map: &mut Vec<Vec<u8>>) {
    let map_clone = deep_clone_2d(&map);

    for (y, row) in map.iter_mut().enumerate() {
        for (x, col) in row.iter_mut().enumerate() {
            let count = get_wall_count(x as i32, y as i32, &map_clone);
            if count > 4{
                *col = 1;
            }else if count < 4{
                *col = 0;
            }
        }
    }
}

fn main() {
    let (width, height) = (30,30);

    // set seed
    let seed;
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH){
        Ok(n) => seed = n.as_secs() as u64,
        Err(_) => panic!("SystemTime before unix epoch"),
    }
    let mut r = StdRng::seed_from_u64(seed);

    let mut map = gen_map(width, height, &mut r);
    
    // multiple iterations of smoothing
    // kinda like erosion lol
    for _ in 0..2 {
        smooth_map(&mut map);
    }

    // for y in &map {
    //     for x in y {
    //         print!("{}", x);
    //     }
    //     print!("\n")
    // }

    let mut window: PistonWindow =
        WindowSettings::new("test", [draw::to_coord_u32(width), draw::to_coord_u32(height)])
            .exit_on_esc(true)
            .build()
            .unwrap();
    while let Some(event) = window.next(){
        window.draw_2d(&event, |c, g, _|{
            clear([0.0,0.0,0.0,1.0], g);

            for y in 0..height {
                for x in 0..width {
                    match &map[y as usize][x as usize]{
                        1 => draw::draw_block(x as i32, y as i32, draw::BACKGROUND_WALL, &c, g),
                        2 => draw::draw_block(x as i32, y as i32, [1.0,0.0,0.0,1.0], &c, g),
                        _ => draw::draw_block(x as i32, y as i32, draw::WALL, &c, g)
                        // _ => continue
                    }
                }
            }
        });
    }
}
