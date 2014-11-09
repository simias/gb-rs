//! ROM (cartridge) emulation

use std::fmt::{Show, Formatter, FormatError};
use std::io::{File, Reader, IoResult};

use super::Addressable;

/// ROM image
pub struct Rom {
    /// Full cartridge data
    data: Vec<u8>,
}

impl Rom {
    /// Load the ROM from a `Reader`
    pub fn from_reader(source: &mut Reader) -> IoResult<Rom> {
        // Only single unbanked ROMs supported for now (so only banks
        // 0 and 1 are available)
        let data = try!(source.read_exact(32 * 1024));

        Ok(Rom { data: data })
    }

    /// Load the ROM from a file
    pub fn from_file(path: &Path) -> IoResult<Rom> {
        let mut file_reader = try!(File::open(path));

        Rom::from_reader(&mut file_reader)
    }

    /// Attempt to retreive the rom's name
    pub fn get_name(&self) -> Option<String> {
        let mut name = String::with_capacity(16);

        for i in range(0, 16) {
            let c = self.data[offsets::TITLE + i].to_ascii();

            // If the name is shorter than 16bytes it's padded with 0s
            if c == 0.to_ascii() {
                break;
            }

            // Only uppercase ASCII is valid
            if !(c.is_uppercase() || c.is_blank() || c.is_punctuation()) {
                return None;
            }

            // Append new character
            name.grow(1, c.to_char());
        }

        Some(name)
    }
}

impl Addressable for Rom {
    fn get_byte(&self, offset: u16) -> u8 {
        self.data[offset as uint]
    }
}

impl Show for Rom {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> {
        try!(write!(f, "'{}'", match self.get_name() {
            Some(s) => s,
            None    => "<INVALID>".to_string(),
        }));

        Ok(())
    }
}

mod offsets {
    //! Various offset values to access special memory locations within the ROM

    /// Title. Upper case ASCII 16bytes long, padded with 0s if shorter
    pub const TITLE: uint = 0x134;
}
