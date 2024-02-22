use std::{fs, io::Read};

use clap::Parser;
/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    file: String,
}
fn main() {
    let mut v: [u64; 256] = [0; 256];
    let mut num_bytes = 0;

    let args = Args::parse();

    let fpath = args.file;
    let mut file = fs::File::open(&fpath).unwrap();

    let mut buffer: [u8; 1] = [0; 1];
    loop {
        match file.read_exact(&mut buffer) {
            Ok(_) => {
                v[buffer[0] as usize] += 1;
                num_bytes += 1;
            }
            Err(_) => break,
        }
    }
    let entropy: f32 = v
        .iter()
        .filter(|u| **u != 0)
        .map(|u| *u as f32 / num_bytes as f32)
        .map(|u| -1.0 * u * f32::log2(u))
        .sum();

    println!("File {fpath} of size {num_bytes} bytes: Shannon entropy {entropy}")
}
