use crate::{part1::part1, part2::part2};

mod part1;
mod part2;

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Value {
    Number(u64),
    Space,
}

impl Value {
    pub fn val(&self) -> u64 {
        match self {
            Value::Number(u) => *u,
            Value::Space => panic!("Bad"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Op {
    Plus,
    Mul,
    Space,
}
