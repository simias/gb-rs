//! gb-rs: Game Boy emulator
//! Ressources:
//!
//! Opcode map: http://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html
//! JS emulator: http://imrannazar.com/GameBoy-Emulation-in-JavaScript:-The-CPU
//! Lots of info about GC quircks: http://www.devrs.com/gb/files/faqs.html
//! Accuracy tests: http://tasvideos.org/EmulatorResources/GBAccuracyTests.html

#![cfg_attr(test, feature(test))]

#![warn(missing_docs)]

#[macro_use]
extern crate log;
extern crate sdl2;
extern crate ascii;
extern crate num;
extern crate libc;
extern crate gl;

#[cfg(test)]
extern crate test;

use std::sync::mpsc::channel;
use ui::Audio;
use std::path::Path;

mod cpu;
mod io;
mod gpu;
mod ui;
mod cartridge;
mod spu;
mod resampler;

#[allow(dead_code)]
fn main() {
    let argv: Vec<_> = std::env::args().collect();

    if argv.len() < 2 {
        println!("Usage: {} <rom-file>", argv[0]);
        return;
    }

    let rompath = Path::new(&argv[1]);

    let cart = match cartridge::Cartridge::from_path(&rompath) {
        Ok(r)  => r,
        Err(e) => panic!("Failed to load ROM: {}", e),
    };

    println!("Loaded ROM {:?}", cart);

    let sdl2 = ui::sdl2::Context::new();

    let mut display = sdl2.opengl_new(1280, 1152);

    let gpu = gpu::Gpu::new(&mut display);

    let (spu, audio_channel) = spu::Spu::new();

    let mut audio = ui::sdl2::Audio::new(audio_channel);

    audio.start();

    let inter = io::Interconnect::new(cart, gpu, spu, sdl2.buttons());

    let mut cpu = cpu::Cpu::new(inter);

    // In order to synchronize the emulation speed with the wall clock
    // we need to wait at some point so that we don't go too
    // fast. Waiting between each cycle would mean a storm of syscalls
    // back and forth between the kernel and us, so instead we execute
    // instructions in batches of GRANULARITY cycles and then sleep
    // for a while. If the GRANULARITY value is too low we'll go to
    // sleep very often which will have poor performance. If it's too
    // high it might look like the emulation is stuttering.

    let batch_duration_ns = GRANULARITY * (1_000_000_000 /
                                           SYSCLK_FREQ);

    // No sub-ms precision in stable rust sleep for now...
    let batch_duration_ms = (batch_duration_ns / 1_000_000) as u32;

    let (tick_tx, tick_rx) = channel();

    // Spawn a thread that will send periodic ticks that we'll use to
    // synchronize ourselves
    ::std::thread::spawn(move || {
        loop {
            std::thread::sleep_ms(batch_duration_ms);
            if let Err(_) = tick_tx.send(()) {
                // End thread
                return;
            }
        }
    });

    let mut audio_adjust_count = 0;

    let mut cycles = 0;

    loop {
        while cycles < GRANULARITY {
            // The actual emulator takes place here!
            cycles += cpu.run_next_instruction() as i64;
        }

        cycles -= GRANULARITY;

        // Update controller status
        match sdl2.update_buttons() {
            ui::Event::PowerOff => break,
            ui::Event::None     => (),
        }

        // Sleep until next batch cycle
        if let Err(e) = tick_rx.recv() {
             panic!("Timer died: {:?}", e);
        }

        audio_adjust_count += GRANULARITY;

        if audio_adjust_count >= SYSCLK_FREQ * AUDIO_ADJUST_SEC {
            // Retrieve the number of samples generated since the last
            // adjustment
            let s = spu::samples_per_steps(audio_adjust_count as u32);

            audio.adjust_resampling(s);

            audio_adjust_count = 0;
        }
    }
}

/// Number of instructions executed between sleeps (i.e. giving the
/// hand back to the scheduler). Low values increase CPU usage and can
/// result in poor performance, high values will cause stuttering.
const GRANULARITY:      i64 = 0x10000;

/// Gameboy sysclk frequency: 4.19Mhz
const SYSCLK_FREQ:      i64 = 0x400000;

/// How often should we adjust the audio resampling rate. In seconds.
const AUDIO_ADJUST_SEC: i64 = 1;

#[cfg(test)]
mod benchmark {
    use test::Bencher;

    use std::thread::spawn;

    #[bench]
    fn bench_rom(b: &mut Bencher) {
        let mut display = ::ui::dummy::DummyDisplay;
        let controller  = ::ui::dummy::DummyController::new();

        let rom = ::std::iter::repeat(0).take(0x4000).collect();
        let cart = ::cartridge::Cartridge::from_vec(rom);

        let (spu, audio_channel) = ::spu::Spu::new();

        spawn(move|| {
            // Dummy consumer
            while let Ok(_) = audio_channel.recv() {
            }
        });

        let gpu = ::gpu::Gpu::new(&mut display);

        let inter = ::io::Interconnect::new(cart,
                                            gpu,
                                            spu,
                                            controller.buttons());

        let mut cpu = ::cpu::Cpu::new(inter);

        b.iter(|| {
            cpu.reset();

            // Simulate 100ms of emulated time so that the benchmark
            // doesn't run for too long.
            let mut cycle = 0;

            while cycle < super::SYSCLK_FREQ / 10 {
                cycle += cpu.run_next_instruction() as i64;
            }
        });
    }
}
