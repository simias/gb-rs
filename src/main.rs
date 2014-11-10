//! gb-rs: Game Boy emulator
//! Ressources:
//!
//! Opcode map: http://www.pastraiser.com/cpu/gameboy/gameboy_opcodes.html
//! JS emulator: http://imrannazar.com/GameBoy-Emulation-in-JavaScript:-The-CPU

#![warn(missing_docs)]

mod cpu;
mod io;
mod gpu;

fn main() {
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

    let mut cpu = cpu::Cpu::new(rom);

    cpu.reset();

    loop {
        cpu.step();
    }
}
