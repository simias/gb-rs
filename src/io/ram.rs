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
        let data = Vec::from_elem(size, Cell::new(0xca));

        Ram { data: data }
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
