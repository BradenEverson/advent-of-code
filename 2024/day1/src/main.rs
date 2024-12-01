use std::{fs::File, io::Read};

fn main() {
    let mut input_file = File::open("data/input").expect("Failed to open file");
    let mut lines = String::new();

    input_file
        .read_to_string(&mut lines)
        .expect("Failed to read file to buffer");

    let mut left = vec![];
    let mut right = vec![];

    for line in lines.lines() {
        let split: Vec<_> = line.split_whitespace().collect();
        left.push(split[0].parse::<i32>().expect("Parse failed"));
        right.push(split[1].parse::<i32>().expect("Parse failed"));
    }

    left.sort();
    right.sort();

    let distance = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum::<i32>();

    let similarity = left
        .iter()
        .map(|elem| *elem * right.iter().filter(|r| *r == elem).count() as i32)
        .sum::<i32>();

    println!("{similarity}")
}
