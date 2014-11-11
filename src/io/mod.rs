//! Input/Output abstraction for memory, ROM and I/O mapped registers

use gpu::Gpu;

pub mod rom;
pub mod ram;

/// Interconnect struct used by the CPU and GPU to access the ROM, RAM
/// and registers
pub struct Interconnect<'a> {
    rom:  rom::Rom,
    ram:  ram::Ram,
    gpu:  Gpu<'a>,
    io:   [u8, ..0x100],
}

impl<'a> Interconnect<'a> {
    /// Create a new Interconnect
    pub fn new<'n>(rom: rom::Rom, gpu: Gpu<'n>) -> Interconnect<'n> {
        // 8kB video RAM  + 2 banks RAM
        let ram = ram::Ram::new(3 * 8 * 1024);
        // IO mapped registers
        let io = [0, ..0x100];

        Interconnect { rom: rom, ram: ram, gpu: gpu, io: io }
    }

    pub fn reset(&mut self) {
        self.ram.reset();
        self.gpu.reset();

        for b in self.io.iter_mut() {
            *b = 0;
        }
    }

    pub fn step(&mut self) {
        self.gpu.step();
    }

    /// Get byte from peripheral mapped at `addr`
    pub fn get_byte(&self, addr: u16) -> u8 {

        if map::in_range(addr, map::ROM_0) ||
           map::in_range(addr, map::ROM_BANK) {
            return self.rom.get_byte(addr);
        }

        if map::in_range(addr, map::VRAM)     ||
           map::in_range(addr, map::RAM_BANK) ||
           map::in_range(addr, map::IRAM) {
            return self.ram.get_byte(addr - map::range_start(map::VRAM));
        }

        if map::in_range(addr, map::IRAM_ECHO) {
            let iram_addr = addr
                - map::range_start(map::IRAM_ECHO)
                + map::range_start(map::IRAM);

            return self.ram.get_byte(iram_addr - map::range_start(map::VRAM));
        }

        if map::in_range(addr, map::OAM) {
            return self.gpu.get_oam((addr - map::range_start(map::OAM)) as u8)
        }

        if map::in_range(addr, map::IO)        ||
           map::in_range(addr, map::ZERO_PAGE) ||
           addr == map::IEN {
            return self.get_io(addr);
        }

        println!("Read from unmapped memory {:04x}", addr);
        0
    }

    /// Store `val` into peripheral mapped at `addr`
    pub fn set_byte(&mut self, addr: u16, val: u8) {

        if map::in_range(addr, map::ROM_0) ||
           map::in_range(addr, map::ROM_BANK) {
               println!("Writing to ROM: {:04x}: {:02x}", addr, val);
               return;
        }

        if map::in_range(addr, map::VRAM)     ||
           map::in_range(addr, map::RAM_BANK) ||
           map::in_range(addr, map::IRAM) {
            return self.ram.set_byte(addr - map::range_start(map::VRAM), val);
           }

        if map::in_range(addr, map::IRAM_ECHO) {
            let iram_addr = addr
                - map::range_start(map::IRAM_ECHO)
                + map::range_start(map::IRAM);

            return self.ram.set_byte(iram_addr - map::range_start(map::VRAM),
                                     val);
        }

        if map::in_range(addr, map::OAM) {
            return self.gpu.set_oam((addr - map::range_start(map::OAM)) as u8,
                                    val)
        }

        if map::in_range(addr, map::IO)        ||
           map::in_range(addr, map::ZERO_PAGE) ||
           addr == map::IEN {
            return self.set_io(addr, val);
        }

        println!("Write to unmapped memory {:04x}: {:02x}", addr, val);
    }

    /// Retrieve value from IO port
    fn get_io(&self, addr: u16) -> u8 {
        match addr {
            map::LCD_LY => {
                // LY register
                self.gpu.get_line()
            }
            _ => {
                println!("Unhandled IO read from 0x{:04x}", addr);
                self.io[(addr & 0xff) as uint]
            }
        }
    }

    /// Set value of IO port
    fn set_io(&mut self, addr: u16, val: u8) {

        match addr {
            map::LCD_LY => {
                panic!("Unhandled write to LY register");
            }
            _ => {
                println!("Unhandled IO write to 0x{:02x}: 0x{:04x}", addr, val)
                    self.io[(addr & 0xff) as uint] = val;
            }
        }
    }
}

mod map {
    //! Game Boy memory map. Memory ranges are inclusive.

    /// ROM Bank #0
    pub const ROM_0:     (u16, u16) = (0x0000, 0x3fff);
    /// ROM Bank N
    pub const ROM_BANK:  (u16, u16) = (0x4000, 0x7fff);
    /// Video RAM
    pub const VRAM:      (u16, u16) = (0x8000, 0x9fff);
    /// RAM Bank N
    pub const RAM_BANK:  (u16, u16) = (0xa000, 0xbfff);
    /// Internal RAM
    pub const IRAM:      (u16, u16) = (0xc000, 0xdfff);
    /// Internal RAM echo
    pub const IRAM_ECHO: (u16, u16) = (0xe000, 0xfdff);
    /// Object Attribute Memory
    pub const OAM:       (u16, u16) = (0xfe00, 0xfe9f);
    /// IO ports
    pub const IO:        (u16, u16) = (0xff00, 0xff4b);
    /// Zero page memory
    pub const ZERO_PAGE: (u16, u16) = (0xff80, 0xfffe);
    pub const IEN:       u16        = 0xffff;

    // IO ports description

    /// Currently displayed line
    pub const LCD_LY:   u16 = 0xff44;

    /// Return `true` if the given address is in the inclusive range
    /// `range`
    pub fn in_range(addr: u16, range: (u16, u16)) -> bool {
        let (first, last) = range;

        addr >= first && addr <= last
    }

    /// Return `range` start
    pub fn range_start(range: (u16, u16)) -> u16 {
        let (start, _) = range;

        start
    }
}
