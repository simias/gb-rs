//! Game Boy sound 3 generates a sound from samples stored in RAM

use spu::{Sample, Mode};

pub struct RamWave {
    /// True if the sound is generating samples
    running:      bool,
    /// True if the sound is enabled
    enabled:      bool,
    /// Counter for counter mode
    remaining:    u32,
    /// RAM Wave data processing
    output_level: OutputLevel,
    /// Frequency divider value
    divider:      u16,
    /// Period counter, the period length is configurable and is used
    /// to select the desired output frequency.
    counter:      u16,
    /// Play mode (continuous or counter)
    mode:         Mode,
    /// Custom sample RAM, 32 samples
    samples:      [Sample; 32],
    /// Currently played sample
    index:        u8,
}

impl RamWave {
    pub fn new() -> RamWave {
        RamWave {
            running:      false,
            enabled:      false,
            remaining:    0x100 * 0x4000,
            output_level: OutputLevel::from_field(0),
            divider:      0,
            counter:      0,
            mode:         Mode::Continuous,
            samples:      [0; 32],
            index:        0,
        }
    }

    /// Create a new RamWare taking the RAM contents from an old one.
    pub fn with_ram(other: &RamWave) -> RamWave {
        let mut rw = RamWave::new();

        rw.samples = other.samples;

        rw
    }

    pub fn step(&mut self) {
        // Counter runs even if the channel is disabled
        if self.mode == Mode::Counter {
            if self.remaining == 0 {
                self.running = false;
                // Reload counter default value
                self.remaining = 0x100 * 0x4000;
                return;
            }

            self.remaining -= 1;
        }

        if !self.running {
            return;
        }

        if self.counter == 0 {
            // Reset the counter. This weird equation is simply how
            // the hardware does it, no tricks here.
            self.counter = 2 * (0x800 - self.divider);

            // Move on to the next sample
            self.index = (self.index + 1) % self.samples.len() as u8;
        }

        self.counter -= 1;
    }

    pub fn sample(&self) -> Sample {

        if !self.running {
            return 0;
        }

        // Return the active sample value after optionally modifying
        // it depending on the configured OutputLevel
        let sample = self.ram_sample(self.index);

        self.output_level.process(sample)
    }

    pub fn ram_sample(&self, index: u8) -> Sample {
        self.samples[index as usize]
    }

    pub fn set_ram_sample(&mut self, index: u8, s: Sample) {
        self.samples[index as usize] = s;
    }

    pub fn running(&self) -> bool {
        self.running
    }

    pub fn start(&mut self) {
        self.running = self.enabled;
    }

    pub fn divider(&self) -> u16 {
        self.divider
    }

    pub fn set_divider(&mut self, divider: u16) {
        if divider >= 0x800 {
            panic!("divider out of range: {:04x}", divider);
        }

        self.divider = divider;
    }

    pub fn mode(&self) -> Mode {
        self.mode
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }

    pub fn set_length(&mut self, len: u8) {
        let len = len as u32;

        self.remaining = (0x100 - len) * 0x4000;
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;

        if !self.enabled {
            self.running = false;
        }
    }

    pub fn output_level(&self) -> OutputLevel {
        self.output_level
    }

    pub fn set_output_level(&mut self, level: OutputLevel) {
        self.output_level = level;
    }
}

/// The wave data can be didived before being sent out
#[derive(Copy)]
pub enum OutputLevel {
    /// Channel is muted
    Mute      = 0,
    /// Output wave data without division
    Full      = 1,
    /// Output wave data halved
    Halved    = 2,
    /// Output wave data divided by four
    Quartered = 3,
}

impl OutputLevel {
    pub fn from_field(field: u8) -> OutputLevel {
        match field {
            0 => OutputLevel::Mute,
            1 => OutputLevel::Full,
            2 => OutputLevel::Halved,
            3 => OutputLevel::Quartered,
            _ => unreachable!(),
        }
    }

    pub fn into_field(self) -> u8 {
        self as u8
    }

    fn process(self, sample: Sample) -> Sample {
        match self {
            OutputLevel::Mute      => 0,
            OutputLevel::Full      => sample,
            OutputLevel::Halved    => sample / 2,
            OutputLevel::Quartered => sample / 4,
        }
    }
}
