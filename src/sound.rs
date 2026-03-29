use rodio::{ChannelCount, Player, SampleRate, buffer::SamplesBuffer};

pub struct SoundPlayer {
    _handle: rodio::MixerDeviceSink,
    player: Player,
    samplerate: u32,
    framerate: usize,
}
impl SoundPlayer {
    pub fn new(sample_rate: u32, framerate: usize) -> SoundPlayer {
        let handle =
            rodio::DeviceSinkBuilder::open_default_sink().expect("open default audio stream");
        let player = rodio::Player::connect_new(&handle.mixer());
        Self {
            _handle: handle,
            player: player,
            samplerate: sample_rate,
            framerate: framerate,
        }
    }
    pub fn play(&mut self, sounddata: &mut Vec<u8>) -> () {
        let data: Vec<f32> = sounddata
            .iter()
            .map(|&s| (s as f32 - 128.0) / 128.0)
            .collect();
        let channels = ChannelCount::new(1).unwrap();
        let sample_rate = SampleRate::new(self.samplerate * self.framerate as u32).unwrap();
        let source = SamplesBuffer::new(channels, sample_rate, data);

        self.player.append(source);
    }
}
