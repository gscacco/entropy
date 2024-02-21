use std::{env, fs, io::Read};

fn main() {
    let mut v: [u64; 256] = [0; 256];
    let mut num_bytes = 0;
    let mut entropy: f32 = 0.0;
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("USAGE: entropy <filename>");
        return;
    }
    let fpath = args.get(1).unwrap();
    let mut file = fs::File::open(fpath).unwrap();

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

    for value in v {
        let p = value as f32 / num_bytes as f32;
        if p > 0.0 {
            entropy += -1.0 * p * f32::log2(p);
        }
    }

    println!("Shannon entropy: {}", entropy)
}
