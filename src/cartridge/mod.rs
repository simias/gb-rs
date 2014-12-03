//! Cartridge emulation. There are multiple cartridge types with
//! different capabilities (bankable ROM/RAM, battery, RTC etc...).

use std::fmt::{Show, Formatter, Error};
use std::io::{File, Reader, IoResult};

mod models;

/// Common state for all cartridge types
pub struct Cartridge {
    /// Cartridge ROM data
    data:      Vec<u8>,
    /// Current bank offset for the bank mapped at [0x4000, 0x7fff].
    /// This value is added to ROM register addresses when they're in
    /// that range.
    high_bank: uint,
    /// Trait object used to handle model specific functions
    model:     &'static (models::Model + 'static),
}

impl Cartridge {
    pub fn reset(&mut self) {
        self.high_bank = 0;
    }

    /// Load a Cartridge from a `Reader`
    pub fn from_reader(source: &mut Reader) -> IoResult<Cartridge> {
        // There must always be at least two ROM banks
        let data = try!(source.read_exact(2 * ROM_BANK_SIZE));

        let mut cartridge = Cartridge {
            data:      data,
            high_bank: 0,
            model:     &models::MBC1,
        };

        let nbanks = match cartridge.rom_banks() {
            Some(n) => n,
            None    => panic!("Can't determine ROM bank number"),
        };

        // Read the remaining roms banks
        if nbanks > 2 {
            let remb      = nbanks - 2;
            let mut off   = 2    * ROM_BANK_SIZE;
            let mut remsz = remb * ROM_BANK_SIZE;

            // Reserve space for the remaining banks
            cartridge.data.grow(remsz, 0);

            while remsz > 0 {
                let r = try!(source.read(cartridge.data.slice_from_mut(off)));

                remsz -= r;
                off   += r;
            }
        }

        Ok(cartridge)
    }

    /// Load the Cartridge from a file
    pub fn from_file(path: &Path) -> IoResult<Cartridge> {
        let mut file_reader = try!(File::open(path));

        Cartridge::from_reader(&mut file_reader)
    }

        /// Attempt to retreive the rom's name
    pub fn name(&self) -> Option<String> {
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
            name.grow(1, c.as_char());
        }

        Some(name)
    }

    /// Return the number of ROM banks for this ROM. Each bank is 16KB.
    pub fn rom_banks(&self) -> Option<uint> {
        let id = self.rom_byte(offsets::ROM_SIZE as u16);

        let nbanks =
            match id {
                0x00 => 2,
                0x01 => 4,
                0x02 => 8,
                0x03 => 16,
                0x04 => 32,
                0x05 => 64,
                0x06 => 128,
                0x52 => 72,
                0x53 => 80,
                0x54 => 96,
                // Unknown value
                _    => return None,
            };

        Some(nbanks)
    }

    /// Return the number of RAM banks for this ROM along with the
    /// size of each bank in bytes.
    pub fn ram_banks(&self) -> Option<(uint, uint)> {
        let id = self.rom_byte(offsets::RAM_SIZE as u16);

        let (nbanks, bank_size_kb) =
            match id {
                0x00 => (0,  0),
                0x01 => (1,  2),
                0x02 => (1,  8),
                0x03 => (4,  8),
                0x04 => (16, 8),
                // Unknown value
                _    => return None,
            };

        Some((nbanks, bank_size_kb * 1024))
    }

    pub fn rom_byte(&self, offset: u16) -> u8 {
        let off = offset as uint;

        if off < ROM_BANK_SIZE {
            self.data[off]
        } else {
            self.data[self.high_bank as uint + off]
        }
    }

    pub fn set_rom_byte(&mut self, offset: u16, val: u8) {
        // Let specialized cartridge type handle that
        self.model.write(self, offset, val)
    }

    fn set_high_bank(&mut self, hb: uint) {
        self.high_bank = hb
    }
}

impl Show for Cartridge {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let name = match self.name() {
            Some(s) => s,
            None    => "<INVALID>".to_string(),
        };

        let rombanks = match self.rom_banks() {
            Some(n) => n,
            None    => 0,
        };

        let (rambanks, rambanksize) = match self.ram_banks() {
            Some(n) => n,
            None    => (0, 0),
        };

        try!(write!(f,
                    "'{}' (Model: {}, \
                           ROM banks: {}, \
                           RAM banks: {}, \
                           RAM bank size: {}KB)",
                    name, self.model.name(), rombanks, rambanks, rambanksize));

        Ok(())
    }
}

// Each ROM bank is always 16KB
const ROM_BANK_SIZE: uint = 16 * 1024;

mod offsets {
    //! Various offset values to access special memory locations within the ROM

    /// Title. Upper case ASCII 16bytes long, padded with 0s if shorter
    pub const TITLE:    uint = 0x134;
    pub const ROM_SIZE: uint = 0x148;
    pub const RAM_SIZE: uint = 0x149;
}
