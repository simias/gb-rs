//! Timer emulation

/// Timer state
pub struct Timer {
    /// Timer counter. Generates an interrupt on overflow.
    counter: u8,
    /// Timer modulo: on overflow `counter` gets reloaded with that
    /// value.
    modulo: u8,
    /// If true timer is counting and generating interrupts
    enabled: bool,
    /// System clock divider. When `enabled`, at each tick of the
    /// divider clock `counter` is incremented.
    divider: Divider,
    /// Free-running internal counter simulating the sysclk
    clk: u32,
    /// Free-running counter at sysclk/256. I need a separate counter
    /// because that one can be reset
    counter_16k: u32,
    /// True if interrupt is pending
    interrupt: bool,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            counter:     0,
            modulo:      0,
            enabled:     false,
            divider:     Divider::Div1024,
            clk:         0,
            counter_16k: 0,
            interrupt:   false,
        }
    }

    pub fn step(&mut self) {
        self.clk         += 1;
        self.counter_16k += 1;

        if !self.enabled {
            return;
        }

        let mask = (1 << (self.divider as uint)) - 1;

        if self.clk & mask == 0 {
            // Divided clock ticked, increment counter
            self.counter += 1;

            if self.counter == 0 {
                // Timer overflowed.
                self.interrupt = true;
                self.counter = self.modulo;
            }
        }
    }

    /// Return current counter value
    pub fn counter(&self) -> u8 {
        self.counter
    }

    /// Reset counter value
    pub fn set_counter(&mut self, counter: u8) {
        self.counter = counter;
    }

    /// Get value of "DIV" register. It contains a free running
    /// counter of SysClk / 256 -> 16.384kHz
    pub fn div(&self) -> u8 {
        (self.counter_16k >> 8) as u8
    }

    /// Reset 16.384kHz counter to 0
    pub fn reset_div(&mut self) {
        self.counter_16k = 0;
    }

    /// Return the current value of the `modulo`
    pub fn modulo(&self) -> u8 {
        self.modulo
    }

    /// Set the value loaded into the counter when it overflows.
    pub fn set_modulo(&mut self, modulo: u8) {
        self.modulo = modulo;
    }

    /// Configure the timer using values specified in the config register
    pub fn set_config(&mut self, cfg: u8) {
        self.enabled = cfg & 4 != 0;

        self.divider = match cfg & 3 {
            0 => Divider::Div1024,
            1 => Divider::Div16,
            2 => Divider::Div64,
            3 => Divider::Div256,
            _ => panic!("Unreachable"),
        };
    }

    /// Return configuration register value
    pub fn config(&self) -> u8 {
        let mut r = 0;

        r |= (self.enabled as u8) << 2;
        r |= self.divider as u8;

        r
    }

    /// Return interrupt status
    pub fn interrupt(&self) -> bool {
        self.interrupt
    }

    /// Acknowledge interrupt
    pub fn ack_interrupt(&mut self) {
        self.interrupt = false;
    }

    /// Force interrupt state
    pub fn force_interrupt(&mut self, set: bool) {
        self.interrupt = set;
    }
}

/// Possible divider values usable as timer clock source.
#[deriving(Copy)]
enum Divider {
    /// Divide sysclk by 16. Timer clock is 262.144kHz
    Div16   = 4,
    /// Divide sysclk by 64. Timer clock is 65.536kHz
    Div64   = 6,
    /// Divide sysclk by 256. Timer clock is 16.384kHz
    Div256  = 8,
    /// Divide sysclk by 1024. Timer clock is 4.096kHz
    Div1024 = 12,
}
