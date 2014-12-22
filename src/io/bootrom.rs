//! The GameBoy ROM that gets mapped at address 0 at startup. It's not
//! accessible during normal game execution.

/// Original GameBoy bootrom. It scrolls the NINTENDO logo down the
/// screen and emits the signature two notes when it reaches the
/// middle. It also checks that the cartridge ROM header is correct
/// and deadlocks if that check fails.
#[cfg(not(feature="sgb_bootrom"))]
pub static BOOTROM: [u8, ..0x100] = [
    // init_stack:
    0x31, 0xFE, 0xFF, // LD     SP 0xfffe
    0xAF,             // XOR    A A
    0x21, 0xFF, 0x9F, // LD     HL #VRAM_END

    // clear_vram:
    0x32,             // LDD    [HL] A
    0xCB, 0x7C,       // BIT    H 7
    0x20, 0xFB,       // JR NZ  clear_vram

    // init_sound:
    0x21, 0x26, 0xFF, // LD     HL 0xff26
    0x0E, 0x11,       // LD     C 0x11
    0x3E, 0x80,       // LD     A 0x80
    0x32,             // LDD    [HL] A
    0xE2,             // LD     [0xff00 + C] A
    0x0C,             // INC    C
    0x3E, 0xF3,       // LD     A 0xf3
    0xE2,             // LD     [0xff00 + C] A
    0x32,             // LDD    [HL] A
    0x3E, 0x77,       // LD     A 0x77
    0x77,             // LD     [HL] A

    // init_palette:
    0x3E, 0xFC,       // LD     A 0xfc
    0xE0, 0x47,       // LD     [0xff00 + #BGP] A

    // init_crc:
    0x11, 0x04, 0x01, // LD     DE 0x0104
    0x21, 0x10, 0x80, // LD     HL 0x8010
    // do_crc:
    0x1A,             // LD     A [DE]
    0xCD, 0x95, 0x00, // CALL   crc_0
    0xCD, 0x96, 0x00, // CALL   crc_1
    0x13,             // INC    DE
    0x7B,             // LD     A E
    0xFE, 0x34,       // CP     A 0x34
    0x20, 0xF3,       // JR NZ  do_crc

    0x11, 0xD8, 0x00, // LD     DE #tile_data
    0x06, 0x08,       // LD     B 8
    // copy_tile_map:
    0x1A,             // LD     A [DE]
    0x13,             // INC    DE
    0x22,             // LDI    [HL] A
    0x23,             // INC    HL
    0x05,             // DEC    B
    0x20, 0xF9,       // JR NZ  copy_tile_map

    // init_tiles:
    0x3E, 0x19,       // LD     A 0x19
    0xEA, 0x10, 0x99, // LD     [0x9910] A
    0x21, 0x2F, 0x99, // LD     HL 0x992f
    // init_tiles_loop:
    0x0E, 0x0C,       // LD     C 0x0c
    // init_tiles_inner:
    0x3D,             // DEC    A
    0x28, 0x08,       // JR Z   init_scroll
    0x32,             // LDD    [HL] A
    0x0D,             // DEC    C
    0x20, 0xF9,       // JR NZ  init_tiles_inner
    0x2E, 0x0F,       // LD     L 0x0f
    0x18, 0xF3,       // JR     init_tiles_loop

    // init_scroll:
    0x67,             // LD     H A
    // Changing the following value to from 0x64 to 0x01 skips the
    // whole scrolling logo by displaying it directly in the middle of
    // the screen which makes the intro much shorter while having no
    // side effect that I know of.
    0x3E, 0x64,       // LD     A 0x64
    0x57,             // LD     D A
    0xE0, 0x42,       // LD     [0xff00 + #SCY] A
    0x3E, 0x91,       // LD     A 0x91
    0xE0, 0x40,       // LD     [0xff00 + #LCDC] A
    0x04,             // INC    B

    // scroll_loop:
    0x1E, 0x02,       // LD     E 0x02

    // wait_next_vblank:
    0x0E, 0x0C,       // LD     C 0x0c

    // wait_vblank:
    0xF0, 0x44,       // LD     A [0xff00 + #LY]
    0xFE, 0x90,       // CP     A 0x90
    0x20, 0xFA,       // JR NZ  wait_vblank

    0x0D,             // DEC    C
    0x20, 0xF7,       // JR NZ  wait_vblank
    0x1D,             // DEC    E
    0x20, 0xF2,       // JR NZ  wait_next_vblank

    0x0E, 0x13,       // LD     C 0x13
    0x24,             // INC    H
    0x7C,             // LD     A H
    0x1E, 0x83,       // LD     E 0x83
    0xFE, 0x62,       // CP     A 0x62
    0x28, 0x06,       // JR Z   play_sound
    0x1E, 0xC1,       // LD     E 0xc1
    0xFE, 0x64,       // CP     A 0x64
    0x20, 0x06,       // JR NZ  skip_sound
    // play_sound:
    0x7B,             // LD     A E
    0xE2,             // LD     [0xff00 + C] A
    0x0C,             // INC    C
    0x3E, 0x87,       // LD     A 0x87
    0xE2,             // LD     [0xff00 + C] A
    // skip_sound:
    0xF0, 0x42,       // LD     A [0xff00 + #SCY]
    0x90,             // SUB    A B
    0xE0, 0x42,       // LD     [0xff00 + #SCY] A
    0x15,             // DEC    D
    0x20, 0xD2,       // JR NZ  scroll_loop
    0x05,             // DEC    B
    0x20, 0x4F,       // JR NZ  validate_cart
    0x16, 0x20,       // LD     D 0x20
    0x18, 0xCB,       // JR     scroll_loop


    // crc_0:
    0x4F,             // LD     C A
    // crc_1:
    0x06, 0x04,       // LD     B 0x04
    // crc_round:
    0xC5,             // PUSH   BC
    0xCB, 0x11,       // RL     C
    0x17,             // RL     A
    0xC1,             // POP    BC
    0xCB, 0x11,       // RL     C
    0x17,             // RL     A
    0x05,             // DEC    B
    0x20, 0xF5,       // JR NZ  crc_round
    0x22,             // LDI    [HL] A
    0x23,             // INC    HL
    0x22,             // LDI    [HL] A
    0x23,             // INC    HL
    0xC9,             // RET

    // expected_header: bytes
    0xCE, 0xED, 0x66, 0x66, 0xCC, 0x0D, 0x00, 0x0B,
    0x03, 0x73, 0x00, 0x83, 0x00, 0x0C, 0x00, 0x0D,
    0x00, 0x08, 0x11, 0x1F, 0x88, 0x89, 0x00, 0x0E,
    0xDC, 0xCC, 0x6E, 0xE6, 0xDD, 0xDD, 0xD9, 0x99,
    0xBB, 0xBB, 0x67, 0x63, 0x6E, 0x0E, 0xEC, 0xCC,
    0xDD, 0xDC, 0x99, 0x9F, 0xBB, 0xB9, 0x33, 0x3E,

    // tile_data: bytes
    0x3C, 0x42, 0xB9, 0xA5, 0xB9, 0xA5, 0x42, 0x3C,

    // validate_cart:
    0x21, 0x04, 0x01, // LD     HL, 0x0104
    0x11, 0xA8, 0x00, // LD     DE, #expected_header
    // check_header:
    0x1A,             // LD     A [DE]
    0x13,             // INC    DE
    0xBE,             // CP     A [HL]
    // This is an infinite loop when the checksum fails. Replacing
    // it with 0x00 0x00 (NOP NOP) will allow invalid ROMs to run.
    0x20, 0xFE,       // JR NZ  .
    0x23,             // INC    HL
    0x7D,             // LD     A L
    0xFE, 0x34,       // CP     A 0x32
    0x20, 0xF5,       // JR NZ  check_header
    0x06, 0x19,       // LD     B 0x19
    0x78,             // LD     A B
    // header_sum
    0x86,             // ADD    A [HL]
    0x23,             // INC    HL
    0x05,             // DEC    B
    0x20, 0xFB,       // JR NZ  header_sum
    0x86,             // ADD    A [HL]
    // same as above, infinite loop if the sum is bad, replace with
    // NOPs to run anyway.
    0x20, 0xFE,       // JR NZ  .
    0x3E, 0x01,       // LD A   0x1
    // There shouldn't be anything at that address, I assume that's
    // how you tell the hardware to unmap the bootrom
    0xE0, 0x50,       // LD [0xff00 + 0x50] A
    ];

/// Super GameBoy bootrom, lifted from
/// http://www.its.caltech.edu/~costis/sgb_hack/. It doesn't scroll
/// the logo down the screen so it's faster than the original GB.
#[cfg(feature="sgb_bootrom")]
pub static BOOTROM: [u8, ..0x100] = [
    // init_stack:
    0x31, 0xFE, 0xFF, // LD     SP 0xfffe

    // This activates both controller lines (D-pad + buttons). I think
    // the SGB uses those two lines to communicate with the
    // SNES. See `snes_tx` bellow.
    0x3E, 0x30,       // LD     A 0x30
    0xE0, 0x00,       // LD     [0xff00 + #INPUT], A
    0xAF,             // XOR    A A
    0x21, 0xFF, 0x9F, // LD     HL #VRAM_END
    // clear_vram:
    0x32,             // LDD    [HL] A
    0xCB, 0x7C,       // BIT    H 7
    0x20, 0xFB,       // JR NZ  clear_vram
    // init_sound:
    0x21, 0x26, 0xFF, // LD     HL 0xff26
    0x0E, 0x11,       // LD     C 0x11
    0x3E, 0x80,       // LD     A 0x80
    0x32,             // LDD    [HL] A
    0xE2,             // LD     [0xff00 + C] A
    0x0C,             // INC    C
    0x3E, 0xF3,       // LD     A 0xf3
    0xE2,             // LD     [0xff00 + C] A
    0x32,             // LDD    [HL] A
    0x3E, 0x77,       // LD     A 0x77
    0x77,             // LD     [HL] A

    // init_palette:
    0x3E, 0xFC,       // LD     A 0xfc
    0xE0, 0x47,       // LD     [0xff00 + #BGP] A

    // init_crc:
    0x21, 0x5F, 0xC0, // LD     HL 0xc05f
    0x0E, 0x08,       // LD     C 0x08
    0xAF,             // XOR    A A
    // Pad the packet sent to the snes with 0s
    // zero_loop:
    0x32,             // LDD    [HL] A
    0x0D,             // DEC    C
    0x20, 0xFC,       // JR NZ  zero_loop
    0x11, 0x4F, 0x01, // LD     DE 0x014f
    0x3E, 0xFB,       // LD     A  0xfb
    0x0E, 0x06,       // LD     C  0x06
    // header_sum:
    0xF5,             // PUSH   AF
    0x06, 0x00,       // LD     B 0
    // header_sum_inner:
    0x1A,             // LD     A [DE]
    0x1B,             // DEC    DE
    0x32,             // LDD    [HL] A
    0x80,             // ADD    A B
    0x47,             // LD     B A
    0x0D,             // DEC    C
    0x20, 0xF8,       // JR NZ  header_sum_inner
    0x32,             // LDD    [HL] A
    0xF1,             // POP    AF
    0x32,             // LDD    [HL] A
    0x0E, 0x0E,       // LD     C 0x0E
    0xD6, 0x02,       // SUB    A 0x02
    0xFE, 0xEF,       // CP     A 0xEF
    0x20, 0xEA,       // JR NZ  header_sum

    // init_crc:
    0x11, 0x04, 0x01, // LD     DE 0x0104
    0x21, 0x10, 0x80, // LD     HL 0x8010
    // do_crc:
    0x1A,             // LD     A [DE]
    0xCD, 0xD3, 0x00, // CALL   crc_0
    0xCD, 0xD4, 0x00, // CALL   crc_1
    0x13,             // INC    DE
    0x7B,             // LD     A E
    0xFE, 0x34,       // CP     A 0x34
    0x20, 0xF3,       // JR NZ  do_crc

    0x11, 0xE6, 0x00, // LD     DE #tile_data
    0x06, 0x08,       // LD     B 0x08
    // copy_tile_map:
    0x1A,             // LD     A [DE]
    0x13,             // INC    DE
    0x22,             // LDI    HL A
    0x23,             // INC    HL
    0x05,             // DEC    B
    0x20, 0xF9,       // JR NZ  copy_tile_map

    // init_tiles:
    0x3E, 0x19,       // LD     A 0x19
    0xEA, 0x10, 0x99, // LD     [0x9910] A
    0x21, 0x2F, 0x99, // LD     HL 0x992f
    // init_tiles_loop:
    0x0E, 0x0C,       // LD     C 0x0C
    // init_tiles_inner:
    0x3D,             // DEC    A
    0x28, 0x08,       // JR Z   start_lcd
    0x32,             // LDD    [HL] A
    0x0D,             // DEC    C
    0x20, 0xF9,       // JR SZ  init_tiles_inner
    0x2E, 0x0F,       // LD     L 0x0f
    0x18, 0xF3,       // JR     init_tiles_loop
    // start_lcd:
    0x3E, 0x91,       // LD     A 0x91
    0xE0, 0x40,       // LD     [0xff00 + #LCDC] A

    // snes_tx:
    0x21, 0x00, 0xC0, // LD     HL 0xc000
    0x0E, 0x00,       // LD     C 0x00
    // snes_continue_tx:
    0x3E, 0x00,       // LD     A 0x00
    0xE2,             // LD     [0xff00 + C] A
    0x3E, 0x30,       // LD     A 0x30
    0xE2,             // LD     [0xff00 + C] A
    0x06, 0x10,       // LD     B 0x10
    // tx_loop:
    0x1E, 0x08,       // LD     E 0x08
    0x2A,             // LDI    A [HL]
    0x57,             // LD     D A
    // tx_byte:
    0xCB, 0x42,       // BIT    D 0
    0x3E, 0x10,       // LD     A 0x10
    0x20, 0x02,       // JR NZ  send_1
    0x3E, 0x20,       // LD     A 0x20
    0xE2,             // LD     [0xff00 + C] A
    // send_1:
    0x3E, 0x30,       // LD     A 0x30
    0xE2,             // LD     [0xff00 + C] A
    0xCB, 0x1A,       // RR     D
    0x1D,             // DEC    E
    0x20, 0xEF,       // JR NZ  tx_byte
    0x05,             // DEC    B
    0x20, 0xE8,       // JR NZ  tx_loop
    0x3E, 0x20,       // LD     A 0x20
    0xE2,             // LD     [0xff00 + C] A
    0x3E, 0x30,       // LD     A 0x30
    0xE2,             // LD     [0xff00 + C] A
    0xCD, 0xC2, 0x00, // CALL   wait_next_vblank
    0x7D,             // LD     A L
    0xFE, 0x60,       // CP     A 0x60
    0x20, 0xD2,       // JR NZ  snes_continue_tx
    // beep:
    0x0E, 0x13,       // LD     C 0x13
    0x3E, 0xC1,       // LD     A 0xc1
    0xE2,             // LD     [0xff00 + C] A
    0x0C,             // INC    C
    0x3E, 0x07,       // LD     A 0x07
    0xE2,             // LD     [0xff00 + C] A
    0x18, 0x3A,       // JR     start_game

    // wait_next_vblank:
    0x16, 0x04,       // LD     D 0x04
    // wait_vblank:
    0xF0, 0x44,       // LD     A [0xff00 + #LY]
    0xFE, 0x90,       // CP     A 0x90
    0x20, 0xFA,       // JR NZ  wait_vblank
    0x1E, 0x00,       // LD     E 0x00
    // wait_255:
    0x1D,             // DEC    E
    0x20, 0xFD,       // JR NZ  wait_255
    0x15,             // DEC    D
    0x20, 0xF2,       // JR NZ  wait_vblank
    0xC9,             // RET
    // crc_0:
    0x4F,             // LD     C A
    // crc_1:
    0x06, 0x04,       // LD     B 0x04
    // crc_round:
    0xC5,             // PUSH   BC
    0xCB, 0x11,       // RL     C
    0x17,             // RL     A
    0xC1,             // POP    BC
    0xCB, 0x11,       // RL     C
    0x17,             // RL     A
    0x05,             // DEC    B
    0x20, 0xF5,       // JR NZ  crc_round
    0x22,             // LDI    [HL] A
    0x23,             // INC    HL
    0x22,             // LDI    [HL] A
    0x23,             // INC    HL
    0xC9,             // RET
    // tile_data: bytes
    0x3C, 0x42, 0xB9, 0xA5, 0xB9, 0xA5, 0x42, 0x3C,
    // padding:
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    // start_game:
    0x3E, 0x01,       // LD     A 1
    0xE0, 0x50,       // LD     [0xff00 + 0x50] A
    ];
