//! CPU emulation

use std::fmt::{Show, Formatter, FormatError};

/// CPU state. Should be considered undetermined as long as
/// `Cpu::reset()` hasn't been called.
pub struct Cpu {
    regs:  Registers,
    flags: Flags,
}

impl Cpu {
    /// Create a new Cpu instance. The register's value should be
    /// treated as undetermined at that point so I fill them with
    /// garbage values (`0xbaad`).
    pub fn new() -> Cpu {
        Cpu {
            regs: Registers { pc: 0xbaad,
                              sp: 0xbaad,
                              af: 0xbaad,
                              bc: 0xbaad,
                              de: 0xbaad,
                              hl: 0xbaad,
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
        self.regs.af = 0;
        self.regs.bc = 0;
        self.regs.de = 0;
        self.regs.hl = 0;

        self.flags.z = false;
        self.flags.n = false;
        self.flags.h = false;
        self.flags.c = false;
    }
}

impl Show for Cpu {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> {
        try!(writeln!(f, "Registers:"));
        try!(write!(f, "{}", self.regs));

        try!(writeln!(f, "Flags:"));
        try!(write!(f, "{}", self.flags));

        Ok(())
    }
}

/// CPU registers. They're 16bit wide but some of them can be accessed
/// as high and low byte.
struct Registers {
    /// Program Counter
    pc: u16,
    /// Stack Pointer
    sp: u16,
    /// a[15:8] f[7:0] register pair
    af: u16,
    /// b[15:8] c[7:0] register pair
    bc: u16,
    /// d[15:8] e[7:0] register pair
    de: u16,
    /// h[15:8] l[7:0] register pair
    hl: u16,
}

impl Registers {
    /// Retreive the value of the `a` 8bit register
    fn a(&self) -> u8 {
        hi_byte(self.af)
    }

    /// Retreive the value of the `f` 8bit register
    fn f(&self) -> u8 {
        lo_byte(self.af)
    }

    /// Retreive the value of the `b` 8bit register
    fn b(&self) -> u8 {
        hi_byte(self.bc)
    }

    /// Retreive the value of the `c` 8bit register
    fn c(&self) -> u8 {
        lo_byte(self.bc)
    }

    /// Retreive the value of the `d` 8bit register
    fn d(&self) -> u8 {
        hi_byte(self.de)
    }

    /// Retreive the value of the `e` 8bit register
    fn e(&self) -> u8 {
        lo_byte(self.de)
    }

    /// Retreive the value of the `h` 8bit register
    fn h(&self) -> u8 {
        hi_byte(self.hl)
    }

    /// Retreive the value of the `l` 8bit register
    fn l(&self) -> u8 {
        lo_byte(self.hl)
    }
}

/// Fetch high 8bit from a 16bit word
fn hi_byte(v: u16) -> u8 {
    (v >> 8) as u8
}

/// Fetch low 8bit from a 16bit word
fn lo_byte(v: u16) -> u8 {
    v as u8
}

impl Show for Registers {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> {

        try!(writeln!(f, "  pc: 0x{:04x}", self.pc));
        try!(writeln!(f, "  sp: 0x{:04x}", self.sp));

        try!(writeln!(f, "  af: 0x{:04x}    a: {:3u}    f: {:3u}",
                      self.af, self.a(), self.f()));
        try!(writeln!(f, "  bc: 0x{:04x}    b: {:3u}    c: {:3u}",
                      self.bc, self.b(), self.c()));
        try!(writeln!(f, "  de: 0x{:04x}    d: {:3u}    d: {:3u}",
                      self.de, self.d(), self.e()));
        try!(writeln!(f, "  hl: 0x{:04x}    h: {:3u}    l: {:3u}",
                      self.hl, self.h(), self.l()));

        Ok(())
    }
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

impl Show for Flags {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> {

        try!(writeln!(f, "  z: {}  n: {}  h: {}  c: {}",
                      self.z as int,
                      self.n as int,
                      self.h as int,
                      self.c as int));

        Ok(())
    }
}
