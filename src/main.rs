#![warn(missing_docs)]

//! gb-rs: Game Boy emulator

mod cpu;
mod io;

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

    let inter = io::Interconnect::new(rom);

    let mut cpu = cpu::Cpu::new(&inter);

    cpu.reset();

    loop {
        cpu.step();
    }
}
