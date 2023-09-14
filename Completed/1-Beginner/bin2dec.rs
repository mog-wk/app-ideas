use std::env;
use std::process;
use std::error::Error;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        usage();
        process::exit(1);
    }

    let d = bin_to_dec(&args[1]);

    println!("Decimal: {}", d);
}

fn usage() {
    println!("bin2dec <binary_number> -> decimal number");
}

// reads in little endian
fn bin_to_dec(bin: &str) -> u32 {
    let mut dec_num: u32 = 0;

    for pair in bin.bytes().rev().enumerate() {
        let (counter, bit) = pair;
        let counter = counter as u32;
        let bit = bit as u32;
        
        println!("{:?} {}", bit, counter);
        match bit {
            48 | 49 => dec_num += 2_u32.pow(counter) * (bit - 48),
            _ => {
                println!("Invalid bit: {} on {} position", bit, counter);
                process::exit(1);
            },
        }
        println!("{}", dec_num);
    }
    dec_num
}
