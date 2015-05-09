//! Game Boy sounds 1 and 2 generate a rectangular waveform
//! with an envelope function. Channel 1 can also sweep through a
//! frequency range.

use spu::{Sample, Mode};
use spu::envelope::Envelope;

pub struct RectangleWave {
    /// True if the sound is generating samples
    running:        bool,
    /// Signal duty cycle
    duty:           DutyCycle,
    /// Period counter, the period length is configurable and is used
    /// to select the desired output frequency. This counter loops 8
    /// times per cycle to let us generate the proper duty cycle.
    counter:        u16,
    /// Divider value configured in the registers. The actual divider
    /// value used for the counter is 4 x (0x800 - <this value>)
    divider:        u16,
    /// Phase counter, increments 8 times per sound period
    phase:          u8,
    /// Enveloppe that will be used at the next start()
    start_envelope: Envelope,
    /// Active envelope
    envelope:       Envelope,
    /// Play mode (continuous or counter)
    mode:           Mode,
    /// Counter for counter mode
    remaining:      u32,
    /// Sweep function (only available on sound 1)
    sweep:          Sweep,
}

impl RectangleWave {
    pub fn new() -> RectangleWave {
        RectangleWave {
            running:        false,
            duty:           DutyCycle::from_field(0),
            counter:        0,
            divider:        0,
            phase:          0,
            start_envelope: Envelope::from_reg(0),
            envelope:       Envelope::from_reg(0),
            mode:           Mode::Continuous,
            remaining:      64 * 0x4000,
            sweep:          Sweep::from_reg(0),
        }
    }

    pub fn step(&mut self) {

        // Counter runs even if the channel is disabled
        if self.mode == Mode::Counter {
            if self.remaining == 0 {
                self.running = false;
                // Reload counter default value
                self.remaining = 64 * 0x4000;
                return;
            }

            self.remaining -= 1;
        }

        if !self.running {
            return;
        }

        self.envelope.step();

        self.divider =
            match self.sweep.step(self.divider) {
                Some(div) => div,
                None      => {
                    // Sweep function ended, sound is stopped
                    self.running = false;
                    return;
                }
            };

        if self.counter == 0 {
            // Reset the counter. This weird equation is simply how
            // the hardware does it, no tricks here.
            self.counter = 4 * (0x800 - self.divider);

            // Move on to the next phase.
            self.phase = (self.phase + 1) % 8;
        }

        self.counter -= 1;
    }

    pub fn sample(&self) -> Sample {

        if !self.running {
            return 0;
        }

        if self.phase < self.duty.active_per_8() {
            // Output is high
            self.envelope.into_sample()
        } else {
            0
        }
    }

    pub fn start(&mut self) {
        self.envelope = self.start_envelope;
        self.running  = self.envelope.dac_enabled();
        // What do I need to do here exactly? Which counters are
        // reset?
    }

    pub fn running(&self) -> bool {
        self.running
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

    pub fn duty(&self) -> DutyCycle {
        self.duty
    }

    pub fn set_duty(&mut self, duty: DutyCycle) {
        self.duty = duty;
    }

    pub fn envelope(&self) -> Envelope {
        self.start_envelope
    }

    pub fn set_envelope(&mut self, envelope: Envelope) {
        // New envelope will become active at the next start
        self.start_envelope = envelope;

        if !envelope.dac_enabled() {
            self.running = false;
        }
    }

    pub fn mode(&self) -> Mode {
        self.mode
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }
    pub fn set_length(&mut self, len: u8) {
        if len >= 64 {
            panic!("sound length out of range: {}", len);
        }

        let len = len as u32;

        self.remaining = (64 - len) * 0x4000;
    }

    pub fn sweep(&self) -> Sweep {
        self.sweep
    }

    pub fn set_sweep(&mut self, sweep: Sweep) {
        self.sweep = sweep;
    }
}

/// Rectangular wave duty cycle.
#[derive(Clone,Copy)]
pub enum DutyCycle {
    /// Duty cycle of 12.5% (1/8)
    Duty13 = 1,
    /// Duty cycle of 25%   (2/8)
    Duty25 = 2,
    /// Duty cycle of 50%   (4/8)
    Duty50 = 4,
    /// Duty cycle of 75%   (6/8)
    Duty75 = 6,
}


impl DutyCycle {
    /// Construct a DutyCycle from a register field (NR11 and
    /// NR21, bits [7:6])
    pub fn from_field(field: u8) -> DutyCycle {
        match field {
            0 => DutyCycle::Duty13,
            1 => DutyCycle::Duty25,
            2 => DutyCycle::Duty50,
            3 => DutyCycle::Duty75,
            _ => unreachable!(),
        }
    }

    /// Convert back into NR11/21 field value
    pub fn into_field(self) -> u8 {
        match self {
            DutyCycle::Duty13 => 0,
            DutyCycle::Duty25 => 1,
            DutyCycle::Duty50 => 2,
            DutyCycle::Duty75 => 3,
        }
    }

    /// Return the number of active samples for a frequency whose
    /// period is 8 samples
    fn active_per_8(self) -> u8 {
        self as u8
    }
}

#[derive(Clone,Copy)]
pub struct Sweep {
    direction:     SweepDirection,
    shift:         u8,
    step_duration: u32,
    counter:       u32,
}

impl Sweep {
    // Build Sweep from NR10 register value
    pub fn from_reg(val: u8) -> Sweep {
        let dir =
            match val & 8 != 0 {
                false => SweepDirection::Up,
                true  => SweepDirection::Down,
            };

        let shift = val & 7;

        let l = ((val & 0x70) >> 4) as u32;

        Sweep {
            direction:     dir,
            shift:         shift,
            step_duration: l * 0x8000,
            counter:       0,
        }
    }

    // Retreive value of NR10 register value
    pub fn into_reg(&self) -> u8 {
        let l   = (self.step_duration / 0x8000) as u8;
        let dir = self.direction as u8;

        // MSB is undefined and always 1
        (1 << 7) | (l << 4) | (dir << 3) | self.shift
    }

    /// Step through the Sweep state machine, returning the updated
    /// divider or None if the sound must be stopped
    fn step(&mut self, div: u16) -> Option<u16> {
        if self.step_duration == 0 {
            // Sweep OFF, do nothing
            return Some(div);
        }

        self.counter += 1;
        self.counter %= self.step_duration;

        if self.counter != 0 {
            // Do nothing and wait for the next step
            return Some(div);
        }

        // Update the frequency
        let offset = div >> (self.shift as usize);

        match self.direction {
            SweepDirection::Up => {
                let div = div + offset;

                if div > 0x7ff {
                    // We stop on overflow
                    None
                } else {
                    Some(div)
                }
            }
            SweepDirection::Down => {
                if self.shift == 0 || offset > div {
                    // If the substraction would underflow we do
                    // nothing
                    Some(div)
                } else {
                    Some(div - offset)
                }
            }
        }
    }
}

// Sound envelopes can become louder or quieter
#[derive(Clone,Copy,PartialEq,Eq)]
enum SweepDirection {
    // Frequency increases at each step
    Up   = 0,
    // Frequency decreases at each step
    Down = 1,
}
