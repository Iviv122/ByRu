use rodio::{ChannelCount, Player, SampleRate, buffer::SamplesBuffer};

pub struct SoundPlayer {
    player: Player,
    sound: Vec<u8>,
}
impl SoundPlayer {
    pub fn new() -> SoundPlayer {
        let handle =
            rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio stream");
        let player = rodio::Player::connect_new(&handle.mixer());
        let s: Vec<u8> = vec![];
        Self {
            player: player,
            sound: s,
        }
    }
    pub fn append(&mut self, sounddata : &mut Vec<u8>) -> () {
        self.sound.append(sounddata);
    }
    pub fn play(&self) -> () {
        let data: Vec<f32> = self.sound 
            .iter()
            .map(|&s| (s as f32 - 128.0) / 128.0)
            .collect();

        let channels = ChannelCount::new(1).unwrap();
        let sample_rate = SampleRate::new(15360).unwrap();
        let source = SamplesBuffer::new(channels, sample_rate, data);

        self.player.append(source);
    }
}
