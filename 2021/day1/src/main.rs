use std::{fs::File, io::Read};

fn main() {
    let mut input_file = File::open("data/input").expect("Failed to open file");
    let mut lines = String::new();

    input_file
        .read_to_string(&mut lines)
        .expect("Failed to read file to buffer");

    let depths: Vec<u32> = lines
        .split("\n")
        .flat_map(|l| u32::from_str_radix(l, 10))
        .collect();

    let mut depth_up = 0;
    let mut prev = depths[0];

    for i in 1..depths.len() {
        if depths[i] > prev {
            depth_up += 1;
        }
        prev = depths[i];
    }

    println!("{depth_up}");
}
