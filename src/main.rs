use std::{fs, io::Read};

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The file under analisys
    #[arg(short, long)]
    file: String,
    /// Block size on which calculate entropy. Zero = all file
    #[arg(short, long, default_value_t = 0)]
    block: usize,

    /// Use percentage
    #[arg(short, long, default_value = "false")]
    percent: bool,
}
fn entropy(v: &[u64; 256], num_bytes: f32) -> f32 {
    v.iter()
        .filter(|u| **u != 0)
        .map(|u| *u as f32 / num_bytes)
        .map(|u| -1.0 * u * f32::log2(u))
        .sum::<f32>()
}
fn main() {
    let mut v: [u64; 256] = [0; 256];
    let mut num_bytes = 0;

    let args = Args::parse();

    let fpath = args.file;
    let mut file = fs::File::open(&fpath).unwrap();

    let mut buffer: [u8; 1] = [0; 1];

    println!(
        "File {fpath} of size {num_bytes} bytes. Block size used: {}",
        args.block
    );
    if args.block == 0 {
        loop {
            match file.read_exact(&mut buffer) {
                Ok(_) => {
                    v[buffer[0] as usize] += 1;
                    num_bytes += 1;
                }
                Err(_) => break,
            }
        }
        let entropy: f32 = entropy(&v, num_bytes as f32);
        if args.percent {
            println!("From 0 to {num_bytes} byte entropy {:.5} %", entropy / 8.0);
        } else {
            println!("From 0 to {num_bytes} byte entropy {entropy}");
        }
    } else {
        let mut block_number = 0;
        let mut current_byte = 0;
        loop {
            match file.read_exact(&mut buffer) {
                Ok(_) => {
                    v[buffer[0] as usize] += 1;
                    num_bytes += 1;
                    current_byte += 1;
                    if current_byte == args.block {
                        let entropy: f32 = entropy(&v, current_byte as f32);
                        print_block(args.percent, entropy, block_number);
                        // reset the state
                        v = [0; 256];
                        block_number += 1;
                        current_byte = 0;
                    }
                }
                Err(_) => {
                    if current_byte > 0 {
                        let entropy: f32 = entropy(&v, current_byte as f32);
                        print_block(args.percent, entropy, block_number);
                    }
                    break;
                }
            }
        }
    }
}

fn print_block(percent: bool, entropy: f32, block_number: usize) {
    if percent {
        println!(
            "Block number {block_number}, entropy {:.5} %",
            entropy / 8.0
        );
    } else {
        println!("Block number {block_number}, entropy {entropy:.5}");
    }
}
