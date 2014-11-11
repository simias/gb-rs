//! Input/Output abstraction for memory, ROM and I/O mapped registers

use std::cell::Cell;

use gpu::Gpu;

pub mod rom;
pub mod ram;

/// Interconnect struct used by the CPU and GPU to access the ROM, RAM
/// and registers
pub struct Interconnect<'a> {
    rom:  rom::Rom,
    ram:  ram::Ram,
    gpu:  Gpu<'a>,
    io:   Vec<Cell<u8>>,
}

impl<'a> Interconnect<'a> {
    /// Create a new Interconnect
    pub fn new<'n>(rom: rom::Rom, gpu: Gpu<'n>) -> Interconnect<'n> {
        // 8kB video RAM  + 2 banks RAM
        let ram = ram::Ram::new(3 * 8 * 1024);
        // IO mapped registers
        let io = Vec::from_elem(0x100, Cell::new(0));

        Interconnect { rom: rom, ram: ram, gpu: gpu, io: io }
    }

    pub fn reset(&mut self) {
        self.ram.reset();
        self.gpu.reset();

        for c in self.io.iter() {
            c.set(0);
        }
    }

    pub fn step(&mut self) {
        self.gpu.step();
    }

    /// Get byte from peripheral mapped at `addr`
    pub fn get_byte(&self, addr: u16) -> u8 {
        let (periph, offset) = self.get_peripheral(addr);

        periph.get_byte(offset)
    }

    /// Store `val` into peripheral mapped at `addr`
    pub fn set_byte(&self, addr: u16, val: u8) {
        let (periph, offset) = self.get_peripheral(addr);

        periph.set_byte(offset, val);
    }

    /// Find the peripheral corresponding to the address space pointed
    /// to by `addr` and return a reference to this peripheral as well
    /// as the offset within the address space.
    fn get_peripheral(&self, addr: u16) -> (&Addressable, u16) {
        if addr < 0x8000 {
            (&self.rom, addr - 0x0000)
        } else if addr < 0xe000 {
            (&self.ram, addr - 0x8000)
        } else if addr < 0xfe00 {
            (&UNMAPPED, addr)
        } else if addr < 0xfea0 {
            // OAM
            (&self.gpu, addr)
        } else if addr < 0xff00 {
            (&EMPTY, addr)
        } else {
            // Handle IO memory ourselves
            (self, addr - 0xff00)
        }
    }
}

/// Common trait for all I/O ressources (ROM, RAM, registers...)
pub trait Addressable {
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

/// IO register handling (0xff00 - 0xffff)
impl<'a> Addressable for Interconnect<'a> {
    fn get_byte(&self, offset: u16) -> u8 {
        match offset {
            0x44 => {
                // LY register
                self.gpu.get_line()
            }
            _ => {
                println!("Unhandled IO read from 0x{:02x}", offset);
                self.io[offset as uint].get()
            }
        }
    }

    fn set_byte(&self, offset: u16, val: u8) {
        match offset {
            0x44 => {
                panic!("Unhandled write to LY register");
            }
            _ => {
                println!("Unhandled IO write to 0x{:02x}: 0x{:02x}", offset, val)
                    self.io[offset as uint].set(val);
            }
        }
    }
}

struct Unmapped;

static UNMAPPED: Unmapped = Unmapped;

impl Addressable for Unmapped {
    fn get_byte(&self, offset: u16) -> u8 {
        panic!("Read from unmapped memory at 0x{:04x}", offset);
    }

    fn set_byte(&self, offset: u16, val: u8) {
        panic!("Write to unmapped memory at 0x{:04x}: 0x{:02x}", offset, val);
    }
}

struct Empty;

static EMPTY: Empty = Empty;

impl Addressable for Empty {
    fn get_byte(&self, offset: u16) -> u8 {
        println!("Read from empty memory at 0x{:04x}", offset);
        0
    }

    fn set_byte(&self, offset: u16, val: u8) {
        println!("Write to empty memory at 0x{:04x}: 0x{:02x}", offset, val);
    }
}
