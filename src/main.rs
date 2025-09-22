use core::time;
use std::thread;
use rand::Rng;

const WIDTH: usize = 100;
const HEIGHT: usize = 50;

fn print_field(arr: &[[bool; WIDTH]; HEIGHT]) {
    for row in arr {
        for &val in row {
            let char = if val { "▄" } else { " " };
            print!("{char} ");
        }
        println!();
    }
}

fn random_initialize(arr: &mut [[bool; WIDTH]; HEIGHT]) {
    let mut rng = rand::rng();
    for row in arr.iter_mut() {
        for val in row.iter_mut() {
            *val = rng.random_range(0..2) == 1;
        }
    }
}

fn next_generation(frame_a: &[[bool; WIDTH]; HEIGHT], frame_b: &mut [[bool; WIDTH]; HEIGHT]) {
    for (y, row) in frame_a.iter().enumerate() {
        for (x, _) in row.iter().enumerate() {

            let mut neighbours = 0;

            for dy in [-1, 0, 1] {
                for dx in [-1, 0, 1] {
                    if dx == 0 && dy == 0 { continue; }
                    let ny = (y as isize + dy + HEIGHT as isize) % HEIGHT as isize;
                    let nx = (x as isize + dx + WIDTH as isize) % WIDTH as isize;
                    if ny >= 0 && ny < HEIGHT as isize && nx >= 0 && nx < WIDTH as isize {
                        if frame_a[ny as usize][nx as usize] {
                            neighbours += 1;
                        }
                    }
                }
            }

            frame_b[y][x] = match neighbours {
                2 => frame_a[y][x],
                3 => true,
                _ => false,
            };
        }
    }
}


fn main() {
    let mut frame_a: [[bool; WIDTH]; HEIGHT] = [[false; WIDTH]; HEIGHT];
    let mut frame_b: [[bool; WIDTH]; HEIGHT] = [[false; WIDTH]; HEIGHT];

    random_initialize(&mut frame_a);
   
    loop {
        print!("\x1B[2J\x1B[1;1H");
        print_field(&frame_a);
        next_generation(&frame_a, &mut frame_b);
        std::mem::swap(&mut frame_a, &mut frame_b);
        thread::sleep(time::Duration::from_millis(10));
    }
}