use std::{fs::File, io::Read};

fn is_safe(report: &[i32]) -> bool {
    if report.len() < 2 {
        return true;
    }

    let leading_factor = report[1] - report[0];
    let leading_factor = if leading_factor != 0 {
        leading_factor / leading_factor.abs()
    } else {
        0
    };

    for i in 0..report.len() - 1 {
        let diff = report[i + 1] - report[i];
        let direction = if diff != 0 { diff / diff.abs() } else { 0 };
        if direction != leading_factor || !(1..=3).contains(&diff.abs()) {
            return false;
        }
    }

    true
}

fn main() {
    let mut input_file = File::open("data/input").expect("Failed to open file");
    let mut lines = String::new();
    input_file
        .read_to_string(&mut lines)
        .expect("Failed to read file to buffer");

    let mut safe_count = 0;

    for line in lines.lines() {
        let levels: Vec<i32> = line
            .split_whitespace()
            .filter_map(|num| num.parse::<i32>().ok())
            .collect();

        if is_safe(&levels) {
            safe_count += 1;
            continue;
        }

        let mut made_safe = false;
        for i in 0..levels.len() {
            let mut modified = levels.clone();
            modified.remove(i);
            if is_safe(&modified) {
                made_safe = true;
                break;
            }
        }

        if made_safe {
            safe_count += 1;
        }
    }

    println!("Number of safe reports: {safe_count}");
}

