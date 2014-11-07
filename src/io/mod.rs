//! Input/Output abstraction for memory, ROM and I/O mapped registers

pub mod rom;

/// Interconnect struct used by the CPU and GPU to access the ROM, RAM
/// and registers
pub struct Interconnect {
    rom: rom::Rom,
}

impl Interconnect {
    /// Create a new Interconnect
    pub fn new(rom: rom::Rom) -> Interconnect {
        Interconnect { rom: rom }
    }

    pub fn get_byte(&self, addr: u16) -> u8 {
        let (periph, offset) = self.get_peripheral(addr);

        periph.get_byte(offset)
    }

    /// Find the peripheral corresponding to the address space pointed
    /// to by `addr` and return a reference to this peripheral as well
    /// as the offset within the address space.
    fn get_peripheral(&self, addr: u16) -> (&Addressable, u16) {
        if addr < 0x8000 {
            (&self.rom, addr - 0x0000)
        } else {
            panic!("Unimplemented interconnect access at 0x{:04x}", addr);
        }
    }
}

/// Common trait for all I/O ressources (ROM, RAM, registers...)
trait Addressable {
    /// Return byte at `offset`
    fn get_byte(&self, offset: u16) -> u8;

    /// Set byte at `offset`. If this is implemented it should use
    /// internal mutability to allow shared references (hence the
    /// `&self`).
    fn set_byte(&self, offset: u16, val: u8) {
        // TODO(lionel) there should be a better way to handle that
        // type of errors. It should probably bubble up.
        println!("Writing to read-only memory [0x{:04x}]: 0x{:02x}", offset, val);
    }
}
