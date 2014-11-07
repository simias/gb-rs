//! Input/Output abstraction for memory, ROM and I/O mapped registers

pub mod rom;

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
