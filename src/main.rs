use std::{thread::sleep, time::Duration, vec};

use minifb::{Key, Window, WindowOptions};

const MEMORY_SIZE: usize = 0x1000008;

const WIDTH: usize = 256;
const HEIGHT: usize = 256;

const RAMETIME: Duration = Duration::from_micros(16_667);

const COLORAMOUNT: usize = 256;
const COLORSTEP: usize = 0x33;

fn init_color_map(map: &mut Vec<u32>) -> () {
    let mut index = 0;
    for i in 0..=5 {
        for j in 0..=5 {
            for k in 0..=5 {
                map[index] =
                    ((i * COLORSTEP) << 16 | (j * COLORSTEP) << 8 | (k * COLORSTEP)) as u32;
                index += 1;
            }
        }
    }
}

fn main() {
    let mut memory: Vec<u32> = vec![0; MEMORY_SIZE];
    let mut video_buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut color_map: Vec<u32> = vec![0; COLORAMOUNT];

    init_color_map(&mut color_map);

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut j = 0;
    for color in color_map.iter() {
        for k in 0..255 {
            video_buffer[256 * j + k] = *color;
        }
        j += 1
    }
    while window.is_open() && !window.is_key_down(Key::Escape) {
        sleep(RAMETIME);
        // TODO: Keyboard

        window
            .update_with_buffer(&video_buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
