//! Game Boy CPU instructions

use cpu::Cpu;

/// Retrieve the next instruction to be executed.
///
/// Returns a tuple `(delay, instruction)` as described in `OPCODES`
pub fn next_instruction(cpu: &mut Cpu) -> fn (&mut Cpu) {
    let pc = cpu.pc();

    let op = cpu.fetch_byte(pc);

    cpu.set_pc(pc + 1);

    let (instruction, _) =
        if op != 0xcb {
            OPCODES[op as usize]
        } else {
            // 0xCB introduces a two-byte bitops opcode
            bitops::next_instruction(cpu)
        };

    instruction
}

/// Array containing tuples `(delay, instruction, desc)`.
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
///
/// `desc` is a human readable assembler-like description of the
/// opcode function
pub static OPCODES: [(fn (&mut Cpu), &'static str); 0x100] = [
    // Opcodes 0X
    (nop,        "NOP"),
    (ld_bc_nn,   "LD BC, NN"),
    (ld_mbc_a,   "LD [BC], A"),
    (inc_bc,     "INC BC"),
    (inc_b,      "INC B"),
    (dec_b,      "DEC B"),
    (ld_b_n,     "LD B, N"),
    (rlca,       "RLCA"),
    (ld_mnn_sp,  "LD [NN], SP"),
    (add_hl_bc,  "ADD HL, BC"),
    (ld_a_mbc,   "LD A, [BC]"),
    (dec_bc,     "DEC BC"),
    (inc_c,      "INC C"),
    (dec_c,      "DEC C"),
    (ld_c_n,     "LD C, N"),
    (rrca,       "RRCA"),
    // Opcodes 1X
    (stop,       "STOP"),
    (ld_de_nn,   "LD DE, NN"),
    (ld_mde_a,   "LD [DE], A"),
    (inc_de,     "INC DE"),
    (inc_d,      "INC D"),
    (dec_d,      "DEC D"),
    (ld_d_n,     "LD D, N"),
    (rla,        "RLA"),
    (jr_sn,      "JR SN"),
    (add_hl_de,  "ADD HL, DE"),
    (ld_a_mde,   "LD A, [DE]"),
    (dec_de,     "DEC DE"),
    (inc_e,      "INC E"),
    (dec_e,      "DEC E"),
    (ld_e_n,     "LD E, N"),
    (rra,        "RRA"),
    // Opcodes 2X
    (jr_nz_sn,   "JR NZ, SN"),
    (ld_hl_nn,   "LD HL, NN"),
    (ldi_mhl_a,  "LDI [HL], A"),
    (inc_hl,     "INC HL"),
    (inc_h,      "INC H"),
    (dec_h,      "DEC H"),
    (ld_h_n,     "LD H, N"),
    (daa,        "DAA"),
    (jr_z_sn,    "JR Z, SN"),
    (add_hl_hl,  "ADD HL, HL"),
    (ldi_a_mhl,  "LDI A, [HL]"),
    (dec_hl,     "DEC HL"),
    (inc_l,      "INC L"),
    (dec_l,      "DEC L"),
    (ld_l_n,     "LD L, N"),
    (cpl,        "CPL"),
    // Opcodes 3X
    (jr_nc_sn,   "JR NC, SN"),
    (ld_sp_nn,   "LD SP, NN"),
    (ldd_mhl_a,  "LDD [HL], A"),
    (inc_sp,     "INC SP"),
    (inc_mhl,    "INC [HL]"),
    (dec_mhl,    "DEC [HL]"),
    (ld_mhl_n,   "LD [HL], N"),
    (scf,        "SCF"),
    (jr_c_sn,    "JR C, SN"),
    (add_hl_sp,  "ADD HL, SP"),
    (ldd_a_mhl,  "LDD A, [HL]"),
    (dec_sp,     "DEC SP"),
    (inc_a,      "INC A"),
    (dec_a,      "DEC A"),
    (ld_a_n,     "LD A, N"),
    (ccf,        "CCF"),
    // Opcodes 4X
    (ld_b_b,     "LD B, B"),
    (ld_b_c,     "LD B, C"),
    (ld_b_d,     "LD B, D"),
    (ld_b_e,     "LD B, E"),
    (ld_b_h,     "LD B, H"),
    (ld_b_l,     "LD B, L"),
    (ld_b_mhl,   "LD B, [HL]"),
    (ld_b_a,     "LD B, A"),
    (ld_c_b,     "LD C, B"),
    (ld_c_c,     "LD C, C"),
    (ld_c_d,     "LD C, D"),
    (ld_c_e,     "LD C, E"),
    (ld_c_h,     "LD C, H"),
    (ld_c_l,     "LD C, L"),
    (ld_c_mhl,   "LD C, [HL]"),
    (ld_c_a,     "LD C, A"),
    // Opcodes 5X
    (ld_d_b,     "LD D, B"),
    (ld_d_c,     "LD D, C"),
    (ld_d_d,     "LD D, D"),
    (ld_d_e,     "LD D, E"),
    (ld_d_h,     "LD D, H"),
    (ld_d_l,     "LD D, L"),
    (ld_d_mhl,   "LD D, [HL]"),
    (ld_d_a,     "LD D, A"),
    (ld_e_b,     "LD E, B"),
    (ld_e_c,     "LD E, C"),
    (ld_e_d,     "LD E, D"),
    (ld_e_e,     "LD E, E"),
    (ld_e_h,     "LD E, H"),
    (ld_e_l,     "LD E, L"),
    (ld_e_mhl,   "LD E, [HL]"),
    (ld_e_a,     "LD E, A"),
    // Opcodes 6X
    (ld_h_b,     "LD H, B"),
    (ld_h_c,     "LD H, C"),
    (ld_h_d,     "LD H, D"),
    (ld_h_e,     "LD H, E"),
    (ld_h_h,     "LD H, H"),
    (ld_h_l,     "LD H, L"),
    (ld_h_mhl,   "LD H, [HL]"),
    (ld_h_a,     "LD H, A"),
    (ld_l_b,     "LD L, B"),
    (ld_l_c,     "LD L, C"),
    (ld_l_d,     "LD L, D"),
    (ld_l_e,     "LD L, E"),
    (ld_l_h,     "LD L, H"),
    (ld_l_l,     "LD L, L"),
    (ld_l_mhl,   "LD L, [HL]"),
    (ld_l_a,     "LD L, A"),
    // Opcodes 7X
    (ld_mhl_b,   "LD [HL], B"),
    (ld_mhl_c,   "LD [HL], C"),
    (ld_mhl_d,   "LD [HL], D"),
    (ld_mhl_e,   "LD [HL], E"),
    (ld_mhl_h,   "LD [HL], H"),
    (ld_mhl_l,   "LD [HL], L"),
    (halt,       "HALT"),
    (ld_mhl_a,   "LD [HL], A"),
    (ld_a_b,     "LD A, B"),
    (ld_a_c,     "LD A, C"),
    (ld_a_d,     "LD A, D"),
    (ld_a_e,     "LD A, E"),
    (ld_a_h,     "LD A, H"),
    (ld_a_l,     "LD A, L"),
    (ld_a_mhl,   "LD A, [HL]"),
    (ld_a_a,     "LD A, A"),
    // Opcodes 8X
    (add_a_b,    "ADD A, B"),
    (add_a_c,    "ADD A, C"),
    (add_a_d,    "ADD A, D"),
    (add_a_e,    "ADD A, E"),
    (add_a_h,    "ADD A, H"),
    (add_a_l,    "ADD A, L"),
    (add_a_mhl,  "ADD A, [HL]"),
    (add_a_a,    "ADD A, A"),
    (adc_a_b,    "ADC A, B"),
    (adc_a_c,    "ADC A, C"),
    (adc_a_d,    "ADC A, D"),
    (adc_a_e,    "ADC A, E"),
    (adc_a_h,    "ADC A, H"),
    (adc_a_l,    "ADC A, L"),
    (adc_a_mhl,  "ADC A, [HL]"),
    (adc_a_a,    "ADC A, A"),
    // Opcodes 9X
    (sub_a_b,    "SUB A, B"),
    (sub_a_c,    "SUB A, C"),
    (sub_a_d,    "SUB A, D"),
    (sub_a_e,    "SUB A, E"),
    (sub_a_h,    "SUB A, H"),
    (sub_a_l,    "SUB A, L"),
    (sub_a_mhl,  "SUB A, [HL]"),
    (sub_a_a,    "SUB A, A"),
    (sbc_a_b,    "SBC A, B"),
    (sbc_a_c,    "SBC A, C"),
    (sbc_a_d,    "SBC A, D"),
    (sbc_a_e,    "SBC A, E"),
    (sbc_a_h,    "SBC A, H"),
    (sbc_a_l,    "SBC A, L"),
    (sbc_a_mhl,  "SBC A, [HL]"),
    (sbc_a_a,    "SBC A, A"),
    // Opcodes AX
    (and_a_b,    "AND A, B"),
    (and_a_c,    "AND A, C"),
    (and_a_d,    "AND A, D"),
    (and_a_e,    "AND A, E"),
    (and_a_h,    "AND A, H"),
    (and_a_l,    "AND A, L"),
    (and_a_mhl,  "AND A, [HL]"),
    (and_a_a,    "AND A, A"),
    (xor_a_b,    "XOR A, B"),
    (xor_a_c,    "XOR A, C"),
    (xor_a_d,    "XOR A, D"),
    (xor_a_e,    "XOR A, E"),
    (xor_a_h,    "XOR A, H"),
    (xor_a_l,    "XOR A, L"),
    (xor_a_mhl,  "XOR A, [HL]"),
    (xor_a_a,    "XOR A, A"),
    // Opcodes BX
    (or_a_b,     "OR A, B"),
    (or_a_c,     "OR A, C"),
    (or_a_d,     "OR A, D"),
    (or_a_e,     "OR A, E"),
    (or_a_h,     "OR A, H"),
    (or_a_l,     "OR A, L"),
    (or_a_mhl,   "OR A, [HL]"),
    (or_a_a,     "OR A, A"),
    (cp_a_b,     "CP A, B"),
    (cp_a_c,     "CP A, C"),
    (cp_a_d,     "CP A, D"),
    (cp_a_e,     "CP A, E"),
    (cp_a_h,     "CP A, H"),
    (cp_a_l,     "CP A, L"),
    (cp_a_mhl,   "CP A, [HL]"),
    (cp_a_a,     "CP A, A"),
    // Opcodes CX
    (ret_nz,     "RET NZ"),
    (pop_bc,     "POP BC"),
    (jp_nz_nn,   "JP NZ, NN"),
    (jp_nn,      "JP NN"),
    (call_nz_nn, "CALL NZ, NN"),
    (push_bc,    "PUSH BC"),
    (add_a_n,    "ADD A, N"),
    (rst_00,     "RST 00"),
    (ret_z,      "RET Z"),
    (ret,        "RET"),
    (jp_z_nn,    "JP Z, NN"),
    (undefined,  "UNDEFINED"), // See bitops opcode map
    (call_z_nn,  "CALL Z, NN"),
    (call_nn,    "CALL NN"),
    (adc_a_n,    "ADC A, N"),
    (rst_08,     "RST 08"),
    // Opcodes DX
    (ret_nc,     "RET NC"),
    (pop_de,     "POP DE"),
    (jp_nc_nn,   "JP NC, NN"),
    (undefined,  "UNDEFINED"),
    (call_nc_nn, "CALL NC, NN"),
    (push_de,    "PUSH DE"),
    (sub_a_n,    "SUB A, N"),
    (rst_10,     "RST 10"),
    (ret_c,      "RET C"),
    (reti,       "RETI"),
    (jp_c_nn,    "JP C, NN"),
    (undefined,  "UNDEFINED"),
    (call_c_nn,  "CALL C, NN"),
    (undefined,  "UNDEFINED"),
    (sbc_a_n,    "SBC A, N"),
    (rst_18,     "RST 18"),
    // Opcodes EX
    (ldh_mn_a,   "LDH [N], A"),
    (pop_hl,     "POP HL"),
    (ldh_mc_a,   "LDH [C], A"),
    (undefined,  "UNDEFINED"),
    (undefined,  "UNDEFINED"),
    (push_hl,    "PUSH HL"),
    (and_a_n,    "AND A, N"),
    (rst_20,     "RST 20"),
    (add_sp_sn,  "ADD SP, SN"),
    (jp_hl,      "JP HL"),
    (ld_mnn_a,   "LD [NN], A"),
    (undefined,  "UNDEFINED"),
    (undefined,  "UNDEFINED"),
    (undefined,  "UNDEFINED"),
    (xor_a_n,    "XOR A, N"),
    (rst_28,     "RST 28"),
    // Opcodes FX
    (ldh_a_mn,   "LDH A, [N]"),
    (pop_af,     "POP AF"),
    (ldh_a_mc,   "LDH A, [C]"),
    (di,         "DI"),
    (undefined,  "UNDEFINED"),
    (push_af,    "PUSH AF"),
    (or_a_n,     "OR A, N"),
    (rst_30,     "RST 30"),
    (ld_hl_sp_sn,"LD HL, SP, SN"),
    (ld_sp_hl,   "LD SP, HL"),
    (ld_a_mnn,   "LD A, [NN]"),
    (ei,         "EI"),
    (undefined,  "UNDEFINED"),
    (undefined,  "UNDEFINED"),
    (cp_a_n,     "CP A, N"),
    (rst_38,     "RST 38"),
];

/// For multi-byte instructions: return the byte at `pc` and increment `pc`
fn next_byte(cpu: &mut Cpu) -> u8 {
    let pc = cpu.pc();

    cpu.set_pc(pc + 1);

    let b = cpu.fetch_byte(pc);

    b
}

/// For multi-byte instructions: return the word at `pc` and increment
/// `pc` twice
fn next_word(cpu: &mut Cpu) -> u16 {
    let b1 = next_byte(cpu) as u16;
    let b2 = next_byte(cpu) as u16;

    b1 | (b2 << 8)
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

    cpu.set_carry(c != 0);
    cpu.set_zero(false);
    cpu.set_halfcarry(false);
    cpu.set_substract(false);
}

/// Rotate `A` left through carry
fn rla(cpu: &mut Cpu) {
    let a = cpu.a();

    let newcarry = (a >> 7) != 0;
    let oldcarry = cpu.carry() as u8;

    cpu.set_a((a << 1) | oldcarry);

    cpu.set_carry(newcarry);
    cpu.set_zero(false);
    cpu.set_halfcarry(false);
    cpu.set_substract(false);
}

/// Rotate `A` right
fn rrca(cpu: &mut Cpu) {
    let a = cpu.a();

    let c = a & 1;

    cpu.set_a((a >> 1) | (c << 7));

    cpu.set_carry(c != 0);
    cpu.set_zero(false);
    cpu.set_halfcarry(false);
    cpu.set_substract(false);
}

/// Rotate `A` right through carry
fn rra(cpu: &mut Cpu) {
    let a = cpu.a();

    let newcarry = (a & 1) != 0;
    let oldcarry = cpu.carry() as u8;

    cpu.set_a((a >> 1) | (oldcarry << 7));

    cpu.set_carry(newcarry);
    cpu.set_zero(false);
    cpu.set_halfcarry(false);
    cpu.set_substract(false);
}

/// Complement `A`
fn cpl(cpu: &mut Cpu) {
    let a = cpu.a();

    cpu.set_a(!a);

    cpu.set_substract(true);
    cpu.set_halfcarry(true);
}

/// Decimal adjust `A` for BCD operations.
fn daa(cpu: &mut Cpu) {
    let a          = cpu.a();
    let mut adjust = 0;

    // See if we had a carry/borrow for the low nibble in the last
    // operation
    if cpu.halfcarry() {
        // Yes, we have to adjust it.
        adjust |= 0x06;
    }

    // See if we had a carry/borrow for the high nibble in the last
    // operation
    if cpu.carry() {
        // Yes, we have to adjust it.
        adjust |= 0x60;
    }

    let res =
        if cpu.substract() {
            // If the operation was a substraction we're done since we
            // can never end up in the A-F range by substracting
            // without generating a (half)carry.
            a - adjust
        } else {
            // Additions are a bit more tricky because we might have
            // to adjust even if we haven't overflowed (and no carry
            // is present). For instance: 0x8 + 0x4 -> 0xc.
	    if a & 0x0F > 0x09 {
                adjust |= 0x06;
            }

	    if a > 0x99 {
                adjust |= 0x60;
            }

	    a + adjust
        };

    cpu.set_a(res);

    cpu.set_zero(res == 0);
    cpu.set_carry(adjust & 0x60 != 0);
    cpu.set_halfcarry(false);
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
    cpu.store_byte(n + 1, (sp >> 8) as u8);
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
    cpu.set_carry((sp ^ nn ^ r) & 0x100 != 0);
    cpu.set_halfcarry((sp ^ nn ^ r) & 0x10 != 0);
    cpu.set_zero(false);

    cpu.set_hl(r as u16);

    cpu.delay(1);
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

    cpu.delay(1);
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
    let n = cpu.pop_word();

    cpu.set_af(n);
}

/// Pop `BC` from the stack
fn pop_bc(cpu: &mut Cpu) {
    let n = cpu.pop_word();

    cpu.set_bc(n);
}

/// Pop `DE` from the stack
fn pop_de(cpu: &mut Cpu) {
    let n = cpu.pop_word();

    cpu.set_de(n);
}

/// Pop `HL` from the stack
fn pop_hl(cpu: &mut Cpu) {
    let n = cpu.pop_word();

    cpu.set_hl(n);
}

/// Push `AF` on the stack
fn push_af(cpu: &mut Cpu) {
    let af = cpu.af();

    cpu.push_word(af);

    cpu.delay(1);
}

/// Push `BC` on the stack
fn push_bc(cpu: &mut Cpu) {
    let bc = cpu.bc();

    cpu.push_word(bc);

    cpu.delay(1);
}

/// Push `DE` on the stack
fn push_de(cpu: &mut Cpu) {
    let de = cpu.de();

    cpu.push_word(de);

    cpu.delay(1);
}

/// Push `HL` on the stack
fn push_hl(cpu: &mut Cpu) {
    let hl = cpu.hl();

    cpu.push_word(hl);

    cpu.delay(1);
}

/// Unconditional jump to absolute address
fn jp_nn(cpu: &mut Cpu) {
    let addr = next_word(cpu);

    cpu.load_pc(addr);
}

/// Unconditional jump to address in `HL`
fn jp_hl(cpu: &mut Cpu) {
    let hl = cpu.hl();

    // Moving from HL to PC does not require additional cycles
    // apparently.
    cpu.set_pc(hl);
}

/// Jump to absolute address if `!Z`
fn jp_nz_nn(cpu: &mut Cpu) {
    let addr = next_word(cpu);

    if !cpu.zero() {
        cpu.load_pc(addr);
    }
}

/// Jump to absolute address if `Z`
fn jp_z_nn(cpu: &mut Cpu) {
    let addr = next_word(cpu);

    if cpu.zero() {
        cpu.load_pc(addr);
    }
}

/// Jump to absolute address if `!C`
fn jp_nc_nn(cpu: &mut Cpu) {
    let addr = next_word(cpu);

    if !cpu.carry() {
        cpu.load_pc(addr);
    }
}

/// Jump to absolute address if `C`
fn jp_c_nn(cpu: &mut Cpu) {
    let addr = next_word(cpu);

    if cpu.carry() {
        cpu.load_pc(addr);
    }
}

/// Unconditional jump to relative address
fn jr_sn(cpu: &mut Cpu) {
    let off = next_byte(cpu) as i8;

    let mut pc = cpu.pc() as i16;

    pc += off as i16;

    cpu.load_pc(pc as u16);
}

/// Jump to relative address if `!Z`
fn jr_nz_sn(cpu: &mut Cpu) {
    let off = next_byte(cpu) as i8;

    if !cpu.zero() {
        let mut pc = cpu.pc() as i16;

        pc += off as i16;

        cpu.load_pc(pc as u16);
    }
}

/// Jump to relative address if `Z`
fn jr_z_sn(cpu: &mut Cpu) {
    let off = next_byte(cpu) as i8;

    if cpu.zero() {
        let mut pc = cpu.pc() as i16;

        pc += off as i16;

        cpu.load_pc(pc as u16);
    }
}

/// Jump to relative address if `!C`
fn jr_nc_sn(cpu: &mut Cpu) {
    let off = next_byte(cpu) as i8;

    if !cpu.carry() {
        let mut pc = cpu.pc() as i16;

        pc += off as i16;

        cpu.load_pc(pc as u16);
    }
}

/// Jump to relative address if `C`
fn jr_c_sn(cpu: &mut Cpu) {
    let off = next_byte(cpu) as i8;

    if cpu.carry() {
        let mut pc = cpu.pc() as i16;

        pc += off as i16;

        cpu.load_pc(pc as u16);
    }
}

/// Helper function for RST instructions
fn rst(cpu: &mut Cpu, addr: u16) {
    let pc = cpu.pc();

    cpu.push_word(pc);

    cpu.load_pc(addr);
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

    cpu.push_word(pc);

    cpu.load_pc(addr);
}

/// If !Z Push return address on stack and jump to immediate address
fn call_nz_nn(cpu: &mut Cpu) {
    let addr = next_word(cpu);

    if !cpu.zero() {
        let pc = cpu.pc();

        cpu.push_word(pc);

        cpu.load_pc(addr);
    }
}

/// If Z Push return address on stack and jump to immediate address
fn call_z_nn(cpu: &mut Cpu) {
    let addr = next_word(cpu);

    if cpu.zero() {
        let pc = cpu.pc();

        cpu.push_word(pc);

        cpu.load_pc(addr);
    }
}

/// If !C Push return address on stack and jump to immediate address
fn call_nc_nn(cpu: &mut Cpu) {
    let addr = next_word(cpu);

    if !cpu.carry() {
        let pc = cpu.pc();

        cpu.push_word(pc);

        cpu.load_pc(addr);
    }
}

/// If C Push return address on stack and jump to immediate address
fn call_c_nn(cpu: &mut Cpu) {
    let addr = next_word(cpu);

    if cpu.carry() {
        let pc = cpu.pc();

        cpu.push_word(pc);

        cpu.load_pc(addr);
    }
}

/// Pop return address from stack and jump to it
fn ret(cpu: &mut Cpu) {
    let addr = cpu.pop_word();

    cpu.load_pc(addr);
}

/// Pop return address from stack and jump to it then enable
/// interrupts.
fn reti(cpu: &mut Cpu) {
    let addr = cpu.pop_word();

    cpu.load_pc(addr);

    cpu.enable_interrupts();
}

/// If !Z pop return address from stack and jump to it
fn ret_nz(cpu: &mut Cpu) {
    if !cpu.zero() {
        let addr = cpu.pop_word();

        cpu.load_pc(addr);
    }

    cpu.delay(1);
}

/// If Z pop return address from stack and jump to it
fn ret_z(cpu: &mut Cpu) {
    if cpu.zero() {
        let addr = cpu.pop_word();

        cpu.load_pc(addr);
    }

    cpu.delay(1);
}

/// If !C pop return address from stack and jump to it
fn ret_nc(cpu: &mut Cpu) {
    if !cpu.carry() {
        let addr = cpu.pop_word();

        cpu.load_pc(addr);
    }

    cpu.delay(1);
}

/// If C pop return address from stack and jump to it
fn ret_c(cpu: &mut Cpu) {
    if cpu.carry() {
        let addr = cpu.pop_word();

        cpu.load_pc(addr);
    }

    cpu.delay(1);
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

    cpu.delay(1);
}

/// Increment `DE`
fn inc_de(cpu: &mut Cpu) {
    let de = cpu.de();

    cpu.set_de(de + 1);

    cpu.delay(1);
}

/// Increment `HL`
fn inc_hl(cpu: &mut Cpu) {
    let hl = cpu.hl();

    cpu.set_hl(hl + 1);

    cpu.delay(1);
}

/// Increment `SP`
fn inc_sp(cpu: &mut Cpu) {
    let sp = cpu.sp();

    cpu.set_sp(sp + 1);

    cpu.delay(1);
}

/// Decrement `BC`
fn dec_bc(cpu: &mut Cpu) {
    let bc = cpu.bc();

    cpu.set_bc(bc - 1);

    cpu.delay(1);
}

/// Decrement `DE`
fn dec_de(cpu: &mut Cpu) {
    let de = cpu.de();

    cpu.set_de(de - 1);

    cpu.delay(1);
}

/// Decrement `HL`
fn dec_hl(cpu: &mut Cpu) {
    let hl = cpu.hl();

    cpu.set_hl(hl - 1);

    cpu.delay(1);
}

/// Decrement `SP`
fn dec_sp(cpu: &mut Cpu) {
    let sp = cpu.sp();

    cpu.set_sp(sp - 1);

    cpu.delay(1);
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

    cpu.delay(1);

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

    // Carry and halfcarry are for the low byte
    cpu.set_carry((sp ^ nn ^ r) & 0x100 != 0);
    cpu.set_halfcarry((sp ^ nn ^ r) & 0x10 != 0);
    cpu.set_sp(r as u16);

    cpu.set_substract(false);
    cpu.set_zero(false);

    cpu.delay(2);
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

/// xor two u8 together, update CPU flags and return the result
fn xor(cpu: &mut Cpu, n: u8, m: u8) -> u8 {
    let r = n ^ m;

    cpu.set_carry(false);
    cpu.set_halfcarry(false);
    cpu.set_substract(false);
    cpu.set_zero(r == 0);

    r
}

/// XOR `A` with itself (set `A` to `0`)
fn xor_a_a(cpu: &mut Cpu) {
    cpu.set_a(0);

    cpu.set_carry(false);
    cpu.set_halfcarry(false);
    cpu.set_substract(false);
    cpu.set_zero(true);
}

/// XOR `B` into `A`
fn xor_a_b(cpu: &mut Cpu) {
    let a = cpu.a();
    let b = cpu.b();

    let r = xor(cpu, a, b);

    cpu.set_a(r);
}

/// XOR `C` into `A`
fn xor_a_c(cpu: &mut Cpu) {
    let a = cpu.a();
    let c = cpu.c();

    let r = xor(cpu, a, c);

    cpu.set_a(r);
}

/// XOR `D` into `A`
fn xor_a_d(cpu: &mut Cpu) {
    let a = cpu.a();
    let d = cpu.d();

    let r = xor(cpu, a, d);

    cpu.set_a(r);
}

/// XOR `E` into `A`
fn xor_a_e(cpu: &mut Cpu) {
    let a = cpu.a();
    let e = cpu.e();

    let r = xor(cpu, a, e);

    cpu.set_a(r);
}

/// XOR `H` into `A`
fn xor_a_h(cpu: &mut Cpu) {
    let a = cpu.a();
    let h = cpu.h();

    let r = xor(cpu, a, h);

    cpu.set_a(r);
}

/// XOR `L` into `A`
fn xor_a_l(cpu: &mut Cpu) {
    let a = cpu.a();
    let l = cpu.l();

    let r = xor(cpu, a, l);

    cpu.set_a(r);
}

/// XOR `[HL]` into `A`
fn xor_a_mhl(cpu: &mut Cpu) {
    let a  = cpu.a();

    let hl = cpu.hl();
    let n  = cpu.fetch_byte(hl);

    let r = xor(cpu, a, n);

    cpu.set_a(r);
}

/// XOR `N` with `A`
fn xor_a_n(cpu: &mut Cpu) {
    let a = cpu.a();
    let n = next_byte(cpu);

    let r = xor(cpu, a, n);

    cpu.set_a(r);
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
    // Enable interrupts after the next instruction
    cpu.enable_interrupts_next();
}

/// Halt and wait for interrupt. Interrupt will wake us up even if
/// they are disabled.
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
    pub fn next_instruction(cpu: &mut Cpu) -> (fn (&mut Cpu),
                                               &'static str) {
        let pc = cpu.pc();

        cpu.set_pc(pc + 1);

        let op = cpu.fetch_byte(pc);

        OPCODES[op as usize]
    }

    /// Array similar to the one above, this time for CB-prefixed
    /// instructions
    pub static OPCODES: [(fn (&mut Cpu), &'static str); 0x100] = [
        // Opcodes CB 0X
        (rlc_b,      "RLC B"),
        (rlc_c,      "RLC C"),
        (rlc_d,      "RLC D"),
        (rlc_e,      "RLC E"),
        (rlc_h,      "RLC H"),
        (rlc_l,      "RLC L"),
        (rlc_mhl,    "RLC [HL]"),
        (rlc_a,      "RLC A"),
        (rrc_b,      "RRC B"),
        (rrc_c,      "RRC C"),
        (rrc_d,      "RRC D"),
        (rrc_e,      "RRC E"),
        (rrc_h,      "RRC H"),
        (rrc_l,      "RRC L"),
        (rrc_mhl,    "RRC [HL]"),
        (rrc_a,      "RRC A"),
        // Opcodes CB 1X
        (rl_b,       "RL B"),
        (rl_c,       "RL C"),
        (rl_d,       "RL D"),
        (rl_e,       "RL E"),
        (rl_h,       "RL H"),
        (rl_l,       "RL L"),
        (rl_mhl,     "RL [HL]"),
        (rl_a,       "RL A"),
        (rr_b,       "RR B"),
        (rr_c,       "RR C"),
        (rr_d,       "RR D"),
        (rr_e,       "RR E"),
        (rr_h,       "RR H"),
        (rr_l,       "RR L"),
        (rr_mhl,     "RR [HL]"),
        (rr_a,       "RR A"),
        // Opcodes CB 2X
        (sla_b,      "SLA B"),
        (sla_c,      "SLA C"),
        (sla_d,      "SLA D"),
        (sla_e,      "SLA E"),
        (sla_h,      "SLA H"),
        (sla_l,      "SLA L"),
        (sla_mhl,    "SLA [HL]"),
        (sla_a,      "SLA A"),
        (sra_b,      "SRA B"),
        (sra_c,      "SRA C"),
        (sra_d,      "SRA D"),
        (sra_e,      "SRA E"),
        (sra_h,      "SRA H"),
        (sra_l,      "SRA L"),
        (sra_mhl,    "SRA [HL]"),
        (sra_a,      "SRA A"),
        // Opcodes CB 3X
        (swap_b,     "SWAP B"),
        (swap_c,     "SWAP C"),
        (swap_d,     "SWAP D"),
        (swap_e,     "SWAP E"),
        (swap_h,     "SWAP H"),
        (swap_l,     "SWAP L"),
        (swap_mhl,   "SWAP [HL]"),
        (swap_a,     "SWAP A"),
        (srl_b,      "SRL B"),
        (srl_c,      "SRL C"),
        (srl_d,      "SRL D"),
        (srl_e,      "SRL E"),
        (srl_h,      "SRL H"),
        (srl_l,      "SRL L"),
        (srl_mhl,    "SRL [HL]"),
        (srl_a,      "SRL A"),
        // Opcodes CB 4X
        (bit_b_0,    "BIT B, 0"),
        (bit_c_0,    "BIT C, 0"),
        (bit_d_0,    "BIT D, 0"),
        (bit_e_0,    "BIT E, 0"),
        (bit_h_0,    "BIT H, 0"),
        (bit_l_0,    "BIT L, 0"),
        (bit_mhl_0,  "BIT [HL], 0"),
        (bit_a_0,    "BIT A, 0"),
        (bit_b_1,    "BIT B, 1"),
        (bit_c_1,    "BIT C, 1"),
        (bit_d_1,    "BIT D, 1"),
        (bit_e_1,    "BIT E, 1"),
        (bit_h_1,    "BIT H, 1"),
        (bit_l_1,    "BIT L, 1"),
        (bit_mhl_1,  "BIT [HL], 1"),
        (bit_a_1,    "BIT A, 1"),
        // Opcodes CB 5X
        (bit_b_2,    "BIT B, 2"),
        (bit_c_2,    "BIT C, 2"),
        (bit_d_2,    "BIT D, 2"),
        (bit_e_2,    "BIT E, 2"),
        (bit_h_2,    "BIT H, 2"),
        (bit_l_2,    "BIT L, 2"),
        (bit_mhl_2,  "BIT [HL], 2"),
        (bit_a_2,    "BIT A, 2"),
        (bit_b_3,    "BIT B, 3"),
        (bit_c_3,    "BIT C, 3"),
        (bit_d_3,    "BIT D, 3"),
        (bit_e_3,    "BIT E, 3"),
        (bit_h_3,    "BIT H, 3"),
        (bit_l_3,    "BIT L, 3"),
        (bit_mhl_3,  "BIT [HL], 3"),
        (bit_a_3,    "BIT A, 3"),
        // Opcodes CB 6X
        (bit_b_4,    "BIT B, 4"),
        (bit_c_4,    "BIT C, 4"),
        (bit_d_4,    "BIT D, 4"),
        (bit_e_4,    "BIT E, 4"),
        (bit_h_4,    "BIT H, 4"),
        (bit_l_4,    "BIT L, 4"),
        (bit_mhl_4,  "BIT [HL], 4"),
        (bit_a_4,    "BIT A, 4"),
        (bit_b_5,    "BIT B, 5"),
        (bit_c_5,    "BIT C, 5"),
        (bit_d_5,    "BIT D, 5"),
        (bit_e_5,    "BIT E, 5"),
        (bit_h_5,    "BIT H, 5"),
        (bit_l_5,    "BIT L, 5"),
        (bit_mhl_5,  "BIT [HL], 5"),
        (bit_a_5,    "BIT A, 5"),
        // Opcodes CB 7X
        (bit_b_6,    "BIT B, 6"),
        (bit_c_6,    "BIT C, 6"),
        (bit_d_6,    "BIT D, 6"),
        (bit_e_6,    "BIT E, 6"),
        (bit_h_6,    "BIT H, 6"),
        (bit_l_6,    "BIT L, 6"),
        (bit_mhl_6,  "BIT [HL], 6"),
        (bit_a_6,    "BIT A, 6"),
        (bit_b_7,    "BIT B, 7"),
        (bit_c_7,    "BIT C, 7"),
        (bit_d_7,    "BIT D, 7"),
        (bit_e_7,    "BIT E, 7"),
        (bit_h_7,    "BIT H, 7"),
        (bit_l_7,    "BIT L, 7"),
        (bit_mhl_7,  "BIT [HL], 7"),
        (bit_a_7,    "BIT A, 7"),
        // Opcodes CB 8X
        (res_b_0,    "RES B, 0"),
        (res_c_0,    "RES C, 0"),
        (res_d_0,    "RES D, 0"),
        (res_e_0,    "RES E, 0"),
        (res_h_0,    "RES H, 0"),
        (res_l_0,    "RES L, 0"),
        (res_mhl_0,  "RES [HL], 0"),
        (res_a_0,    "RES A, 0"),
        (res_b_1,    "RES B, 1"),
        (res_c_1,    "RES C, 1"),
        (res_d_1,    "RES D, 1"),
        (res_e_1,    "RES E, 1"),
        (res_h_1,    "RES H, 1"),
        (res_l_1,    "RES L, 1"),
        (res_mhl_1,  "RES [HL], 1"),
        (res_a_1,    "RES A, 1"),
        // Opcodes CB 9X
        (res_b_2,    "RES B, 2"),
        (res_c_2,    "RES C, 2"),
        (res_d_2,    "RES D, 2"),
        (res_e_2,    "RES E, 2"),
        (res_h_2,    "RES H, 2"),
        (res_l_2,    "RES L, 2"),
        (res_mhl_2,  "RES [HL], 2"),
        (res_a_2,    "RES A, 2"),
        (res_b_3,    "RES B, 3"),
        (res_c_3,    "RES C, 3"),
        (res_d_3,    "RES D, 3"),
        (res_e_3,    "RES E, 3"),
        (res_h_3,    "RES H, 3"),
        (res_l_3,    "RES L, 3"),
        (res_mhl_3,  "RES [HL], 3"),
        (res_a_3,    "RES A, 3"),
        // Opcodes CB AX
        (res_b_4,    "RES B, 4"),
        (res_c_4,    "RES C, 4"),
        (res_d_4,    "RES D, 4"),
        (res_e_4,    "RES E, 4"),
        (res_h_4,    "RES H, 4"),
        (res_l_4,    "RES L, 4"),
        (res_mhl_4,  "RES [HL], 4"),
        (res_a_4,    "RES A, 4"),
        (res_b_5,    "RES B, 5"),
        (res_c_5,    "RES C, 5"),
        (res_d_5,    "RES D, 5"),
        (res_e_5,    "RES E, 5"),
        (res_h_5,    "RES H, 5"),
        (res_l_5,    "RES L, 5"),
        (res_mhl_5,  "RES [HL], 5"),
        (res_a_5,    "RES A, 5"),
        // Opcodes CB BX
        (res_b_6,    "RES B, 6"),
        (res_c_6,    "RES C, 6"),
        (res_d_6,    "RES D, 6"),
        (res_e_6,    "RES E, 6"),
        (res_h_6,    "RES H, 6"),
        (res_l_6,    "RES L, 6"),
        (res_mhl_6,  "RES [HL], 6"),
        (res_a_6,    "RES A, 6"),
        (res_b_7,    "RES B, 7"),
        (res_c_7,    "RES C, 7"),
        (res_d_7,    "RES D, 7"),
        (res_e_7,    "RES E, 7"),
        (res_h_7,    "RES H, 7"),
        (res_l_7,    "RES L, 7"),
        (res_mhl_7,  "RES [HL], 7"),
        (res_a_7,    "RES A, 7"),
        // Opcodes CB CX
        (set_b_0,    "SET B, 0"),
        (set_c_0,    "SET C, 0"),
        (set_d_0,    "SET D, 0"),
        (set_e_0,    "SET E, 0"),
        (set_h_0,    "SET H, 0"),
        (set_l_0,    "SET L, 0"),
        (set_mhl_0,  "SET [HL], 0"),
        (set_a_0,    "SET A, 0"),
        (set_b_1,    "SET B, 1"),
        (set_c_1,    "SET C, 1"),
        (set_d_1,    "SET D, 1"),
        (set_e_1,    "SET E, 1"),
        (set_h_1,    "SET H, 1"),
        (set_l_1,    "SET L, 1"),
        (set_mhl_1,  "SET [HL], 1"),
        (set_a_1,    "SET A, 1"),
        // Opcodes CB DX
        (set_b_2,    "SET B, 2"),
        (set_c_2,    "SET C, 2"),
        (set_d_2,    "SET D, 2"),
        (set_e_2,    "SET E, 2"),
        (set_h_2,    "SET H, 2"),
        (set_l_2,    "SET L, 2"),
        (set_mhl_2,  "SET [HL], 2"),
        (set_a_2,    "SET A, 2"),
        (set_b_3,    "SET B, 3"),
        (set_c_3,    "SET C, 3"),
        (set_d_3,    "SET D, 3"),
        (set_e_3,    "SET E, 3"),
        (set_h_3,    "SET H, 3"),
        (set_l_3,    "SET L, 3"),
        (set_mhl_3,  "SET [HL], 3"),
        (set_a_3,    "SET A, 3"),
        // Opcodes CB EX
        (set_b_4,    "SET B, 4"),
        (set_c_4,    "SET C, 4"),
        (set_d_4,    "SET D, 4"),
        (set_e_4,    "SET E, 4"),
        (set_h_4,    "SET H, 4"),
        (set_l_4,    "SET L, 4"),
        (set_mhl_4,  "SET [HL], 4"),
        (set_a_4,    "SET A, 4"),
        (set_b_5,    "SET B, 5"),
        (set_c_5,    "SET C, 5"),
        (set_d_5,    "SET D, 5"),
        (set_e_5,    "SET E, 5"),
        (set_h_5,    "SET H, 5"),
        (set_l_5,    "SET L, 5"),
        (set_mhl_5,  "SET [HL], 5"),
        (set_a_5,    "SET A, 5"),
        // Opcodes CB FX
        (set_b_6,    "SET B, 6"),
        (set_c_6,    "SET C, 6"),
        (set_d_6,    "SET D, 6"),
        (set_e_6,    "SET E, 6"),
        (set_h_6,    "SET H, 6"),
        (set_l_6,    "SET L, 6"),
        (set_mhl_6,  "SET [HL], 6"),
        (set_a_6,    "SET A, 6"),
        (set_b_7,    "SET B, 7"),
        (set_c_7,    "SET C, 7"),
        (set_d_7,    "SET D, 7"),
        (set_e_7,    "SET E, 7"),
        (set_h_7,    "SET H, 7"),
        (set_l_7,    "SET L, 7"),
        (set_mhl_7,  "SET [HL], 7"),
        (set_a_7,    "SET A, 7"),
    ];

    /// Helper function to swap the two nibbles in a `u8` and update
    /// cpu flags.
    fn swap(cpu: &mut Cpu, v: u8) -> u8 {
        cpu.set_zero(v == 0);
        cpu.set_substract(false);
        cpu.set_halfcarry(false);
        cpu.set_carry(false);

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
        (val & (1u8 << (bit as usize))) == 0
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
        val & !(1u8 << (bit as usize))
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
        val | (1u8 << (bit as usize))
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

    /// Helper function to rotate an `u8` to the left through carry
    /// and update CPU flags.
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


    /// Helper function to rotate an `u8` to the right through carry
    /// and update CPU flags.
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
