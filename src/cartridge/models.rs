//! Cartridge model specific emulation

use super::Cartridge;
use super::ROM_BANK_SIZE;

/// Trait interface to model-specific operations
pub trait Model {
    /// Return string identifier
    fn name(&self) -> &'static str;
    /// Handle ROM write
    fn write(&self, cart: &mut Cartridge, addr: u16, val: u8);
}

struct Mbc1;

impl Model for Mbc1 {
    fn name(&self) -> &'static str {
        "MBC1"
    }

    fn write(&self, cart: &mut Cartridge, addr: u16, val: u8) {
        if addr >= 0x2000 && addr < 0x4000 {
            // Select a new rom bank
            let high_bank_offset = ROM_BANK_SIZE *
                match val & 0x1f {
                    /// We can't select bank 0, it defaults to 1
                    0 => 0,
                    n => (n - 1) as uint,
                };

            cart.set_high_bank(high_bank_offset);
        } else {
            debug!("Unhandled ROM write: {:04x} {:02x}", addr, val);
        }
    }
}

pub static MBC1: Mbc1 = Mbc1;
