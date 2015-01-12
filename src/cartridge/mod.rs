//! Cartridge emulation. There are multiple cartridge types with
//! different capabilities (bankable ROM/RAM, battery, RTC etc...).

use std::fmt::{Show, Formatter, Error};
use std::io::{File, Reader, Writer, IoResult, Open, ReadWrite, SeekSet};
use std::iter::repeat;
use ascii::AsciiCast;

mod models;

/// Common state for all cartridge types
pub struct Cartridge {
    /// Cartridge ROM data
    rom:        Vec<u8>,
    /// Cartridge RAM data
    ram:        Vec<u8>,
    /// Current number of the rom bank mapped at [0x4000, 0x7fff]
    rom_bank:   u8,
    /// Current bank offset for the bank mapped at [0x4000, 0x7fff].
    /// This value is added to ROM register addresses when they're in
    /// that range.
    rom_offset: usize,
    /// Current bank offset for the RAM
    ram_offset: usize,
    /// If `true` RAM is write protected
    ram_wp:     bool,
    /// Certain cartridges allow banking either the RAM or ROM
    /// depending on the value of this flag.
    bank_ram:   bool,
    /// struct used to handle model specific functions
    model:      models::Model,
    /// Path to the ROM image for this cartridge
    path:       Path,
    /// optional save file used to store non-volatile RAM on emulator
    /// shutdown
    save_file:  Option<File>,
}

impl Cartridge {
    /// Load a Cartridge ROM from `path`.
    pub fn from_path(rom_path: &Path) -> IoResult<Cartridge> {
        let mut source = try!(File::open(rom_path));

        // There must always be at least two ROM banks
        let rom = try!(source.read_exact(2 * ROM_BANK_SIZE));

        let model = models::from_id(rom[offsets::TYPE]);

        let mut cartridge = Cartridge {
            rom:        rom,
            ram:        Vec::new(),
            // Default to bank 1 for bankable region
            rom_bank:   1,
            rom_offset: 0,
            ram_offset: 0,
            ram_wp:     true,
            bank_ram:   false,
            model:      model,
            path:       rom_path.clone(),
            save_file:  None,
        };

        let rombanks = match cartridge.rom_banks() {
            Some(n) => n,
            None    => panic!("Can't determine ROM size"),
        };

        // Read the remaining roms banks
        if rombanks > 2 {
            let remb      = rombanks - 2;
            let mut off   = 2    * ROM_BANK_SIZE;
            let mut remsz = remb * ROM_BANK_SIZE;

            // Reserve space for the remaining banks
            cartridge.rom.extend(repeat(0).take(remsz));

            while remsz > 0 {
                let r = try!(source.read(cartridge.rom.slice_from_mut(off)));

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
        let (rambanks, banksize) = match self.ram_banks() {
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

        let mut save_file = try!(File::open_mode(&savepath,
                                                Open,
                                                ReadWrite));

        let save_size = try!(save_file.stat()).size;

        if save_size == 0 {
            // The file is empty (probably new). initialize
            // the RAM with 0s.
            self.ram.resize(ramsize, 0);
            // Then fill the file with the right amount of 0s
            // to reserve enough space for saving later.
            try!(save_file.write(self.ram.as_slice()));
        } else if save_size == (ramsize as u64) {
            // The file contains a RAM image
            self.ram = try!(save_file.read_exact(ramsize as usize));
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
            println!("Saving to {}", f.path().display());

            try!(f.seek(0, SeekSet));
            try!(f.write(self.ram.as_slice()));
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

    /// Return the number of ROM banks for this ROM. Each bank is 16KB.
    pub fn rom_banks(&self) -> Option<usize> {
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
    pub fn ram_banks(&self) -> Option<(usize, usize)> {
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
        let off = offset as usize;

        if off < ROM_BANK_SIZE {
            self.rom[off]
        } else {
            self.rom[self.rom_offset as usize + off]
        }
    }

    pub fn set_rom_byte(&mut self, offset: u16, val: u8) {
        (self.model.write_rom)(self, offset, val)
    }

    /// Return the value of RAM byte at `offset` in the currently
    /// selected RAM bank
    pub fn ram_byte(&self, offset: u16) -> u8 {
        let addr = self.ram_offset + offset as usize;

        (self.model.read_ram)(self, addr)
    }

    /// Return the value of a RAM byte at absolute address `addr`
    fn ram_byte_absolute(&self, addr: usize) -> u8 {
        *self.ram.get(addr as usize).unwrap_or(&0)
    }

    fn ram_byte_absolute_mut(&mut self, addr: usize) -> Option<&mut u8> {
        self.ram.get_mut(addr as usize)
    }

    /// Set value of RAM byte at `offset` in the curretly selected RAM
    /// bank
    pub fn set_ram_byte(&mut self, offset: u16, val: u8) {
        let addr = self.ram_offset + offset as usize;

        if self.ram_wp {
            debug!("Attempt to write to cartridge RAM while protected");
            return;
        }

        (self.model.write_ram)(self, addr, val);
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

        // Recompute offset value to avoid doing it at each ROM read.
        self.rom_offset = ROM_BANK_SIZE *
            match self.rom_bank {
                /// We can't select bank 0, it defaults to 1
                0 => 0,
                n => (n - 1) as usize,
            };
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
        self.ram_offset = bank as usize * 8 * 1024;
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
            rom_offset: 0,
            ram_offset: 0,
            ram_wp:     true,
            bank_ram:   false,
            model:      models::from_id(0x00),
            path:       Path::new("dummy"),
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
                    name, self.model.name, rombanks, rambanks, rambanksize));

        Ok(())
    }
}

// Each ROM bank is always 16KB
const ROM_BANK_SIZE: usize = 16 * 1024;

mod offsets {
    //! Various offset values to access special memory locations within the ROM

    /// Title. Upper case ASCII 16bytes long, padded with 0s if shorter
    pub const TITLE:    usize = 0x134;
    /// Cartridge type
    pub const TYPE:     usize = 0x147;
    pub const ROM_SIZE: usize = 0x148;
    pub const RAM_SIZE: usize = 0x149;
}
