 //! Game Boy CPU instructions

use cpu::Cpu;

/// Retrieve the next instruction to be executed.
///
/// Returns a tuple `(delay, instruction)` as described in `OPCODES`
pub fn next_instruction(cpu: &mut Cpu) -> (u32, fn (&mut Cpu)) {
    let pc = cpu.pc();

    cpu.set_pc(pc + 1);

    let op = cpu.fetch_byte(pc);

    let (delay, instruction) =
        if op != 0xcb {
            OPCODES[op as uint]
        } else {
            // 0xCB introduces a two-byte opcode
            cb::next_instruction(cpu)
        };

    if delay == 0 {
        panic!("Unimplemented instruction [{:02X}]", op);
    }

    (delay, instruction)
}

/// Array containing tuples `(delay, instruction)`.
///
/// `delay` is an `u32` describing how many machine cycles an
/// instruction takes to execute. One machine cycle is 4 clock cycles.
///
/// `instruction` is an `fn (&mut Cpu)` used to emulate the
/// instruction.
pub static OPCODES: [(u32, fn (&mut Cpu)), ..0x100] = [
    // Opcodes 0X
    (1, nop),
    (3, ld_bc_nn),
    (2, ld_mbc_a),
    (0, nop),
    (0, nop),
    (1, dec_b),
    (2, ld_b_n),
    (0, nop),
    (0, nop),
    (0, nop),
    (2, ld_a_mbc),
    (0, nop),
    (0, nop),
    (1, dec_c),
    (2, ld_c_n),
    (0, nop),
    // Opcodes 1X
    (0, nop),
    (3, ld_de_nn),
    (2, ld_mde_a),
    (0, nop),
    (0, nop),
    (1, dec_d),
    (2, ld_d_n),
    (0, nop),
    (2, jr_n),
    (0, nop),
    (2, ld_a_mde),
    (0, nop),
    (0, nop),
    (1, dec_e),
    (2, ld_e_n),
    (0, nop),
    // Opcodes 2X
    (2, jr_nz_n),
    (3, ld_hl_nn),
    (0, nop),
    (0, nop),
    (0, nop),
    (1, dec_h),
    (2, ld_h_n),
    (0, nop),
    (2, jr_z_n),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (1, dec_l),
    (2, ld_l_n),
    (0, nop),
    // Opcodes 3X
    (2, jr_nc_n),
    (3, ld_sp_nn),
    (2, ldd_a_mhl),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (2, jr_c_n),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (1, dec_a),
    (2, ld_a_n),
    (0, nop),
    // Opcodes 4X
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (1, ld_b_a),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (1, ld_c_a),
    // Opcodes 5X
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (1, ld_d_a),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (1, ld_e_a),
    // Opcodes 6X
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (1, ld_h_a),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (1, ld_l_a),
    // Opcodes 7X
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (2, ld_mhl_a),
    (1, ld_a_b),
    (1, ld_a_c),
    (1, ld_a_d),
    (1, ld_a_e),
    (1, ld_a_h),
    (1, ld_a_l),
    (2, ld_a_mhl),
    (1, ld_a_a),
    // Opcodes 8X
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    // Opcodes 9X
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    // Opcodes AX
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (1, xor_a_b),
    (1, xor_a_c),
    (1, xor_a_d),
    (1, xor_a_e),
    (1, xor_a_h),
    (1, xor_a_l),
    (0, nop),
    (1, xor_a_a),
    // Opcodes BX
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (1, cp_a_b),
    (1, cp_a_c),
    (1, cp_a_d),
    (1, cp_a_e),
    (1, cp_a_h),
    (1, cp_a_l),
    (2, cp_a_mhl),
    (1, cp_a_a),
    // Opcodes CX
    (0, nop),
    (0, nop),
    (3, jp_nz_nn),
    (3, jp_nn),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (3, jp_z_nn),
    (0, nop), // See CB opcode map
    (0, nop),
    (3, call),
    (0, nop),
    (0, nop),
    // Opcodes DX
    (0, nop),
    (0, nop),
    (3, jp_nc_nn),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (3, jp_c_nn),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    // Opcodes EX
    (3, ldh_a_mn),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (4, ld_mnn_a),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    // Opcodes FX
    (3, ldh_mn_a),
    (3, pop_af),
    (0, nop),
    (1, di),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (0, nop),
    (2, ld_a_mnn),
    (0, nop),
    (0, nop),
    (0, nop),
    (2, cp_a_n),
    (0, nop),
];

/// For multi-byte instructions: return the byte at `pc` and increment `pc`
fn next_byte(cpu: &mut Cpu) -> u8 {
    let pc = cpu.pc();

    let b = cpu.fetch_byte(pc);

    cpu.set_pc(pc + 1);

    b
}

/// For multi-byte instructions: return the word at `pc` and increment
/// `pc` twice
fn next_word(cpu: &mut Cpu) -> u16 {
    let b1 = next_byte(cpu) as u16;
    let b2 = next_byte(cpu) as u16;

    b1 | (b2 << 8)
}

/// Push one byte onto the stack and decrement the stack pointer
fn push_byte(cpu: &mut Cpu, val: u8){
    let mut sp = cpu.sp();

    sp -= 1;

    cpu.set_sp(sp);
    cpu.store_byte(sp, val);
}

/// Push two bytes onto the stack and decrement the stack pointer
/// twice
fn push_word(cpu: &mut Cpu, val: u16){
    push_byte(cpu, (val >> 8) as u8);
    push_byte(cpu, val as u8)
}

/// Retreive one byte from the stack and increment the stack pointer
fn pop_byte(cpu: &mut Cpu) -> u8 {
    let sp = cpu.sp();

    let b = cpu.fetch_byte(sp);

    cpu.set_sp(sp + 1);

    b
}

/// Retreive two bytes from the stack and increment the stack pointer
/// twice
fn pop_word(cpu: &mut Cpu) -> u16 {
    let lo = pop_byte(cpu) as u16;
    let hi = pop_byte(cpu) as u16;

    (hi << 8) | lo
}

/// Helper function to substract two `u8`s and update the CPU flags
fn sub_and_set_flags(cpu: &mut Cpu, x: u8, y: u8) -> u8 {
    let r = x - y;

    cpu.set_zero(r == 0);
    cpu.set_halfcarry((x & 0xf) < (y & 0xf));
    cpu.set_carry(x < y);
    cpu.set_substract(true);

    r
}

/// No operation
pub fn nop(_: &mut Cpu) {
}

/// Load 8 bit immediate value into `A`
fn ld_a_n(cpu: &mut Cpu) {
    let n = next_byte(cpu);
    cpu.set_a(n);
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

/// Load `[BC]` into `A`
fn ld_a_mbc(cpu: &mut Cpu) {
    let bc = cpu.bc();

    let v = cpu.fetch_byte(bc);

    cpu.set_a(v);
}

/// Load `[DE]` into `A`
fn ld_a_mde(cpu: &mut Cpu) {
    let de = cpu.de();

    let v = cpu.fetch_byte(de);

    cpu.set_a(v);
}

/// Load `[HL]` into `A`
fn ld_a_mhl(cpu: &mut Cpu) {
    let hl = cpu.hl();

    let v = cpu.fetch_byte(hl);

    cpu.set_a(v);
}

/// Load `[nn]` into `A`
fn ld_a_mnn(cpu: &mut Cpu) {
    let n = next_word(cpu);

    let v = cpu.fetch_byte(n);

    cpu.set_a(v);
}

/// Load `A` into `B`
fn ld_b_a(cpu: &mut Cpu) {
    let a = cpu.a();

    cpu.set_b(a);
}

/// Load `A` into `C`
fn ld_c_a(cpu: &mut Cpu) {
    let a = cpu.a();

    cpu.set_c(a);
}

/// Load `A` into `D`
fn ld_d_a(cpu: &mut Cpu) {
    let a = cpu.a();

    cpu.set_d(a);
}

/// Load `A` into `E`
fn ld_e_a(cpu: &mut Cpu) {
    let a = cpu.a();

    cpu.set_e(a);
}

/// Load `A` into `H`
fn ld_h_a(cpu: &mut Cpu) {
    let a = cpu.a();

    cpu.set_h(a);
}

/// Load `A` into `L`
fn ld_l_a(cpu: &mut Cpu) {
    let a = cpu.a();

    cpu.set_l(a);
}

/// Load `A` into `[BC]`
fn ld_mbc_a(cpu: &mut Cpu) {
    let a  = cpu.a();
    let bc = cpu.bc();

    cpu.store_byte(bc, a);
}

/// Load `A` into `[DE]`
fn ld_mde_a(cpu: &mut Cpu) {
    let a  = cpu.a();
    let de = cpu.de();

    cpu.store_byte(de, a);
}

/// Load `A` into `[HL]`
fn ld_mhl_a(cpu: &mut Cpu) {
    let a  = cpu.a();
    let hl = cpu.hl();

    cpu.store_byte(hl, a);
}

/// Load `A` into `[NN]`
fn ld_mnn_a(cpu: &mut Cpu) {
    let a  = cpu.a();
    let n = next_word(cpu);

    cpu.store_byte(n, a);
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

/// Unconditional jump to absolute address
fn jp_nn(cpu: &mut Cpu) {
    let addr = next_word(cpu);

    cpu.set_pc(addr);
}

/// Jump to absolute address if `!Z`
fn jp_nz_nn(cpu: &mut Cpu) {
    let addr = next_word(cpu);

    if !cpu.zero() {
        cpu.set_pc(addr);
    }
}

/// Jump to absolute address if `Z`
fn jp_z_nn(cpu: &mut Cpu) {
    let addr = next_word(cpu);

    if cpu.zero() {
        cpu.set_pc(addr);
    }
}

/// Jump to absolute address if `!C`
fn jp_nc_nn(cpu: &mut Cpu) {
    let addr = next_word(cpu);

    if !cpu.carry() {
        cpu.set_pc(addr);
    }
}

/// Jump to absolute address if `C`
fn jp_c_nn(cpu: &mut Cpu) {
    let addr = next_word(cpu);

    if cpu.carry() {
        cpu.set_pc(addr);
    }
}

/// Unconditional jump to relative address
fn jr_n(cpu: &mut Cpu) {
    let off = next_byte(cpu) as i8;

    let mut pc = cpu.pc() as i16;

    pc += off as i16;

    cpu.set_pc(pc as u16);
}

/// Jump to relative address if `!Z`
fn jr_nz_n(cpu: &mut Cpu) {
    let off = next_byte(cpu) as i8;

    if !cpu.zero() {
        let mut pc = cpu.pc() as i16;

        pc += off as i16;

        cpu.set_pc(pc as u16);
    }
}

/// Jump to relative address if `Z`
fn jr_z_n(cpu: &mut Cpu) {
    let off = next_byte(cpu) as i8;

    if cpu.zero() {
        let mut pc = cpu.pc() as i16;

        pc += off as i16;

        cpu.set_pc(pc as u16);
    }
}

/// Jump to relative address if `!C`
fn jr_nc_n(cpu: &mut Cpu) {
    let off = next_byte(cpu) as i8;

    if !cpu.carry() {
        let mut pc = cpu.pc() as i16;

        pc += off as i16;

        cpu.set_pc(pc as u16);
    }
}

/// Jump to relative address if `C`
fn jr_c_n(cpu: &mut Cpu) {
    let off = next_byte(cpu) as i8;

    if cpu.carry() {
        let mut pc = cpu.pc() as i16;

        pc += off as i16;

        cpu.set_pc(pc as u16);
    }
}

/// Push return address on stack and jump to immediate address
fn call(cpu: &mut Cpu) {
    let addr = next_word(cpu);
    let pc = cpu.pc();

    push_word(cpu, pc);

    cpu.set_pc(addr);
}

/// XOR `A` with itself (set `A` to `0`)
fn xor_a_a(cpu: &mut Cpu) {
    cpu.set_a(0);

    cpu.clear_flags();
    cpu.set_zero(true);
}

/// XOR `B` into `A`
fn xor_a_b(cpu: &mut Cpu) {
    let a = cpu.a();
    let b = cpu.b();

    let r = a ^ b;

    cpu.set_a(r);

    cpu.clear_flags();
    cpu.set_zero(r == 0);
}

/// XOR `C` into `A`
fn xor_a_c(cpu: &mut Cpu) {
    let a = cpu.a();
    let c = cpu.c();

    let r = a ^ c;

    cpu.set_a(r);

    cpu.clear_flags();
    cpu.set_zero(r == 0);
}

/// XOR `D` into `A`
fn xor_a_d(cpu: &mut Cpu) {
    let a = cpu.a();
    let d = cpu.d();

    let r = a ^ d;

    cpu.set_a(r);

    cpu.clear_flags();
    cpu.set_zero(r == 0);
}

/// XOR `E` into `A`
fn xor_a_e(cpu: &mut Cpu) {
    let a = cpu.a();
    let e = cpu.e();

    let r = a ^ e;

    cpu.set_a(r);

    cpu.clear_flags();
    cpu.set_zero(r == 0);
}

/// XOR `H` into `A`
fn xor_a_h(cpu: &mut Cpu) {
    let a = cpu.a();
    let h = cpu.h();

    let r = a ^ h;

    cpu.set_a(r);

    cpu.clear_flags();
    cpu.set_zero(r == 0);
}

/// XOR `L` into `A`
fn xor_a_l(cpu: &mut Cpu) {
    let a = cpu.a();
    let l = cpu.l();

    let r = a ^ l;

    cpu.set_a(r);

    cpu.clear_flags();
    cpu.set_zero(r == 0);
}

/// Store `A` into `[HL]` and decrement `HL`
fn ldd_a_mhl(cpu: &mut Cpu) {
    let hl = cpu.hl();
    let a  = cpu.a();

    cpu.store_byte(hl, a);

    cpu.set_hl(hl - 1);
}

/// Store `A` into `[0xff00 + n]`
fn ldh_a_mn(cpu: &mut Cpu) {
    let n = next_byte(cpu) as u16;
    let a = cpu.a();

    cpu.store_byte(0xff00 | n, a);
}

/// Load `[0xff00 + n]` into `[A]`
fn ldh_mn_a(cpu: &mut Cpu) {
    let n = next_byte(cpu) as u16;
    let v = cpu.fetch_byte(0xff00 | n);

    cpu.set_a(v);
}

/// Decrement `A`
fn dec_a(cpu: &mut Cpu) {
    let mut a = cpu.a();

    // bit will carry over if the low nibble is 0
    cpu.set_halfcarry(a & 0xf == 0);

    a -= 1;

    cpu.set_a(a);

    cpu.set_zero(a == 0);
    cpu.set_substract(true);
}

/// Decrement `B`
fn dec_b(cpu: &mut Cpu) {
    let mut b = cpu.b();

    // bit will carry over if the low nibble is 0
    cpu.set_halfcarry(b & 0xf == 0);

    b -= 1;

    cpu.set_b(b);

    cpu.set_zero(b == 0);
    cpu.set_substract(true);
}

/// Decrement `C`
fn dec_c(cpu: &mut Cpu) {
    let mut c = cpu.c();

    // bit will carry over if the low nibble is 0
    cpu.set_halfcarry(c & 0xf == 0);

    c -= 1;

    cpu.set_c(c);

    cpu.set_zero(c == 0);
    cpu.set_substract(true);
}

/// Decrement `D`
fn dec_d(cpu: &mut Cpu) {
    let mut d = cpu.d();

    // bit will carry over if the low nibble is 0
    cpu.set_halfcarry(d & 0xf == 0);

    d -= 1;

    cpu.set_d(d);

    cpu.set_zero(d == 0);
    cpu.set_substract(true);
}

/// Decrement `E`
fn dec_e(cpu: &mut Cpu) {
    let mut e = cpu.e();

    // bit will carry over if the low nibble is 0
    cpu.set_halfcarry(e & 0xf == 0);

    e -= 1;

    cpu.set_e(e);

    cpu.set_zero(e == 0);
    cpu.set_substract(true);
}

/// Decrement `H`
fn dec_h(cpu: &mut Cpu) {
    let mut h = cpu.h();

    // bit will carry over if the low nibble is 0
    cpu.set_halfcarry(h & 0xf == 0);

    h -= 1;

    cpu.set_h(h);

    cpu.set_zero(h == 0);
    cpu.set_substract(true);
}

/// Decrement `L`
fn dec_l(cpu: &mut Cpu) {
    let mut l = cpu.l();

    // bit will carry over if the low nibble is 0
    cpu.set_halfcarry(l & 0xf == 0);

    l -= 1;

    cpu.set_l(l);

    cpu.set_zero(l == 0);
    cpu.set_substract(true);
}

/// Compare `A` with itself
fn cp_a_a(cpu: &mut Cpu) {
    let a = cpu.a();

    // Let's hope LLVM is clever enough to optimize that one...
    sub_and_set_flags(cpu, a, a);
}

/// Compare `A` with `B`
fn cp_a_b(cpu: &mut Cpu) {
    let a = cpu.a();
    let b = cpu.b();

    sub_and_set_flags(cpu, a, b);
}

/// Compare `A` with `C`
fn cp_a_c(cpu: &mut Cpu) {
    let a = cpu.a();
    let c = cpu.c();

    sub_and_set_flags(cpu, a, c);
}

/// Compare `A` with `D`
fn cp_a_d(cpu: &mut Cpu) {
    let a = cpu.a();
    let d = cpu.d();

    sub_and_set_flags(cpu, a, d);
}

/// Compare `A` with `E`
fn cp_a_e(cpu: &mut Cpu) {
    let a = cpu.a();
    let e = cpu.e();

    sub_and_set_flags(cpu, a, e);
}

/// Compare `A` with `H`
fn cp_a_h(cpu: &mut Cpu) {
    let a = cpu.a();
    let h = cpu.h();

    sub_and_set_flags(cpu, a, h);
}

/// Compare `A` with `L`
fn cp_a_l(cpu: &mut Cpu) {
    let a = cpu.a();
    let l = cpu.l();

    sub_and_set_flags(cpu, a, l);
}

/// Compare `A` with `[HL]`
fn cp_a_mhl(cpu: &mut Cpu) {
    let a  = cpu.a();
    let hl = cpu.hl();

    let n = cpu.fetch_byte(hl);

    sub_and_set_flags(cpu, a, n);
}

/// Compare `A` with immediate value
fn cp_a_n(cpu: &mut Cpu) {
    let a  = cpu.a();
    let n = next_byte(cpu);

    sub_and_set_flags(cpu, a, n);
}

/// Disable interrupts
fn di(cpu: &mut Cpu) {
    cpu.disable_interrupts();
}

mod cb {
    //! Emulation of instructions prefixed by 0xCB

    use super::nop;
    use cpu::Cpu;

    /// Return the 0xCB instruction to be executed
    pub fn next_instruction(cpu: &mut Cpu) -> (u32, fn (&mut Cpu)) {
        let pc = cpu.pc();

        cpu.set_pc(pc + 1);

        let op = cpu.fetch_byte(pc);

        let (delay, instruction) = OPCODES[op as uint];

        if delay == 0 {
            panic!("Unimplemented CB instruction [{:02X}]", op);
        }

        (delay, instruction)
    }

    /// Array similar to the one above, this time for CB-prefixed
    /// instructions
    pub static OPCODES: [(u32, fn (&mut Cpu)), ..0x100] = [
        // Opcodes CB 0X
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        // Opcodes CB 1X
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        // Opcodes CB 2X
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        // Opcodes CB 3X
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        // Opcodes CB 4X
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        // Opcodes CB 5X
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        // Opcodes CB 6X
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        // Opcodes CB 7X
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        // Opcodes CB 8X
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (2, res_a_0),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (2, res_a_1),
        // Opcodes CB 9X
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (2, res_a_2),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (2, res_a_3),
        // Opcodes CB AX
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (2, res_a_4),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (2, res_a_5),
        // Opcodes CB BX
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (2, res_a_6),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (2, res_a_7),
        // Opcodes CB CX
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        // Opcodes CB DX
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        // Opcodes CB EX
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        // Opcodes CB FX
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
        (0, nop),
    ];

    /// Helper function to clear one bit in a u8
    fn res(val: u8, bit: u8) -> u8 {
        val & !(1u8 << (bit as uint))
    }

    /// Helper function to clear bits in `A`
    fn res_a(cpu: &mut Cpu, bit: u8) {
        let a = cpu.a();

        cpu.set_a(res(a, bit));
    }

    /// Clear `A` bit 0
    fn res_a_0(cpu: &mut Cpu) {
        res_a(cpu, 0);
    }

    /// Clear `A` bit 1
    fn res_a_1(cpu: &mut Cpu) {
        res_a(cpu, 1);
    }

    /// Clear `A` bit 2
    fn res_a_2(cpu: &mut Cpu) {
        res_a(cpu, 2);
    }

    /// Clear `A` bit 3
    fn res_a_3(cpu: &mut Cpu) {
        res_a(cpu, 3);
    }

    /// Clear `A` bit 4
    fn res_a_4(cpu: &mut Cpu) {
        res_a(cpu, 4);
    }

    /// Clear `A` bit 5
    fn res_a_5(cpu: &mut Cpu) {
        res_a(cpu, 5);
    }

    /// Clear `A` bit 6
    fn res_a_6(cpu: &mut Cpu) {
        res_a(cpu, 6);
    }

    /// Clear `A` bit 7
    fn res_a_7(cpu: &mut Cpu) {
        res_a(cpu, 7);
    }
}
