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

use cpu::Cpu;

mod cpu;
mod io;
mod gpu;
mod cartridge;
mod spu;
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

fn button_state(b: libretro::JoyPadButton) -> ui::ButtonState {
    if libretro::button_pressed(b) {
        ui::ButtonState::Down
    } else {
        ui::ButtonState::Up
    }
}

fn render_frame(cpu: &mut Cpu) {
    let buttons = ui::Buttons {
        up: button_state(libretro::JoyPadButton::Up),
        down: button_state(libretro::JoyPadButton::Down),
        left: button_state(libretro::JoyPadButton::Left),
        right: button_state(libretro::JoyPadButton::Right),
        a: button_state(libretro::JoyPadButton::A),
        b: button_state(libretro::JoyPadButton::B),
        start: button_state(libretro::JoyPadButton::Start),
        select: button_state(libretro::JoyPadButton::Select),
    };

    cpu.set_buttons(buttons);

    for _ in 0..(456 * 154) {
        cpu.run_next_instruction();
    }
}
