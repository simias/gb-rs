//! gb-rs: Game Boy emulator
//! Ressources:
//!
//! Opcode map: http://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html
//! JS emulator: http://imrannazar.com/GameBoy-Emulation-in-JavaScript:-The-CPU
//! Lots of info about GC quircks: http://www.devrs.com/gb/files/faqs.html
//! Accuracy tests: http://tasvideos.org/EmulatorResources/GBAccuracyTests.html

#![feature(if_let)]
#![warn(missing_docs)]

extern crate sdl2;

mod cpu;
mod io;
mod gpu;
mod ui;

fn main() {
    let mut display    = ui::sdl2::Display::new();

    let argv = std::os::args();

    if argv.len() < 2 {
        println!("Usage: {} <rom-file>", argv[0]);
        return;
    }

    let romfile = &argv[1];

    let rom = match io::rom::Rom::from_file(&Path::new(romfile)) {
        Ok(r)  => r,
        Err(e) => panic!("Failed to load ROM: {}", e),
    };

    println!("Loaded ROM {}", rom);

    let gpu = gpu::Gpu::new(&mut display);

    let mut controller = ui::sdl2::Controller::new();

    let inter = io::Interconnect::new(rom, gpu, &mut controller);

    let mut cpu = cpu::Cpu::new(inter);

    cpu.reset();

    loop {
        cpu.step();
    }
}
