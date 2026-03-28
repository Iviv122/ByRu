use rodio::{ChannelCount, Player, SampleRate, Source, buffer::SamplesBuffer};

pub struct SoundPlayer {
    handle: rodio::MixerDeviceSink,
    player: Player,
    sound: Vec<u8>,
    samplerate: u32,
}
impl SoundPlayer {
    pub fn new(sample_rate: u32) -> SoundPlayer {
        let handle =
            rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio stream");
        let player = rodio::Player::connect_new(&handle.mixer());
        let s: Vec<u8> = vec![];
        Self {
            handle,
            player: player,
            sound: s,
            samplerate: sample_rate,
        }
    }
    pub fn append(&mut self, sounddata: &mut Vec<u8>) -> () {
        self.sound.append(sounddata);
    }
    pub fn play(&mut self) -> () {
        let data: Vec<f32> = self
            .sound
            .iter()
            .map(|&s| (s as f32 - 128.0) / 128.0 / 2.)
            .collect();
        self.sound.clear();
        let channels = ChannelCount::new(1).unwrap();
        let sample_rate = SampleRate::new(256 * 60).unwrap();
        let source = SamplesBuffer::new(channels, sample_rate, data);

        self.player.append(source);
    }
}
