//! Cartridge emulation. There are multiple cartridge types with
//! different capabilities (bankable ROM/RAM, battery, RTC etc...).

use std::fmt::{Debug, Formatter, Error};
use std::path::{Path, PathBuf};
use std::fs::{File, OpenOptions};
use std::iter::repeat;
use ascii::AsciiCast;
use std::io::{SeekFrom, Read, Write, Seek};
use std::io::Result as IoResult;

mod models;

/// Common state for all cartridge types
pub struct Cartridge {
    /// Cartridge ROM data
    rom:        Vec<u8>,
    /// Cartridge RAM data
    ram:        Vec<u8>,
    /// Total number of ROM banks in this cart
    rom_banks:  u8,
    /// Current number of the rom bank mapped at [0x4000, 0x7fff]
    rom_bank:   u8,
    /// Current bank offset for the bank mapped at [0x4000, 0x7fff].
    /// This value is added to ROM register addresses when they're in
    /// that range.
    rom_offset: i32,
    /// Current bank offset for the RAM
    ram_offset: u32,
    /// If `true` RAM is write protected
    ram_wp:     bool,
    /// Certain cartridges allow banking either the RAM or ROM
    /// depending on the value of this flag.
    bank_ram:   bool,
    /// struct used to handle model specific functions
    model:      models::Model,
    /// Path to the ROM image for this cartridge
    path:       PathBuf,
    /// optional save file used to store non-volatile RAM on emulator
    /// shutdown
    save_file:  Option<File>,
}

impl Cartridge {
    /// Load a Cartridge ROM from `path`.
    pub fn from_path(rom_path: &Path) -> IoResult<Cartridge> {
        let mut source = try!(File::open(rom_path));

        let mut rom = Vec::new();

        // There must always be at least two ROM banks
        try!((&mut source).take(2 * ROM_BANK_SIZE as u64)
             .read_to_end(&mut rom));

        let model = models::from_id(rom[offsets::TYPE]);

        let mut cartridge = Cartridge {
            rom:        rom,
            ram:        Vec::new(),
            rom_banks:  2,
            // Default to bank 1 for bankable region
            rom_bank:   1,
            rom_offset: 0,
            ram_offset: 0,
            ram_wp:     true,
            bank_ram:   false,
            model:      model,
            path:       PathBuf::from(rom_path),
            save_file:  None,
        };

        let rombanks = match cartridge.parse_rom_banks() {
            Some(n) => n,
            None    => panic!("Can't determine ROM size"),
        };

        cartridge.rom_banks = rombanks;

        // Read the remaining roms banks
        if rombanks > 2 {
            let remb      = (rombanks - 2) as usize;
            let mut off   = 2    * ROM_BANK_SIZE as usize;
            let mut remsz = remb * ROM_BANK_SIZE as usize;

            // Reserve space for the remaining banks
            cartridge.rom.extend(repeat(0).take(remsz));

            while remsz > 0 {
                let r = try!(source.read(&mut cartridge.rom[off..]));

                remsz -= r;
                off   += r;
            }
        }

        try!(cartridge.init_ram());

        Ok(cartridge)
    }

    /// Init cartridge RAM and tie it with a `File` for saving if
    /// necessary.
    fn init_ram(&mut self) -> IoResult<()> {
        let (rambanks, banksize) = match self.parse_ram_banks() {
            Some(v) => v,
            None    => panic!("Can't determine RAM size"),
        };

        let ramsize = rambanks * banksize;

        if ramsize == 0 {
            // No RAM on this cartridge, we're done
            return Ok(());
        }

        // We have some RAM, open the save file or create it if it
        // doesn't exist yet
        let mut savepath = self.path.clone();
        savepath.set_extension("sav");

        let mut save_file = try!(OpenOptions::new().read(true).write(true)
                                 .open(savepath.clone()));

        let save_size = try!(save_file.metadata()).len();

        if save_size == 0 {
            // The file is empty (probably new). initialize
            // the RAM with 0s.
            self.ram.resize(ramsize, 0);
            // Then fill the file with the right amount of 0s
            // to reserve enough space for saving later.
            try!(save_file.write_all(&self.ram));
        } else if save_size == (ramsize as u64) {
            // The file contains a RAM image
            try!((&mut save_file).take(ramsize as u64).read_to_end(&mut self.ram));
        } else {
            panic!("Unexpected save file size for {}: expected {} got {}",
                   savepath.display(), ramsize, save_size);
        }

        // Store the file handle to save progress later
        self.save_file = Some(save_file);

        Ok(())
    }

    /// Update the save file
    pub fn save_ram(&mut self) -> IoResult<()> {
        if let Some(mut f) = self.save_file.as_mut() {
            // Rewind to the beginning of the file and update its
            // contents
            match f.path() {
                Some(path) => println!("Saving to {}", path.display()),
                None       => println!("Saving"),
            }

            try!(f.seek(SeekFrom::Start(0)));
            try!(f.write_all(&self.ram));
        }

        Ok(())
    }

    /// Attempt to retreive the rom's name
    pub fn name(&self) -> Option<String> {
        let mut name = String::with_capacity(16);

        for i in 0..16 {
            let c =
                match self.rom[offsets::TITLE + i].to_ascii() {
                    Ok(c) => c,
                    _     => return None,
                };

            // If the name is shorter than 16bytes it's padded with 0s
            if c.as_byte() == 0 {
                break;
            }

            // Only uppercase ASCII is valid, but let's be a little
            // more lenient
            if !c.is_print() {
                return None;
            }

            // Append new character
            name.push(c.as_char());
        }

        Some(name)
    }

    /// Return the number of ROM banks declared in the header. Each
    /// bank is 16KB.
    fn parse_rom_banks(&self) -> Option<u8> {
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
    pub fn parse_ram_banks(&self) -> Option<(usize, usize)> {

        let model = models::from_id(self.rom_byte(offsets::TYPE as u16));

        // Special case for MBC2, the RAM_SIZE field is not
        // trustworthy here (it advertises 0 banks but there's still
        // some RAM on the cartridge).
        if model.name == "MBC2" {
            // MBC2 contains 1 "bank" of 256bytes
            return Some((1, 256));
        }

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
        let off = offset as i32;

        if off < ROM_BANK_SIZE {
            self.rom[off as usize]
        } else {
            self.rom[(self.rom_offset + off) as usize]
        }
    }

    pub fn set_rom_byte(&mut self, offset: u16, val: u8) {
        (self.model.write_rom)(self, offset, val)
    }

    /// Return the value of RAM byte at `offset` in the currently
    /// selected RAM bank
    pub fn ram_byte(&self, offset: u16) -> u8 {
        let addr = self.ram_offset + offset as u32;

        (self.model.read_ram)(self, addr)
    }

    /// Return the value of a RAM byte at absolute address `addr`
    fn ram_byte_absolute(&self, addr: u32) -> u8 {
        *self.ram.get(addr as usize).unwrap_or(&0)
    }

    fn ram_byte_absolute_mut(&mut self, addr: u32) -> Option<&mut u8> {
        self.ram.get_mut(addr as usize)
    }

    /// Set value of RAM byte at `offset` in the curretly selected RAM
    /// bank
    pub fn set_ram_byte(&mut self, offset: u16, val: u8) {
        let addr = self.ram_offset + offset as u32;

        if self.ram_wp {
            debug!("Attempt to write to cartridge RAM while protected");
            return;
        }

        (self.model.write_ram)(self, addr, val);
    }

    /// Retreive the number of ROM banks in the cartridge
    pub fn rom_banks(&self) -> u8 {
        self.rom_banks
    }

    /// Retrieve current ROM bank number for the bankable range at
    /// [0x4000, 0x7fff]
    pub fn rom_bank(&self) -> u8 {
        self.rom_bank
    }

    /// Set new ROM bank number for the bankable range at
    /// [0x4000, 0x7fff]
    pub fn set_rom_bank(&mut self, bank: u8) {
        self.rom_bank = bank;
    }

    pub fn set_rom_offset(&mut self, offset: i32) {
        self.rom_offset = offset;
    }

    /// Enable or disable RAM write protect
    pub fn set_ram_wp(&mut self, wp: bool) {
        self.ram_wp = wp
    }

    /// Return the value of the `bank_ram` flag
    pub fn bank_ram(&self) -> bool {
        self.bank_ram
    }

    /// Set the value of the `bank_ram` flag
    pub fn set_bank_ram(&mut self, v: bool) {
        self.bank_ram = v
    }

    /// Set new RAM bank number
    pub fn set_ram_bank(&mut self, bank: u8) {
        // Bankable RAM is always 8KB per bank
        self.ram_offset = bank as u32 * 8 * 1024;
    }

    /// Create a Cartridge instance from a ROM provided in a
    /// Vec<u8>. Usefull for tests. Creates a MBC0 model without
    /// banking.
    #[cfg(test)]
    pub fn from_vec(rom: Vec<u8>) -> Cartridge {
        Cartridge {
            rom:        rom,
            ram:        Vec::new(),
            rom_bank:   1,
            rom_banks:  2,
            rom_offset: 0,
            ram_offset: 0,
            ram_wp:     true,
            bank_ram:   false,
            model:      models::from_id(0x00),
            path:       PathBuf::from("dummy"),
            save_file:  None,
        }
    }
}

impl Drop for Cartridge {
    fn drop(&mut self) {
        // Update save file when Cartridge is dropped
        if let Err(e) = self.save_ram() {
            // Display the error but don't panic since we might
            // already be in the middle of a panic unwinding
            println!("Couldn't save: {}", e);
        }
    }
}

impl Debug for Cartridge {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let name = match self.name() {
            Some(s) => s,
            None    => "<INVALID>".to_string(),
        };

        let rombanks = self.rom_banks();

        let (rambanks, rambanksize) = match self.parse_ram_banks() {
            Some(n) => n,
            None    => (0, 0),
        };

        try!(write!(f,
                    "'{}' (Model: {}, \
                           ROM banks: {}, \
                           RAM banks: {}, \
                           RAM bank size: {}B)",
                    name, self.model.name, rombanks, rambanks, rambanksize));

        Ok(())
    }
}

// Each ROM bank is always 16KB
const ROM_BANK_SIZE: i32 = 16 * 1024;

mod offsets {
    //! Various offset values to access special memory locations within the ROM

    /// Title. Upper case ASCII 16bytes long, padded with 0s if shorter
    pub const TITLE:    usize = 0x134;
    /// Cartridge type
    pub const TYPE:     usize = 0x147;
    pub const ROM_SIZE: usize = 0x148;
    pub const RAM_SIZE: usize = 0x149;
}
