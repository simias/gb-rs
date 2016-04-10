//! Game Boy sound emulation

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
    /// True if the sound circuit is enabled
    enabled:  bool,
    /// Counter for the SAMPLER_DIVIDER
    divider:  u32,
    /// Current sample buffer
    buffer:   [i16; 1024],
    /// Position in the sample buffer
    position: usize,
    /// Sound 1, rectangular wave with envelope function and
    /// frequency sweep
    sound1:   RectangleWave,
    /// Sound 2, same as sound 1 without frequency sweep
    sound2:   RectangleWave,
    /// Sound 3 plays samples stored in RAM
    sound3:   RamWave,
    /// Sound 4, LFSR based noise generator with envelope function
    sound4:   LfsrWave,
    /// Sound Output 1
    so1:      SoundOutput,
    /// Sound Output 2
    so2:      SoundOutput,
}

impl Spu {
    pub fn new() -> Spu {

        Spu {
            enabled:  false,
            divider:  0,
            buffer:   [0; 1024],
            position: 0,
            sound1:   RectangleWave::new(),
            sound2:   RectangleWave::new(),
            sound3:   RamWave::new(),
            sound4:   LfsrWave::new(),
            so1:      SoundOutput::new(),
            so2:      SoundOutput::new(),
        }
    }

    pub fn step(&mut self) {

        if !self.enabled {
            return
        }

        self.sound1.step();
        self.sound2.step();
        self.sound3.step();
        self.sound4.step();

        if self.divider == 0 {
            self.divider = SAMPLER_DIVIDER;

            self.sample();
        }

        self.divider -= 1;
    }

    fn sample(&mut self) {
        let sounds =
                [self.sound1.sample(),
                 self.sound2.sample(),
                 self.sound3.sample(),
                 self.sound4.sample()];

        let left = self.so1.sample(sounds);
        let right = self.so2.sample(sounds);

        self.output_sample(left, right);
    }

    /// Handle sample buffering and sending them through the
    /// asynchronous channel.
    fn output_sample(&mut self, l: Sample, r: Sample) {

        if self.buffer.len() - self.position < 2 {
            // Buffer filled, send it over and reset the position

            ::libretro::send_audio_samples(&self.buffer);

            self.position = 0;
        }

        let l = l as i16;
        let r = r as i16;

        self.buffer[self.position] = l << 7;
        self.buffer[self.position + 1] = r << 7;

        self.position += 2;
    }

    /// Retreive sound 1 sweep function
    pub fn nr10(&self) -> u8 {
        let sweep = self.sound1.sweep();

        sweep.into_reg()
    }

    /// Configure sound 1 sweep function
    pub fn set_nr10(&mut self, val: u8) {
        if !self.enabled {
            return;
        }

        let sweep = Sweep::from_reg(val);

        self.sound1.set_sweep(sweep);
    }

    /// Retreive sound 1 duty cycle. The rest is write only.
    pub fn nr11(&self) -> u8 {
        let duty = self.sound1.duty().into_field();

        duty << 6 | 0x3f
    }

    /// Configure sound 1 length and duty cycle
    pub fn set_nr11(&mut self, val: u8) {
        if !self.enabled {
            return;
        }

        let duty = DutyCycle::from_field(val >> 6);

        self.sound1.set_duty(duty);

        self.sound1.set_length(val & 0x3f);
    }

    /// Retreive envelope config for sound 1
    pub fn nr12(&self) -> u8 {
        let envelope = self.sound1.envelope();

        envelope.into_reg()
    }

    /// Configure envelope: initial volume, step duration and
    /// direction
    pub fn set_nr12(&mut self, val: u8) {
        if !self.enabled {
            return;
        }

        let envelope = Envelope::from_reg(val);

        self.sound1.set_envelope(envelope);
    }

    // NR13 is write only
    pub fn nr13(&self) -> u8 {
        0xff
    }

    /// Set frequency divider bits [7:0]
    pub fn set_nr13(&mut self, val: u8) {
        if !self.enabled {
            return;
        }

        let mut d = self.sound1.divider();

        // Update the low 8 bits
        d &= 0x700;
        d |= val as u16;

        self.sound1.set_divider(d);
    }

    /// Retreive mode. Other NR14 fields are write only
    pub fn nr14(&self) -> u8 {
        let mode = self.sound1.mode() as u8;

        mode << 6 | 0xbf
    }

    /// Set frequency bits [10:8], Mode and Initialize bit
    pub fn set_nr14(&mut self, val: u8) {
        if !self.enabled {
            return;
        }

        let mut d = self.sound1.divider();

        // Update high 3 bits
        d &= 0xff;
        d |= ((val & 7) as u16) << 8;

        self.sound1.set_divider(d);

        self.sound1.set_mode(
            match val & 0x40 != 0 {
                true  => Mode::Counter,
                false => Mode::Continuous,
            });

        // Initialize bit
        if val & 0x80 != 0 {
            self.sound1.start();
        }
    }

    /// Retreive sound 2 duty cycle. Length field is write only.
    pub fn nr21(&self) -> u8 {
        let duty = self.sound2.duty().into_field();

        duty << 6 | 0x3f
    }

    /// Configure sound 2 sound length and duty cycle
    pub fn set_nr21(&mut self, val: u8) {
        if !self.enabled {
            return;
        }

        let duty = DutyCycle::from_field(val >> 6);

        self.sound2.set_duty(duty);

        self.sound2.set_length(val & 0x3f);
    }

    /// Retreive envelope config for sound 2
    pub fn nr22(&self) -> u8 {
        let envelope = self.sound2.envelope();

        envelope.into_reg()
    }

    /// Configure envelope: initial volume, step duration and
    /// direction
    pub fn set_nr22(&mut self, val: u8) {
        if !self.enabled {
            return;
        }

        let envelope = Envelope::from_reg(val);

        self.sound2.set_envelope(envelope);
    }

    // NR23 is write only
    pub fn nr23(&self) -> u8 {
        0xff
    }

    /// Set frequency divider bits [7:0]
    pub fn set_nr23(&mut self, val: u8) {
        if !self.enabled {
            return;
        }

        let mut d = self.sound2.divider();

        // Update the low 8 bits
        d &= 0x700;
        d |= val as u16;

        self.sound2.set_divider(d);
    }

    /// Retreive mode. Other NR14 fields are write only
    pub fn nr24(&self) -> u8 {
        let mode = self.sound2.mode() as u8;

        mode << 6 | 0xbf
    }

    /// Set frequency bits [10:8], Mode and Initialize bit
    pub fn set_nr24(&mut self, val: u8) {
        if !self.enabled {
            return;
        }

        let mut d = self.sound2.divider();

        // Update high 3 bits
        d &= 0xff;
        d |= ((val & 7) as u16) << 8;

        self.sound2.set_divider(d);

        self.sound2.set_mode(
            match val & 0x40 != 0 {
                true  => Mode::Counter,
                false => Mode::Continuous,
            });

        // Initialize bit
        if val & 0x80 != 0 {
            self.sound2.start();
        }
    }

    /// Retreive sound 3 enable
    pub fn nr30(&self) -> u8 {
        let enabled = self.sound3.enabled() as u8;

        (enabled << 7) | 0x7f
    }

    /// Set sound 3 enable
    pub fn set_nr30(&mut self, val: u8) {
        if !self.enabled {
            return;
        }

        self.sound3.set_enabled(val & 0x80 != 0);
    }

    /// NR32 is write only
    pub fn nr31(&self) -> u8 {
        0xff
    }

    /// Configure sound 3 sound length
    pub fn set_nr31(&mut self, val: u8) {
        if !self.enabled {
            return;
        }

        self.sound3.set_length(val);
    }

    pub fn nr32(&self) -> u8 {
        let level = self.sound3.output_level().into_field();

        (level << 5) | 0x9f
    }

    /// Configure sound 3 output level
    pub fn set_nr32(&mut self, val: u8) {
        if !self.enabled {
            return;
        }

        let level = OutputLevel::from_field((val >> 5) & 3);

        self.sound3.set_output_level(level);
    }

    /// NR33 is write only
    pub fn nr33(&self) -> u8 {
        0xff
    }

    /// Set frequency divider bits [7:0]
    pub fn set_nr33(&mut self, val: u8) {
        if !self.enabled {
            return;
        }

        let mut d = self.sound3.divider();

        // Update the low 8 bits
        d &= 0x700;
        d |= val as u16;

        self.sound3.set_divider(d);
    }

    /// Retreive mode. Other NR34 fields are write only
    pub fn nr34(&self) -> u8 {
        let mode = self.sound3.mode() as u8;

        mode << 6 | 0xbf
    }

    /// Set frequency bits [10:8], Mode and Initialize bit
    pub fn set_nr34(&mut self, val: u8) {
        if !self.enabled {
            return;
        }

        let mut d = self.sound3.divider();

        // Update high 3 bits
        d &= 0xff;
        d |= ((val & 7) as u16) << 8;

        self.sound3.set_divider(d);

        self.sound3.set_mode(
            match val & 0x40 != 0 {
                true  => Mode::Counter,
                false => Mode::Continuous,
            });

        // Initialize bit
        if val & 0x80 != 0 {
            self.sound3.start();
        }
    }

    /// Write to sound 3 sample RAM. There are two 4bit samples per
    /// 8bit register
    pub fn set_nr3_ram(&mut self, index: u8, val: u8) {
        // Should I disallow writing to the RAM when the SPU is disabled?

        let index = index * 2;

        let s0 = (val >> 4)  as Sample;
        let s1 = (val & 0xf) as Sample;

        self.sound3.set_ram_sample(index,     s0);
        self.sound3.set_ram_sample(index + 1, s1);
    }

    /// Retreive sound 3 sample RAM register value
    pub fn nr3_ram(&self, index: u8) -> u8 {
        let index = index * 2;
        let s0    = self.sound3.ram_sample(index)     as u8;
        let s1    = self.sound3.ram_sample(index + 1) as u8;

        s0 << 4 | s1
    }

    /// NR41 is write only
    pub fn nr41(&self) -> u8 {
        0xff
    }

    /// Configure sound 4 sound length
    pub fn set_nr41(&mut self, val: u8) {
        if !self.enabled {
            return;
        }

        self.sound4.set_length(val & 0x3f);
    }

    /// Retreive envelope config for sound 1
    pub fn nr42(&self) -> u8 {
        let envelope = self.sound4.envelope();

        envelope.into_reg()
    }

    /// Configure envelope: initial volume, step duration and
    /// direction
    pub fn set_nr42(&mut self, val: u8) {
        if !self.enabled {
            return;
        }

        let envelope = Envelope::from_reg(val);

        self.sound4.set_envelope(envelope);
    }

    pub fn nr43(&self) -> u8 {
        let lfsr = self.sound4.lfsr();

        lfsr.into_reg()
    }

    pub fn set_nr43(&mut self, val: u8) {
        if !self.enabled {
            return;
        }

        let lfsr = Lfsr::from_reg(val);

        self.sound4.set_lfsr(lfsr);
    }

    /// Retreive mode. Other NR34 fields are write only
    pub fn nr44(&self) -> u8 {
        let mode = self.sound4.mode() as u8;

        mode << 6 | 0xbf
    }

    /// Set frequency bits [10:8], Mode and Initialize bit
    pub fn set_nr44(&mut self, val: u8) {
        if !self.enabled {
            return;
        }

        self.sound4.set_mode(
            match val & 0x40 != 0 {
                true  => Mode::Counter,
                false => Mode::Continuous,
            });

        // Initialize bit
        if val & 0x80 != 0 {
            self.sound4.start();
        }
    }

    /// Retreive sound output volume register
    pub fn nr50(&self) -> u8 {
        let v1 = self.so1.volume().into_field();
        let v2 = self.so2.volume().into_field();

        (v2 << 4) | v1
    }

    /// Set sound output volume
    pub fn set_nr50(&mut self, val: u8) {
        if !self.enabled {
            return;
        }

        self.so1.set_volume(OutputVolume::from_field(val & 0xf));
        self.so2.set_volume(OutputVolume::from_field(val >> 4));
    }

    /// Retreive sound output mixer register
    pub fn nr51(&self) -> u8 {
        let v1 = self.so1.mixer().into_field();
        let v2 = self.so2.mixer().into_field();

        (v2 << 4) | v1
    }

    /// Set sound output mixers
    pub fn set_nr51(&mut self, val: u8) {
        if !self.enabled {
            return;
        }

        self.so1.set_mixer(Mixer::from_field(val & 0xf));
        self.so2.set_mixer(Mixer::from_field(val >> 4));
    }

    /// Get global sound enable and sound status
    pub fn nr52(&self) -> u8 {
        let enabled = self.enabled          as u8;
        let r1      = self.sound1.running() as u8;
        let r2      = self.sound2.running() as u8;
        let r3      = self.sound3.running() as u8;
        let r4      = self.sound4.running() as u8;

        enabled << 7 | 0x70 | (r4 << 3) | (r3 << 2) | (r2 << 1) | r1
    }

    /// Set SPU enable
    pub fn set_nr52(&mut self, val: u8) {
        self.enabled = val & 0x80 != 0;

        if !self.enabled {
            self.reset()
        }
    }

    /// Reinitialize the entire SPU to default values. The only
    /// exception is the waveform RAM that remains untouched.
    fn reset(&mut self) {
        self.sound1 = RectangleWave::new();
        self.sound2 = RectangleWave::new();
        // Save the RAM contents
        self.sound3 = RamWave::with_ram(&self.sound3);
        self.sound4 = LfsrWave::new();

        self.so1 = SoundOutput::new();
        self.so2 = SoundOutput::new();
    }
}

/// Sound can be continuous or stop based on a counter
#[derive(Clone,Copy,PartialEq,Eq)]
pub enum Mode {
    Continuous = 0,
    Counter    = 1,
}

/// The Game Boy has two sound outputs: SO0 and SO1
struct SoundOutput {
    /// Sound mixer for this output
    mixer:  Mixer,
    volume: OutputVolume,
}

impl SoundOutput {
    fn new() -> SoundOutput {
        SoundOutput {
            mixer:  Mixer::from_field(0),
            volume: OutputVolume::from_field(0),
        }
    }

    fn sample(&self, sounds: [Sample; 4]) -> Sample {
        let mixed = self.mixer.mix(sounds);

        self.volume.process(mixed)
    }

    fn mixer(&self) -> Mixer {
        self.mixer
    }

    fn set_mixer(&mut self, mixer: Mixer) {
        self.mixer = mixer;
    }

    fn volume(&self) -> OutputVolume {
        self.volume
    }

    fn set_volume(&mut self, volume: OutputVolume) {
        self.volume = volume;
    }
}

/// Each of the 4 sounds can be enabled or disabled
/// independantly for each sound output.
#[derive(Clone,Copy)]
struct Mixer {
    /// The mixer config, says which of the 4 sounds are
    /// selected.
    sounds: [bool; 4],
}

impl Mixer {
    /// Build a mixer from the NR51 4bit field
    fn from_field(field: u8) -> Mixer {

        let mut mixer = Mixer { sounds: [false; 4] };

        for i in 0..4 {
            mixer.sounds[i] = (field & (1 << i)) != 0;
        }

        mixer
    }

    fn into_field(self) -> u8 {
        let mut f = 0;

        for i in 0..self.sounds.len() {
            f |= (self.sounds[i] as u8) << i
        }

        f
    }

    fn mix(self, sounds: [Sample; 4]) -> Sample {
        let mut r = 0;

        for i in 0..4 {
            if self.sounds[i] {
                // Sound is enabled for this output
                r += sounds[i];
            }
        }

        r
    }
}

/// Sound output volume configuration
#[derive(Clone,Copy)]
struct OutputVolume {
    /// Not used for now
    vin:  bool,
    /// Sound volume divider, between 1 (full volume) and 8(min)
    level: u8,
}

impl OutputVolume {
    /// Build an OutputVolume from a N50 field
    fn from_field(field: u8) -> OutputVolume {
        // TODO: Handle bit 4. Seems to be related to the microphone
        // input? Maybe GameBoy Color only?

        OutputVolume {
            vin:   field & 8 != 0,
            level: 8 - (field & 7),
        }
    }

    fn into_field(self) -> u8 {
        ((self.vin as u8) << 3) | (8 - self.level)
    }

    fn process(self, s: Sample) -> Sample {
        s / self.level as Sample
    }
}

/// Each sound uses a 4bit DAC which means it they can only output
/// 16 sound levels each. There are 4 channels in total which means
/// that the sum is in the range [0, 60], so a u8 is plenty enough.
pub type Sample = u8;

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

/// Maximum possible volume for a single sound
const SOUND_MAX: Sample = 15;

#[cfg(test)]
mod tests {

    mod readback {
        //! Test SPU register readback. Test ported from
        //! 01-registers.s in the GB Accuracy Tests

        use spu::Spu;

        /// Test register readback value. Write-only fields are
        /// supposed to read as 1s.
        macro_rules! readback_test {
            ($reg: ident, $setter: ident, $write_only: expr) => (
                #[test]
                fn $reg() {
                    let (mut spu, _) = Spu::new();

                    // Enable SPU
                    spu.set_nr52(0x80);

                    for v in 0u16..0x100 {
                        let v = v as u8;

                        let expected = v | $write_only;

                        spu.$setter(v);
                        let r = spu.$reg();

                        assert!(r == expected);
                    }
                })
        }

        readback_test!{nr10, set_nr10, 0x80}
        readback_test!{nr11, set_nr11, 0x3f}
        readback_test!{nr12, set_nr12, 0x00}
        readback_test!{nr13, set_nr13, 0xff}
        readback_test!{nr14, set_nr14, 0xbf}

        readback_test!{nr21, set_nr21, 0x3f}
        readback_test!{nr22, set_nr22, 0x00}
        readback_test!{nr23, set_nr23, 0xff}
        readback_test!{nr24, set_nr24, 0xbf}

        readback_test!{nr30, set_nr30, 0x7f}
        readback_test!{nr31, set_nr31, 0xff}
        readback_test!{nr32, set_nr32, 0x9f}
        readback_test!{nr33, set_nr33, 0xff}
        readback_test!{nr34, set_nr34, 0xbf}

        readback_test!{nr41, set_nr41, 0xff}
        readback_test!{nr42, set_nr42, 0x00}
        readback_test!{nr43, set_nr43, 0x00}
        readback_test!{nr44, set_nr44, 0xbf}

        readback_test!{nr50, set_nr50, 0x00}
        readback_test!{nr51, set_nr51, 0x00}

        #[test]
        fn wave_ram() {
            let (mut spu, _) = Spu::new();

            // Enable SPU
            spu.set_nr52(0x80);

            for v in 0u16..0x100 {
                let v = v as u8;

                for i in 0u8..16 {
                    spu.set_nr3_ram(i, v);
                    let r = spu.nr3_ram(i);

                    assert!(r == v);
                }
            }
        }

        #[test]
        fn nr52() {
            let (mut spu, _) = Spu::new();

            spu.set_nr52(0);

            assert!(spu.nr52() == 0x70);

            spu.set_nr52(0xff);

            assert!(spu.nr52() == 0xf0);
        }
    }

    mod reset {
        //! Disabling the SPU (bit 7 of NR52 to 0) should clear all
        //! registers. Furthermore the SPU should ignore register
        //! writes while it's disabled.

        use spu::Spu;

        macro_rules! readback_test {
            ($reg: ident, $setter: ident, $write_only: expr) => (
                #[test]
                fn $reg() {
                    let (mut spu, _) = Spu::new();

                    // Enable SPU
                    spu.set_nr52(0x80);

                    // Write full-1s to the register
                    spu.$setter(0xff);

                    // Disable the SPU
                    spu.set_nr52(0x00);
                    // Write to the register (should be ignored while
                    // the SPU is disabled)
                    spu.$setter(0xff);

                    // Re-enable SPU
                    spu.set_nr52(0x80);

                    // Read register
                    let r = spu.$reg();

                    // All bits except the write-only ones must have
                    // been cleared.
                    assert!(r == $write_only);
                })
        }

        readback_test!{nr10, set_nr10, 0x80}
        readback_test!{nr11, set_nr11, 0x3f}
        readback_test!{nr12, set_nr12, 0x00}
        readback_test!{nr13, set_nr13, 0xff}
        readback_test!{nr14, set_nr14, 0xbf}

        readback_test!{nr21, set_nr21, 0x3f}
        readback_test!{nr22, set_nr22, 0x00}
        readback_test!{nr23, set_nr23, 0xff}
        readback_test!{nr24, set_nr24, 0xbf}

        readback_test!{nr30, set_nr30, 0x7f}
        readback_test!{nr31, set_nr31, 0xff}
        readback_test!{nr32, set_nr32, 0x9f}
        readback_test!{nr33, set_nr33, 0xff}
        readback_test!{nr34, set_nr34, 0xbf}

        readback_test!{nr41, set_nr41, 0xff}
        readback_test!{nr42, set_nr42, 0x00}
        readback_test!{nr43, set_nr43, 0x00}
        readback_test!{nr44, set_nr44, 0xbf}

        readback_test!{nr50, set_nr50, 0x00}
        readback_test!{nr51, set_nr51, 0x00}
    }
}
