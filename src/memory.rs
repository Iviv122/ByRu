pub struct Memory {
    pub mem: Vec<u8>,
    pub video_buffer: Vec<u32>,
    pub color_map: Vec<u32>,
    intsize: usize,
}

impl Memory {
    pub fn new(
        mem_size: usize,
        vid_size: usize,
        color_size: usize,
        color_step: usize,
        intsize: usize,
    ) -> Self {
        let mem: Vec<u8> = vec![0; mem_size];
        let video_buffer: Vec<u32> = vec![0; vid_size];
        let mut color_map: Vec<u32> = vec![0; color_size];

        Memory::init_color_map(&mut color_map, color_step);

        Self {
            mem: mem,
            video_buffer: video_buffer,
            color_map: color_map,
            intsize: intsize,
        }
    }

    pub fn load(&mut self,file: Vec<u8>) -> (){

        let len = file.len().min(self.mem.len());
        self.mem[..len].copy_from_slice(&file[..len]);
    }

    fn init_color_map(map: &mut Vec<u32>, color_step: usize) -> () {
        let mut index = 0;
        for i in 0..=5 {
            for j in 0..=5 {
                for k in 0..=5 {
                    map[index] =
                        ((i * color_step) << 16 | (j * color_step) << 8 | (k * color_step)) as u32;
                    index += 1;
                }
            }
        }
    }

    pub fn update(&mut self) -> () {
        let mut offset = slice_to_u32(&self.mem[2..2 + self.intsize]) as usize;

        for _ in 1..=65536 {
            let src = slice_to_u32(&self.mem[offset..offset + self.intsize]) as usize;
            offset += self.intsize;
            let dst = slice_to_u32(&self.mem[offset..offset + self.intsize]) as usize;
            offset += self.intsize;
            offset = slice_to_u32(&self.mem[offset..offset + self.intsize]) as usize;
            self.mem[dst] = self.mem[src];
        }
    }
}

fn slice_to_u32(bytes: &[u8]) -> u32 {
    let ret: u32 = ((bytes[0] as u32) << 16) | ((bytes[1] as u32) << 8) | (bytes[2] as u32);
    return ret;
}
