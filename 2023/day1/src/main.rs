use std::{fs::File, io::Read};

fn main() {
    let mut input_file = File::open("data/input.in").expect("Failed to open file");
    let mut line = String::new();

    input_file
        .read_to_string(&mut line)
        .expect("Failed to read file to buffer");

    let line_new = preprocess_calibration_lines(&line);
    println!("{}", read_calibration_values(&line_new))
}

pub fn preprocess_calibration_lines(input: &str) -> String {
    let replacements = [
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];

    let mut result = String::new();
    let mut remaining = input;

    while !remaining.is_empty() {
        let mut matched = false;

        for &(pattern, replacement) in &replacements {
            if remaining.starts_with(pattern) {
                result.push_str(replacement);
                remaining = &remaining[1..];
                matched = true;
                break;
            }
        }

        if !matched {
            let mut chars = remaining.chars();
            result.push(chars.next().unwrap());
            remaining = chars.as_str();
        }
    }

    result
}

pub fn read_calibration_values(values: &str) -> i32 {
    let mut total = 0;

    for line in values.lines() {
        let trimmed: Vec<char> = line.chars().filter(|ch| ch.is_digit(10)).collect();
        if trimmed.len() > 0 {
            let num = format!("{}{}", trimmed[0], trimmed[trimmed.len() - 1]);
            total += num.parse::<i32>().expect("Failed to parse int");
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use crate::preprocess_calibration_lines;
    use crate::read_calibration_values;

    #[test]
    fn test_calibration_string() {
        let input = r#"
        1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet
        "#;
        let calibration = read_calibration_values(input);
        assert_eq!(142, calibration)
    }

    #[test]
    fn test_calibration_string_with_preprocessing() {
        let input = r#"
        two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen
        eighthree
        "#;
        let input = preprocess_calibration_lines(input);
        let calibration = read_calibration_values(&input);
        assert_eq!(281 + 83, calibration)
    }
}
