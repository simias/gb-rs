//! RAM emulation

use std::iter;

/// RAM image
pub struct Ram {
    data: Vec<u8>,
}

impl Ram {
    /// Create a new RAM. The default RAM values are undetermined so I
    /// just fill it with garbage
    pub fn new(size: uint) -> Ram {
        let data = iter::repeat(0xca).take(size).collect();

        Ram { data: data }
    }

    pub fn byte(&self, offset: u16) -> u8 {
       self.data[offset as uint]
    }

    pub fn set_byte(&mut self, offset: u16, val: u8) {
        self.data[offset as uint] = val;
    }
}
