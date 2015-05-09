use std::sync::Arc;
use std::sync::mpsc::Receiver;
use sdl2::audio::{AudioDevice, AudioCallback, AudioSpecDesired};
use resampler::{Resampler, Async};

/// Reader struct used to feed the samples to the SDL callback
struct Reader {
    resampler: Resampler<Sample>,
}

impl Reader {
    pub fn new<'n>(resampler: Resampler<Sample>) -> Reader {
        Reader {
            resampler: resampler,
        }
    }
}

impl AudioCallback for Reader {
    type Channel = Sample;

    fn callback(&mut self, buf: &mut [Sample]) {
        self.resampler.fill_buf(buf);
    }
}

pub struct Audio {
    dev:   AudioDevice<Reader>,
    async: Arc<Async<Sample>>
}

impl Audio {
    pub fn new(channel: Receiver<::spu::SampleBuffer>) -> Audio {

        let resampler = Resampler::new(channel, SAMPLE_RATE);

        let async = resampler.async();

        let reader = Reader::new(resampler);

        let spec = AudioSpecDesired {
            freq:     Some(SAMPLE_RATE as i32),
            channels: Some(1),
            samples:  Some(::spu::SAMPLES_PER_BUFFER as u16),
        };

        let dev = 
            match AudioDevice::open_playback(None,
                                             spec,
                                             |_| reader) {
                Ok(d)  => d,
                Err(e) => panic!("{}", e),
            };

        Audio {
            dev:   dev,
            async: async,
        }
    }

    pub fn start(&self) {
        self.dev.resume();
    }
}

impl ::ui::Audio for Audio {
    fn adjust_resampling(&mut self, in_samples: u32) {
        self.async.adjust_resampling(in_samples);
    }
}

// Use 8bit sound samples
type Sample = u8;

/// Audio output sample rate
const SAMPLE_RATE: u32 = 44_100;
