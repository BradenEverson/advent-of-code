use std::{fs::File, io::Read, str::FromStr};

fn main() {
    let mut file = File::open("data/input").expect("Open data");
    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .expect("Failed to write to buffer");

    let mut sum = SumState::default();

    for line in buf.lines() {
        let mut remaining = line;
        while !remaining.is_empty() {
            if let Some(len) = Instruction::starts_with_op(remaining) {
                let mut end_idx = len;

                while end_idx < remaining.len() - 1
                    && is_valid_expr_char(remaining.chars().nth(end_idx).expect("Get valid char"))
                {
                    end_idx += 1;
                }

                let curr_instr = Instruction::from_str(&remaining[0..=end_idx]);
                if let Ok(instr) = curr_instr {
                    instr.eval(&mut sum);
                }
            }
            remaining = &remaining[1..]
        }
    }

    println!("Instruction sum {}", sum.sum());
}

fn is_valid_expr_char(check: char) -> bool {
    check.is_digit(10) || check == ','
}

pub struct SumState {
    enabled: bool,
    sum: i32,
}

impl Default for SumState {
    fn default() -> Self {
        Self {
            enabled: true,
            sum: 0,
        }
    }
}

impl SumState {
    pub fn sum(&self) -> i32 {
        self.sum
    }

    pub fn add(&mut self, add: i32) {
        if self.enabled {
            self.sum += add
        }
    }

    pub fn enable(&mut self) {
        self.enabled = true
    }

    pub fn disable(&mut self) {
        self.enabled = false
    }
}

pub enum Instruction {
    Mul(i32, i32),
    Do,
    Dont,
}

impl FromStr for Instruction {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("mul(") && s.ends_with(")") {
            let operands: Vec<i32> = s[4..s.len() - 1]
                .split(",")
                .filter_map(|op| op.parse::<i32>().ok())
                .collect();
            if operands.len() == 2 {
                return Ok(Instruction::Mul(operands[0], operands[1]));
            }
        } else if s.starts_with("don't(") && s.ends_with(")") {
            return Ok(Instruction::Dont);
        } else if s.starts_with("do(") && s.ends_with(")") {
            return Ok(Instruction::Do);
        }
        Err(())
    }
}

impl Instruction {
    pub fn starts_with_op(reading: &str) -> Option<usize> {
        if reading.starts_with("mul(") {
            Some(4)
        } else if reading.starts_with("don't(") {
            Some(6)
        } else if reading.starts_with("do(") {
            Some(3)
        } else {
            None
        }
    }

    pub fn eval(&self, sum: &mut SumState) {
        match self {
            Instruction::Mul(l, r) => sum.add(l * r),
            Instruction::Do => sum.enable(),
            Instruction::Dont => sum.disable(),
        }
    }
}
