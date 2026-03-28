use std::{panic::PanicHookInfo, thread::sleep, time::Duration, vec};

use minifb::{Key, Window, WindowOptions};

const MEMORY_SIZE: usize = 0x1000008;

const WIDTH: usize = 256;
const HEIGHT: usize = 256;
const VIDBUFFSIZE : usize = 256*256;

const RAMETIME: Duration = Duration::from_micros(16_667);

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
    // big indian
    let ret: u32 = ((bytes[0] as u32) << 16) | ((bytes[1] as u32) << 8) | (bytes[2] as u32);
    return ret;
}

fn update(mem: &mut Vec<u8>) -> () {
    let mut pc: usize = slice_to_u32(&mem[2..2 + INTSIZE]) as usize;

    for _ in 1..=65536 {
        let adr_a: usize = slice_to_u32(&mem[pc..pc + INTSIZE]) as usize;
        let adr_b: usize = slice_to_u32(&mem[pc + INTSIZE..pc + INTSIZE * 2]) as usize;
        let adr_c: usize = slice_to_u32(&mem[pc + INTSIZE * 2..pc + INTSIZE * 3]) as usize;

        mem[adr_a] = mem[adr_b];
        pc = adr_c;
    }
}

fn draw(mem: &mut Vec<u8>,vid: &mut Vec<u32>,colors: &Vec<u32>) -> () {
    let pixels_addr: usize = ((mem[5] as u32) << 24) as usize;
    
    for i in 0..=VIDBUFFSIZE-1{
        let y = i / WIDTH;
        let x = i % WIDTH;
        let color_index = slice_to_u32(&mem[pixels_addr+i..pixels_addr+i+INTSIZE]) as usize;
        vid[WIDTH*y+x] = colors[color_index];
    }
}
fn main() {
    let mut memory: Vec<u8> = vec![0; MEMORY_SIZE];
    let mut video_buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
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
    while window.is_open() && !window.is_key_down(Key::Escape) {
        sleep(RAMETIME);
        update(&mut memory);
        draw(&mut memory,&mut video_buffer,&color_map);
        window
            .update_with_buffer(&video_buffer, WIDTH, HEIGHT)
            .unwrap();
    }
}
