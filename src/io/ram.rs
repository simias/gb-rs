//! RAM emulation

use std::cell::Cell;

use super::Addressable;

/// RAM image
pub struct Ram {
    data: Vec<Cell<u8>>,
}

impl Ram {
    /// Create a new RAM. The default RAM values are undetermined so I
    /// fill it with a "garbage" pattern.
    pub fn new(size: uint) -> Ram {
        let data = Vec::from_elem(size, Cell::new(RAM_DEFAULT));

        Ram { data: data }
    }

    /// Clear the ram to the default garbage value (allows for
    /// deterministic resets
    pub fn reset(&mut self) {
        for c in self.data.iter() {
            c.set(RAM_DEFAULT);
        }
    }
}

impl Addressable for Ram {
    fn get_byte(&self, offset: u16) -> u8 {
        self.data[offset as uint].get()
    }

    fn set_byte(&self, offset: u16, val: u8) {
        self.data[offset as uint].set(val);
    }
}

/// Default value of each RAM Cell on reset
const RAM_DEFAULT: u8 = 0xca;
