use std::{fs::File, io::Read};

fn main() {
    let mut input_file = File::open("data/input").expect("Failed to open file");
    let mut lines = String::new();

    input_file
        .read_to_string(&mut lines)
        .expect("Failed to read file to buffer");

    let lines_iter = lines.split("\n");

    let mut x = 0;
    let mut y = 0;

    let mut aim = 0;

    for line in lines_iter {
        let parts: Vec<&str> = line.split(" ").collect();

        if parts.len() == 2 {
            let dir = parts[0];
            let amnt = i32::from_str_radix(parts[1], 10).unwrap();

            match dir {
                "forward" => {
                    x += amnt;
                    y += aim * amnt;
                }
                "backward" => x -= amnt,
                "down" => aim += amnt,
                "up" => aim -= amnt,
                _ => {}
            }
        }
    }

    println!("{}", x * y);
}
