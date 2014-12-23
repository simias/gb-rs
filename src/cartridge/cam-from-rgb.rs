use std::old_io::{File, Truncate, Write};

fn main() {
    let argv = std::os::args();

    if argv.len() < 3 {
        println!("Usage: {} <rgb-file> <gb-file>", argv[0]);
        return;
    }

    let infile = Path::new(&argv[1]);
    let outfile = Path::new(&argv[2]);

    let mut src = File::open(&infile).unwrap();
    let mut dst = File::open_mode(&outfile, Truncate, Write);

    let rgb = src.read_exact(256 * 256 * 3).unwrap();
    let mut gb: Vec<_>  = std::iter::repeat(0).take((256 * 256 * 2) / 8).collect();

    for x in range(0, 128) {
        for y in range(0 ,256) {
            let b = rgb[y * (256 * 3) + x * 3 + 0] as u32;
            let g = rgb[y * (256 * 3) + x * 3 + 1] as u32;
            let r = rgb[y * (256 * 3) + x * 3 + 2] as u32;

            let col = ((66 * r + 129 * g + 25 * b + 128 + 16) >> 8) as u8;

            let col = r / 64;

            let b0 = (col & 1 == 0) as u8;
            let b1 = (col & 2 == 0) as u8;

            let mut byte = (y / 8) << 8;
            byte += (y & 0x7) << 1;
            byte += (x / 0x8) << 4;

            let bit = 0x7 - (x & 0x7);

            gb[byte]     |= b0 << bit;
            gb[byte + 1] |= b1 << bit;
        }
    }

    dst.write(gb.as_slice());
}
