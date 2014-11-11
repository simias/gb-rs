//! Input/Output abstraction for memory, ROM and I/O mapped registers

use gpu::Gpu;

pub mod rom;
pub mod ram;

/// Interconnect struct used by the CPU and GPU to access the ROM, RAM
/// and registers
pub struct Interconnect<'a> {
    rom:   rom::Rom,
    ram:   ram::Ram,
    iram:  ram::Ram,
    zpage: ram::Ram,
    gpu:   Gpu<'a>,
    /// Used to store the value of IO Port when not properly
    /// implemented.
    io:   [u8, ..0x4c],
}

impl<'a> Interconnect<'a> {
    /// Create a new Interconnect
    pub fn new<'n>(rom: rom::Rom, gpu: Gpu<'n>) -> Interconnect<'n> {
        // Bankable RAM
        let ram = ram::Ram::new(0x2000);
        // internal RAM
        let iram = ram::Ram::new(0x2000);
        // 0-page RAM
        let zpage = ram::Ram::new(0x7f);
        // IO mapped registers
        let io = [0, ..0x4c];

        Interconnect { rom:   rom,
                       ram:   ram,
                       iram:  iram,
                       zpage: zpage,
                       gpu:   gpu,
                       io:    io,
        }
    }

    pub fn reset(&mut self) {
        self.ram.reset();
        self.iram.reset();
        self.gpu.reset();
        self.zpage.reset();

        for b in self.io.iter_mut() {
            *b = 0;
        }
    }

    pub fn step(&mut self) {
        self.gpu.step();
    }

    /// Get byte from peripheral mapped at `addr`
    pub fn get_byte(&self, addr: u16) -> u8 {

        if let Some(off) = map::in_range(addr, map::ROM) {
            return self.rom.get_byte(off);
        }

        if let Some(off) = map::in_range(addr, map::VRAM) {
            return self.gpu.get_vram(off);
        }

        if let Some(off) = map::in_range(addr, map::RAM_BANK) {
            return self.ram.get_byte(off);
        }

        if let Some(off) = map::in_range(addr, map::IRAM) {
            return self.iram.get_byte(off);
        }

        if let Some(off) = map::in_range(addr, map::IRAM_ECHO) {
            return self.iram.get_byte(off);
        }

        if let Some(off) = map::in_range(addr, map::OAM) {
            return self.gpu.get_oam(off);
        }

        if let Some(off) = map::in_range(addr, map::IO) {
            return self.get_io(off);
        }

        if let Some(off) = map::in_range(addr, map::ZERO_PAGE) {
            return self.zpage.get_byte(off);
        }

        if addr == map::IEN {
            return 0;
        }

        println!("Read from unmapped memory {:04x}", addr);
        0
    }

    /// Store `val` into peripheral mapped at `addr`
    pub fn set_byte(&mut self, addr: u16, val: u8) {
        if let Some(_) = map::in_range(addr, map::ROM) {
            println!("Writing to ROM: {:04x}: {:02x}", addr, val);
        }

        if let Some(off) = map::in_range(addr, map::VRAM) {
            return self.gpu.set_vram(off, val);
        }

        if let Some(off) = map::in_range(addr, map::RAM_BANK) {
            return self.ram.set_byte(off, val);
        }

        if let Some(off) = map::in_range(addr, map::IRAM) {
            return self.iram.set_byte(off, val);
        }

        if let Some(off) = map::in_range(addr, map::IRAM_ECHO) {
            return self.iram.set_byte(off, val);
        }

        if let Some(off) = map::in_range(addr, map::OAM) {
            return self.gpu.set_oam(off, val);
        }

        if let Some(off) = map::in_range(addr, map::IO) {
            return self.set_io(off, val);
        }

        if let Some(off) = map::in_range(addr, map::ZERO_PAGE) {
            return self.zpage.set_byte(off, val);
        }

        if addr == map::IEN {
            println!("Interrupt enable {:02x}", val);
        }

        println!("Write to unmapped memory {:04x}: {:02x}", addr, val);
    }

    /// Retrieve value from IO port
    fn get_io(&self, addr: u16) -> u8 {
        match addr {
            io_map::LCD_LY => {
                // LY register
                return self.gpu.get_line()
            }
            io_map::LCD_BGP => {
                return self.gpu.bgp()
            }
            _ => {
                println!("Unhandled IO read from 0x{:04x}", addr);

            }
        }

        self.io[(addr & 0xff) as uint]
    }

    /// Set value of IO port
    fn set_io(&mut self, addr: u16, val: u8) {
        self.io[(addr & 0xff) as uint] = val;

        match addr {
            io_map::LCD_LY => {
                panic!("Unhandled write to LY register");
            },
            io_map::LCD_BGP => {
                return self.gpu.set_bgp(val)
            }
            io_map::LCDC => {
                return self.gpu.set_lcdc(val);
            },
            _ => {
                println!("Unhandled IO write to 0x{:02x}: 0x{:02x}", addr, val);
            }
        }
    }
}

mod map {
    //! Game Boy memory map. Memory ranges are inclusive.

    /// ROM
    pub const ROM:       (u16, u16) = (0x0000, 0x7fff);
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

    /// Return `Some(offset)` if the given address is in the inclusive
    /// range `range`, Where `offset` is an u16 equal to the offset of
    /// `addr` within the `range`.
    pub fn in_range(addr: u16, range: (u16, u16)) -> Option<u16> {
        let (first, last) = range;

        if addr >= first && addr <= last {
            Some(addr - first)
        } else {
            None
        }
    }
}

mod io_map {
    //! IO Address Map (offset from 0xff00)

    /// LCD Control
    pub const LCDC:     u16 = 0x40;
    /// Currently displayed line
    pub const LCD_LY:   u16 = 0x44;
    /// Background palette
    pub const LCD_BGP:  u16 = 0x47;

}
