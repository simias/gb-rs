//! Game Boy sound 4 generates noise from a Linear Feedback Shift
//! Register.

use spu::{Sample, Mode};
use spu::envelope::Envelope;

pub struct LfsrWave {
    /// True if the wave is generating samples
    running:        bool,
    /// Linear Feedback Shift Register
    lfsr:           Lfsr,
    /// Enveloppe that will be used at the next start()
    start_envelope: Envelope,
    /// Active envelope
    envelope:       Envelope,
    /// Play mode (continuous or counter)
    mode:           Mode,
    /// Counter for counter mode
    remaining:      u32,
}

impl LfsrWave {
    pub fn new() -> LfsrWave {
        LfsrWave {
            lfsr:           Lfsr::from_reg(0),
            start_envelope: Envelope::from_reg(0),
            envelope:       Envelope::from_reg(0),
            remaining:      0,
            mode:           Mode::Continuous,
            running:        false,
        }
    }

    pub fn step(&mut self) {
        if !self.running {
             return;
        }

        if self.mode == Mode::Counter {
            if self.remaining == 0 {
                self.running = false;
                return;
            }

            self.remaining -= 1;
        }

        self.envelope.step();
        self.lfsr.step();
    }

    pub fn sample(&self) -> Sample {

        if !self.running {
            return 0;
        }

        if self.lfsr.high() {
            self.envelope.into_sample()
        } else {
            0
        }
    }

    pub fn set_envelope(&mut self, envelope: Envelope) {
        self.start_envelope = envelope;
    }

    pub fn set_length(&mut self, len: u8) {
        if len >= 64 {
            panic!("sound length out of range: {}", len);
        }

        let len = len as u32;

        self.remaining = (64 - len) * 0x4000;
    }

    pub fn set_mode(&mut self, mode: Mode) {
        self.mode = mode;
    }

    pub fn set_lfsr(&mut self, lfsr: Lfsr) {
        self.lfsr = lfsr;
    }

    pub fn start(&mut self) {
        self.envelope  = self.start_envelope;
        self.running = true;
    }
}

#[derive(Copy)]
pub struct Lfsr {
    register:      u16,
    width:         LfsrWidth,
    step_duration: u32,
    counter:       u32,
}

#[derive(Copy)]
enum LfsrWidth {
    Lfsr15bit = 0,
    Lfsr7bit  = 1,
}

impl Lfsr {
    pub fn from_reg(val: u8) -> Lfsr {
        let (reg, width) =
            match (val & 8) != 0 {
                true  => ((1 <<  7) - 1,  LfsrWidth::Lfsr7bit),
                false => ((1 << 15) - 1,  LfsrWidth::Lfsr15bit),
            };

        // There are two divisors in series to generate the LFSR
        // clock.
        let mut l =
            match val & 7 {
                // 0 is 8 * 0.5 so we need to special case it since
                // we're using integer arithmetics.
                0 => 8 / 2,
                n => 8 * n as u32,
            };

        l *= 1 << ((val >> 4) + 1) as usize;

        Lfsr {
            register:      reg,
            width:         width,
            step_duration: l,
            counter:       0,
        }
    }

    fn step(&mut self) {
        self.counter += 1;
        self.counter %= self.step_duration;

        if self.counter == 0 {
            self.shift();
        }
    }

    fn high(self) -> bool {
        self.register & 1 != 0
    }

    fn shift(&mut self) {
        let shifted = self.register >> 1;
        let carry   = (self.register ^ shifted) & 1;

        self.register =
            match self.width {
                LfsrWidth::Lfsr7bit  => shifted | (carry << 6),
                LfsrWidth::Lfsr15bit => shifted | (carry << 14),
            };
    }
}
