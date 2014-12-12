//! Cartridge model specific emulation

use super::Cartridge;

/// Interface to model-specific operations
pub struct Model {
    /// String identifier
    pub name: &'static str,
    /// Handle ROM write
    pub write: fn(cart: &mut Cartridge, addr: u16, val: u8),
}

mod mbc0 {
    use super::Model;
    use cartridge::Cartridge;

    fn write(_: &mut Cartridge, addr: u16, val: u8) {
        debug!("Unhandled ROM write: {:04x} {:02x}", addr, val);
    }

    pub static MODEL: Model = Model { name: "MBC0", write: write };
}

mod mbc1 {
    use super::Model;
    use cartridge::Cartridge;

    fn write(cart: &mut Cartridge, addr: u16, val: u8) {
        match addr {
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
                debug!("Unhandled ROM write: {:04x} {:02x}", addr, val),
        }
    }

    pub static MODEL: Model = Model { name: "MBC1", write: write };
}

mod mbc3 {
    use super::Model;
    use cartridge::Cartridge;

    fn write(cart: &mut Cartridge, addr: u16, val: u8) {
        match addr {
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
                debug!("Unhandled RTC access"),
            _ =>
                debug!("Unhandled ROM write: {:04x} {:02x}", addr, val),
        }
    }

    pub static MODEL: Model = Model { name: "MBC1", write: write };
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
