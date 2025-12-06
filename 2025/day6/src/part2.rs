use std::{fs::File, io::Read};

use crate::{Op, Value};

pub fn part2() -> u64 {
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
            let mut chars = line.chars();

            let mut line = vec![];

            while let Some(ch) = chars.next() {
                match ch {
                    ' ' => line.push(Value::Space),
                    c => line.push(Value::Number((c as u8 - ('0' as u8)) as u64)),
                }
            }

            inputs.push(line);
        }
    }

    while let Some(op) = ops.pop() {
        let mut col = vec![];

        let mut op = op;

        while op == Op::Space {
            let mut val = vec![];
            for row in &mut inputs {
                match row.pop().unwrap() {
                    Value::Number(n) => val.push(n),
                    Value::Space => {}
                };
            }

            op = ops.pop().unwrap();

            let mut parsed_val = 0;

            for (idx, c) in val.iter().enumerate() {
                parsed_val += c * (10u32.pow((val.len() - 1 - idx) as u32) as u64);
            }

            if parsed_val != 0 {
                col.push(parsed_val);
            }
        }

        let mut val = vec![];
        for row in &mut inputs {
            match row.pop().unwrap() {
                Value::Number(n) => {
                    val.push(n);
                }
                Value::Space => {}
            };
        }

        let mut parsed_val = 0;

        for (idx, c) in val.iter().enumerate() {
            parsed_val += c * (10u32.pow((val.len() - 1 - idx) as u32) as u64);
        }

        if parsed_val != 0 {
            col.push(parsed_val);
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
