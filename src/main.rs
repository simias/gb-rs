#![warn(missing_doc)]

//! gb-rs: Game Boy emulator

mod cpu;
mod io;

fn main() {
    let mut cpu = cpu::Cpu::new();
    let rom = match io::rom::Rom::from_file(&Path::new("roms/tetris.gb")) {
        Ok(r)  => r,
        Err(e) => panic!("Failed to load ROM: {}", e),
    };

    println!("{}", rom);

    cpu.reset();

    cpu.step();

    println!("{}", cpu);
}
