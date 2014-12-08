//! RAM emulation

/// RAM image
pub struct Ram {
    data: Vec<u8>,
}

impl Ram {
    /// Create a new RAM. The default RAM values are undetermined so I
    /// fill it with a "garbage" pattern.
    pub fn new(size: uint) -> Ram {
        let data = Vec::from_elem(size, RAM_DEFAULT);

        Ram { data: data }
    }

    /// Clear the ram to the default garbage value (allows for
    /// deterministic resets
    pub fn reset(&mut self) {
        for b in self.data.iter_mut() {
            *b = RAM_DEFAULT;
        }
    }

    pub fn byte(&self, offset: u16) -> u8 {
        self.data[offset as uint]
    }

    pub fn set_byte(&mut self, offset: u16, val: u8) {
        self.data[offset as uint] = val;
    }
}

/// Default value of each RAM Cell on reset
const RAM_DEFAULT: u8 = 0x0;
