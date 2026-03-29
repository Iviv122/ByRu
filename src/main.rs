use std::{env::args, fs, process::exit};

use crate::{display::Display, memory::Memory, sound::SoundPlayer};

pub mod display;
pub mod memory;
pub mod sound;

const MEMORY_SIZE: usize = 0x1000008;

const WIDTH: usize = 256;
const HEIGHT: usize = 256;
const VIDBUFFSIZE: usize = WIDTH * HEIGHT;
const AUDIOBUFFSIZE: usize = 256;

const COLORAMOUNT: usize = 256;
const COLORSTEP: usize = 0x33;

const INTSIZE: usize = 3; // 3 bytes for int

const FPS: usize = 60;

fn file_name() -> String {
    args()
        .nth(1)
        .ok_or_else(|| {
            println!("No file provided");
            exit(1);
        })
        .unwrap()
}

fn play(mem: &mut Memory, player: &mut SoundPlayer) {
    let sound_addr = ((mem.mem[6] as usize) << 16) | ((mem.mem[7] as usize) << 8);
    let mut v = mem.mem[sound_addr..sound_addr + AUDIOBUFFSIZE].to_vec();
    player.play(&mut v);
}

fn main() {
    let mut memory = Memory::new(MEMORY_SIZE, VIDBUFFSIZE, COLORAMOUNT, COLORSTEP, INTSIZE);
    let file = fs::read(file_name()).unwrap();
    memory.load(file);

    let mut player = SoundPlayer::new((AUDIOBUFFSIZE) as u32, FPS);
    let mut display = Display::new(WIDTH, HEIGHT, FPS);

    while display.window.is_open() && !display.window.is_key_down(minifb::Key::Escape) {
        memory.update();
        display.draw(&mut memory);
        play(&mut memory, &mut player);
    }
}
