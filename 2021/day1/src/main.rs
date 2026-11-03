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

    let mut window = vec![];

    for i in 2..depths.len() {
        window.push(depths[i - 2] + depths[i - 1] + depths[i]);
    }

    let mut depth_up = 0;
    let mut prev = window[0];

    for i in 1..window.len() {
        if window[i] > prev {
            depth_up += 1;
        }
        prev = window[i];
    }

    println!("{depth_up}");
}
