use std::{fs::File, io::Read};

use crate::{Op, Value};

pub fn part1() -> u64 {
    let mut input_file = File::open("data/input").expect("Failed to open file");
    let mut lines = String::new();
    input_file
        .read_to_string(&mut lines)
        .expect("Failed to read file to buffer");

    let mut inputs = vec![];
    let mut ops = vec![];

    let mut sum = 0;

    for line in lines.lines() {
        if line.contains("+") {
            // last line
            for c in line.chars() {
                let op = match c {
                    ' ' => Op::Space,
                    '+' => Op::Plus,
                    '*' => Op::Mul,
                    _ => panic!(":("),
                };

                ops.push(op);
            }
        } else {
            let mut parsed: Vec<u64> = line
                .split(" ")
                .flat_map(|n| u64::from_str_radix(n, 10))
                .collect();

            let mut chars = line.chars();

            let mut line = vec![];

            while let Some(ch) = chars.next() {
                match ch {
                    ' ' => line.push(Value::Space),
                    _ => {
                        while let Some(c) = chars.next()
                            && c != ' '
                        {}
                        line.push(Value::Number(parsed.remove(0)));
                        line.push(Value::Space);
                    }
                }
            }

            inputs.push(line);
        }
    }

    while let Some(op) = ops.pop() {
        let mut col = vec![];

        let mut op = op;
        while op == Op::Space {
            op = ops.pop().unwrap();
        }

        for row in &mut inputs {
            let mut row_curr = row.pop().unwrap();
            while row_curr == Value::Space {
                row_curr = row.pop().unwrap();
            }

            col.push(row_curr.val());
        }

        let val: u64 = match op {
            Op::Mul => col.iter().product(),
            Op::Plus => col.iter().sum(),
            _ => panic!("Something is so wrong"),
        };

        sum += val;
    }

    sum
}
