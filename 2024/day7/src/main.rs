use std::{fs::File, io::Read};

fn main() {
    let mut file = File::open("data/input").expect("Open file");
    let mut buf = String::new();
    file.read_to_string(&mut buf).expect("Read to string");

    let lines: Vec<_> = buf.lines().collect();
    let mut total_calibration_result = 0;

    for line in lines {
        if let Some((test, numbers)) = line.split_once(":") {
            let test_value: i64 = test.trim().parse().expect("Valid test value");
            let numbers: Vec<i64> = numbers
                .split_whitespace()
                .map(|num| num.parse().expect("Valid number"))
                .collect();

            if is_valid_equation(test_value, &numbers) {
                total_calibration_result += test_value;
            }
        }
    }

    println!("Total calibration result: {}", total_calibration_result);
}

fn is_valid_equation(test_value: i64, numbers: &[i64]) -> bool {
    fn evaluate(test_value: i64, numbers: &[i64], current_value: i64) -> bool {
        if numbers.is_empty() {
            current_value == test_value
        } else if evaluate(test_value, &numbers[1..], current_value + numbers[0]) {
            true
        } else if evaluate(test_value, &numbers[1..], current_value * numbers[0]) {
            true
        } else if evaluate(
            test_value,
            &numbers[1..],
            format!("{}{}", current_value, numbers[0])
                .parse()
                .expect("Concat"),
        ) {
            true
        } else {
            false
        }
    }

    evaluate(test_value, &numbers[1..], numbers[0])
}
