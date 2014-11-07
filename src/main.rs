#![warn(missing_doc)]

//! gb-rs: Game Boy emulator

mod cpu;

fn main() {
    let mut cpu = cpu::Cpu::new();

    cpu.reset();

    cpu.step();

    println!("{}", cpu);
}
