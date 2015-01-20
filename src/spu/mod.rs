//! Game Boy sound emulation

use std::sync::mpsc::{sync_channel, Receiver, SyncSender, TrySendError};

use spu::rectangle_wave::{RectangleWave, DutyCycle, Sweep};
use spu::envelope::Envelope;
use spu::lfsr_wave::{LfsrWave, Lfsr};
use spu::ram_wave::{RamWave, OutputLevel};

mod envelope;
mod rectangle_wave;
mod ram_wave;
mod lfsr_wave;

/// Sound Processing Unit state.
pub struct Spu {
    /// Counter for the SAMPLER_DIVIDER
    divider:  u32,
    /// Channel used to send the generated audio samples to the
    /// backend.
    output:   SyncSender<SampleBuffer>,
    /// Current sample buffer
    buffer:  SampleBuffer,
    /// Position in the sample buffer
    position: usize,
    /// Sound channel 1, rectangular wave with envelope function and
    /// frequency sweep
    channel1: RectangleWave,
    /// Channel 2, same as channel 1 without frequency sweep
    channel2: RectangleWave,
    /// Channel 3 plays samples stored in RAM
    channel3: RamWave,
    /// Channel 4, LFSR based noise generator with envelope function
    channel4: LfsrWave,
}

impl Spu {
    pub fn new() -> (Spu, Receiver<SampleBuffer>) {

        let (tx, rx) = sync_channel(CHANNEL_DEPTH);

        let spu = Spu {
            divider:  0,
            output:   tx,
            buffer:  [0; SAMPLES_PER_BUFFER],
            position: 0,
            channel1: RectangleWave::new(),
            channel2: RectangleWave::new(),
            channel3: RamWave::new(),
            channel4: LfsrWave::new(),
        };

        (spu , rx)
    }

    pub fn step(&mut self) {

        self.channel1.step();
        self.channel2.step();
        self.channel3.step();
        self.channel4.step();

        if self.divider == 0 {
            // Generate a sound sample
            let sample =
                self.channel1.sample() +
                self.channel2.sample() +
                self.channel3.sample() +
                self.channel4.sample();

            self.sample(sample);

            // Reset counter
            self.divider = SAMPLER_DIVIDER;
        }

        self.divider -= 1;
    }

    fn sample(&mut self, sample: Sample) {

        self.buffer[self.position] = sample;

        self.position += 1;

        if self.position == self.buffer.len() {
            // Buffer filled, send it over and reset the position
            if let Err(e) = self.output.try_send(self.buffer) {
                match e {
                    TrySendError::Full(_) =>
                        error!("Sound channel is full, dropping {} samples",
                               self.buffer.len()),
                    e => panic!("Couldn't send audio buffer: {:?}", e),
                }
            }

            self.position = 0;
        }
    }

    pub fn set_nr10(&mut self, val: u8) {
        let sweep = Sweep::from_reg(val);

        self.channel1.set_sweep(sweep);
    }

    /// Configure channel 2 sound length and duty cycle
    pub fn set_nr11(&mut self, val: u8) {
        let duty = DutyCycle::from_field(val >> 6);

        self.channel1.set_duty(duty);

        self.channel1.set_length(val & 0x3f);
    }

    /// Configure envelope: initial volume, step duration and
    /// direction
    pub fn set_nr12(&mut self, val: u8) {
        let envelope = Envelope::from_reg(val);

        self.channel1.set_envelope(envelope);
    }

    /// Set frequency divider bits [7:0]
    pub fn set_nr13(&mut self, val: u8) {
        let mut d = self.channel1.divider();

        // Update the low 8 bits
        d &= 0x700;
        d |= val as u16;

        self.channel1.set_divider(d);
    }

    /// Set frequency bits [10:8], Mode and Initialize bit
    pub fn set_nr14(&mut self, val: u8) {
        let mut d = self.channel1.divider();

        // Update high 3 bits
        d &= 0xff;
        d |= ((val & 7) as u16) << 8;

        self.channel1.set_divider(d);

        self.channel1.set_mode(
            match val & 0x40 != 0 {
                true  => Mode::Counter,
                false => Mode::Continuous,
            });

        // Initialize bit
        if val & 0x80 != 0 {
            self.channel1.start();
        }
    }

    /// Configure channel 2 sound length and duty cycle
    pub fn set_nr21(&mut self, val: u8) {
        let duty = DutyCycle::from_field(val >> 6);

        self.channel2.set_duty(duty);

        self.channel2.set_length(val & 0x3f);
    }

    /// Configure envelope: initial volume, step duration and
    /// direction
    pub fn set_nr22(&mut self, val: u8) {
        let envelope = Envelope::from_reg(val);

        self.channel2.set_envelope(envelope);
    }

    /// Set frequency divider bits [7:0]
    pub fn set_nr23(&mut self, val: u8) {
        let mut d = self.channel2.divider();

        // Update the low 8 bits
        d &= 0x700;
        d |= val as u16;

        self.channel2.set_divider(d);
    }

    /// Set frequency bits [10:8], Mode and Initialize bit
    pub fn set_nr24(&mut self, val: u8) {
        let mut d = self.channel2.divider();

        // Update high 3 bits
        d &= 0xff;
        d |= ((val & 7) as u16) << 8;

        self.channel2.set_divider(d);

        self.channel2.set_mode(
            match val & 0x40 != 0 {
                true  => Mode::Counter,
                false => Mode::Continuous,
            });

        // Initialize bit
        if val & 0x80 != 0 {
            self.channel2.start();
        }
    }

    /// Set channel 3 enable
    pub fn set_nr30(&mut self, val: u8) {
        self.channel3.set_enabled(val & 0x80 != 0);
    }

    /// Configure channel 3 sound length
    pub fn set_nr31(&mut self, val: u8) {
        self.channel3.set_length(val);
    }

    /// Configure channel 3 output level
    pub fn set_nr32(&mut self, val: u8) {
        let level = OutputLevel::from_field((val >> 5) & 3);

        self.channel3.set_output_level(level);
    }

    /// Set frequency divider bits [7:0]
    pub fn set_nr33(&mut self, val: u8) {
        let mut d = self.channel3.divider();

        // Update the low 8 bits
        d &= 0x700;
        d |= val as u16;

        self.channel3.set_divider(d);
    }

    /// Set frequency bits [10:8], Mode and Initialize bit
    pub fn set_nr34(&mut self, val: u8) {
        let mut d = self.channel3.divider();

        // Update high 3 bits
        d &= 0xff;
        d |= ((val & 7) as u16) << 8;

        self.channel3.set_divider(d);

        self.channel3.set_mode(
            match val & 0x40 != 0 {
                true  => Mode::Counter,
                false => Mode::Continuous,
            });

        // Initialize bit
        if val & 0x80 != 0 {
            self.channel3.start();
        }
    }

    /// Write to channel 3 sample RAM. There are two 4bit samples per
    /// 8bit register
    pub fn set_nr3_ram(&mut self, index: u8, val: u8) {
        let index = index * 2;

        let s0 = (val >> 4)  as Sample;
        let s1 = (val & 0xf) as Sample;

        self.channel3.set_ram_sample(index,     s0);
        self.channel3.set_ram_sample(index + 1, s1);
    }

    /// Configure channel 4 sound length
    pub fn set_nr41(&mut self, val: u8) {
        self.channel4.set_length(val & 0x3f);
    }

    /// Configure envelope: initial volume, step duration and
    /// direction
    pub fn set_nr42(&mut self, val: u8) {
        let envelope = Envelope::from_reg(val);

        self.channel4.set_envelope(envelope);
    }

    pub fn set_nr43(&mut self, val: u8) {
        let lfsr = Lfsr::from_reg(val);

        self.channel4.set_lfsr(lfsr);
    }

    /// Set frequency bits [10:8], Mode and Initialize bit
    pub fn set_nr44(&mut self, val: u8) {
        self.channel4.set_mode(
            match val & 0x40 != 0 {
                true  => Mode::Counter,
                false => Mode::Continuous,
            });

        // Initialize bit
        if val & 0x80 != 0 {
            self.channel4.start();
        }
    }
}

/// The game boy sound uses 4bit DACs and can therefore only output 16
/// sound levels
#[derive(Copy)]
struct Volume(u8);

impl Volume {
    fn new(vol: u8) -> Volume {
        if vol > CHANNEL_MAX {
            panic!("Volume out of range: {}", vol);
        }

        Volume(vol)
    }

    /// Convert from 4-bit volume value to Sample range
    fn into_sample(self) -> Sample {
        let Volume(v) = self;

        v as Sample
    }

    fn up(&mut self) {
        let Volume(v) = *self;

        // I'm not sure how to handle overflows, let's saturate for
        // now
        if v < CHANNEL_MAX {
            *self = Volume(v + 1);
        }
    }

    fn down(&mut self) {
        let Volume(v) = *self;

        if v > 0 {
            *self = Volume(v - 1);
        }
    }
}

/// Sound can be continuous or stop based on a counter
#[derive(PartialEq,Eq,Show)]
enum Mode {
    Continuous = 0,
    Counter    = 1,
}

/// Return the number of sound samples that are generated during a
/// period of `steps` SysClk ticks.
pub fn samples_per_steps(steps: u32) -> u32 {
    steps / SAMPLER_DIVIDER
}

/// Each channel uses a 4bit DAC which means it they can only output
/// 16 sound levels each. There are 4 channels in total which means
/// that the sum is in the range [0, 60], so a u8 is plenty enough.
pub type Sample = u8;

pub type SampleBuffer = [Sample; SAMPLES_PER_BUFFER];

/// We buffer the sound samples before we send them to the next
/// stage. Bigger buffers will reduce the contention on the channel
/// but it will also increase latency.
pub const SAMPLES_PER_BUFFER: usize = 0x200;

/// This variable says how many SysClk cycles to wait between each
/// sound sample. In other words, SYSCLK_FREQ / SAMPLER_DIVIDER gives
/// the audio sample rate. If this value is too high we won't be able
/// to generate high audio frequencies, if it's too low we'll generate
/// inaudible audio frequencies which will be a waste of RAM and CPU
/// cycles. A value of 95 gives us a sample frequency slightly above
/// 44.1kHz which is a reasonable default. The resulting audio stream
/// will have to be resampled by the audio backend to the target
/// frequency.
const SAMPLER_DIVIDER: u32 = 95;

pub const SAMPLE_RATE: u32 = ::SYSCLK_FREQ as u32 / SAMPLER_DIVIDER;

/// Depth of the channel between the Spu and the audio
/// backend. Ideally the channel should be empty most of the time and
/// no more than one sample should be queued at any moment but this
/// buffer can be used to absorb a momentary slowdown of the
/// backend. If the channel is ever full it'll prevent the SPU from
/// queuing new samples which will will cause the audio samples to be
/// dropped.
const CHANNEL_DEPTH: usize = 4;

/// Maximum possible volume for a single channel
const CHANNEL_MAX:    Sample = 15;

/// Maximum possible value for a sample
pub const SAMPLE_MAX: Sample = CHANNEL_MAX * 4;
