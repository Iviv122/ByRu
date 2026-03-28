use minifb::{Key, Window, WindowOptions};
use std::{env::args, fs, process::exit, vec};

use crate::sound::SoundPlayer;

pub mod sound;

const MEMORY_SIZE: usize = 0x1000008;

const WIDTH: usize = 256;
const HEIGHT: usize = 256;
const VIDBUFFSIZE: usize = 256 * 256;
const AUDIOBUFFSIZE: usize = 256;

const COLORAMOUNT: usize = 256;
const COLORSTEP: usize = 0x33;

const INTSIZE: usize = 3; // 3 bytes for int

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

fn slice_to_u32(bytes: &[u8]) -> u32 {
    let ret: u32 = ((bytes[0] as u32) << 16) | ((bytes[1] as u32) << 8) | (bytes[2] as u32);
    return ret;
}

fn update(mem: &mut Vec<u8>) -> () {
    let mut offset = slice_to_u32(&mem[2..2 + INTSIZE]) as usize;

    for _ in 1..=65536 {
        let src = slice_to_u32(&mem[offset..offset + INTSIZE]) as usize;
        offset += INTSIZE;
        let dst = slice_to_u32(&mem[offset..offset + INTSIZE]) as usize;
        offset += INTSIZE;
        offset = slice_to_u32(&mem[offset..offset + INTSIZE]) as usize;
        mem[dst] = mem[src];
    }
}
fn play(mem: &mut Vec<u8>, player: &mut SoundPlayer) {
    let sound_addr = ((mem[6] as usize) << 16) | ((mem[7] as usize) << 8);
    let mut v = mem[sound_addr..sound_addr + AUDIOBUFFSIZE].to_vec();
    player.play(&mut v);
}

fn draw(mem: &mut Vec<u8>, vid: &mut Vec<u32>, colors: &Vec<u32>) -> () {
    let pixels_addr: usize = (mem[5] as usize) << 16; // 2 bytes

    for i in 0..=VIDBUFFSIZE - 1 {
        let y = i / WIDTH;
        let x = i % WIDTH;
        let color_index = mem[pixels_addr + i] as usize;
        vid[WIDTH * y + x] = colors[color_index];
    }
}

fn file_name() -> String {
    args()
        .nth(1)
        .ok_or_else(|| {
            println!("No file provided");
            exit(1);
        })
        .unwrap()
}

fn main() {
    let mut memory: Vec<u8> = vec![0; MEMORY_SIZE];
    let file = fs::read(file_name()).unwrap();
    let len = file.len().min(MEMORY_SIZE);
    memory[..len].copy_from_slice(&file[..len]);

    let mut player = SoundPlayer::new((AUDIOBUFFSIZE) as u32);
    let mut video_buffer: Vec<u32> = vec![0; VIDBUFFSIZE];
    let mut color_map: Vec<u32> = vec![0; COLORAMOUNT];

    init_color_map(&mut color_map);

    let mut window = Window::new(
        "BytePusher - ESC to exit",
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
    window.set_target_fps(60);
    while window.is_open() && !window.is_key_down(Key::Escape) {
        update(&mut memory);
        draw(&mut memory, &mut video_buffer, &color_map);
        play(&mut memory, &mut player);
        window
            .update_with_buffer(&video_buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
