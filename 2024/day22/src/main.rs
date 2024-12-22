use std::{fs::File, io::Read};

const MODULO: u64 = 16_777_216;

fn next_secret(mut secret: u64) -> u64 {
    secret ^= (secret * 64) % MODULO;
    secret ^= (secret / 32) % MODULO;
    secret ^= (secret * 2048) % MODULO;

    secret % MODULO
}

fn simulate_buyer(mut secret: u64, steps: usize) -> u64 {
    for _ in 0..steps {
        secret = next_secret(secret);
    }
    secret
}

fn main() {
    let path = "data/input";
    let mut input = File::open(&path).expect("Unable to open input file");
    let mut buf = String::new();
    input
        .read_to_string(&mut buf)
        .expect("Unable to read to file");

    let buyers: Vec<u64> = buf
        .lines()
        .filter_map(|line| line.parse::<u64>().ok())
        .collect();

    let steps = 2000;
    let result: u64 = buyers
        .iter()
        .map(|&initial_secret| simulate_buyer(initial_secret, steps))
        .sum();

    println!("The sum of the 2000th secret numbers is: {}", result);
}
