use minifb::{Window, WindowOptions};

use crate::memory::Memory;

pub struct Display {
    width: usize,
    height: usize,

    pub window: Window,
}

impl Display {
    pub fn new(width: usize, height: usize, fps: usize) -> Self {
        let mut window = Window::new(
            "BytePusher - ESC to exit",
            width,
            height,
            WindowOptions::default(),
        )
        .unwrap_or_else(|e| {
            panic!("{}", e);
        });
        window.set_target_fps(fps);
        Self {
            width: width,
            height: height,
            window: window,
        }
    }

    pub fn draw(&mut self, mem: &mut Memory) -> () {
        let pixels_addr: usize = (mem.mem[5] as usize) << 16; // 2 bytes

        for i in 0..=mem.video_buffer.len() - 1 {
            let y = i / self.width;
            let x = i % self.width;
            let color_index = mem.mem[pixels_addr + i] as usize;
            mem.video_buffer[self.width * y + x] = mem.color_map[color_index];
        }
        self.window
            .update_with_buffer(&mem.video_buffer, self.width, self.height)
            .unwrap();
    }
}
