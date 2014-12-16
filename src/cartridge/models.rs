//! Cartridge model specific emulation

use super::Cartridge;

/// Interface to model-specific operations
#[deriving(Copy)]
pub struct Model {
    /// String identifier
    pub name:      &'static str,
    /// Handle ROM write
    pub write_rom: fn(cart: &mut Cartridge, offset: u16, val: u8),
    /// Handle RAM write. `index` is an absolute index in the RAM
    /// memory array.
    pub write_ram: fn(cart: &mut Cartridge, index: uint, val: u8),
    /// Handle RAM read. `index` is an absolute index in the RAM
    /// memory array.
    pub read_ram:  fn(cart: &Cartridge, index: uint) -> u8,
}

/// Default implementation of write_ram, suitable for most cartridges
fn write_ram(cart: &mut Cartridge, index: uint, val: u8) {
    if let Some(b) = cart.ram_byte_absolute_mut(index) {
        *b = val;
    }
}

/// Default implementation of read_ram, suitable for most cartridges
fn read_ram(cart: &Cartridge, index: uint) -> u8 {
    cart.ram_byte_absolute(index)
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
    use cartridge::Cartridge;

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

                cart.set_rom_bank(cur_bank | (val & 0x1f));
            }
            0x4000...0x5fff =>
                if cart.bank_ram() {
                    // Select a new RAM bank
                    cart.set_ram_bank(val & 0x3);
                } else {
                    // Select a new ROM bank, bits [6:5]
                    let cur_bank = cart.rom_bank() & !0x60;

                    cart.set_rom_bank(cur_bank | ((val << 5) & 0x60));
                },
            0x6000...0x7fff =>
                // Switch RAM/ROM banking mode
                cart.set_bank_ram(val & 1 != 0),
            _ =>
                debug!("Unhandled ROM write: {:04x} {:02x}", offset, val),
        }
    }

    pub static MODEL: Model =
        Model { name:      "MBC1",
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
                cart.set_rom_bank(val & 0x7f),
            0x4000...0x5fff =>
                // Select a new RAM bank
                cart.set_ram_bank(val),
            0x6000...0x7fff =>
                if val & 1 == 0 {
                    // Unlatch the RTC
                    cart.rtc_freerun();
                } else {
                    // Attempt to latch the RTC
                    cart.rtc_latch();
                },
            _ =>
                debug!("Unhandled ROM write: {:04x} {:02x}", offset, val),
        }
    }

    /// MBC3 supports an RTC which is mapped in RAM banks
    fn read_ram(cart: &Cartridge, index: uint) -> u8 {
        let bank = cart.ram_bank();

        let date = cart.rtc_active_date();

        match bank {
            // Regular RAM
            0...3 => super::read_ram(cart, index),
            // Unmapped, not sure what I'm supposed to do, let's
            // return 0 for now
            4...7 => 0,
            // RTC registers start here
            0x8   => date.seconds(),
            0x9   => date.minutes(),
            0xa   => date.hours(),
            // Days bits [7:0]
            0xb   => date.days() as u8,
            0xc   => 0,
            // Unmapped
            _     => 0,
        }
    }

    pub static MODEL: Model =
        Model { name:     "MBC1",
                write_rom: write_rom,
                write_ram: super::write_ram,
                read_ram:  read_ram,
        };
}

/// Return a cartridge instance for a given cartridge type
pub fn from_id(id: u8) -> Model {
    match id {
        0           => mbc0::MODEL,
        0x01...0x03 => mbc1::MODEL,
        0x0f...0x13 => mbc3::MODEL,
        _           => panic!("Unknown cartridge model 0x{:02x}", id),
    }
}
