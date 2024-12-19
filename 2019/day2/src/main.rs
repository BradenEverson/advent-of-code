use std::{fs::File, io::Read};

fn main() {
    let mut file = File::open("data/input").expect("Open data");
    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .expect("Failed to write to buffer");
    let mut vals = buf
        .split(",")
        .filter_map(|val| val.parse::<usize>().ok())
        .collect::<Vec<_>>();

    vals[1] = 12;
    vals[2] = 2;
    let mut intcode = IntcodeMachine::new(vals.clone());
    while let Some(()) = intcode.tick() {}
    println!("Final value of position 0: {}", intcode.instructions[0]);

    for noun in 0..=100 {
        for verb in 0..=100 {
            vals[1] = noun;
            vals[2] = verb;
            let mut intcode = IntcodeMachine::new(vals.clone());
            while let Some(()) = intcode.tick() {}
            if intcode.instructions[0] == 19690720 {
                println!("{}", noun * 100 + verb);
                break;
            }
        }
    }
}

pub struct IntcodeMachine {
    instructions: Vec<usize>,
    position: usize,
}

impl IntcodeMachine {
    pub fn new(instructions: Vec<usize>) -> Self {
        Self {
            instructions,
            position: 0,
        }
    }

    pub fn tick(&mut self) -> Option<()> {
        let op =
            self.instructions[self.position].to_instruction(&self.instructions[self.position..]);
        let res = self.process(op);
        self.position += 4;
        res
    }

    pub fn process(&mut self, op: InstructionCode) -> Option<()> {
        match op {
            InstructionCode::Add(l, r, to) => {
                self.instructions[to] = self.instructions[l] + self.instructions[r];
                Some(())
            }
            InstructionCode::Mul(l, r, to) => {
                self.instructions[to] = self.instructions[l] * self.instructions[r];
                Some(())
            }
            InstructionCode::Done => None,
        }
    }
}

#[derive(Debug)]
pub enum InstructionCode {
    Add(usize, usize, usize),
    Mul(usize, usize, usize),
    Done,
}

pub trait Instruction {
    fn to_instruction(&self, instructions: &[usize]) -> InstructionCode;
}

impl Instruction for usize {
    fn to_instruction(&self, instructions: &[usize]) -> InstructionCode {
        match *self {
            1 => InstructionCode::Add(instructions[1], instructions[2], instructions[3]),
            2 => InstructionCode::Mul(instructions[1], instructions[2], instructions[3]),
            99 => InstructionCode::Done,
            _ => unreachable!("Invalid Instruction"),
        }
    }
}
