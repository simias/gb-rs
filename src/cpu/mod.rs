//! Game Boy CPU emulation

use std::fmt::{Show, Formatter, FormatError};
use io::Interconnect;

use cpu::instructions::{next_instruction, nop};

mod instructions;

/// CPU state. Should be considered undetermined as long as
/// `Cpu::reset()` hasn't been called.
pub struct Cpu {
    // Instruction currently being executed
    current_instruction: fn (&mut Cpu),
    // Time remaining for the instruction to finish
    instruction_delay:   u32,
    // CPU registers (except for `F` register)
    regs:                Registers,
    // CPU flags (`F` register)
    flags:               Flags,
    // Interconnect to access external ressources (RAM, ROM, peripherals...)
    inter:               Interconnect,
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

impl Cpu {
    /// Create a new Cpu instance. The register's value should be
    /// treated as undetermined at that point so I fill them with
    /// garbage values.
    pub fn new (rom: ::io::rom::Rom) -> Cpu {
        Cpu {
            // Use NOP as default instruction.
            current_instruction: nop,
            instruction_delay:   0,
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
            },
            inter: Interconnect::new(rom),
        }
    }

    /// Reset CPU state to power up values
    pub fn reset(&mut self) {
        self.inter.reset();

        self.current_instruction = nop;
        self.instruction_delay   = 0;

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

        self.clear_flags();
    }

    /// Called at each tick of the system clock. Move the emulated
    /// state one step forward.
    pub fn step(&mut self) {
        self.inter.step();

        // Are we done running the current instruction?
        if self.instruction_delay > 0 {
            // Nope, wait for the next cycle
            self.instruction_delay -= 1;
            return;
        }

        // The instruction should have finished executed, update CPU state
        (self.current_instruction)(self);

        //println!("{}", *self);

        // Now we fetch the next instruction
        let (delay, instruction) = next_instruction(self);

        // Instruction delays are in CPU Machine Cycles. There's 4
        // Clock cycles in one Machine Cycle.
        self.instruction_delay   = delay * 4 - 1;
        self.current_instruction = instruction;
    }

    /// Fetch byte at `addr` from the interconnect
    fn fetch_byte(&self, addr: u16) -> u8 {
        self.inter.get_byte(addr)
    }

    /// Store byte `val` at `addr` in the interconnect
    fn store_byte(&self, addr: u16, val: u8) {
        self.inter.set_byte(addr, val)
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

    /// Clear all flags
    fn clear_flags(&mut self) {
        self.flags.z = false;
        self.flags.n = false;
        self.flags.h = false;
        self.flags.c = false;
    }

    /// Get value of 'Z' flag
    fn zero(&self) -> bool {
        self.flags.z
    }

    /// set value of 'Z' flag
    fn set_zero(&mut self, s: bool) {
        self.flags.z = s;
    }

    /// Get value of 'C' flag
    fn carry(&self) -> bool {
        self.flags.c
    }

    /// Set value of 'C' flag
    fn set_carry(&mut self, s: bool) {
        self.flags.c = s;
    }

    /// Set value of 'H' flag
    fn set_halfcarry(&mut self, s: bool) {
        self.flags.h = s;
    }

    /// Set value of 'N' flag
    fn set_substract(&mut self, s: bool) {
        self.flags.n = s;
    }

    /// Disable Interrupts
    fn disable_interrupts(&mut self) {
        // TODO
    }

    /// Enable Interrupts
    fn enable_interrupts(&mut self) {
        // TODO
    }
}

impl Show for Cpu {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FormatError> {
        try!(writeln!(f, "Registers:"));

        try!(writeln!(f, "  pc: 0x{:04x} [{:02X}]",
                      self.pc(), self.fetch_byte(self.pc())));
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
