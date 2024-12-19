use std::{fs::File, io::Read};

fn main() {
    let mut file = File::open("data/input").expect("Open data");
    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .expect("Failed to write to buffer");
    let fuels: isize = buf
        .lines()
        .filter_map(|line| line.parse::<isize>().ok())
        .flat_map(|val| {
            let mut fuels = vec![];
            let mut initial = val;
            while initial > 0 {
                initial = (initial / 3) - 2;
                if initial > 0 {
                    fuels.push(initial);
                }
            }

            fuels
        })
        .sum();

    println!("{fuels}")
}
