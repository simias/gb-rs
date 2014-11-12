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
            // 0xCB introduces a two-byte bitops opcode
            bitops::next_instruction(cpu)
        };

    if delay == 0 {
        println!("{}", cpu);
        panic!("Unimplemented instruction [{:02X}]", op);
    }

    (delay, instruction)
}

/// Array containing tuples `(delay, instruction)`.
///
/// `delay` is an `u32` describing how many machine cycles an
/// instruction takes to execute. One machine cycle is 4 clock
/// cycles. Certain instructions are conditional and might take longer
/// to execute depending on the current CPU state. For those I set the
/// delay to the smallest value and I call cpu.additional_delay(x)
/// from the instruction implementation if needed.
///
/// `instruction` is an `fn (&mut Cpu)` used to emulate the
/// instruction.
pub static OPCODES: [(u32, fn (&mut Cpu)), ..0x100] = [
    // Opcodes 0X
    (1, nop),
    (3, ld_bc_nn),
    (2, ld_mbc_a),
    (2, inc_bc),
    (1, inc_b),
    (1, dec_b),
    (2, ld_b_n),
    (1, rlca),
    (5, ld_mnn_sp),
    (2, add_hl_bc),
    (2, ld_a_mbc),
    (2, dec_bc),
    (1, inc_c),
    (1, dec_c),
    (2, ld_c_n),
    (1, rrca),
    // Opcodes 1X
    (1, stop),
    (3, ld_de_nn),
    (2, ld_mde_a),
    (2, inc_de),
    (1, inc_d),
    (1, dec_d),
    (2, ld_d_n),
    (1, rla),
    (3, jr_sn),
    (2, add_hl_de),
    (2, ld_a_mde),
    (2, dec_de),
    (1, inc_e),
    (1, dec_e),
    (2, ld_e_n),
    (1, rra),
    // Opcodes 2X
    (2, jr_nz_sn),
    (3, ld_hl_nn),
    (2, ldi_mhl_a),
    (2, inc_hl),
    (1, inc_h),
    (1, dec_h),
    (2, ld_h_n),
    (0, nop),    // TODO: DAA, decimal adjust for BCD.
    (2, jr_z_sn),
    (2, add_hl_hl),
    (2, ldi_a_mhl),
    (2, dec_hl),
    (1, inc_l),
    (1, dec_l),
    (2, ld_l_n),
    (1, cpl),
    // Opcodes 3X
    (2, jr_nc_sn),
    (3, ld_sp_nn),
    (2, ldd_mhl_a),
    (2, inc_sp),
    (3, inc_mhl),
    (3, dec_mhl),
    (3, ld_mhl_n),
    (1, scf),
    (2, jr_c_sn),
    (2, add_hl_sp),
    (2, ldd_a_mhl),
    (2, dec_sp),
    (1, inc_a),
    (1, dec_a),
    (2, ld_a_n),
    (1, ccf),
    // Opcodes 4X
    (1, ld_b_b),
    (1, ld_b_c),
    (1, ld_b_d),
    (1, ld_b_e),
    (1, ld_b_h),
    (1, ld_b_l),
    (2, ld_b_mhl),
    (1, ld_b_a),
    (1, ld_c_b),
    (1, ld_c_c),
    (1, ld_c_d),
    (1, ld_c_e),
    (1, ld_c_h),
    (1, ld_c_l),
    (2, ld_c_mhl),
    (1, ld_c_a),
    // Opcodes 5X
    (1, ld_d_b),
    (1, ld_d_c),
    (1, ld_d_d),
    (1, ld_d_e),
    (1, ld_d_h),
    (1, ld_d_l),
    (2, ld_d_mhl),
    (1, ld_d_a),
    (1, ld_e_b),
    (1, ld_e_c),
    (1, ld_e_d),
    (1, ld_e_e),
    (1, ld_e_h),
    (1, ld_e_l),
    (2, ld_e_mhl),
    (1, ld_e_a),
    // Opcodes 6X
    (1, ld_h_b),
    (1, ld_h_c),
    (1, ld_h_d),
    (1, ld_h_e),
    (1, ld_h_h),
    (1, ld_h_l),
    (2, ld_h_mhl),
    (1, ld_h_a),
    (1, ld_l_b),
    (1, ld_l_c),
    (1, ld_l_d),
    (1, ld_l_e),
    (1, ld_l_h),
    (1, ld_l_l),
    (2, ld_l_mhl),
    (1, ld_l_a),
    // Opcodes 7X
    (2, ld_mhl_b),
    (2, ld_mhl_c),
    (2, ld_mhl_d),
    (2, ld_mhl_e),
    (2, ld_mhl_h),
    (2, ld_mhl_l),
    (1, halt),
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
    (1, add_a_b),
    (1, add_a_c),
    (1, add_a_d),
    (1, add_a_e),
    (1, add_a_h),
    (1, add_a_l),
    (2, add_a_mhl),
    (1, add_a_a),
    (1, adc_a_b),
    (1, adc_a_c),
    (1, adc_a_d),
    (1, adc_a_e),
    (1, adc_a_h),
    (1, adc_a_l),
    (2, adc_a_mhl),
    (1, adc_a_a),
    // Opcodes 9X
    (1, sub_a_b),
    (1, sub_a_c),
    (1, sub_a_d),
    (1, sub_a_e),
    (1, sub_a_h),
    (1, sub_a_l),
    (2, sub_a_mhl),
    (1, sub_a_a),
    (1, sbc_a_b),
    (1, sbc_a_c),
    (1, sbc_a_d),
    (1, sbc_a_e),
    (1, sbc_a_h),
    (1, sbc_a_l),
    (2, sbc_a_mhl),
    (1, sbc_a_a),
    // Opcodes AX
    (1, and_a_b),
    (1, and_a_c),
    (1, and_a_d),
    (1, and_a_e),
    (1, and_a_h),
    (1, and_a_l),
    (2, and_a_mhl),
    (1, and_a_a),
    (1, xor_a_b),
    (1, xor_a_c),
    (1, xor_a_d),
    (1, xor_a_e),
    (1, xor_a_h),
    (1, xor_a_l),
    (2, xor_a_mhl),
    (1, xor_a_a),
    // Opcodes BX
    (1, or_a_b),
    (1, or_a_c),
    (1, or_a_d),
    (1, or_a_e),
    (1, or_a_h),
    (1, or_a_l),
    (2, or_a_mhl),
    (1, or_a_a),
    (1, cp_a_b),
    (1, cp_a_c),
    (1, cp_a_d),
    (1, cp_a_e),
    (1, cp_a_h),
    (1, cp_a_l),
    (2, cp_a_mhl),
    (1, cp_a_a),
    // Opcodes CX
    (2, ret_nz),
    (3, pop_bc),
    (3, jp_nz_nn),
    (3, jp_nn),
    (3, call_nz_nn),
    (4, push_bc),
    (2, add_a_n),
    (4, rst_00),
    (2, ret_z),
    (4, ret),
    (3, jp_z_nn),
    (0, undefined), // See bitops opcode map
    (3, call_z_nn),
    (6, call_nn),
    (2, adc_a_n),
    (4, rst_08),
    // Opcodes DX
    (2, ret_nc),
    (3, pop_de),
    (3, jp_nc_nn),
    (1, undefined),
    (3, call_nc_nn),
    (4, push_de),
    (2, sub_a_n),
    (4, rst_10),
    (2, ret_c),
    (4, reti),
    (3, jp_c_nn),
    (1, undefined),
    (3, call_c_nn),
    (1, undefined),
    (2, sbc_a_n),
    (4, rst_18),
    // Opcodes EX
    (3, ldh_mn_a),
    (3, pop_hl),
    (2, ldh_mc_a),
    (1, undefined),
    (1, undefined),
    (4, push_hl),
    (2, and_a_n),
    (4, rst_20),
    (4, add_sp_sn),
    (1, jp_hl),
    (4, ld_mnn_a),
    (1, undefined),
    (1, undefined),
    (1, undefined),
    (2, xor_a_n),
    (4, rst_28),
    // Opcodes FX
    (3, ldh_a_mn),
    (3, pop_af),
    (2, ldh_a_mc),
    (1, di),
    (1, undefined),
    (4, push_af),
    (2, or_a_n),
    (4, rst_30),
    (3, ld_hl_sp_sn),
    (2, ld_sp_hl),
    (2, ld_a_mnn),
    (1, ei),
    (1, undefined),
    (1, undefined),
    (2, cp_a_n),
    (4, rst_38),
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

/// No operation
fn nop(_: &mut Cpu) {
}

/// Undefined opcode. Stall the CPU.
fn undefined(cpu: &mut Cpu) {
    let pc = cpu.pc() - 1;

    println!("Invalid instruction called at 0x{:04x}. CPU stalled.", pc);

    cpu.set_pc(pc);
}

/// Rotate `A` left
fn rlca(cpu: &mut Cpu) {
    let a = cpu.a();

    let c = a >> 7;

    cpu.set_a((a << 1) | c);

    // Not sure about whether or not to set the Z flag and looking at
    // other emulators I'm not the only one.
    //
    // The Z80 doc says Z is untouched, the unofficial "Game Boy CPU
    // manual" says it's set if the result is 0, unset otherwise.
    //
    // http://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html on
    // the other hand says it's set to 0 along with N and H.
    //
    // VisualBoyAdvance follows the Z80 doc and doesn't touch any flag
    // except for C, I'm going to assume they know what they're doing
    // and do the same.
    cpu.set_carry(c != 0);
}

/// Rotate `A` left through carry
fn rla(cpu: &mut Cpu) {
    let a = cpu.a();

    let newcarry = (a >> 7) != 0;
    let oldcarry = cpu.carry() as u8;

    cpu.set_a((a << 1) | oldcarry);

    // Same remark as RLCA regarding other flags
    cpu.set_carry(newcarry);
}

/// Rotate `A` right
fn rrca(cpu: &mut Cpu) {
    let a = cpu.a();

    let c = a & 1;

    cpu.set_a((a >> 1) | (c << 7));

    // Same remark as RLCA regarding other flags
    cpu.set_carry(c != 0);
}

/// Rotate `A` right through carry
fn rra(cpu: &mut Cpu) {
    let a = cpu.a();

    let newcarry = (a & 1) != 0;
    let oldcarry = cpu.carry() as u8;

    cpu.set_a((a >> 1) | (oldcarry << 7));

    // Same remark as RLCA regarding other flags
    cpu.set_carry(newcarry);
}

/// Complement `A`
fn cpl(cpu: &mut Cpu) {
    let a = cpu.a();

    cpu.set_a(!a);

    cpu.set_substract(true);
    cpu.set_halfcarry(true);
}

/// Complement carry flag
fn ccf(cpu: &mut Cpu) {
    let carry = cpu.carry();

    cpu.set_carry(!carry);
    cpu.set_substract(false);
    cpu.set_halfcarry(false);
}

/// Set carry flag
fn scf(cpu: &mut Cpu) {
    cpu.set_carry(true);
    cpu.set_substract(false);
    cpu.set_halfcarry(false);
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

/// Load `B` into `B` (NOP)
fn ld_b_b(_: &mut Cpu) {
}

/// Load `C` into `C` (NOP)
fn ld_c_c(_: &mut Cpu) {
}

/// Load `D` into `D` (NOP)
fn ld_d_d(_: &mut Cpu) {
}

/// Load `E` into `E` (NOP)
fn ld_e_e(_: &mut Cpu) {
}

/// Load `H` into `H` (NOP)
fn ld_h_h(_: &mut Cpu) {
}

/// Load `L` into `L` (NOP)
fn ld_l_l(_: &mut Cpu) {
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

/// Load `[HL]` into `B`
fn ld_b_mhl(cpu: &mut Cpu) {
    let hl = cpu.hl();

    let v = cpu.fetch_byte(hl);

    cpu.set_b(v);
}

/// Load `[HL]` into `C`
fn ld_c_mhl(cpu: &mut Cpu) {
    let hl = cpu.hl();

    let v = cpu.fetch_byte(hl);

    cpu.set_c(v);
}

/// Load `[HL]` into `D`
fn ld_d_mhl(cpu: &mut Cpu) {
    let hl = cpu.hl();

    let v = cpu.fetch_byte(hl);

    cpu.set_d(v);
}

/// Load `[HL]` into `E`
fn ld_e_mhl(cpu: &mut Cpu) {
    let hl = cpu.hl();

    let v = cpu.fetch_byte(hl);

    cpu.set_e(v);
}

/// Load `[HL]` into `H`
fn ld_h_mhl(cpu: &mut Cpu) {
    let hl = cpu.hl();

    let v = cpu.fetch_byte(hl);

    cpu.set_h(v);
}

/// Load `[HL]` into `L`
fn ld_l_mhl(cpu: &mut Cpu) {
    let hl = cpu.hl();

    let v = cpu.fetch_byte(hl);

    cpu.set_l(v);
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

/// Store `A` into `[BC]`
fn ld_mbc_a(cpu: &mut Cpu) {
    let a  = cpu.a();
    let bc = cpu.bc();

    cpu.store_byte(bc, a);
}

/// Store `A` into `[DE]`
fn ld_mde_a(cpu: &mut Cpu) {
    let a  = cpu.a();
    let de = cpu.de();

    cpu.store_byte(de, a);
}

/// Store `A` into `[HL]`
fn ld_mhl_a(cpu: &mut Cpu) {
    let a  = cpu.a();
    let hl = cpu.hl();

    cpu.store_byte(hl, a);
}

/// Store `B` into `[HL]`
fn ld_mhl_b(cpu: &mut Cpu) {
    let b  = cpu.b();
    let hl = cpu.hl();

    cpu.store_byte(hl, b);
}

/// Store `C` into `[HL]`
fn ld_mhl_c(cpu: &mut Cpu) {
    let c  = cpu.c();
    let hl = cpu.hl();

    cpu.store_byte(hl, c);
}

/// Store `D` into `[HL]`
fn ld_mhl_d(cpu: &mut Cpu) {
    let d  = cpu.d();
    let hl = cpu.hl();

    cpu.store_byte(hl, d);
}

/// Store `E` into `[HL]`
fn ld_mhl_e(cpu: &mut Cpu) {
    let e  = cpu.e();
    let hl = cpu.hl();

    cpu.store_byte(hl, e);
}

/// Store `H` into `[HL]`
fn ld_mhl_h(cpu: &mut Cpu) {
    let h  = cpu.h();
    let hl = cpu.hl();

    cpu.store_byte(hl, h);
}

/// Store `L` into `[HL]`
fn ld_mhl_l(cpu: &mut Cpu) {
    let l  = cpu.l();
    let hl = cpu.hl();

    cpu.store_byte(hl, l);
}

/// Store `N` into `[HL]
fn ld_mhl_n(cpu: &mut Cpu) {
    let n  = next_byte(cpu);
    let hl = cpu.hl();

    cpu.store_byte(hl, n);
}

/// Store `A` into `[NN]`
fn ld_mnn_a(cpu: &mut Cpu) {
    let a  = cpu.a();
    let n = next_word(cpu);

    cpu.store_byte(n, a);
}

/// Store `SP` into `[NN]`
fn ld_mnn_sp(cpu: &mut Cpu) {
    let sp  = cpu.sp();
    let n = next_word(cpu);

    cpu.store_byte(n, sp as u8);
    cpu.store_byte(n, (sp >> 8) as u8);
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

/// Load `SP + N` into `HL`
fn ld_hl_sp_sn(cpu: &mut Cpu) {
    let sp = cpu.sp() as i32;
    let n  = next_byte(cpu) as i8;

    let nn = n as i32;

    let r = sp + nn;

    cpu.set_substract(false);
    cpu.set_carry(r & 0x10000 != 0);
    cpu.set_halfcarry((sp ^ nn ^ r) & 0x1000 != 0);
    cpu.set_hl(r as u16);
}

/// Load 16bits immediate value into `SP`
fn ld_sp_nn(cpu: &mut Cpu) {
    let n = next_word(cpu);

    cpu.set_sp(n);
}

/// Load `HL` into `SP`
fn ld_sp_hl(cpu: &mut Cpu) {
    let hl = cpu.hl();

    cpu.set_sp(hl);
}

/// Load `C` into `B`
fn ld_b_c(cpu: &mut Cpu) {
    let c = cpu.c();

    cpu.set_b(c);
}

/// Load `D` into `B`
fn ld_b_d(cpu: &mut Cpu) {
    let d = cpu.d();

    cpu.set_b(d);
}

/// Load `E` into `B`
fn ld_b_e(cpu: &mut Cpu) {
    let e = cpu.e();

    cpu.set_b(e);
}

/// Load `H` into `B`
fn ld_b_h(cpu: &mut Cpu) {
    let h = cpu.h();

    cpu.set_b(h);
}

/// Load `L` into `B`
fn ld_b_l(cpu: &mut Cpu) {
    let l = cpu.l();

    cpu.set_b(l);
}

/// Load `B` into `C`
fn ld_c_b(cpu: &mut Cpu) {
    let b = cpu.b();

    cpu.set_c(b);
}

/// Load `D` into `C`
fn ld_c_d(cpu: &mut Cpu) {
    let d = cpu.d();

    cpu.set_c(d);
}

/// Load `E` into `C`
fn ld_c_e(cpu: &mut Cpu) {
    let e = cpu.e();

    cpu.set_c(e);
}

/// Load `H` into `C`
fn ld_c_h(cpu: &mut Cpu) {
    let h = cpu.h();

    cpu.set_c(h);
}

/// Load `L` into `C`
fn ld_c_l(cpu: &mut Cpu) {
    let l = cpu.l();

    cpu.set_c(l);
}

/// Load `B` into `D`
fn ld_d_b(cpu: &mut Cpu) {
    let b = cpu.b();

    cpu.set_d(b);
}

/// Load `C` into `D`
fn ld_d_c(cpu: &mut Cpu) {
    let c = cpu.c();

    cpu.set_d(c);
}

/// Load `E` into `D`
fn ld_d_e(cpu: &mut Cpu) {
    let e = cpu.e();

    cpu.set_d(e);
}

/// Load `H` into `D`
fn ld_d_h(cpu: &mut Cpu) {
    let h = cpu.h();

    cpu.set_d(h);
}

/// Load `L` into `D`
fn ld_d_l(cpu: &mut Cpu) {
    let l = cpu.l();

    cpu.set_d(l);
}

/// Load `B` into `E`
fn ld_e_b(cpu: &mut Cpu) {
    let b = cpu.b();

    cpu.set_e(b);
}

/// Load `C` into `E`
fn ld_e_c(cpu: &mut Cpu) {
    let c = cpu.c();

    cpu.set_e(c);
}

/// Load `D` into `E`
fn ld_e_d(cpu: &mut Cpu) {
    let d = cpu.d();

    cpu.set_e(d);
}

/// Load `H` into `E`
fn ld_e_h(cpu: &mut Cpu) {
    let h = cpu.h();

    cpu.set_e(h);
}

/// Load `L` into `E`
fn ld_e_l(cpu: &mut Cpu) {
    let l = cpu.l();

    cpu.set_e(l);
}

/// Load `B` into `H`
fn ld_h_b(cpu: &mut Cpu) {
    let b = cpu.b();

    cpu.set_h(b);
}

/// Load `C` into `H`
fn ld_h_c(cpu: &mut Cpu) {
    let c = cpu.c();

    cpu.set_h(c);
}

/// Load `D` into `H`
fn ld_h_d(cpu: &mut Cpu) {
    let d = cpu.d();

    cpu.set_h(d);
}

/// Load `E` into `H`
fn ld_h_e(cpu: &mut Cpu) {
    let e = cpu.e();

    cpu.set_h(e);
}

/// Load `B` into `L`
fn ld_l_b(cpu: &mut Cpu) {
    let b = cpu.b();

    cpu.set_l(b);
}

/// Load `C` into `L`
fn ld_l_c(cpu: &mut Cpu) {
    let c = cpu.c();

    cpu.set_l(c);
}

/// Load `D` into `L`
fn ld_l_d(cpu: &mut Cpu) {
    let d = cpu.d();

    cpu.set_l(d);
}

/// Load `E` into `L`
fn ld_l_e(cpu: &mut Cpu) {
    let e = cpu.e();

    cpu.set_l(e);
}

/// Load `H` into `L`
fn ld_l_h(cpu: &mut Cpu) {
    let h = cpu.h();

    cpu.set_l(h);
}


/// Load `L` into `H`
fn ld_h_l(cpu: &mut Cpu) {
    let l = cpu.l();

    cpu.set_h(l);
}

/// Pop `AF` from the stack
fn pop_af(cpu: &mut Cpu) {
    let n = pop_word(cpu);

    cpu.set_af(n);
}

/// Pop `BC` from the stack
fn pop_bc(cpu: &mut Cpu) {
    let n = pop_word(cpu);

    cpu.set_bc(n);
}

/// Pop `DE` from the stack
fn pop_de(cpu: &mut Cpu) {
    let n = pop_word(cpu);

    cpu.set_de(n);
}

/// Pop `HL` from the stack
fn pop_hl(cpu: &mut Cpu) {
    let n = pop_word(cpu);

    cpu.set_hl(n);
}

/// Push `AF` on the stack
fn push_af(cpu: &mut Cpu) {
    let af = cpu.af();

    push_word(cpu, af);
}

/// Push `BC` on the stack
fn push_bc(cpu: &mut Cpu) {
    let bc = cpu.bc();

    push_word(cpu, bc);
}

/// Push `DE` on the stack
fn push_de(cpu: &mut Cpu) {
    let de = cpu.de();

    push_word(cpu, de);
}

/// Push `HL` on the stack
fn push_hl(cpu: &mut Cpu) {
    let hl = cpu.hl();

    push_word(cpu, hl);
}

/// Unconditional jump to absolute address
fn jp_nn(cpu: &mut Cpu) {
    let addr = next_word(cpu);

    cpu.set_pc(addr);
}

/// Unconditional jump to address in `HL`
fn jp_hl(cpu: &mut Cpu) {
    let hl = cpu.hl();

    cpu.set_pc(hl);
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
fn jr_sn(cpu: &mut Cpu) {
    let off = next_byte(cpu) as i8;

    let mut pc = cpu.pc() as i16;

    pc += off as i16;

    cpu.set_pc(pc as u16);
}

/// Jump to relative address if `!Z`
fn jr_nz_sn(cpu: &mut Cpu) {
    let off = next_byte(cpu) as i8;

    if !cpu.zero() {
        let mut pc = cpu.pc() as i16;

        pc += off as i16;

        cpu.set_pc(pc as u16);

        cpu.additional_delay(1);
    }
}

/// Jump to relative address if `Z`
fn jr_z_sn(cpu: &mut Cpu) {
    let off = next_byte(cpu) as i8;

    if cpu.zero() {
        let mut pc = cpu.pc() as i16;

        pc += off as i16;

        cpu.set_pc(pc as u16);

        cpu.additional_delay(1);
    }
}

/// Jump to relative address if `!C`
fn jr_nc_sn(cpu: &mut Cpu) {
    let off = next_byte(cpu) as i8;

    if !cpu.carry() {
        let mut pc = cpu.pc() as i16;

        pc += off as i16;

        cpu.set_pc(pc as u16);

        cpu.additional_delay(1);
    }
}

/// Jump to relative address if `C`
fn jr_c_sn(cpu: &mut Cpu) {
    let off = next_byte(cpu) as i8;

    if cpu.carry() {
        let mut pc = cpu.pc() as i16;

        pc += off as i16;

        cpu.set_pc(pc as u16);

        cpu.additional_delay(1);
    }
}

/// Helper function for RST instructions
fn rst(cpu: &mut Cpu, addr: u16) {
    let pc = cpu.pc();

    push_word(cpu, pc);

    cpu.set_pc(addr);
}

/// Push return address on stack and jump to 0x00
fn rst_00(cpu: &mut Cpu) {
    rst(cpu, 0x00);
}

/// Push return address on stack and jump to 0x08
fn rst_08(cpu: &mut Cpu) {
    rst(cpu, 0x08);
}

/// Push return address on stack and jump to 0x10
fn rst_10(cpu: &mut Cpu) {
    rst(cpu, 0x10);
}

/// Push return address on stack and jump to 0x18
fn rst_18(cpu: &mut Cpu) {
    rst(cpu, 0x18);
}

/// Push return address on stack and jump to 0x20
fn rst_20(cpu: &mut Cpu) {
    rst(cpu, 0x20);
}

/// Push return address on stack and jump to 0x28
fn rst_28(cpu: &mut Cpu) {
    rst(cpu, 0x28);
}

/// Push return address on stack and jump to 0x30
fn rst_30(cpu: &mut Cpu) {
    rst(cpu, 0x30);
}

/// Push return address on stack and jump to 0x38
fn rst_38(cpu: &mut Cpu) {
    rst(cpu, 0x38);
}

/// Push return address on stack and jump to immediate address
fn call_nn(cpu: &mut Cpu) {
    let addr = next_word(cpu);
    let pc = cpu.pc();

    push_word(cpu, pc);

    cpu.set_pc(addr);
}

/// If !Z Push return address on stack and jump to immediate address
fn call_nz_nn(cpu: &mut Cpu) {
    let addr = next_word(cpu);

    if !cpu.zero() {
        let pc = cpu.pc();

        push_word(cpu, pc);

        cpu.set_pc(addr);

        cpu.additional_delay(3);
    }
}

/// If Z Push return address on stack and jump to immediate address
fn call_z_nn(cpu: &mut Cpu) {
    let addr = next_word(cpu);

    if cpu.zero() {
        let pc = cpu.pc();

        push_word(cpu, pc);

        cpu.set_pc(addr);

        cpu.additional_delay(3);
    }
}

/// If !C Push return address on stack and jump to immediate address
fn call_nc_nn(cpu: &mut Cpu) {
    let addr = next_word(cpu);

    if !cpu.carry() {
        let pc = cpu.pc();

        push_word(cpu, pc);

        cpu.set_pc(addr);

        cpu.additional_delay(3);
    }
}

/// If C Push return address on stack and jump to immediate address
fn call_c_nn(cpu: &mut Cpu) {
    let addr = next_word(cpu);

    if cpu.carry() {
        let pc = cpu.pc();

        push_word(cpu, pc);

        cpu.set_pc(addr);

        cpu.additional_delay(3);
    }
}

/// Pop return address from stack and jump to it
fn ret(cpu: &mut Cpu) {
    let addr = pop_word(cpu);

    cpu.set_pc(addr);
}

/// Pop return address from stack and jump to it then enable
/// interrupts.
fn reti(cpu: &mut Cpu) {
    let addr = pop_word(cpu);

    cpu.set_pc(addr);

    cpu.enable_interrupts();
}

/// If !Z pop return address from stack and jump to it
fn ret_nz(cpu: &mut Cpu) {
    if !cpu.zero() {
        let addr = pop_word(cpu);

        cpu.set_pc(addr);

        cpu.additional_delay(3);
    }
}

/// If Z pop return address from stack and jump to it
fn ret_z(cpu: &mut Cpu) {
    if cpu.zero() {
        let addr = pop_word(cpu);

        cpu.set_pc(addr);

        cpu.additional_delay(3);
    }
}

/// If !C pop return address from stack and jump to it
fn ret_nc(cpu: &mut Cpu) {
    if !cpu.carry() {
        let addr = pop_word(cpu);

        cpu.set_pc(addr);

        cpu.additional_delay(3);
    }
}

/// If C pop return address from stack and jump to it
fn ret_c(cpu: &mut Cpu) {
    if cpu.carry() {
        let addr = pop_word(cpu);

        cpu.set_pc(addr);

        cpu.additional_delay(3);
    }
}

/// Store `A` into `[HL]` and decrement `HL`
fn ldd_mhl_a(cpu: &mut Cpu) {
    let hl = cpu.hl();
    let a  = cpu.a();

    cpu.store_byte(hl, a);

    cpu.set_hl(hl - 1);
}

/// Load `[HL]` into `A` and decrement `HL`
fn ldd_a_mhl(cpu: &mut Cpu) {
    let hl = cpu.hl();

    let a = cpu.fetch_byte(hl);

    cpu.set_a(a);
    cpu.set_hl(hl - 1);
}

/// Store `A` into `[HL]` and increment `HL`
fn ldi_mhl_a(cpu: &mut Cpu) {
    let hl = cpu.hl();
    let a  = cpu.a();

    cpu.store_byte(hl, a);

    cpu.set_hl(hl + 1);
}

/// Load `[HL]` into `A` and increment `HL`
fn ldi_a_mhl(cpu: &mut Cpu) {
    let hl = cpu.hl();

    let a = cpu.fetch_byte(hl);

    cpu.set_a(a);
    cpu.set_hl(hl + 1);
}

/// Store `A` into `[0xff00 + N]`
fn ldh_mn_a(cpu: &mut Cpu) {
    let a = cpu.a();
    let n = next_byte(cpu) as u16;

    cpu.store_byte(0xff00 | n, a);
}

/// Store `A` into `[0xff00 + C]`
fn ldh_mc_a(cpu: &mut Cpu) {
    let a = cpu.a();
    let c = cpu.c() as u16;

    cpu.store_byte(0xff00 | c, a);
}

/// Load `[0xff00 + N]` into `[A]`
fn ldh_a_mn(cpu: &mut Cpu) {
    let n = next_byte(cpu) as u16;
    let v = cpu.fetch_byte(0xff00 | n);

    cpu.set_a(v);
}

/// Load `[0xff00 + C]` into `[A]`
fn ldh_a_mc(cpu: &mut Cpu) {
    let c = cpu.c() as u16;

    let v = cpu.fetch_byte(0xff00 | c);

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

/// Decrement `[HL]`
fn dec_mhl(cpu: &mut Cpu) {
    let hl = cpu.hl();
    let mut n = cpu.fetch_byte(hl);

    // bit will carry over if the low nibble is 0
    cpu.set_halfcarry(n & 0xf == 0);

    n -= 1;

    cpu.store_byte(hl, n);

    cpu.set_zero(n == 0);
    cpu.set_substract(true);
}

/// Increment `A`
fn inc_a(cpu: &mut Cpu) {
    let a = cpu.a();

    let r = a + 1;

    cpu.set_substract(false);
    cpu.set_zero(r == 0);
    cpu.set_halfcarry(a & 0xf == 0xf);

    cpu.set_a(r);
}

/// Increment `B`
fn inc_b(cpu: &mut Cpu) {
    let b = cpu.b();

    let r = b + 1;

    cpu.set_substract(false);
    cpu.set_zero(r == 0);
    cpu.set_halfcarry(b & 0xf == 0xf);

    cpu.set_b(r);
}

/// Increment `C`
fn inc_c(cpu: &mut Cpu) {
    let c = cpu.c();

    let r = c + 1;

    cpu.set_substract(false);
    cpu.set_zero(r == 0);
    cpu.set_halfcarry(c & 0xf == 0xf);

    cpu.set_c(r);
}

/// Increment `D`
fn inc_d(cpu: &mut Cpu) {
    let d = cpu.d();

    let r = d + 1;

    cpu.set_substract(false);
    cpu.set_zero(r == 0);
    cpu.set_halfcarry(d & 0xf == 0xf);

    cpu.set_d(r);
}

/// Increment `E`
fn inc_e(cpu: &mut Cpu) {
    let e = cpu.e();

    let r = e + 1;

    cpu.set_substract(false);
    cpu.set_zero(r == 0);
    cpu.set_halfcarry(e & 0xf == 0xf);

    cpu.set_e(r);
}

/// Increment `H`
fn inc_h(cpu: &mut Cpu) {
    let h = cpu.h();

    let r = h + 1;

    cpu.set_substract(false);
    cpu.set_zero(r == 0);
    cpu.set_halfcarry(h & 0xf == 0xf);

    cpu.set_h(r);
}

/// Increment `L`
fn inc_l(cpu: &mut Cpu) {
    let l = cpu.l();

    let r = l + 1;

    cpu.set_substract(false);
    cpu.set_zero(r == 0);
    cpu.set_halfcarry(l & 0xf == 0xf);

    cpu.set_l(r);
}

/// Increment `[HL]`
fn inc_mhl(cpu: &mut Cpu) {
    let hl = cpu.hl();

    let n = cpu.fetch_byte(hl);

    let r = n + 1;

    cpu.set_substract(false);
    cpu.set_zero(r == 0);
    cpu.set_halfcarry(n & 0xf == 0xf);

    cpu.store_byte(hl, r);
}

/// Increment `BC`
fn inc_bc(cpu: &mut Cpu) {
    let bc = cpu.bc();

    cpu.set_bc(bc + 1);
}

/// Increment `DE`
fn inc_de(cpu: &mut Cpu) {
    let de = cpu.de();

    cpu.set_de(de + 1);
}

/// Increment `HL`
fn inc_hl(cpu: &mut Cpu) {
    let hl = cpu.hl();

    cpu.set_hl(hl + 1);
}

/// Increment `SP`
fn inc_sp(cpu: &mut Cpu) {
    let sp = cpu.sp();

    cpu.set_sp(sp + 1);
}

/// Decrement `BC`
fn dec_bc(cpu: &mut Cpu) {
    let bc = cpu.bc();

    cpu.set_bc(bc - 1);
}

/// Decrement `DE`
fn dec_de(cpu: &mut Cpu) {
    let de = cpu.de();

    cpu.set_de(de - 1);
}

/// Decrement `HL`
fn dec_hl(cpu: &mut Cpu) {
    let hl = cpu.hl();

    cpu.set_hl(hl - 1);
}

/// Decrement `SP`
fn dec_sp(cpu: &mut Cpu) {
    let sp = cpu.sp();

    cpu.set_sp(sp - 1);
}

/// Helper function to substract two `u8`s and update the CPU flags
fn sub_and_set_flags(cpu: &mut Cpu, x: u8, y: u8) -> u8 {
    // Check for borrow using 32bit arithmetics
    let x = x as u32;
    let y = y as u32;

    let r = x - y;

    let rb = r as u8;

    cpu.set_zero(rb == 0);
    cpu.set_halfcarry((x ^ y ^ r) & 0x10 != 0);
    cpu.set_carry(r & 0x100 != 0);
    cpu.set_substract(true);

    rb
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

/// Substract `A` from `A`
fn sub_a_a(cpu: &mut Cpu) {
    cpu.set_zero(true);
    cpu.set_substract(true);
    cpu.set_carry(false);
    cpu.set_halfcarry(false);

    cpu.set_a(0);
}

/// Substract `B` from `A`
fn sub_a_b(cpu: &mut Cpu) {
    let a = cpu.a();
    let b = cpu.b();

    let r = sub_and_set_flags(cpu, a, b);

    cpu.set_a(r);
}

/// Substract `C` from `A`
fn sub_a_c(cpu: &mut Cpu) {
    let a = cpu.a();
    let c = cpu.c();

    let r = sub_and_set_flags(cpu, a, c);

    cpu.set_a(r);
}

/// Substract `D` from `A`
fn sub_a_d(cpu: &mut Cpu) {
    let a = cpu.a();
    let d = cpu.d();

    let r = sub_and_set_flags(cpu, a, d);

    cpu.set_a(r);
}

/// Substract `E` from `A`
fn sub_a_e(cpu: &mut Cpu) {
    let a = cpu.a();
    let e = cpu.e();

    let r = sub_and_set_flags(cpu, a, e);

    cpu.set_a(r);
}

/// Substract `H` from `A`
fn sub_a_h(cpu: &mut Cpu) {
    let a = cpu.a();
    let h = cpu.h();

    let r = sub_and_set_flags(cpu, a, h);

    cpu.set_a(r);
}

/// Substract `L` from `A`
fn sub_a_l(cpu: &mut Cpu) {
    let a = cpu.a();
    let l = cpu.l();

    let r = sub_and_set_flags(cpu, a, l);

    cpu.set_a(r);
}

/// Substract `[HL]` from `A`
fn sub_a_mhl(cpu: &mut Cpu) {
    let a  = cpu.a();
    let hl = cpu.hl();

    let n = cpu.fetch_byte(hl);

    let r = sub_and_set_flags(cpu, a, n);

    cpu.set_a(r);
}

/// Substract `N` from `A`
fn sub_a_n(cpu: &mut Cpu) {
    let a = cpu.a();
    let n = next_byte(cpu);

    let r = sub_and_set_flags(cpu, a, n);

    cpu.set_a(r);
}

/// Helper function to substract two `u8`s with carry and update the CPU flags
fn sub_with_carry_and_set_flags(cpu: &mut Cpu, x: u8, y: u8) -> u8 {
    // Check for borrow using 32bit arithmetics
    let x = x as u32;
    let y = y as u32;
    let carry = cpu.carry() as u32;

    let r = x - y - carry;

    let rb = r as u8;

    cpu.set_zero(rb == 0);
    cpu.set_halfcarry((x ^ y ^ r) & 0x10 != 0);
    cpu.set_carry(r & 0x100 != 0);
    cpu.set_substract(true);

    rb
}

/// Substract `A` from `A` with carry
fn sbc_a_a(cpu: &mut Cpu) {
    let a = cpu.a();

    let r = sub_with_carry_and_set_flags(cpu, a, a);

    cpu.set_a(r);
}

/// Substract `B` from `A` with carry
fn sbc_a_b(cpu: &mut Cpu) {
    let a = cpu.a();
    let b = cpu.b();

    let r = sub_with_carry_and_set_flags(cpu, a, b);

    cpu.set_a(r);
}

/// Substract `C` from `A` with carry
fn sbc_a_c(cpu: &mut Cpu) {
    let a = cpu.a();
    let c = cpu.c();

    let r = sub_with_carry_and_set_flags(cpu, a, c);

    cpu.set_a(r);
}

/// Substract `D` from `A` with carry
fn sbc_a_d(cpu: &mut Cpu) {
    let a = cpu.a();
    let d = cpu.d();

    let r = sub_with_carry_and_set_flags(cpu, a, d);

    cpu.set_a(r);
}

/// Substract `E` from `A` with carry
fn sbc_a_e(cpu: &mut Cpu) {
    let a = cpu.a();
    let e = cpu.e();

    let r = sub_with_carry_and_set_flags(cpu, a, e);

    cpu.set_a(r);
}

/// Substract `H` from `A` with carry
fn sbc_a_h(cpu: &mut Cpu) {
    let a = cpu.a();
    let h = cpu.h();

    let r = sub_with_carry_and_set_flags(cpu, a, h);

    cpu.set_a(r);
}

/// Substract `L` from `A` with carry
fn sbc_a_l(cpu: &mut Cpu) {
    let a = cpu.a();
    let l = cpu.l();

    let r = sub_with_carry_and_set_flags(cpu, a, l);

    cpu.set_a(r);
}

/// Substract `[HL]` from `A` with carry
fn sbc_a_mhl(cpu: &mut Cpu) {
    let a  = cpu.a();
    let hl = cpu.hl();

    let n = cpu.fetch_byte(hl);

    let r = sub_with_carry_and_set_flags(cpu, a, n);

    cpu.set_a(r);
}

/// Substract `N` from `A` with carry
fn sbc_a_n(cpu: &mut Cpu) {
    let a = cpu.a();
    let n = next_byte(cpu);

    let r = sub_with_carry_and_set_flags(cpu, a, n);

    cpu.set_a(r);
}

/// Helper function to add two `u8`s and update the CPU flags
fn add_and_set_flags(cpu: &mut Cpu, x: u8, y: u8) -> u8 {
    // Check for carry using 32bit arithmetics
    let x = x as u32;
    let y = y as u32;

    let r = x + y;

    let rb = r as u8;

    cpu.set_zero(rb == 0);
    cpu.set_halfcarry((x ^ y ^ r) & 0x10 != 0);
    cpu.set_carry(r & 0x100 != 0);
    cpu.set_substract(false);

    rb
}

/// Add `A` to `A`
fn add_a_a(cpu: &mut Cpu) {
    let a = cpu.a();

    let r = add_and_set_flags(cpu, a, a);

    cpu.set_a(r);
}

/// Add `B` to `A`
fn add_a_b(cpu: &mut Cpu) {
    let a = cpu.a();
    let b = cpu.b();

    let r = add_and_set_flags(cpu, a, b);

    cpu.set_a(r);
}

/// Add `C` to `A`
fn add_a_c(cpu: &mut Cpu) {
    let a = cpu.a();
    let c = cpu.c();

    let r = add_and_set_flags(cpu, a, c);

    cpu.set_a(r);
}

/// Add `D` to `A`
fn add_a_d(cpu: &mut Cpu) {
    let a = cpu.a();
    let d = cpu.d();

    let r = add_and_set_flags(cpu, a, d);

    cpu.set_a(r);
}

/// Add `E` to `A`
fn add_a_e(cpu: &mut Cpu) {
    let a = cpu.a();
    let e = cpu.e();

    let r = add_and_set_flags(cpu, a, e);

    cpu.set_a(r);
}

/// Add `H` to `A`
fn add_a_h(cpu: &mut Cpu) {
    let a = cpu.a();
    let h = cpu.h();

    let r = add_and_set_flags(cpu, a, h);

    cpu.set_a(r);
}

/// Add `L` to `A`
fn add_a_l(cpu: &mut Cpu) {
    let a = cpu.a();
    let l = cpu.l();

    let r = add_and_set_flags(cpu, a, l);

    cpu.set_a(r);
}

/// Add `[HL]` to `A`
fn add_a_mhl(cpu: &mut Cpu) {
    let a  = cpu.a();
    let hl = cpu.hl();

    let n = cpu.fetch_byte(hl);

    let r = add_and_set_flags(cpu, a, n);

    cpu.set_a(r);
}

/// Add `N` to `A`
fn add_a_n(cpu: &mut Cpu) {
    let a = cpu.a();
    let n = next_byte(cpu);

    let r = add_and_set_flags(cpu, a, n);

    cpu.set_a(r);
}

/// Helper function to add two `u8`s with carry and update the CPU flags
fn add_with_carry_and_set_flags(cpu: &mut Cpu, x: u8, y: u8) -> u8 {
    // Check for carry using 32bit arithmetics
    let x = x as u32;
    let y = y as u32;
    let carry = cpu.carry() as u32;

    let r = x + y + carry;

    let rb = r as u8;

    cpu.set_zero(rb == 0);
    cpu.set_halfcarry((x ^ y ^ r) & 0x10 != 0);
    cpu.set_carry(r & 0x100 != 0);
    cpu.set_substract(false);

    rb
}

/// Add `A` to `A` with carry
fn adc_a_a(cpu: &mut Cpu) {
    let a = cpu.a();

    let r = add_with_carry_and_set_flags(cpu, a, a);

    cpu.set_a(r);
}

/// Add `B` to `A` with carry
fn adc_a_b(cpu: &mut Cpu) {
    let a = cpu.a();
    let b = cpu.b();

    let r = add_with_carry_and_set_flags(cpu, a, b);

    cpu.set_a(r);
}

/// Add `C` to `A` with carry
fn adc_a_c(cpu: &mut Cpu) {
    let a = cpu.a();
    let c = cpu.c();

    let r = add_with_carry_and_set_flags(cpu, a, c);

    cpu.set_a(r);
}

/// Add `D` to `A` with carry
fn adc_a_d(cpu: &mut Cpu) {
    let a = cpu.a();
    let d = cpu.d();

    let r = add_with_carry_and_set_flags(cpu, a, d);

    cpu.set_a(r);
}

/// Add `E` to `A` with carry
fn adc_a_e(cpu: &mut Cpu) {
    let a = cpu.a();
    let e = cpu.e();

    let r = add_with_carry_and_set_flags(cpu, a, e);

    cpu.set_a(r);
}

/// Add `H` to `A` with carry
fn adc_a_h(cpu: &mut Cpu) {
    let a = cpu.a();
    let h = cpu.h();

    let r = add_with_carry_and_set_flags(cpu, a, h);

    cpu.set_a(r);
}

/// Add `L` to `A` with carry
fn adc_a_l(cpu: &mut Cpu) {
    let a = cpu.a();
    let l = cpu.l();

    let r = add_with_carry_and_set_flags(cpu, a, l);

    cpu.set_a(r);
}

/// Add `[HL]` to `A` with carry
fn adc_a_mhl(cpu: &mut Cpu) {
    let a  = cpu.a();
    let hl = cpu.hl();

    let n = cpu.fetch_byte(hl);

    let r = add_with_carry_and_set_flags(cpu, a, n);

    cpu.set_a(r);
}

/// Add `N` to `A` with carry
fn adc_a_n(cpu: &mut Cpu) {
    let a = cpu.a();
    let n = next_byte(cpu);

    let r = add_with_carry_and_set_flags(cpu, a, n);

    cpu.set_a(r);
}

/// Helper function to add two `u16` and update the CPU flags
fn add_word_and_set_flags(cpu: &mut Cpu, x: u16, y: u16) -> u16 {
    // Check for carry using 32bit arithmetics
    let x = x as u32;
    let y = y as u32;
    let r = x + y;

    cpu.set_substract(false);
    cpu.set_carry(r & 0x10000 != 0);
    cpu.set_halfcarry((x ^ y ^ r) & 0x1000 != 0);
    // zero flag is untouched.

    r as u16
}

/// Add `BC` to `HL`
fn add_hl_bc(cpu: &mut Cpu) {
    let hl = cpu.hl();
    let bc = cpu.bc();

    let r = add_word_and_set_flags(cpu, hl, bc);

    cpu.set_hl(r);
}

/// Add `DE` to `HL`
fn add_hl_de(cpu: &mut Cpu) {
    let hl = cpu.hl();
    let de = cpu.de();

    let r = add_word_and_set_flags(cpu, hl, de);

    cpu.set_hl(r);
}

/// Add `HL` to `HL`
fn add_hl_hl(cpu: &mut Cpu) {
    let hl = cpu.hl();

    let r = add_word_and_set_flags(cpu, hl, hl);

    cpu.set_hl(r);
}

/// Add `SP` to `HL`
fn add_hl_sp(cpu: &mut Cpu) {
    let hl = cpu.hl();
    let sp = cpu.sp();

    let r = add_word_and_set_flags(cpu, hl, sp);

    cpu.set_hl(r);
}

/// Add signed 8bit immediate value to `SP`
fn add_sp_sn(cpu: &mut Cpu) {
    let sp = cpu.sp() as i32;
    let n  = next_byte(cpu) as i8;

    let nn = n as i32;

    let r = sp + nn;

    cpu.set_substract(false);
    cpu.set_carry(r & 0x10000 != 0);
    cpu.set_halfcarry((sp ^ nn ^ r) & 0x1000 != 0);
    cpu.set_sp(r as u16);

    // pastraiser's page say that this 16bit add clears `Z` but other
    // sources disagree.
}

/// AND `A` with `A`
fn and_a_a(cpu: &mut Cpu) {
    let a = cpu.a();

    cpu.set_zero(a == 0);
    cpu.set_substract(false);
    cpu.set_halfcarry(true);
    cpu.set_carry(false);
}

/// AND `B` with `A`
fn and_a_b(cpu: &mut Cpu) {
    let a = cpu.a();
    let b = cpu.b();

    let r = a & b;

    cpu.set_zero(r == 0);
    cpu.set_substract(false);
    cpu.set_halfcarry(true);
    cpu.set_carry(false);

    cpu.set_a(r);
}

/// AND `C` with `A`
fn and_a_c(cpu: &mut Cpu) {
    let a = cpu.a();
    let c = cpu.c();

    let r = a & c;

    cpu.set_zero(r == 0);
    cpu.set_substract(false);
    cpu.set_halfcarry(true);
    cpu.set_carry(false);

    cpu.set_a(r);
}

/// AND `D` with `A`
fn and_a_d(cpu: &mut Cpu) {
    let a = cpu.a();
    let d = cpu.d();

    let r = a & d;

    cpu.set_zero(r == 0);
    cpu.set_substract(false);
    cpu.set_halfcarry(true);
    cpu.set_carry(false);

    cpu.set_a(r);
}

/// AND `E` with `A`
fn and_a_e(cpu: &mut Cpu) {
    let a = cpu.a();
    let e = cpu.e();

    let r = a & e;

    cpu.set_zero(r == 0);
    cpu.set_substract(false);
    cpu.set_halfcarry(true);
    cpu.set_carry(false);

    cpu.set_a(r);
}

/// AND `H` with `A`
fn and_a_h(cpu: &mut Cpu) {
    let a = cpu.a();
    let h = cpu.h();

    let r = a & h;

    cpu.set_zero(r == 0);
    cpu.set_substract(false);
    cpu.set_halfcarry(true);
    cpu.set_carry(false);

    cpu.set_a(r);
}

/// AND `L` with `A`
fn and_a_l(cpu: &mut Cpu) {
    let a = cpu.a();
    let l = cpu.l();

    let r = a & l;

    cpu.set_zero(r == 0);
    cpu.set_substract(false);
    cpu.set_halfcarry(true);
    cpu.set_carry(false);

    cpu.set_a(r);
}

/// AND `[HL]` with `A`
fn and_a_mhl(cpu: &mut Cpu) {
    let a  = cpu.a();
    let hl = cpu.hl();

    let n = cpu.fetch_byte(hl);

    let r = a & n;

    cpu.set_zero(r == 0);
    cpu.set_substract(false);
    cpu.set_halfcarry(true);
    cpu.set_carry(false);

    cpu.set_a(r);
}

/// AND `N` with `A`
fn and_a_n(cpu: &mut Cpu) {
    let a = cpu.a();
    let n = next_byte(cpu);

    let r = a & n;

    cpu.set_zero(r == 0);
    cpu.set_substract(false);
    cpu.set_halfcarry(true);
    cpu.set_carry(false);

    cpu.set_a(r);
}

/// OR `A` with `A`
fn or_a_a(cpu: &mut Cpu) {
    let a = cpu.a();

    cpu.set_zero(a == 0);
    cpu.set_substract(false);
    cpu.set_halfcarry(false);
    cpu.set_carry(false);
}

/// OR `B` with `A`
fn or_a_b(cpu: &mut Cpu) {
    let a = cpu.a();
    let b = cpu.b();

    let r = a | b;

    cpu.set_zero(r == 0);
    cpu.set_substract(false);
    cpu.set_halfcarry(false);
    cpu.set_carry(false);

    cpu.set_a(r);
}

/// OR `C` with `A`
fn or_a_c(cpu: &mut Cpu) {
    let a = cpu.a();
    let c = cpu.c();

    let r = a | c;

    cpu.set_zero(r == 0);
    cpu.set_substract(false);
    cpu.set_halfcarry(false);
    cpu.set_carry(false);

    cpu.set_a(r);
}

/// OR `D` with `A`
fn or_a_d(cpu: &mut Cpu) {
    let a = cpu.a();
    let d = cpu.d();

    let r = a | d;

    cpu.set_zero(r == 0);
    cpu.set_substract(false);
    cpu.set_halfcarry(false);
    cpu.set_carry(false);

    cpu.set_a(r);
}

/// OR `E` with `A`
fn or_a_e(cpu: &mut Cpu) {
    let a = cpu.a();
    let e = cpu.e();

    let r = a | e;

    cpu.set_zero(r == 0);
    cpu.set_substract(false);
    cpu.set_halfcarry(false);
    cpu.set_carry(false);

    cpu.set_a(r);
}

/// OR `H` with `A`
fn or_a_h(cpu: &mut Cpu) {
    let a = cpu.a();
    let h = cpu.h();

    let r = a | h;

    cpu.set_zero(r == 0);
    cpu.set_substract(false);
    cpu.set_halfcarry(false);
    cpu.set_carry(false);

    cpu.set_a(r);
}

/// OR `[HL]` with `A`
fn or_a_mhl(cpu: &mut Cpu) {
    let a  = cpu.a();
    let hl = cpu.hl();

    let n = cpu.fetch_byte(hl);

    let r = a | n;

    cpu.set_zero(r == 0);
    cpu.set_substract(false);
    cpu.set_halfcarry(false);
    cpu.set_carry(false);

    cpu.set_a(r);
}

/// OR `N` with `A`
fn or_a_n(cpu: &mut Cpu) {
    let a = cpu.a();
    let n = next_byte(cpu);

    let r = a | n;

    cpu.set_zero(r == 0);
    cpu.set_substract(false);
    cpu.set_halfcarry(false);
    cpu.set_carry(false);

    cpu.set_a(r);
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

/// XOR `[HL]` into `A`
fn xor_a_mhl(cpu: &mut Cpu) {
    let a  = cpu.a();

    let hl = cpu.hl();
    let n  = cpu.fetch_byte(hl);

    let r = a ^ n;

    cpu.set_a(r);

    cpu.clear_flags();
    cpu.set_zero(r == 0);
}

/// XOR `N` with `A`
fn xor_a_n(cpu: &mut Cpu) {
    let a = cpu.a();
    let n = next_byte(cpu);

    let r = a ^ n;

    cpu.set_a(r);

    cpu.clear_flags();
    cpu.set_zero(r == 0);
}

/// OR `L` with `A`
fn or_a_l(cpu: &mut Cpu) {
    let a = cpu.a();
    let l = cpu.l();

    let r = a | l;

    cpu.set_zero(r == 0);
    cpu.set_substract(false);
    cpu.set_halfcarry(false);
    cpu.set_carry(false);

    cpu.set_a(r);
}

/// Disable interrupts
fn di(cpu: &mut Cpu) {
    cpu.disable_interrupts();
}


/// Enable interrupts
fn ei(cpu: &mut Cpu) {
    cpu.enable_interrupts();
}

/// Halt and wait for interrupt
fn halt(cpu: &mut Cpu) {
    cpu.halt();
}

/// Stop, blank the screen and wait for button press
fn stop(cpu: &mut Cpu) {
    // The opcode takes two bytes for some reason, the 2nd byte should
    // be 00 but I don't know if it's important. Just skip it for now.
    let _ = next_byte(cpu);

    cpu.stop();
}

mod bitops {
    //! Emulation of instructions prefixed by 0xCB. They are all
    //! operations dealing with bit manipulation (rotations, shifts,
    //! bit set, bit clear...)

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
        (2, rlc_b),
        (2, rlc_c),
        (2, rlc_d),
        (2, rlc_e),
        (2, rlc_h),
        (2, rlc_l),
        (4, rlc_mhl),
        (2, rlc_a),
        (2, rrc_b),
        (2, rrc_c),
        (2, rrc_d),
        (2, rrc_e),
        (2, rrc_h),
        (2, rrc_l),
        (4, rrc_mhl),
        (2, rrc_a),
        // Opcodes CB 1X
        (2, rl_b),
        (2, rl_c),
        (2, rl_d),
        (2, rl_e),
        (2, rl_h),
        (2, rl_l),
        (4, rl_mhl),
        (2, rl_a),
        (2, rr_b),
        (2, rr_c),
        (2, rr_d),
        (2, rr_e),
        (2, rr_h),
        (2, rr_l),
        (4, rr_mhl),
        (2, rr_a),
        // Opcodes CB 2X
        (2, sla_b),
        (2, sla_c),
        (2, sla_d),
        (2, sla_e),
        (2, sla_h),
        (2, sla_l),
        (4, sla_mhl),
        (2, sla_a),
        (2, sra_b),
        (2, sra_c),
        (2, sra_d),
        (2, sra_e),
        (2, sra_h),
        (2, sra_l),
        (4, sra_mhl),
        (2, sra_a),
        // Opcodes CB 3X
        (2, swap_b),
        (2, swap_c),
        (2, swap_d),
        (2, swap_e),
        (2, swap_h),
        (2, swap_l),
        (4, swap_mhl),
        (2, swap_a),
        (2, srl_b),
        (2, srl_c),
        (2, srl_d),
        (2, srl_e),
        (2, srl_h),
        (2, srl_l),
        (4, srl_mhl),
        (2, srl_a),
        // Opcodes CB 4X
        (2, bit_b_0),
        (2, bit_c_0),
        (2, bit_d_0),
        (2, bit_e_0),
        (2, bit_h_0),
        (2, bit_l_0),
        (4, bit_mhl_0),
        (2, bit_a_0),
        (2, bit_b_1),
        (2, bit_c_1),
        (2, bit_d_1),
        (2, bit_e_1),
        (2, bit_h_1),
        (2, bit_l_1),
        (4, bit_mhl_1),
        (2, bit_a_1),
        // Opcodes CB 5X
        (2, bit_b_2),
        (2, bit_c_2),
        (2, bit_d_2),
        (2, bit_e_2),
        (2, bit_h_2),
        (2, bit_l_2),
        (4, bit_mhl_2),
        (2, bit_a_2),
        (2, bit_b_3),
        (2, bit_c_3),
        (2, bit_d_3),
        (2, bit_e_3),
        (2, bit_h_3),
        (2, bit_l_3),
        (4, bit_mhl_3),
        (2, bit_a_3),
        // Opcodes CB 6X
        (2, bit_b_4),
        (2, bit_c_4),
        (2, bit_d_4),
        (2, bit_e_4),
        (2, bit_h_4),
        (2, bit_l_4),
        (4, bit_mhl_4),
        (2, bit_a_4),
        (2, bit_b_5),
        (2, bit_c_5),
        (2, bit_d_5),
        (2, bit_e_5),
        (2, bit_h_5),
        (2, bit_l_5),
        (4, bit_mhl_5),
        (2, bit_a_5),
        // Opcodes CB 7X
        (2, bit_b_6),
        (2, bit_c_6),
        (2, bit_d_6),
        (2, bit_e_6),
        (2, bit_h_6),
        (2, bit_l_6),
        (4, bit_mhl_6),
        (2, bit_a_6),
        (2, bit_b_7),
        (2, bit_c_7),
        (2, bit_d_7),
        (2, bit_e_7),
        (2, bit_h_7),
        (2, bit_l_7),
        (4, bit_mhl_7),
        (2, bit_a_7),
        // Opcodes CB 8X
        (2, res_b_0),
        (2, res_c_0),
        (2, res_d_0),
        (2, res_e_0),
        (2, res_h_0),
        (2, res_l_0),
        (4, res_mhl_0),
        (2, res_a_0),
        (2, res_b_1),
        (2, res_c_1),
        (2, res_d_1),
        (2, res_e_1),
        (2, res_h_1),
        (2, res_l_1),
        (4, res_mhl_1),
        (2, res_a_1),
        // Opcodes CB 9X
        (2, res_b_2),
        (2, res_c_2),
        (2, res_d_2),
        (2, res_e_2),
        (2, res_h_2),
        (2, res_l_2),
        (4, res_mhl_2),
        (2, res_a_2),
        (2, res_b_3),
        (2, res_c_3),
        (2, res_d_3),
        (2, res_e_3),
        (2, res_h_3),
        (2, res_l_3),
        (4, res_mhl_3),
        (2, res_a_3),
        // Opcodes CB AX
        (2, res_b_4),
        (2, res_c_4),
        (2, res_d_4),
        (2, res_e_4),
        (2, res_h_4),
        (2, res_l_4),
        (4, res_mhl_4),
        (2, res_a_4),
        (2, res_b_5),
        (2, res_c_5),
        (2, res_d_5),
        (2, res_e_5),
        (2, res_h_5),
        (2, res_l_5),
        (4, res_mhl_5),
        (2, res_a_5),
        // Opcodes CB BX
        (2, res_b_6),
        (2, res_c_6),
        (2, res_d_6),
        (2, res_e_6),
        (2, res_h_6),
        (2, res_l_6),
        (4, res_mhl_6),
        (2, res_a_6),
        (2, res_b_7),
        (2, res_c_7),
        (2, res_d_7),
        (2, res_e_7),
        (2, res_h_7),
        (2, res_l_7),
        (4, res_mhl_7),
        (2, res_a_7),
        // Opcodes CB CX
        (2, set_b_0),
        (2, set_c_0),
        (2, set_d_0),
        (2, set_e_0),
        (2, set_h_0),
        (2, set_l_0),
        (4, set_mhl_0),
        (2, set_a_0),
        (2, set_b_1),
        (2, set_c_1),
        (2, set_d_1),
        (2, set_e_1),
        (2, set_h_1),
        (2, set_l_1),
        (4, set_mhl_1),
        (2, set_a_1),
        // Opcodes CB DX
        (2, set_b_2),
        (2, set_c_2),
        (2, set_d_2),
        (2, set_e_2),
        (2, set_h_2),
        (2, set_l_2),
        (4, set_mhl_2),
        (2, set_a_2),
        (2, set_b_3),
        (2, set_c_3),
        (2, set_d_3),
        (2, set_e_3),
        (2, set_h_3),
        (2, set_l_3),
        (4, set_mhl_3),
        (2, set_a_3),
        // Opcodes CB EX
        (2, set_b_4),
        (2, set_c_4),
        (2, set_d_4),
        (2, set_e_4),
        (2, set_h_4),
        (2, set_l_4),
        (4, set_mhl_4),
        (2, set_a_4),
        (2, set_b_5),
        (2, set_c_5),
        (2, set_d_5),
        (2, set_e_5),
        (2, set_h_5),
        (2, set_l_5),
        (4, set_mhl_5),
        (2, set_a_5),
        // Opcodes CB FX
        (2, set_b_6),
        (2, set_c_6),
        (2, set_d_6),
        (2, set_e_6),
        (2, set_h_6),
        (2, set_l_6),
        (4, set_mhl_6),
        (2, set_a_6),
        (2, set_b_7),
        (2, set_c_7),
        (2, set_d_7),
        (2, set_e_7),
        (2, set_h_7),
        (2, set_l_7),
        (4, set_mhl_7),
        (2, set_a_7),
    ];

    /// Helper function to swap the two nibbles in a `u8` and update
    /// cpu flags.
    fn swap(cpu: &mut Cpu, v: u8) -> u8 {
        cpu.set_zero(v == 0);

        (v << 4) | (v >> 4)
    }

    /// Swap low and high nibbles of `A`
    fn swap_a(cpu: &mut Cpu) {
        let a = cpu.a();

        let r = swap(cpu, a);

        cpu.set_a(r);
    }

    /// Swap low and high nibbles of `B`
    fn swap_b(cpu: &mut Cpu) {
        let b = cpu.b();

        let r = swap(cpu, b);

        cpu.set_b(r);
    }

    /// Swap low and high nibbles of `C`
    fn swap_c(cpu: &mut Cpu) {
        let c = cpu.c();

        let r = swap(cpu, c);

        cpu.set_c(r);
    }

    /// Swap low and high nibbles of `D`
    fn swap_d(cpu: &mut Cpu) {
        let d = cpu.d();

        let r = swap(cpu, d);

        cpu.set_d(r);
    }

    /// Swap low and high nibbles of `E`
    fn swap_e(cpu: &mut Cpu) {
        let e = cpu.e();

        let r = swap(cpu, e);

        cpu.set_e(r);
    }

    /// Swap low and high nibbles of `H`
    fn swap_h(cpu: &mut Cpu) {
        let h = cpu.h();

        let r = swap(cpu, h);

        cpu.set_h(r);
    }

    /// Swap low and high nibbles of `L`
    fn swap_l(cpu: &mut Cpu) {
        let l = cpu.l();

        let r = swap(cpu, l);

        cpu.set_l(r);
    }

    /// Swap low and high nibbles of `[HL]`
    fn swap_mhl(cpu: &mut Cpu) {
        let hl = cpu.hl();
        let n  = cpu.fetch_byte(hl);

        let r = swap(cpu, n);

        cpu.store_byte(hl, r);
    }

    /// Helper function to test one bit in a u8. Return true if bit is
    /// 0.
    fn bit_zero(val: u8, bit: u8) -> bool {
        (val & (1u8 << (bit as uint))) != 0
    }

    /// Helper function to test bits in `A`
    fn bit_a(cpu: &mut Cpu, bit: u8) {
        let a = cpu.a();

        cpu.set_zero(bit_zero(a, bit));
        cpu.set_substract(false);
        cpu.set_halfcarry(true);
    }

    /// Test `A` bit 0
    fn bit_a_0(cpu: &mut Cpu) {
        bit_a(cpu, 0);
    }

    /// Test `A` bit 1
    fn bit_a_1(cpu: &mut Cpu) {
        bit_a(cpu, 1);
    }

    /// Test `A` bit 2
    fn bit_a_2(cpu: &mut Cpu) {
        bit_a(cpu, 2);
    }

    /// Test `A` bit 3
    fn bit_a_3(cpu: &mut Cpu) {
        bit_a(cpu, 3);
    }

    /// Test `A` bit 4
    fn bit_a_4(cpu: &mut Cpu) {
        bit_a(cpu, 4);
    }

    /// Test `A` bit 5
    fn bit_a_5(cpu: &mut Cpu) {
        bit_a(cpu, 5);
    }

    /// Test `A` bit 6
    fn bit_a_6(cpu: &mut Cpu) {
        bit_a(cpu, 6);
    }

    /// Test `A` bit 7
    fn bit_a_7(cpu: &mut Cpu) {
        bit_a(cpu, 7);
    }

    /// Helper function to test bits in `B`
    fn bit_b(cpu: &mut Cpu, bit: u8) {
        let b = cpu.b();

        cpu.set_zero(bit_zero(b, bit));
        cpu.set_substract(false);
        cpu.set_halfcarry(true);
    }

    /// Test `B` bit 0
    fn bit_b_0(cpu: &mut Cpu) {
        bit_b(cpu, 0);
    }

    /// Test `B` bit 1
    fn bit_b_1(cpu: &mut Cpu) {
        bit_b(cpu, 1);
    }

    /// Test `B` bit 2
    fn bit_b_2(cpu: &mut Cpu) {
        bit_b(cpu, 2);
    }

    /// Test `B` bit 3
    fn bit_b_3(cpu: &mut Cpu) {
        bit_b(cpu, 3);
    }

    /// Test `B` bit 4
    fn bit_b_4(cpu: &mut Cpu) {
        bit_b(cpu, 4);
    }

    /// Test `B` bit 5
    fn bit_b_5(cpu: &mut Cpu) {
        bit_b(cpu, 5);
    }

    /// Test `B` bit 6
    fn bit_b_6(cpu: &mut Cpu) {
        bit_b(cpu, 6);
    }

    /// Test `B` bit 7
    fn bit_b_7(cpu: &mut Cpu) {
        bit_b(cpu, 7);
    }

    /// Helper function to test bits in `C`
    fn bit_c(cpu: &mut Cpu, bit: u8) {
        let c = cpu.c();

        cpu.set_zero(bit_zero(c, bit));
        cpu.set_substract(false);
        cpu.set_halfcarry(true);
    }

    /// Test `C` bit 0
    fn bit_c_0(cpu: &mut Cpu) {
        bit_c(cpu, 0);
    }

    /// Test `C` bit 1
    fn bit_c_1(cpu: &mut Cpu) {
        bit_c(cpu, 1);
    }

    /// Test `C` bit 2
    fn bit_c_2(cpu: &mut Cpu) {
        bit_c(cpu, 2);
    }

    /// Test `C` bit 3
    fn bit_c_3(cpu: &mut Cpu) {
        bit_c(cpu, 3);
    }

    /// Test `C` bit 4
    fn bit_c_4(cpu: &mut Cpu) {
        bit_c(cpu, 4);
    }

    /// Test `C` bit 5
    fn bit_c_5(cpu: &mut Cpu) {
        bit_c(cpu, 5);
    }

    /// Test `C` bit 6
    fn bit_c_6(cpu: &mut Cpu) {
        bit_c(cpu, 6);
    }

    /// Test `C` bit 7
    fn bit_c_7(cpu: &mut Cpu) {
        bit_c(cpu, 7);
    }

    /// Helper function to test bits in `D`
    fn bit_d(cpu: &mut Cpu, bit: u8) {
        let d = cpu.d();

        cpu.set_zero(bit_zero(d, bit));
        cpu.set_substract(false);
        cpu.set_halfcarry(true);
    }

    /// Test `D` bit 0
    fn bit_d_0(cpu: &mut Cpu) {
        bit_d(cpu, 0);
    }

    /// Test `D` bit 1
    fn bit_d_1(cpu: &mut Cpu) {
        bit_d(cpu, 1);
    }

    /// Test `D` bit 2
    fn bit_d_2(cpu: &mut Cpu) {
        bit_d(cpu, 2);
    }

    /// Test `D` bit 3
    fn bit_d_3(cpu: &mut Cpu) {
        bit_d(cpu, 3);
    }

    /// Test `D` bit 4
    fn bit_d_4(cpu: &mut Cpu) {
        bit_d(cpu, 4);
    }

    /// Test `D` bit 5
    fn bit_d_5(cpu: &mut Cpu) {
        bit_d(cpu, 5);
    }

    /// Test `D` bit 6
    fn bit_d_6(cpu: &mut Cpu) {
        bit_d(cpu, 6);
    }

    /// Test `D` bit 7
    fn bit_d_7(cpu: &mut Cpu) {
        bit_d(cpu, 7);
    }

    /// Helper function to test bits in `E`
    fn bit_e(cpu: &mut Cpu, bit: u8) {
        let e = cpu.e();

        cpu.set_zero(bit_zero(e, bit));
        cpu.set_substract(false);
        cpu.set_halfcarry(true);
    }

    /// Test `E` bit 0
    fn bit_e_0(cpu: &mut Cpu) {
        bit_e(cpu, 0);
    }

    /// Test `E` bit 1
    fn bit_e_1(cpu: &mut Cpu) {
        bit_e(cpu, 1);
    }

    /// Test `E` bit 2
    fn bit_e_2(cpu: &mut Cpu) {
        bit_e(cpu, 2);
    }

    /// Test `E` bit 3
    fn bit_e_3(cpu: &mut Cpu) {
        bit_e(cpu, 3);
    }

    /// Test `E` bit 4
    fn bit_e_4(cpu: &mut Cpu) {
        bit_e(cpu, 4);
    }

    /// Test `E` bit 5
    fn bit_e_5(cpu: &mut Cpu) {
        bit_e(cpu, 5);
    }

    /// Test `E` bit 6
    fn bit_e_6(cpu: &mut Cpu) {
        bit_e(cpu, 6);
    }

    /// Test `E` bit 7
    fn bit_e_7(cpu: &mut Cpu) {
        bit_e(cpu, 7);
    }

    /// Helper function to test bits in `H`
    fn bit_h(cpu: &mut Cpu, bit: u8) {
        let h = cpu.h();

        cpu.set_zero(bit_zero(h, bit));
        cpu.set_substract(false);
        cpu.set_halfcarry(true);
    }

    /// Test `H` bit 0
    fn bit_h_0(cpu: &mut Cpu) {
        bit_h(cpu, 0);
    }

    /// Test `H` bit 1
    fn bit_h_1(cpu: &mut Cpu) {
        bit_h(cpu, 1);
    }

    /// Test `H` bit 2
    fn bit_h_2(cpu: &mut Cpu) {
        bit_h(cpu, 2);
    }

    /// Test `H` bit 3
    fn bit_h_3(cpu: &mut Cpu) {
        bit_h(cpu, 3);
    }

    /// Test `H` bit 4
    fn bit_h_4(cpu: &mut Cpu) {
        bit_h(cpu, 4);
    }

    /// Test `H` bit 5
    fn bit_h_5(cpu: &mut Cpu) {
        bit_h(cpu, 5);
    }

    /// Test `H` bit 6
    fn bit_h_6(cpu: &mut Cpu) {
        bit_h(cpu, 6);
    }

    /// Test `H` bit 7
    fn bit_h_7(cpu: &mut Cpu) {
        bit_h(cpu, 7);
    }

    /// Helper function to test bits in `L`
    fn bit_l(cpu: &mut Cpu, bit: u8) {
        let l = cpu.l();

        cpu.set_zero(bit_zero(l, bit));
        cpu.set_substract(false);
        cpu.set_halfcarry(true);
    }

    /// Test `L` bit 0
    fn bit_l_0(cpu: &mut Cpu) {
        bit_l(cpu, 0);
    }

    /// Test `L` bit 1
    fn bit_l_1(cpu: &mut Cpu) {
        bit_l(cpu, 1);
    }

    /// Test `L` bit 2
    fn bit_l_2(cpu: &mut Cpu) {
        bit_l(cpu, 2);
    }

    /// Test `L` bit 3
    fn bit_l_3(cpu: &mut Cpu) {
        bit_l(cpu, 3);
    }

    /// Test `L` bit 4
    fn bit_l_4(cpu: &mut Cpu) {
        bit_l(cpu, 4);
    }

    /// Test `L` bit 5
    fn bit_l_5(cpu: &mut Cpu) {
        bit_l(cpu, 5);
    }

    /// Test `L` bit 6
    fn bit_l_6(cpu: &mut Cpu) {
        bit_l(cpu, 6);
    }

    /// Test `L` bit 7
    fn bit_l_7(cpu: &mut Cpu) {
        bit_l(cpu, 7);
    }

    /// Helper function to test bits in `[HL]`
    fn bit_mhl(cpu: &mut Cpu, bit: u8) {
        let hl = cpu.hl();

        let n = cpu.fetch_byte(hl);

        cpu.set_zero(bit_zero(n, bit));
        cpu.set_substract(false);
        cpu.set_halfcarry(true);
    }

    /// Test `[HL]` bit 0
    fn bit_mhl_0(cpu: &mut Cpu) {
        bit_mhl(cpu, 0);
    }

    /// Test `[HL]` bit 1
    fn bit_mhl_1(cpu: &mut Cpu) {
        bit_mhl(cpu, 1);
    }

    /// Test `[HL]` bit 2
    fn bit_mhl_2(cpu: &mut Cpu) {
        bit_mhl(cpu, 2);
    }

    /// Test `[HL]` bit 3
    fn bit_mhl_3(cpu: &mut Cpu) {
        bit_mhl(cpu, 3);
    }

    /// Test `[HL]` bit 4
    fn bit_mhl_4(cpu: &mut Cpu) {
        bit_mhl(cpu, 4);
    }

    /// Test `[HL]` bit 5
    fn bit_mhl_5(cpu: &mut Cpu) {
        bit_mhl(cpu, 5);
    }

    /// Test `[HL]` bit 6
    fn bit_mhl_6(cpu: &mut Cpu) {
        bit_mhl(cpu, 6);
    }

    /// Test `[HL]` bit 7
    fn bit_mhl_7(cpu: &mut Cpu) {
        bit_mhl(cpu, 7);
    }

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

    /// Helper function to clear bits in `B`
    fn res_b(cpu: &mut Cpu, bit: u8) {
        let b = cpu.b();

        cpu.set_b(res(b, bit));
    }

    /// Clear `B` bit 0
    fn res_b_0(cpu: &mut Cpu) {
        res_b(cpu, 0);
    }

    /// Clear `B` bit 1
    fn res_b_1(cpu: &mut Cpu) {
        res_b(cpu, 1);
    }

    /// Clear `B` bit 2
    fn res_b_2(cpu: &mut Cpu) {
        res_b(cpu, 2);
    }

    /// Clear `B` bit 3
    fn res_b_3(cpu: &mut Cpu) {
        res_b(cpu, 3);
    }

    /// Clear `B` bit 4
    fn res_b_4(cpu: &mut Cpu) {
        res_b(cpu, 4);
    }

    /// Clear `B` bit 5
    fn res_b_5(cpu: &mut Cpu) {
        res_b(cpu, 5);
    }

    /// Clear `B` bit 6
    fn res_b_6(cpu: &mut Cpu) {
        res_b(cpu, 6);
    }

    /// Clear `B` bit 7
    fn res_b_7(cpu: &mut Cpu) {
        res_b(cpu, 7);
    }

    /// Helper function to clear bits in `C`
    fn res_c(cpu: &mut Cpu, bit: u8) {
        let c = cpu.c();

        cpu.set_c(res(c, bit));
    }

    /// Clear `C` bit 0
    fn res_c_0(cpu: &mut Cpu) {
        res_c(cpu, 0);
    }

    /// Clear `C` bit 1
    fn res_c_1(cpu: &mut Cpu) {
        res_c(cpu, 1);
    }

    /// Clear `C` bit 2
    fn res_c_2(cpu: &mut Cpu) {
        res_c(cpu, 2);
    }

    /// Clear `C` bit 3
    fn res_c_3(cpu: &mut Cpu) {
        res_c(cpu, 3);
    }

    /// Clear `C` bit 4
    fn res_c_4(cpu: &mut Cpu) {
        res_c(cpu, 4);
    }

    /// Clear `C` bit 5
    fn res_c_5(cpu: &mut Cpu) {
        res_c(cpu, 5);
    }

    /// Clear `C` bit 6
    fn res_c_6(cpu: &mut Cpu) {
        res_c(cpu, 6);
    }

    /// Clear `C` bit 7
    fn res_c_7(cpu: &mut Cpu) {
        res_c(cpu, 7);
    }

    /// Helper function to clear bits in `D`
    fn res_d(cpu: &mut Cpu, bit: u8) {
        let d = cpu.d();

        cpu.set_d(res(d, bit));
    }

    /// Clear `D` bit 0
    fn res_d_0(cpu: &mut Cpu) {
        res_d(cpu, 0);
    }

    /// Clear `D` bit 1
    fn res_d_1(cpu: &mut Cpu) {
        res_d(cpu, 1);
    }

    /// Clear `D` bit 2
    fn res_d_2(cpu: &mut Cpu) {
        res_d(cpu, 2);
    }

    /// Clear `D` bit 3
    fn res_d_3(cpu: &mut Cpu) {
        res_d(cpu, 3);
    }

    /// Clear `D` bit 4
    fn res_d_4(cpu: &mut Cpu) {
        res_d(cpu, 4);
    }

    /// Clear `D` bit 5
    fn res_d_5(cpu: &mut Cpu) {
        res_d(cpu, 5);
    }

    /// Clear `D` bit 6
    fn res_d_6(cpu: &mut Cpu) {
        res_d(cpu, 6);
    }

    /// Clear `D` bit 7
    fn res_d_7(cpu: &mut Cpu) {
        res_d(cpu, 7);
    }

    /// Helper function to clear bits in `E`
    fn res_e(cpu: &mut Cpu, bit: u8) {
        let e = cpu.e();

        cpu.set_e(res(e, bit));
    }

    /// Clear `E` bit 0
    fn res_e_0(cpu: &mut Cpu) {
        res_e(cpu, 0);
    }

    /// Clear `E` bit 1
    fn res_e_1(cpu: &mut Cpu) {
        res_e(cpu, 1);
    }

    /// Clear `E` bit 2
    fn res_e_2(cpu: &mut Cpu) {
        res_e(cpu, 2);
    }

    /// Clear `E` bit 3
    fn res_e_3(cpu: &mut Cpu) {
        res_e(cpu, 3);
    }

    /// Clear `E` bit 4
    fn res_e_4(cpu: &mut Cpu) {
        res_e(cpu, 4);
    }

    /// Clear `E` bit 5
    fn res_e_5(cpu: &mut Cpu) {
        res_e(cpu, 5);
    }

    /// Clear `E` bit 6
    fn res_e_6(cpu: &mut Cpu) {
        res_e(cpu, 6);
    }

    /// Clear `E` bit 7
    fn res_e_7(cpu: &mut Cpu) {
        res_e(cpu, 7);
    }

    /// Helper function to clear bits in `H`
    fn res_h(cpu: &mut Cpu, bit: u8) {
        let h = cpu.h();

        cpu.set_h(res(h, bit));
    }

    /// Clear `H` bit 0
    fn res_h_0(cpu: &mut Cpu) {
        res_h(cpu, 0);
    }

    /// Clear `H` bit 1
    fn res_h_1(cpu: &mut Cpu) {
        res_h(cpu, 1);
    }

    /// Clear `H` bit 2
    fn res_h_2(cpu: &mut Cpu) {
        res_h(cpu, 2);
    }

    /// Clear `H` bit 3
    fn res_h_3(cpu: &mut Cpu) {
        res_h(cpu, 3);
    }

    /// Clear `H` bit 4
    fn res_h_4(cpu: &mut Cpu) {
        res_h(cpu, 4);
    }

    /// Clear `H` bit 5
    fn res_h_5(cpu: &mut Cpu) {
        res_h(cpu, 5);
    }

    /// Clear `H` bit 6
    fn res_h_6(cpu: &mut Cpu) {
        res_h(cpu, 6);
    }

    /// Clear `H` bit 7
    fn res_h_7(cpu: &mut Cpu) {
        res_h(cpu, 7);
    }

    /// Helper function to clear bits in `L`
    fn res_l(cpu: &mut Cpu, bit: u8) {
        let l = cpu.l();

        cpu.set_l(res(l, bit));
    }

    /// Clear `L` bit 0
    fn res_l_0(cpu: &mut Cpu) {
        res_l(cpu, 0);
    }

    /// Clear `L` bit 1
    fn res_l_1(cpu: &mut Cpu) {
        res_l(cpu, 1);
    }

    /// Clear `L` bit 2
    fn res_l_2(cpu: &mut Cpu) {
        res_l(cpu, 2);
    }

    /// Clear `L` bit 3
    fn res_l_3(cpu: &mut Cpu) {
        res_l(cpu, 3);
    }

    /// Clear `L` bit 4
    fn res_l_4(cpu: &mut Cpu) {
        res_l(cpu, 4);
    }

    /// Clear `L` bit 5
    fn res_l_5(cpu: &mut Cpu) {
        res_l(cpu, 5);
    }

    /// Clear `L` bit 6
    fn res_l_6(cpu: &mut Cpu) {
        res_l(cpu, 6);
    }

    /// Clear `L` bit 7
    fn res_l_7(cpu: &mut Cpu) {
        res_l(cpu, 7);
    }

    /// Helper function to clear bits in `[HL]`
    fn res_mhl(cpu: &mut Cpu, bit: u8) {
        let hl = cpu.hl();

        let n = cpu.fetch_byte(hl);

        cpu.store_byte(hl, res(n, bit))
    }

    /// Clear `[HL]` bit 0
    fn res_mhl_0(cpu: &mut Cpu) {
        res_mhl(cpu, 0);
    }

    /// Clear `[HL]` bit 1
    fn res_mhl_1(cpu: &mut Cpu) {
        res_mhl(cpu, 1);
    }

    /// Clear `[HL]` bit 2
    fn res_mhl_2(cpu: &mut Cpu) {
        res_mhl(cpu, 2);
    }

    /// Clear `[HL]` bit 3
    fn res_mhl_3(cpu: &mut Cpu) {
        res_mhl(cpu, 3);
    }

    /// Clear `[HL]` bit 4
    fn res_mhl_4(cpu: &mut Cpu) {
        res_mhl(cpu, 4);
    }

    /// Clear `[HL]` bit 5
    fn res_mhl_5(cpu: &mut Cpu) {
        res_mhl(cpu, 5);
    }

    /// Clear `[HL]` bit 6
    fn res_mhl_6(cpu: &mut Cpu) {
        res_mhl(cpu, 6);
    }

    /// Clear `[HL]` bit 7
    fn res_mhl_7(cpu: &mut Cpu) {
        res_mhl(cpu, 7);
    }

    /// Helper function to set one bit in a u8
    fn set(val: u8, bit: u8) -> u8 {
        val | (1u8 << (bit as uint))
    }

    /// Helper function to set bits in `A`
    fn set_a(cpu: &mut Cpu, bit: u8) {
        let a = cpu.a();

        cpu.set_a(set(a, bit));
    }

    /// Set `A` bit 0
    fn set_a_0(cpu: &mut Cpu) {
        set_a(cpu, 0);
    }

    /// Set `A` bit 1
    fn set_a_1(cpu: &mut Cpu) {
        set_a(cpu, 1);
    }

    /// Set `A` bit 2
    fn set_a_2(cpu: &mut Cpu) {
        set_a(cpu, 2);
    }

    /// Set `A` bit 3
    fn set_a_3(cpu: &mut Cpu) {
        set_a(cpu, 3);
    }

    /// Set `A` bit 4
    fn set_a_4(cpu: &mut Cpu) {
        set_a(cpu, 4);
    }

    /// Set `A` bit 5
    fn set_a_5(cpu: &mut Cpu) {
        set_a(cpu, 5);
    }

    /// Set `A` bit 6
    fn set_a_6(cpu: &mut Cpu) {
        set_a(cpu, 6);
    }

    /// Set `A` bit 7
    fn set_a_7(cpu: &mut Cpu) {
        set_a(cpu, 7);
    }

    /// Helper function to set bits in `B`
    fn set_b(cpu: &mut Cpu, bit: u8) {
        let b = cpu.b();

        cpu.set_b(set(b, bit));
    }

    /// Set `B` bit 0
    fn set_b_0(cpu: &mut Cpu) {
        set_b(cpu, 0);
    }

    /// Set `B` bit 1
    fn set_b_1(cpu: &mut Cpu) {
        set_b(cpu, 1);
    }

    /// Set `B` bit 2
    fn set_b_2(cpu: &mut Cpu) {
        set_b(cpu, 2);
    }

    /// Set `B` bit 3
    fn set_b_3(cpu: &mut Cpu) {
        set_b(cpu, 3);
    }

    /// Set `B` bit 4
    fn set_b_4(cpu: &mut Cpu) {
        set_b(cpu, 4);
    }

    /// Set `B` bit 5
    fn set_b_5(cpu: &mut Cpu) {
        set_b(cpu, 5);
    }

    /// Set `B` bit 6
    fn set_b_6(cpu: &mut Cpu) {
        set_b(cpu, 6);
    }

    /// Set `B` bit 7
    fn set_b_7(cpu: &mut Cpu) {
        set_b(cpu, 7);
    }

    /// Helper function to set bits in `C`
    fn set_c(cpu: &mut Cpu, bit: u8) {
        let c = cpu.c();

        cpu.set_c(set(c, bit));
    }

    /// Set `C` bit 0
    fn set_c_0(cpu: &mut Cpu) {
        set_c(cpu, 0);
    }

    /// Set `C` bit 1
    fn set_c_1(cpu: &mut Cpu) {
        set_c(cpu, 1);
    }

    /// Set `C` bit 2
    fn set_c_2(cpu: &mut Cpu) {
        set_c(cpu, 2);
    }

    /// Set `C` bit 3
    fn set_c_3(cpu: &mut Cpu) {
        set_c(cpu, 3);
    }

    /// Set `C` bit 4
    fn set_c_4(cpu: &mut Cpu) {
        set_c(cpu, 4);
    }

    /// Set `C` bit 5
    fn set_c_5(cpu: &mut Cpu) {
        set_c(cpu, 5);
    }

    /// Set `C` bit 6
    fn set_c_6(cpu: &mut Cpu) {
        set_c(cpu, 6);
    }

    /// Set `C` bit 7
    fn set_c_7(cpu: &mut Cpu) {
        set_c(cpu, 7);
    }

    /// Helper function to set bits in `D`
    fn set_d(cpu: &mut Cpu, bit: u8) {
        let d = cpu.d();

        cpu.set_d(set(d, bit));
    }

    /// Set `D` bit 0
    fn set_d_0(cpu: &mut Cpu) {
        set_d(cpu, 0);
    }

    /// Set `D` bit 1
    fn set_d_1(cpu: &mut Cpu) {
        set_d(cpu, 1);
    }

    /// Set `D` bit 2
    fn set_d_2(cpu: &mut Cpu) {
        set_d(cpu, 2);
    }

    /// Set `D` bit 3
    fn set_d_3(cpu: &mut Cpu) {
        set_d(cpu, 3);
    }

    /// Set `D` bit 4
    fn set_d_4(cpu: &mut Cpu) {
        set_d(cpu, 4);
    }

    /// Set `D` bit 5
    fn set_d_5(cpu: &mut Cpu) {
        set_d(cpu, 5);
    }

    /// Set `D` bit 6
    fn set_d_6(cpu: &mut Cpu) {
        set_d(cpu, 6);
    }

    /// Set `D` bit 7
    fn set_d_7(cpu: &mut Cpu) {
        set_d(cpu, 7);
    }

    /// Helper function to set bits in `E`
    fn set_e(cpu: &mut Cpu, bit: u8) {
        let e = cpu.e();

        cpu.set_e(set(e, bit));
    }

    /// Set `E` bit 0
    fn set_e_0(cpu: &mut Cpu) {
        set_e(cpu, 0);
    }

    /// Set `E` bit 1
    fn set_e_1(cpu: &mut Cpu) {
        set_e(cpu, 1);
    }

    /// Set `E` bit 2
    fn set_e_2(cpu: &mut Cpu) {
        set_e(cpu, 2);
    }

    /// Set `E` bit 3
    fn set_e_3(cpu: &mut Cpu) {
        set_e(cpu, 3);
    }

    /// Set `E` bit 4
    fn set_e_4(cpu: &mut Cpu) {
        set_e(cpu, 4);
    }

    /// Set `E` bit 5
    fn set_e_5(cpu: &mut Cpu) {
        set_e(cpu, 5);
    }

    /// Set `E` bit 6
    fn set_e_6(cpu: &mut Cpu) {
        set_e(cpu, 6);
    }

    /// Set `E` bit 7
    fn set_e_7(cpu: &mut Cpu) {
        set_e(cpu, 7);
    }

    /// Helper function to set bits in `H`
    fn set_h(cpu: &mut Cpu, bit: u8) {
        let h = cpu.h();

        cpu.set_h(set(h, bit));
    }

    /// Set `H` bit 0
    fn set_h_0(cpu: &mut Cpu) {
        set_h(cpu, 0);
    }

    /// Set `H` bit 1
    fn set_h_1(cpu: &mut Cpu) {
        set_h(cpu, 1);
    }

    /// Set `H` bit 2
    fn set_h_2(cpu: &mut Cpu) {
        set_h(cpu, 2);
    }

    /// Set `H` bit 3
    fn set_h_3(cpu: &mut Cpu) {
        set_h(cpu, 3);
    }

    /// Set `H` bit 4
    fn set_h_4(cpu: &mut Cpu) {
        set_h(cpu, 4);
    }

    /// Set `H` bit 5
    fn set_h_5(cpu: &mut Cpu) {
        set_h(cpu, 5);
    }

    /// Set `H` bit 6
    fn set_h_6(cpu: &mut Cpu) {
        set_h(cpu, 6);
    }

    /// Set `H` bit 7
    fn set_h_7(cpu: &mut Cpu) {
        set_h(cpu, 7);
    }

    /// Helper function to set bits in `L`
    fn set_l(cpu: &mut Cpu, bit: u8) {
        let l = cpu.l();

        cpu.set_l(set(l, bit));
    }

    /// Set `L` bit 0
    fn set_l_0(cpu: &mut Cpu) {
        set_l(cpu, 0);
    }

    /// Set `L` bit 1
    fn set_l_1(cpu: &mut Cpu) {
        set_l(cpu, 1);
    }

    /// Set `L` bit 2
    fn set_l_2(cpu: &mut Cpu) {
        set_l(cpu, 2);
    }

    /// Set `L` bit 3
    fn set_l_3(cpu: &mut Cpu) {
        set_l(cpu, 3);
    }

    /// Set `L` bit 4
    fn set_l_4(cpu: &mut Cpu) {
        set_l(cpu, 4);
    }

    /// Set `L` bit 5
    fn set_l_5(cpu: &mut Cpu) {
        set_l(cpu, 5);
    }

    /// Set `L` bit 6
    fn set_l_6(cpu: &mut Cpu) {
        set_l(cpu, 6);
    }

    /// Set `L` bit 7
    fn set_l_7(cpu: &mut Cpu) {
        set_l(cpu, 7);
    }

    /// Helper function to set bits in `[HL]`
    fn set_mhl(cpu: &mut Cpu, bit: u8) {
        let hl = cpu.hl();

        let n = cpu.fetch_byte(hl);

        cpu.store_byte(hl, set(n, bit))
    }

    /// Set `[HL]` bit 0
    fn set_mhl_0(cpu: &mut Cpu) {
        set_mhl(cpu, 0);
    }

    /// Set `[HL]` bit 1
    fn set_mhl_1(cpu: &mut Cpu) {
        set_mhl(cpu, 1);
    }

    /// Set `[HL]` bit 2
    fn set_mhl_2(cpu: &mut Cpu) {
        set_mhl(cpu, 2);
    }

    /// Set `[HL]` bit 3
    fn set_mhl_3(cpu: &mut Cpu) {
        set_mhl(cpu, 3);
    }

    /// Set `[HL]` bit 4
    fn set_mhl_4(cpu: &mut Cpu) {
        set_mhl(cpu, 4);
    }

    /// Set `[HL]` bit 5
    fn set_mhl_5(cpu: &mut Cpu) {
        set_mhl(cpu, 5);
    }

    /// Set `[HL]` bit 6
    fn set_mhl_6(cpu: &mut Cpu) {
        set_mhl(cpu, 6);
    }

    /// Set `[HL]` bit 7
    fn set_mhl_7(cpu: &mut Cpu) {
        set_mhl(cpu, 7);
    }

    /// Helper function to shift an `u8` to the right and update CPU
    /// flags.
    fn srl(cpu: &mut Cpu, v: u8)  -> u8 {
        cpu.set_carry(v & 1 != 0);

        let r = v >> 1;

        cpu.set_zero(r == 0);

        cpu.set_substract(false);
        cpu.set_halfcarry(false);

        r
    }

    /// Shift `A` to the right
    fn srl_a(cpu: &mut Cpu) {
        let a = cpu.a();

        let r = srl(cpu, a);

        cpu.set_a(r);
    }

    /// Shift `B` to the right
    fn srl_b(cpu: &mut Cpu) {
        let b = cpu.b();

        let r = srl(cpu, b);

        cpu.set_b(r);
    }

    /// Shift `C` to the right
    fn srl_c(cpu: &mut Cpu) {
        let c = cpu.c();

        let r = srl(cpu, c);

        cpu.set_c(r);
    }

    /// Shift `D` to the right
    fn srl_d(cpu: &mut Cpu) {
        let d = cpu.d();

        let r = srl(cpu, d);

        cpu.set_d(r);
    }

    /// Shift `E` to the right
    fn srl_e(cpu: &mut Cpu) {
        let e = cpu.e();

        let r = srl(cpu, e);

        cpu.set_e(r);
    }

    /// Shift `H` to the right
    fn srl_h(cpu: &mut Cpu) {
        let h = cpu.h();

        let r = srl(cpu, h);

        cpu.set_h(r);
    }

    /// Shift `L` to the right
    fn srl_l(cpu: &mut Cpu) {
        let l = cpu.l();

        let r = srl(cpu, l);

        cpu.set_l(r);
    }

    /// Shift `[HL]` to the right
    fn srl_mhl(cpu: &mut Cpu) {
        let hl = cpu.hl();
        let n  = cpu.fetch_byte(hl);

        let r = srl(cpu, n);

        cpu.store_byte(hl, r);
    }

    /// Helper function to shift an `u8` to the left and update CPU
    /// flags.
    fn sla(cpu: &mut Cpu, v: u8)  -> u8 {
        cpu.set_carry(v & 0x80 != 0);

        let r = v << 1;

        cpu.set_zero(r == 0);

        cpu.set_substract(false);
        cpu.set_halfcarry(false);

        r
    }

    /// Shift `A` to the left
    fn sla_a(cpu: &mut Cpu) {
        let a = cpu.a();

        let r = sla(cpu, a);

        cpu.set_a(r);
    }

    /// Shift `B` to the left
    fn sla_b(cpu: &mut Cpu) {
        let b = cpu.b();

        let r = sla(cpu, b);

        cpu.set_b(r);
    }

    /// Shift `C` to the left
    fn sla_c(cpu: &mut Cpu) {
        let c = cpu.c();

        let r = sla(cpu, c);

        cpu.set_c(r);
    }

    /// Shift `D` to the left
    fn sla_d(cpu: &mut Cpu) {
        let d = cpu.d();

        let r = sla(cpu, d);

        cpu.set_d(r);
    }

    /// Shift `E` to the left
    fn sla_e(cpu: &mut Cpu) {
        let e = cpu.e();

        let r = sla(cpu, e);

        cpu.set_e(r);
    }

    /// Shift `H` to the left
    fn sla_h(cpu: &mut Cpu) {
        let h = cpu.h();

        let r = sla(cpu, h);

        cpu.set_h(r);
    }

    /// Shift `L` to the left
    fn sla_l(cpu: &mut Cpu) {
        let l = cpu.l();

        let r = sla(cpu, l);

        cpu.set_l(r);
    }

    /// Shift `[HL]` to the left
    fn sla_mhl(cpu: &mut Cpu) {
        let hl = cpu.hl();
        let n  = cpu.fetch_byte(hl);

        let r = sla(cpu, n);

        cpu.store_byte(hl, r);
    }

    /// Helper function to shift an `u8` to the right and update CPU
    /// flags. MSB is not affected.
    fn sra(cpu: &mut Cpu, v: u8)  -> u8 {
        cpu.set_carry(v & 1 != 0);

        let r = (v >> 1) | (v & 0x80);

        cpu.set_zero(r == 0);

        cpu.set_substract(false);
        cpu.set_halfcarry(false);

        r
    }

    /// Shift `A` to the right. MSB is not affected.
    fn sra_a(cpu: &mut Cpu) {
        let a = cpu.a();

        let r = sra(cpu, a);

        cpu.set_a(r);
    }

    /// Shift `B` to the right. MSB is not affected.
    fn sra_b(cpu: &mut Cpu) {
        let b = cpu.b();

        let r = sra(cpu, b);

        cpu.set_b(r);
    }

    /// Shift `C` to the right. MSB is not affected.
    fn sra_c(cpu: &mut Cpu) {
        let c = cpu.c();

        let r = sra(cpu, c);

        cpu.set_c(r);
    }

    /// Shift `D` to the right. MSB is not affected.
    fn sra_d(cpu: &mut Cpu) {
        let d = cpu.d();

        let r = sra(cpu, d);

        cpu.set_d(r);
    }

    /// Shift `E` to the right. MSB is not affected.
    fn sra_e(cpu: &mut Cpu) {
        let e = cpu.e();

        let r = sra(cpu, e);

        cpu.set_e(r);
    }

    /// Shift `H` to the right. MSB is not affected.
    fn sra_h(cpu: &mut Cpu) {
        let h = cpu.h();

        let r = sra(cpu, h);

        cpu.set_h(r);
    }

    /// Shift `L` to the right. MSB is not affected.
    fn sra_l(cpu: &mut Cpu) {
        let l = cpu.l();

        let r = sra(cpu, l);

        cpu.set_l(r);
    }

    /// Shift `[HL]` to the right. MSB is not affected.
    fn sra_mhl(cpu: &mut Cpu) {
        let hl = cpu.hl();
        let n  = cpu.fetch_byte(hl);

        let r = sra(cpu, n);

        cpu.store_byte(hl, r);
    }

    /// Helper function to rotate an `u8` to the left and update CPU
    /// flags.
    fn rlc(cpu: &mut Cpu, v: u8)  -> u8 {
        cpu.set_carry(v & 0x80 != 0);

        let r = (v << 1) | (v >> 7);

        cpu.set_zero(r == 0);

        cpu.set_substract(false);
        cpu.set_halfcarry(false);

        r
    }

    /// Rotate `A` to the left. It's slower than RLCA and doesn't set
    /// the same flags.
    fn rlc_a(cpu: &mut Cpu) {
        let a = cpu.a();

        let r = rlc(cpu, a);

        cpu.set_a(r);
    }

    /// Rotate `B` to the left
    fn rlc_b(cpu: &mut Cpu) {
        let b = cpu.b();

        let r = rlc(cpu, b);

        cpu.set_b(r);
    }

    /// Rotate `C` to the left
    fn rlc_c(cpu: &mut Cpu) {
        let c = cpu.c();

        let r = rlc(cpu, c);

        cpu.set_c(r);
    }

    /// Rotate `D` to the left
    fn rlc_d(cpu: &mut Cpu) {
        let d = cpu.d();

        let r = rlc(cpu, d);

        cpu.set_d(r);
    }

    /// Rotate `E` to the left
    fn rlc_e(cpu: &mut Cpu) {
        let e = cpu.e();

        let r = rlc(cpu, e);

        cpu.set_e(r);
    }

    /// Rotate `H` to the left
    fn rlc_h(cpu: &mut Cpu) {
        let h = cpu.h();

        let r = rlc(cpu, h);

        cpu.set_h(r);
    }

    /// Rotate `L` to the left
    fn rlc_l(cpu: &mut Cpu) {
        let l = cpu.l();

        let r = rlc(cpu, l);

        cpu.set_l(r);
    }

    /// Rotate `[HL]` to the left
    fn rlc_mhl(cpu: &mut Cpu) {
        let hl = cpu.hl();
        let n  = cpu.fetch_byte(hl);

        let r = rlc(cpu, n);

        cpu.store_byte(hl, r);
    }

    /// Helper function to rotate an `u8` to the right and update CPU
    /// flags.
    fn rrc(cpu: &mut Cpu, v: u8)  -> u8 {
        cpu.set_carry(v & 1 != 0);

        let r = (v >> 1) | (v << 7);

        cpu.set_zero(r == 0);

        cpu.set_substract(false);
        cpu.set_halfcarry(false);

        r
    }

    /// Rotate `A` to the right. It's slower than RRCA and doesn't set
    /// the same flags.
    fn rrc_a(cpu: &mut Cpu) {
        let a = cpu.a();

        let r = rrc(cpu, a);

        cpu.set_a(r);
    }

    /// Rotate `B` to the right
    fn rrc_b(cpu: &mut Cpu) {
        let b = cpu.b();

        let r = rrc(cpu, b);

        cpu.set_b(r);
    }

    /// Rotate `C` to the right
    fn rrc_c(cpu: &mut Cpu) {
        let c = cpu.c();

        let r = rrc(cpu, c);

        cpu.set_c(r);
    }

    /// Rotate `D` to the right
    fn rrc_d(cpu: &mut Cpu) {
        let d = cpu.d();

        let r = rrc(cpu, d);

        cpu.set_d(r);
    }

    /// Rotate `E` to the right
    fn rrc_e(cpu: &mut Cpu) {
        let e = cpu.e();

        let r = rrc(cpu, e);

        cpu.set_e(r);
    }

    /// Rotate `H` to the right
    fn rrc_h(cpu: &mut Cpu) {
        let h = cpu.h();

        let r = rrc(cpu, h);

        cpu.set_h(r);
    }

    /// Rotate `L` to the right
    fn rrc_l(cpu: &mut Cpu) {
        let l = cpu.l();

        let r = rrc(cpu, l);

        cpu.set_l(r);
    }

    /// Rotate `[HL]` to the right
    fn rrc_mhl(cpu: &mut Cpu) {
        let hl = cpu.hl();
        let n  = cpu.fetch_byte(hl);

        let r = rrc(cpu, n);

        cpu.store_byte(hl, r);
    }

    /// Helper function to rotate an `u8` to the left through carry and update CPU
    /// flags.
    fn rl(cpu: &mut Cpu, v: u8)  -> u8 {
        let oldcarry = cpu.carry() as u8;

        cpu.set_carry(v & 0x80 != 0);

        let r = (v << 1) | oldcarry;

        cpu.set_zero(r == 0);

        cpu.set_substract(false);
        cpu.set_halfcarry(false);

        r
    }

    /// Rotate `A` to the left through carry. It's slower than RLA and
    /// doesn't set the same flags.
    fn rl_a(cpu: &mut Cpu) {
        let a = cpu.a();

        let r = rl(cpu, a);

        cpu.set_a(r);
    }

    /// Rotate `B` to the left through carry
    fn rl_b(cpu: &mut Cpu) {
        let b = cpu.b();

        let r = rl(cpu, b);

        cpu.set_b(r);
    }

    /// Rotate `C` to the left through carry
    fn rl_c(cpu: &mut Cpu) {
        let c = cpu.c();

        let r = rl(cpu, c);

        cpu.set_c(r);
    }

    /// Rotate `D` to the left through carry
    fn rl_d(cpu: &mut Cpu) {
        let d = cpu.d();

        let r = rl(cpu, d);

        cpu.set_d(r);
    }

    /// Rotate `E` to the left through carry
    fn rl_e(cpu: &mut Cpu) {
        let e = cpu.e();

        let r = rl(cpu, e);

        cpu.set_e(r);
    }

    /// Rotate `H` to the left through carry
    fn rl_h(cpu: &mut Cpu) {
        let h = cpu.h();

        let r = rl(cpu, h);

        cpu.set_h(r);
    }

    /// Rotate `L` to the left through carry
    fn rl_l(cpu: &mut Cpu) {
        let l = cpu.l();

        let r = rl(cpu, l);

        cpu.set_l(r);
    }

    /// Rotate `[HL]` to the left through carry
    fn rl_mhl(cpu: &mut Cpu) {
        let hl = cpu.hl();
        let n  = cpu.fetch_byte(hl);

        let r = rl(cpu, n);

        cpu.store_byte(hl, r);
    }


    /// Helper function to rotate an `u8` to the right through carry and update CPU
    /// flags.
    fn rr(cpu: &mut Cpu, v: u8)  -> u8 {
        let oldcarry = cpu.carry() as u8;

        cpu.set_carry(v & 0x1 != 0);

        let r = (v >> 1) | (oldcarry << 7);

        cpu.set_zero(r == 0);

        cpu.set_substract(false);
        cpu.set_halfcarry(false);

        r
    }

    /// Rotate `A` to the right through carry. It's slower than RRA and
    /// doesn't set the same flags.
    fn rr_a(cpu: &mut Cpu) {
        let a = cpu.a();

        let r = rr(cpu, a);

        cpu.set_a(r);
    }

    /// Rotate `B` to the right through carry
    fn rr_b(cpu: &mut Cpu) {
        let b = cpu.b();

        let r = rr(cpu, b);

        cpu.set_b(r);
    }

    /// Rotate `C` to the right through carry
    fn rr_c(cpu: &mut Cpu) {
        let c = cpu.c();

        let r = rr(cpu, c);

        cpu.set_c(r);
    }

    /// Rotate `D` to the right through carry
    fn rr_d(cpu: &mut Cpu) {
        let d = cpu.d();

        let r = rr(cpu, d);

        cpu.set_d(r);
    }

    /// Rotate `E` to the right through carry
    fn rr_e(cpu: &mut Cpu) {
        let e = cpu.e();

        let r = rr(cpu, e);

        cpu.set_e(r);
    }

    /// Rotate `H` to the right through carry
    fn rr_h(cpu: &mut Cpu) {
        let h = cpu.h();

        let r = rr(cpu, h);

        cpu.set_h(r);
    }

    /// Rotate `L` to the right through carry
    fn rr_l(cpu: &mut Cpu) {
        let l = cpu.l();

        let r = rr(cpu, l);

        cpu.set_l(r);
    }

    /// Rotate `[HL]` to the right through carry
    fn rr_mhl(cpu: &mut Cpu) {
        let hl = cpu.hl();
        let n  = cpu.fetch_byte(hl);

        let r = rr(cpu, n);

        cpu.store_byte(hl, r);
    }
}
