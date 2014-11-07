 //! Game Boy CPU instructions

use super::Cpu;

/// Instruction description
pub struct Instruction {
    /// Instruction implementation
    pub execute: fn (&mut Cpu),
    /// Number of CPU machine cycles taken by the instruction to
    /// execute. One machine cycle is equal to 4 clock cycles.
    pub cycles:  u32,
}

/// For multi-byte instructions: return the byte at `pc` and increment `pc`
fn next_byte(cpu: &mut Cpu) -> u8 {
    let pc = cpu.pc();

    let b = cpu.fetch_byte(pc);

    cpu.set_pc(pc + 1);

    b
}

/// For multi-byte instructions: return the 16bit word at `pc` and
/// increment `pc`
fn next_word(cpu: &mut Cpu) -> u16 {
    let pc = cpu.pc();

    cpu.set_pc(pc + 2);

    0x4242
}

/// Retreive two bytes from the stack and increment the stack pointer
fn pop_word(cpu: &mut Cpu) -> u16 {
    let sp = cpu.sp();

    cpu.set_sp(sp + 2);

    0x3232
}

/// Array of Instructions, the array index is the 8bit opcode.
pub static OPCODES: [Instruction, ..0x100] = [
    // Opcodes 0X
    Instruction { cycles: 1, execute: nop },
    Instruction { cycles: 3, execute: ld_bc_nn },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 2, execute: ld_b_n },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 2, execute: ld_c_n },
    Instruction { cycles: 0, execute: nop },
    // Opcodes 1X
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 3, execute: ld_de_nn },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 2, execute: ld_d_n },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 2, execute: ld_e_n },
    Instruction { cycles: 0, execute: nop },
    // Opcodes 2X
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 3, execute: ld_hl_nn },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 2, execute: ld_h_n },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 2, execute: ld_l_n },
    Instruction { cycles: 0, execute: nop },
    // Opcodes 3X
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 3, execute: ld_sp_nn },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    // Opcodes 4X
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    // Opcodes 5X
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    // Opcodes 6X
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    // Opcodes 7X
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 1, execute: ld_a_b },
    Instruction { cycles: 1, execute: ld_a_c },
    Instruction { cycles: 1, execute: ld_a_d },
    Instruction { cycles: 1, execute: ld_a_e },
    Instruction { cycles: 1, execute: ld_a_h },
    Instruction { cycles: 1, execute: ld_a_l },
    Instruction { cycles: 1, execute: nop },
    Instruction { cycles: 1, execute: ld_a_a },
    // Opcodes 8X
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    // Opcodes 9X
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    // Opcodes AX
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    // Opcodes BX
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    // Opcodes CX
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    // Opcodes DX
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    // Opcodes EX
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    // Opcodes FX
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 3, execute: pop_af },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
    Instruction { cycles: 0, execute: nop },
];

/// No operation
fn nop(_: &mut Cpu) {
}

/// Load 8 bit immediate value into `B`
fn ld_b_n(cpu: &mut Cpu) {
    let n = next_byte(cpu);
    cpu.set_b(n);
}

/// Load 8 bit immediate value into `C`
fn ld_c_n(cpu: &mut Cpu) {
    let n = next_byte(cpu);
    cpu.set_c(n);
}

/// Load 8 bit immediate value into `D`
fn ld_d_n(cpu: &mut Cpu) {
    let n = next_byte(cpu);
    cpu.set_d(n);
}

/// Load 8 bit immediate value into `E`
fn ld_e_n(cpu: &mut Cpu) {
    let n = next_byte(cpu);
    cpu.set_e(n);
}

/// Load 8 bit immediate value into `H`
fn ld_h_n(cpu: &mut Cpu) {
    let n = next_byte(cpu);
    cpu.set_h(n);
}

/// Load 8 bit immediate value into `L`
fn ld_l_n(cpu: &mut Cpu) {
    let n = next_byte(cpu);
    cpu.set_l(n);
}

/// Load `A` into `A` (NOP)
fn ld_a_a(_: &mut Cpu) {
}

/// Load `B` into `A`
fn ld_a_b(cpu: &mut Cpu) {
    let v = cpu.b();

    cpu.set_a(v);
}

/// Load `C` into `A`
fn ld_a_c(cpu: &mut Cpu) {
    let v = cpu.c();

    cpu.set_a(v);
}

/// Load `D` into `A`
fn ld_a_d(cpu: &mut Cpu) {
    let v = cpu.d();

    cpu.set_a(v);
}

/// Load `E` into `A`
fn ld_a_e(cpu: &mut Cpu) {
    let v = cpu.e();

    cpu.set_a(v);
}

/// Load `H` into `A`
fn ld_a_h(cpu: &mut Cpu) {
    let v = cpu.h();

    cpu.set_a(v);
}

/// Load `L` into `A`
fn ld_a_l(cpu: &mut Cpu) {
    let v = cpu.l();

    cpu.set_a(v);
}

/// Load 16bits immediate value into `BC`
fn ld_bc_nn(cpu: &mut Cpu) {
    let n = next_word(cpu);

    cpu.set_bc(n);
}

/// Load 16bits immediate value into `DE`
fn ld_de_nn(cpu: &mut Cpu) {
    let n = next_word(cpu);

    cpu.set_de(n);
}

/// Load 16bits immediate value into `HL`
fn ld_hl_nn(cpu: &mut Cpu) {
    let n = next_word(cpu);

    cpu.set_hl(n);
}

/// Load 16bits immediate value into `SP`
fn ld_sp_nn(cpu: &mut Cpu) {
    let n = next_word(cpu);

    cpu.set_sp(n);
}

/// Pop `AF` from the stack
fn pop_af(cpu: &mut Cpu) {
    let n = pop_word(cpu);

    cpu.set_af(n);
}
