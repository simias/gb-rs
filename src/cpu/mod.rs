//! Game Boy CPU emulation

use std::fmt::{Show, Formatter, FormatError};
use io::Interconnect;

mod instructions;

/// CPU state. Should be considered undetermined as long as
/// `Cpu::reset()` hasn't been called.
pub struct Cpu<'a> {
    regs:  Registers,
    flags: Flags,
    inter: &'a Interconnect,
}

/// CPU registers. They're 16bit wide but some of them can be accessed
/// as high and low byte.
struct Registers {
    /// 16bit Program Counter
    pc: u16,
    /// 16bit Stack Pointer
    sp: u16,
    /// 8bit `A` register
    a:  u8,
    /// 8bit `B` register
    b:  u8,
    /// 8bit `C` register
    c:  u8,
    /// 8bit `D` register
    d:  u8,
    /// 8bit `E` register
    e:  u8,
    /// 8bit `H` register
    h:  u8,
    /// 8bit `L` register
    l:  u8,
}

/// Flags contain `bool`s which are set or unset as a side effect of
/// the commands being executed. In turn, certain commands change
/// their behaviour based on the flag values.
struct Flags {
    /// Zero: set if the result of a math operation is zero or two
    /// values compare equal
    z: bool,
    /// Substract Flag: set if the last math operation performed a
    /// substraction
    n: bool,
    /// Half Carry Flag: set if a carry occurred from the lower nibble
    /// in the last math operation.
    h: bool,
    /// Carry Flag: set if a carry occured during the last math
    /// operation or if the first operand register compared smaller.
    c: bool,
}

impl<'a> Cpu<'a> {
    /// Create a new Cpu instance. The register's value should be
    /// treated as undetermined at that point so I fill them with
    /// garbage values.
    pub fn new<'a> (inter: &'a Interconnect) -> Cpu<'a> {
        Cpu {
            inter: inter,
            regs: Registers { pc: 0xbaad,
                              sp: 0xbaad,
                              a : 0x01,
                              b : 0x23,
                              c : 0x45,
                              d : 0x67,
                              e : 0x89,
                              h : 0xcd,
                              l : 0xef,
            },
            flags: Flags { z: false,
                           n: false,
                           h: false,
                           c: false,
            }
        }
    }

    /// Reset CPU state to power up values
    pub fn reset(&mut self) {
        // Code always starts at 0x100
        self.regs.pc = 0x100;
        // Stack pointer default value
        self.regs.sp = 0xfffe;
        self.regs.a  = 0;
        self.regs.b  = 0;
        self.regs.c  = 0;
        self.regs.d  = 0;
        self.regs.e  = 0;
        self.regs.h  = 0;
        self.regs.l  = 0;

        self.flags.z = false;
        self.flags.n = false;
        self.flags.h = false;
        self.flags.c = false;
    }

    pub fn step(&mut self) {

        let op = self.fetch_byte(self.regs.pc) as uint;

        self.regs.pc += 1;

        let instruction = &instructions::OPCODES[op];

        if instruction.cycles == 0 {
            panic!("Unimplemented instruction {:02x}!", op);
        }

        (instruction.execute)(self);
    }

    fn fetch_byte(&self, addr: u16) -> u8 {
        self.inter.get_byte(addr)
    }

    /// Retrieve value of the `PC` register
    fn pc(&self) -> u16 {
        self.regs.pc
    }

    /// Set value of the `PC` register
    fn set_pc(&mut self, pc: u16) {
        self.regs.pc = pc;
    }

    /// Retrieve value of the `SP` register
    fn sp(&self) -> u16 {
        self.regs.sp
    }

    /// Set value of the `SP` register
    fn set_sp(&mut self, sp: u16) {
        self.regs.sp = sp;
    }

    /// Retrieve value of the `AF` register
    fn af(&self) -> u16 {
        let mut v = self.f() as u16;

        v |= (self.regs.a as u16) << 8;

        v
    }

    /// Set value of the `AF` register
    fn set_af(&mut self, af: u16) {
        self.regs.a = (af >> 8) as u8;
        self.set_f(af as u8);
    }

    /// Retrieve value of the `BC` register
    fn bc(&self) -> u16 {
        let mut v = self.regs.c as u16;

        v |= (self.regs.b as u16) << 8;

        v
    }

    /// Set value of the `BC` register
    fn set_bc(&mut self, bc: u16) {
        self.regs.b = (bc >> 8) as u8;
        self.regs.c = bc as u8;
    }

    /// Retrieve value of the `DE` register
    fn de(&self) -> u16 {
        let mut v = self.regs.e as u16;

        v |= (self.regs.d as u16) << 8;

        v
    }

    /// Set value of the `DE` register
    fn set_de(&mut self, de: u16) {
        self.regs.d = (de >> 8) as u8;
        self.regs.e = de as u8;
    }

    /// Retrieve value of the `HL` register
    fn hl(&self) -> u16 {
        let mut v = self.regs.l as u16;

        v |= (self.regs.h as u16) << 8;

        v
    }

    /// Set value of the `HL` register
    fn set_hl(&mut self, hl: u16) {
        self.regs.h = (hl >> 8) as u8;
        self.regs.l = hl as u8;
    }

    /// Retrieve value of the `A` register
    fn a(&self) -> u8 {
        self.regs.a
    }

    /// Set value of the `A` register
    fn set_a(&mut self, v: u8) {
        self.regs.a = v;
    }

    /// Retrieve value of the `B` register
    fn b(&self) -> u8 {
        self.regs.b
    }

    /// Set value of the `B` register
    fn set_b(&mut self, v: u8) {
        self.regs.b = v;
    }

    /// Retrieve value of the `C` register
    fn c(&self) -> u8 {
        self.regs.c
    }

    /// Set value of the `C` register
    fn set_c(&mut self, v: u8) {
        self.regs.c = v;
    }

    /// Retrieve value of the `D` register
    fn d(&self) -> u8 {
        self.regs.d
    }

    /// Set value of the `D` register
    fn set_d(&mut self, v: u8) {
        self.regs.d = v;
    }

    /// Retrieve value of the `E` register
    fn e(&self) -> u8 {
        self.regs.e
    }

    /// Set value of the `E` register
    fn set_e(&mut self, v: u8) {
        self.regs.e = v;
    }

    /// Retrieve value of the `F` register
    fn f(&self) -> u8 {
        let z = self.flags.z as u8;
        let n = self.flags.n as u8;
        let h = self.flags.h as u8;
        let c = self.flags.c as u8;

        (z << 7) | (n << 6) | ( h << 5) | (c << 4)
    }

    /// Set value of the `F` register
    fn set_f(&mut self, v: u8) {
        self.flags.z = (v & (1 << 7)) != 0;
        self.flags.n = (v & (1 << 6)) != 0;
        self.flags.h = (v & (1 << 5)) != 0;
        self.flags.c = (v & (1 << 4)) != 0;
    }

    /// Retrieve value of the `H` register
    fn h(&self) -> u8 {
        self.regs.h
    }

    /// Set value of the `H` register
    fn set_h(&mut self, v: u8) {
        self.regs.h = v;
    }

    /// Retrieve value of the `L` register
    fn l(&self) -> u8 {
        self.regs.l
    }

    /// Set value of the `L` register
    fn set_l(&mut self, v: u8) {
        self.regs.l = v;
    }
}

impl<'a> Show for Cpu<'a> {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> {
        try!(writeln!(f, "Registers:"));

        try!(writeln!(f, "  pc: 0x{:04x}", self.pc()));
        try!(writeln!(f, "  sp: 0x{:04x}", self.sp()));
        try!(writeln!(f, "  af: 0x{:04x}    a: {:3u}    f: {:3u}",
                      self.af(), self.a(), self.f()));
        try!(writeln!(f, "  bc: 0x{:04x}    b: {:3u}    c: {:3u}",
                      self.bc(), self.b(), self.c()));
        try!(writeln!(f, "  de: 0x{:04x}    d: {:3u}    d: {:3u}",
                      self.de(), self.d(), self.e()));
        try!(writeln!(f, "  hl: 0x{:04x}    h: {:3u}    l: {:3u}",
                      self.hl(), self.h(), self.l()));

        try!(writeln!(f, "Flags:"));

        try!(writeln!(f, "  z: {}  n: {}  h: {}  c: {}",
                      self.flags.z as int,
                      self.flags.n as int,
                      self.flags.h as int,
                      self.flags.c as int));

        Ok(())
    }
}
