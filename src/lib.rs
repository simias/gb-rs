//! gb-rs: Game Boy emulator
//! Ressources:
//!
//! Opcode map: http://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html
//! JS emulator: http://imrannazar.com/GameBoy-Emulation-in-JavaScript:-The-CPU
//! Lots of info about GC quircks: http://www.devrs.com/gb/files/faqs.html
//! Accuracy tests: http://tasvideos.org/EmulatorResources/GBAccuracyTests.html

#![cfg_attr(test, feature(test))]

#[macro_use]
extern crate log;
extern crate ascii;
extern crate num;
extern crate libc;

#[cfg(test)]
extern crate test;

pub mod libretro;

use std::path::Path;
use std::cell::Cell;
use std::boxed::Box;

use cpu::Cpu;

mod cpu;
mod io;
mod gpu;
mod cartridge;
mod spu;
mod resampler;
mod ui;

fn load_game(rompath: &Path) -> Cpu {

    let cart = match cartridge::Cartridge::from_path(&rompath) {
        Ok(r)  => r,
        Err(e) => panic!("Failed to load ROM: {}", e),
    };

    let gpu = gpu::Gpu::new();

    let spu = spu::Spu::new();

    let inter = io::Interconnect::new(cart, gpu, spu);

    Cpu::new(inter)
}

fn render_frame(cpu: &mut Cpu) {
    for _ in 0..(456 * 154) {
        cpu.run_next_instruction();
    }
}

/// Gameboy sysclk frequency: 4.19Mhz
const SYSCLK_FREQ:      i64 = 0x400000;
