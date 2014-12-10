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
        if addr < 0x2000 {
            // Writing a low nibble 0xa to anywhere in that address
            // range removes RAM write protect, All other values
            // enable it.
            cart.set_ram_wp(val & 0xf != 0xa);
        } else if addr >= 0x2000 && addr < 0x4000 {
            // Select a new ROM bank, bits [4:0]
            let cur_bank = cart.rom_bank() & !0x1f;

            cart.set_rom_bank(cur_bank | (val & 0x1f));
        } else if addr >= 0x4000 && addr < 0x6000 {
            // Select a new ROM bank, bits [6:5]
            let cur_bank = cart.rom_bank() & !0x60;

            cart.set_rom_bank(cur_bank | ((val << 5) & 0x60));
        } else if addr >= 0x6000 && addr < 0x8000 {
            // Switch RAM/ROM split mode
            if val & 1 != 0 {
                panic!("Unsupported MBC1 mode");
            }
        } else {
            debug!("Unhandled ROM write: {:04x} {:02x}", addr, val);
        }
    }

    pub static MODEL: Model = Model { name: "MBC1", write: write };
}

/// Return a cartridge instance for a given cartridge type
pub fn from_id(id: u8) -> Model {
    match id {
        0     => mbc0::MODEL,
        1...3 => mbc1::MODEL,
        _     => panic!("Unknown cartridge model {:02x}", id),
    }
}
