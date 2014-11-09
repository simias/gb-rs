//! Memory mapped IO registers

use std::cell::Cell;

use super::Addressable;

/// Register state
pub struct Regs {
    
}

impl Regs {
    /// Create a new register instance. Set everything to 0.
    pub fn new() -> Regs {
        let regs = Vec::from_elem(0x100, Cell::new(0));

        Regs { regs: regs }
    }

    /// Reset all registers to 0
    pub fn reset(&mut self) {
        for c in self.regs.iter() {
            c.set(0);
        }
    }
}

impl Addressable for Regs {
    fn get_byte(&self, offset: u16) -> u8 {
        println!("IO read from 0x{:02x}", offset)
        self.regs[offset as uint].get()
    }

    fn set_byte(&self, offset: u16, val: u8) {
        println!("IO write to 0x{:02x}: 0x{:02x}", offset, val)
        self.regs[offset as uint].set(val);
    }
}
