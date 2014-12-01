//! ROM (cartridge) emulation

use std::fmt::{Show, Formatter, Error};
use std::io::{File, Reader, IoResult};

/// ROM image
pub struct Rom {
    /// Full cartridge data
    data: Vec<u8>,
    /// Current bank offset for the "high" bank.
    high_bank: uint,
}

impl Rom {
    /// Load the ROM from a `Reader`
    pub fn from_reader(source: &mut Reader) -> IoResult<Rom> {
        // There must always be at least two ROM banks
        let data = try!(source.read_exact(2 * ROM_BANK_SIZE));

        let mut rom = Rom { data: data, high_bank: 0 };

        let nbanks = match rom.rom_banks() {
            Some(n) => n,
            None    => panic!("Can't determine ROM bank number"),
        };

        // Read the remaining roms banks
        if nbanks > 2 {
            let remb      = nbanks - 2;
            let mut off   = 2    * ROM_BANK_SIZE;
            let mut remsz = remb * ROM_BANK_SIZE;

            // Reserve space for the remaining banks
            rom.data.grow(remsz, 0);

            while remsz > 0 {
                let r = try!(source.read(rom.data.slice_from_mut(off)));

                remsz -= r;
                off   += r;
            }
        }

        Ok(rom)
    }

    /// Load the ROM from a file
    pub fn from_file(path: &Path) -> IoResult<Rom> {
        let mut file_reader = try!(File::open(path));

        Rom::from_reader(&mut file_reader)
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
        let id = self.get_byte(offsets::ROM_SIZE as u16);

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
        let id = self.get_byte(offsets::RAM_SIZE as u16);

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

    pub fn get_byte(&self, offset: u16) -> u8 {
        let off = offset as uint;

        if off < ROM_BANK_SIZE {
            self.data[off]
        } else {
            self.data[self.high_bank + off]
        }
    }

    pub fn set_byte(&mut self, offset: u16, val: u8) {
        if offset >= 0x2000 && offset < 0x8000 {
            // Select a new rom bank
            self.high_bank = ROM_BANK_SIZE *
                match val {
                    /// We can't select bank 0, it defaults to 1
                    0 => 0,
                    n => (n - 1) as uint,
                };
        } else {
            println!("Unhandled ROM write: {:04x} {:02x}", offset, val);
        }
    }
}

impl Show for Rom {
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
                    "'{}' (ROM banks: {}, RAM banks: {}, RAM bank size: {}KB)",
                    name, rombanks, rambanks, rambanksize));

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
