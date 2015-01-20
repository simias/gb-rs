//! Envelope function used by sound channels 1 2 and 4

use spu::{Volume, Sample};

#[derive(Copy)]
pub struct Envelope {
    direction:     EnvelopeDirection,
    volume:        Volume,
    step_duration: u32,
    counter:       u32,
}

impl Envelope {
    pub fn from_reg(val: u8) -> Envelope {

        let vol = Volume::new(val >> 4);

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
}

// Sound envelopes can become louder or quieter
#[derive(Copy,PartialEq,Eq)]
pub enum EnvelopeDirection {
    // Volume increases at each step
    Up,
    // Volume decreases at each step
    Down,
}
