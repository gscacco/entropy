use std::{env, fs};

fn main() {
    let mut v: [u64; 256] = [0; 256];
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("USAGE: entropy <filename>");
        return;
    }
    let fpath = args.get(1).unwrap();
    let buff = fs::read(fpath).unwrap();
    let nbytes = buff.len();
    let mut entropy = 0.0;
    for b in buff {
        v[b as usize] += 1;
    }
    for value in v {
        let p = value as f32 / nbytes as f32;
        if p > 0.0 {
            entropy += -1.0 * p * f32::log2(p);
        }
    }

    println!("Shannon entropy: {}", entropy)
}
