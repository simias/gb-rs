//! Cartridge model specific emulation

use super::{Cartridge, ROM_BANK_SIZE};

/// Interface to model-specific operations
#[derive(Copy)]
pub struct Model {
    /// String identifier
    pub name:      &'static str,
    /// Handle ROM write
    pub write_rom: fn(cart: &mut Cartridge, offset: u16, val: u8),
    /// Handle RAM write
    pub write_ram: fn(cart: &mut Cartridge, addr: u32, val: u8),
    /// Handle RAM read
    pub read_ram:  fn(cart: &Cartridge, addr: u32) -> u8,
}

/// Default implementation of write_ram, suitable for most cartridges
fn write_ram(cart: &mut Cartridge, addr: u32, val: u8) {
    if let Some(b) = cart.ram_byte_absolute_mut(addr) {
        *b = val;
    }
}

/// Default implementation of read_ram, suitable for most cartridges
fn read_ram(cart: &Cartridge, addr: u32) -> u8 {
    cart.ram_byte_absolute(addr)
}

/// Default implementation of bank reconfiguration
fn set_rom_bank(cart: &mut Cartridge, bank: u8) {
    cart.set_rom_bank(bank);

    let rom_offset = ROM_BANK_SIZE *
        match bank {
            // We can't select bank 0, it defaults to 1
            0 => 0,
            // The offset is added to the address of the CPU
            // access. This bankable ROM is just after the bank0 it
            // means we always have a 1 bank offset already in the
            // address, so we need to substract 1 here.
            n => (n - 1) as i32,
        };

    cart.set_rom_offset(rom_offset);
}

mod mbc0 {
    use super::Model;
    use cartridge::Cartridge;

    fn write_rom(_: &mut Cartridge, offset: u16, val: u8) {
        debug!("Unhandled ROM write: {:04x} {:02x}", offset, val);
    }

    pub static MODEL: Model =
        Model { name:      "MBC0",
                write_rom: write_rom,
                write_ram: super::write_ram,
                read_ram:  super::read_ram,
        };
}

mod mbc1 {
    use super::Model;
    use cartridge::{Cartridge, ROM_BANK_SIZE};

    fn write_rom(cart: &mut Cartridge, offset: u16, val: u8) {
        match offset {
            0x0000...0x1fff =>
                // Writing a low nibble 0xa to anywhere in that
                // address range removes RAM write protect, All other
                // values enable it.
                cart.set_ram_wp(val & 0xf != 0xa),
            0x2000...0x3fff => {
                // Select a new ROM bank, bits [4:0]
                let cur_bank = cart.rom_bank() & !0x1f;

                let bank = cur_bank | (val & 0x1f);

                set_rom_bank(cart, bank);
            }
            0x4000...0x5fff =>
                if cart.bank_ram() {
                    // Select a new RAM bank
                    cart.set_ram_bank(val & 0x3);
                } else {
                    // Select a new ROM bank, bits [6:5]
                    let cur_bank = cart.rom_bank() & !0x60;

                    let bank = cur_bank | ((val << 5) & 0x60);

                    set_rom_bank(cart, bank);
                },
            0x6000...0x7fff =>
                // Switch RAM/ROM banking mode
                cart.set_bank_ram(val & 1 != 0),
            _ => debug!("Unhandled ROM write: {:04x} {:02x}", offset, val),
        }
    }

    /// MBC1 implementation of bank reconfiguration. I couldn't find
    /// that documented anywhere but The Legend of Zelda crashes at
    /// certain points if this is not accurate. I took the algorithm
    /// from gambatte.
    fn set_rom_bank(cart: &mut Cartridge, bank: u8) {
        cart.set_rom_bank(bank);

        // I don't really understand this part, I know that bank can't
        // be 0 (since bank 0 is always mapped at the begining of the
        // address space) but this would also rewrite bank 32 to 33
        // for instance. Maybe a quirck of MBC1?
        let bank =
            if bank & 0x1f != 0 {
                bank
            } else {
                bank | 1
            };

        // If the bank overflows we wrap it around. This assumes that
        // MBC1 cart can only have a power of two number of banks.
        let bank = bank & (cart.rom_banks() - 1);

        // Same as super::set_rom_bank: we already have a one bank
        // offset in the CPU address when accessing bankable ROM.
        let bank = (bank as i32) - 1;

        let rom_offset = ROM_BANK_SIZE * bank;

        cart.set_rom_offset(rom_offset);
    }


    pub static MODEL: Model =
        Model { name:      "MBC1",
                write_rom: write_rom,
                write_ram: super::write_ram,
                read_ram:  super::read_ram,
        };
}

mod mbc2 {
    use super::Model;
    use cartridge::Cartridge;

    fn write_rom(cart: &mut Cartridge, offset: u16, val: u8) {
        match offset {
            0x0000...0x1fff =>
                // Writing a low nibble 0xa to anywhere in that
                // address range removes RAM write protect, All other
                // values enable it.
                cart.set_ram_wp(val & 0xf != 0xa),
            0x2000...0x3fff => {
                super::set_rom_bank(cart, val & 0xf);
            }
            _ => debug!("Unhandled ROM write: {:04x} {:02x}", offset, val),
        }
    }

    pub static MODEL: Model =
        Model { name:      "MBC2",
                write_rom: write_rom,
                write_ram: super::write_ram,
                read_ram:  super::read_ram,
        };
}

mod mbc3 {
    use super::Model;
    use cartridge::Cartridge;

    fn write_rom(cart: &mut Cartridge, offset: u16, val: u8) {
        match offset {
            0x0000...0x1fff =>
                // Writing a low nibble 0xa to anywhere in that
                // address range removes RAM write protect, All other
                // values enable it.
                cart.set_ram_wp(val & 0xf != 0xa),
            0x2000...0x3fff =>
                // Select a new ROM bank
                super::set_rom_bank(cart, val & 0x7f),
            0x4000...0x5fff =>
                // Select a new RAM bank
                cart.set_ram_bank(val),
            0x6000...0x7fff => debug!("Unhandled RTC access"),
            _ => debug!("Unhandled ROM write: {:04x} {:02x}", offset, val),
        }
    }

    pub static MODEL: Model =
        Model { name:     "MBC3",
                write_rom: write_rom,
                write_ram: super::write_ram,
                read_ram:  super::read_ram,
        };
}

mod camera {
    //! Game Boy Camera cartridge emulation
    use super::Model;
    use cartridge::Cartridge;

    fn write_rom(cart: &mut Cartridge, offset: u16, val: u8) {
        match offset {
            0x0000...0x1fff => {
                // Writing a low nibble 0xa to anywhere in that
                // address range removes RAM write protect, All other
                // values enable it.
                //println!("set ram_wp {:02x}", val);
                cart.set_ram_wp(false);
                //cart.set_ram_wp(val & 0xf != 0xa);
            }
            0x2000...0x3fff => {
                // Select a new ROM bank
                //println!("set rom bank {:02x}", val);
                super::set_rom_bank(cart, val & 0x7f);
            }
            0x4000...0x5fff => {
                //println!("set ram bank {:02x}", val);
                // Select a new RAM bank
                cart.set_ram_bank(val);
            }
            _ =>
                debug!("Unhandled ROM write: {:04x} {:02x}", offset, val),
        }
    }

    /// Default implementation of write_ram, suitable for most cartridges
    fn write_ram(cart: &mut Cartridge, addr: u32, val: u8) {
        if let Some(b) = cart.ram_byte_absolute_mut(addr) {
            *b = val;
        }
    }

    /// Default implementation of read_ram, suitable for most cartridges
    fn read_ram(cart: &Cartridge, addr: u32) -> u8 {
        let img = include_bytes!("img.gbcam");

        if addr >= 0x100 && addr < 0x1000 {
            *img.get(addr as usize - 0x100).unwrap_or(&0xff)
        } else {
            cart.ram_byte_absolute(addr)
        }
    }

    pub static MODEL: Model =
        Model { name:      "GB Camera",
                write_rom: write_rom,
                write_ram: write_ram,
                read_ram:  read_ram,
        };
}


/// Return a cartridge instance for a given cartridge type
pub fn from_id(id: u8) -> Model {
    match id {
        0           => mbc0::MODEL,
        0x01...0x03 => mbc1::MODEL,
        0x05...0x06 => mbc2::MODEL,
        0x0f...0x13 => mbc3::MODEL,
        0xfc        => camera::MODEL,
        _           => panic!("Unknown cartridge model 0x{:02x}", id),
    }
}
