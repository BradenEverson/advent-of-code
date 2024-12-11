use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

fn main() {
    let mut file = File::open("data/input").expect("Unable to open input file");
    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .expect("Unable to read input file");
    let input = buf.trim();

    let mut counts = HashMap::new();

    for stone in input
        .split_whitespace()
        .filter_map(|s| s.parse::<i64>().ok())
    {
        *counts.entry(stone).or_insert(0) += 1;
    }

    let steps = 75;

    for _ in 0..steps {
        counts = simulate_step(counts);
    }

    let total_stones: usize = counts.values().sum();

    println!(
        "After {} blinks, there will be {} stones.",
        steps, total_stones
    );
}

fn simulate_step(current: HashMap<i64, usize>) -> HashMap<i64, usize> {
    let mut next_counts = HashMap::new();

    for (value, count) in current {
        if value == 0 {
            *next_counts.entry(1).or_insert(0) += count;
        } else {
            let val_str = value.to_string();
            if val_str.len() % 2 == 0 {
                let mid = val_str.len() / 2;
                let left: i64 = val_str[..mid].parse().unwrap_or(0);
                let right: i64 = val_str[mid..].parse().unwrap_or(0);
                *next_counts.entry(left).or_insert(0) += count;
                *next_counts.entry(right).or_insert(0) += count;
            } else {
                let new_value = value * 2024;
                *next_counts.entry(new_value).or_insert(0) += count;
            }
        }
    }

    next_counts
}
