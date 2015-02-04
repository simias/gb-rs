//! Envelope function used by sounds 1, 2 and 4

use spu::{Sample, SOUND_MAX};

#[derive(Copy)]
pub struct Envelope {
    direction:     EnvelopeDirection,
    volume:        Volume,
    step_duration: u32,
    counter:       u32,
}

impl Envelope {
    pub fn from_reg(val: u8) -> Envelope {

        let vol = Volume::from_field(val >> 4);

        let dir =
            match val & 8 != 0 {
                true  => EnvelopeDirection::Up,
                false => EnvelopeDirection::Down,
            };

        let l = (val & 7) as u32;

        Envelope {
            direction:     dir,
            volume:        vol,
            step_duration: l * 0x10000,
            counter:       0,
        }
    }

    pub fn into_reg(&self) -> u8 {
        let vol = self.volume.into_field();
        let dir = self.direction as u8;
        let l   = (self.step_duration / 0x10000) as u8;

        (vol << 4) | (dir << 3) | l
    }

    pub fn step(&mut self) {
        if self.step_duration == 0 {
            // If the step duration is 0 the envelope is not active
            return;
        }

        self.counter += 1;
        self.counter %= self.step_duration;

        if self.counter == 0 {
            // Move on to the next step
            match self.direction {
                EnvelopeDirection::Up   => self.volume.up(),
                EnvelopeDirection::Down => self.volume.down(),
            }
        }
    }

    pub fn into_sample(&self) -> Sample {
        self.volume.into_sample()
    }

    /// DAC is disabled when envelope direction goes down and volume is 0
    pub fn dac_enabled(&self) -> bool {
        self.direction != EnvelopeDirection::Down ||
            self.volume.into_sample() != 0
    }
}

// Sound envelopes can become louder or quieter
#[derive(Copy,PartialEq,Eq)]
pub enum EnvelopeDirection {
    // Volume increases at each step
    Up   = 1,
    // Volume decreases at each step
    Down = 0,
}

/// The game boy sound uses 4bit DACs and can therefore only output 16
/// sound levels
#[derive(Copy)]
struct Volume(u8);

impl Volume {
    fn from_field(vol: u8) -> Volume {
        if vol > SOUND_MAX {
            panic!("Volume out of range: {}", vol);
        }

        Volume(vol)
    }

    fn into_field(self) -> u8 {
        let Volume(v) = self;

        v
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
        if v < SOUND_MAX {
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
