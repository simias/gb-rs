#![warn(missing_docs)]

//! gb-rs: Game Boy emulator

mod cpu;
mod io;

fn main() {
    let rom = match io::rom::Rom::from_file(&Path::new("roms/tetris.gb")) {
        Ok(r)  => r,
        Err(e) => panic!("Failed to load ROM: {}", e),
    };

    let inter = io::Interconnect::new(rom);

    let mut cpu = cpu::Cpu::new(&inter);

    cpu.reset();

    cpu.step();

    println!("{}", cpu);
}
