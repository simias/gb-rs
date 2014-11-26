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

use std::io::Timer;
use std::time::Duration;

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

    // In order to synchronize the emulation speed with the wall clock
    // we need to wait at some point so that we don't go too
    // fast. Waiting between each cycle would mean a storm of syscalls
    // back and forth between the kernel and us, so instead we execute
    // instructions in batches of GRANULARITY cycles and then sleep
    // for a while. If the GRANULARITY value is too low we'll go to
    // sleep very often which will have poor performance. If it's too
    // high it might look like the emulation is stuttering.
    let mut timer = match Timer::new() {
        Ok(t)  => t,
        Err(e) => panic!("Couldn't create timer: {}", e),
    };

    let batch_duration = Duration::nanoseconds(GRANULARITY * (1_000_000_000 /
                                                              SYSCLK_FREQ));

    let tick = timer.periodic(batch_duration);

    loop {
        for _ in range(0, GRANULARITY) {
            cpu.step();
        }
        // Sleep until next batch cycle
        tick.recv();
    }
}

/// Number of instructions executed between sleeps (i.e. giving the
/// hand back to the scheduler). Low values increase CPU usage and can
/// result in poor performance, high values will cause stuttering.
const GRANULARITY:      i64 = 0x10000;

/// Gameboy sysclk frequency: 4.19Mhz
const SYSCLK_FREQ:      i64 = 0x400000;
