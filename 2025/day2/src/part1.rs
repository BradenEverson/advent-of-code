use std::{fs::File, io::Read};

pub fn solve() {
    let mut input_file = File::open("data/input").expect("Failed to open file");
    let mut lines = String::new();

    let mut invalid = 0;

    input_file
        .read_to_string(&mut lines)
        .expect("Failed to read file to buffer");

    let ranges = lines.split(",");

    for range in ranges {
        let split: Vec<&str> = range.trim().split("-").collect();
        let start: u64 = split[0].parse().expect("Failed to parse");
        let end: u64 = split[1].parse().expect("Failed to parse");

        for i in start..=end {
            let strnum = i.to_string();
            if strnum.len() % 2 == 0 {
                let half1 = &strnum[0..(strnum.len() / 2)];
                let half2 = &strnum[(strnum.len() / 2)..];

                if half1 == half2 {
                    invalid += i;
                }
            }
        }
    }

    println!("{invalid} Invalid IDs")
}
