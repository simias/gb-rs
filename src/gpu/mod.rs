//! Game Boy GPU emulation

use std::cell::Cell;
use std::fmt::{Show, Formatter, FormatError};

use io::Addressable;

/// GPU state.
pub struct Gpu {
    /// Current line. [0,143] is active video, [144,153] is blanking.
    line: u8,
    /// Position on the current line.
    col:  u16,
    /// Object attritube memory
    oam:  [Cell<u8>, ..0xa0],
}

/// Current GPU mode
#[deriving(Show)]
pub enum Mode {
    /// In horizontal blanking
    HBlank = 0,
    /// In vertical blanking
    VBlank = 1,
    /// Accessing sprite memory, Sprite attributes RAM [0xfe00, 0xfe9f]
    /// can't be accessed
    Prelude = 2,
    /// Accessing sprite memory and video memory [0x8000, 0x9fff],
    /// both can't be accessed from CPU
    Active = 3,
}

impl Gpu {
    /// Create a new Gpu instance.
    pub fn new() -> Gpu {
        Gpu { line: 0, col: 0, oam: [Cell::new(0xca), ..0xa0] }
    }

    /// Reset the GPU state to power up values
    pub fn reset(&mut self) {
        self.line = 0;
        self.col  = 0;
        self.oam  = [Cell::new(0xca), ..0xa0];
    }

    /// Called at each tick of the system clock. Move the emulated
    /// state one step forward.
    pub fn step(&mut self) {

        println!("{}", *self);

        if self.col < 456 {
            self.col += 1;
        } else {
            self.col = 0;

            // Move on to the next line
            if self.line < 154 {
                self.line += 1
            } else {
                // New frame
                self.line = 0;
            }
        }
    }

    pub fn get_mode(&self) -> Mode {
        if self.line < 144 {
            match self.col {
                0  ... 79  => Prelude,
                80 ... 172 => Active,
                _          => HBlank,
            }
        } else {
            VBlank
        }
    }

    pub fn get_line(&self) -> u8 {
        self.line
    }
}

impl Addressable for Gpu {
    fn get_byte(&self, addr: u16) -> u8 {
        if addr >= 0xfe00 {
            match self.get_mode() {
                Prelude | Active => panic!("OAM access while in use {:04x}", addr),
                _                => self.oam[(addr & 0xff) as uint].get()
            }
        } else {
            panic!("Unexpected GPU access at {:04x}", addr);
        }
    }

    fn set_byte(&self, addr: u16, val: u8) {
        if addr >= 0xfe00 {
            match self.get_mode() {
                Prelude | Active => panic!("OAM access while in use {:04x}", addr),
                _                => self.oam[(addr & 0xff) as uint].set(val)
            }
        } else {
            panic!("Unexpected GPU write at {:04x}: {:02x}", addr, val);
        }
    }

}

impl Show for Gpu {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> {
        try!(write!(f, "Gpu at ({}, {}) [{}] ", self.col, self.line, self.get_mode()));

        Ok(())
    }
}
