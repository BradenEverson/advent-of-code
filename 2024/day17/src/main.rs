use std::{fs::File, io::Read};

fn main() {
    let mut file = File::open("data/input").expect("Open data");
    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .expect("Failed to write to buffer");

    let lines = buf
        .lines()
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>();

    let parsed = lines
        .iter()
        .filter_map(|line| line.split(": ").last())
        .collect::<Vec<_>>();

    let a: isize = parsed[0].parse().expect("Parse register value A");
    let b: isize = parsed[1].parse().expect("Parse register value B");
    let c: isize = parsed[2].parse().expect("Parse register value C");

    let instructions: Vec<isize> = parsed[3]
        .split(",")
        .filter_map(|val| val.parse().ok())
        .collect::<Vec<isize>>();

    let mut runtime = ChronospatialRuntime::new(a, b, c, instructions.clone());

    while let Some(_) = runtime.tick() {}
    let stdout = runtime.get_stdout();

    println!("{}", stdout);

    // Part 2 Brute Force Lol

    let mut a = 0;
    loop {
        let mut runtime = ChronospatialRuntime::new(a, b, c, instructions.clone());
        while let Some(_) = runtime.tick() {}
        let stdout = runtime.get_stdout();
        if stdout == parsed[3] {
            println!("WE GOT A QUINE with {} in Register A", a);
            break;
        }
        a += 1;
    }
}

pub struct ChronospatialRuntime {
    register_a: isize,
    register_b: isize,
    register_c: isize,

    instruction_pointer: usize,
    instructions: Vec<isize>,
    std_out: Vec<isize>,
}

impl ChronospatialRuntime {
    pub fn new(
        register_a: isize,
        register_b: isize,
        register_c: isize,
        instructions: Vec<isize>,
    ) -> Self {
        Self {
            register_a,
            register_b,
            register_c,
            instructions,

            instruction_pointer: 0,
            std_out: vec![],
        }
    }

    pub fn get_stdout(&self) -> String {
        self.std_out
            .iter()
            .map(|val| val.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }

    pub fn tick(&mut self) -> Option<()> {
        if self.instruction_pointer >= self.instructions.len() - 1 {
            return None;
        }

        let (instruction, val) = (
            self.instructions[self.instruction_pointer],
            self.instructions[self.instruction_pointer + 1],
        );
        self.exec(instruction, val);

        Some(())
    }

    pub fn get_value(&self, value: ComboOperand) -> isize {
        match value {
            ComboOperand::Literal(val) => val,
            ComboOperand::RegA => self.register_a,
            ComboOperand::RegB => self.register_b,
            ComboOperand::RegC => self.register_c,
            ComboOperand::NoOp => 0,
        }
    }

    pub fn exec(&mut self, instruction: isize, value: isize) {
        let val = self.get_value(value.to_combo());
        let literal = value.to_literal();

        match instruction.to_instruction() {
            Instruction::Adv => {
                // integer division of A / 2^l
                let denominator = 2isize.pow(val as u32);
                let result = self.register_a / denominator;

                self.register_a = result;
                self.instruction_pointer += 2;
            }

            Instruction::Bxl => {
                // Bitwise XOR of register B and l
                let result = self.register_b ^ literal;
                self.register_b = result;
                self.instruction_pointer += 2;
            }

            Instruction::Bst => {
                // Combo operand mod 8
                let result = val % 8;
                self.register_b = result;
                self.instruction_pointer += 2;
            }

            Instruction::Jnz => {
                // Jump to Instruction Pointer at l if A is NOT 0
                if self.register_a != 0 {
                    self.instruction_pointer = literal as usize;
                } else {
                    self.instruction_pointer += 2;
                }
            }

            Instruction::Bxc => {
                // Bitwise xor of B and C and stores it in B
                let result = self.register_b ^ self.register_c;
                self.register_b = result;
                self.instruction_pointer += 2;
            }

            Instruction::Out => {
                // Print l % 8 to stdout
                let result = val % 8;
                self.std_out.push(result);
                self.instruction_pointer += 2;
            }

            Instruction::Bdv => {
                // Adv but stored in B register
                let denominator = 2isize.pow(val as u32);
                let result = self.register_a / denominator;

                self.register_b = result;
                self.instruction_pointer += 2;
            }

            Instruction::Cdv => {
                // Adv but stored in C register
                let denominator = 2isize.pow(val as u32);
                let result = self.register_a / denominator;

                self.register_c = result;
                self.instruction_pointer += 2;
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ComboOperand {
    Literal(isize),
    RegA,
    RegB,
    RegC,
    NoOp,
}

impl ComboOperand {
    pub fn from_val(val: isize) -> Option<Self> {
        match val {
            a if (0..=3).contains(&a) => Some(ComboOperand::Literal(a)),
            4 => Some(ComboOperand::RegA),
            5 => Some(ComboOperand::RegB),
            6 => Some(ComboOperand::RegC),
            7 => Some(ComboOperand::NoOp),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Instruction {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl Instruction {
    pub fn from_val(val: isize) -> Option<Self> {
        match val {
            0 => Some(Instruction::Adv),
            1 => Some(Instruction::Bxl),
            2 => Some(Instruction::Bst),
            3 => Some(Instruction::Jnz),
            4 => Some(Instruction::Bxc),
            5 => Some(Instruction::Out),
            6 => Some(Instruction::Bdv),
            7 => Some(Instruction::Cdv),
            _ => None,
        }
    }
}

pub trait MemorySlot {
    fn to_combo(&self) -> ComboOperand;
    fn to_literal(&self) -> isize;
    fn to_instruction(&self) -> Instruction;
}

impl MemorySlot for isize {
    fn to_combo(&self) -> ComboOperand {
        ComboOperand::from_val(*self).unwrap()
    }

    fn to_literal(&self) -> isize {
        *self
    }

    fn to_instruction(&self) -> Instruction {
        Instruction::from_val(*self).unwrap()
    }
}
