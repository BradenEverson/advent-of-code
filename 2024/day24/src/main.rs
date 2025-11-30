use std::{collections::HashMap, fs::File, io::Read};

fn main() {
    let mut input_file = File::open("data/input").expect("Failed to open file");
    let mut lines = String::new();

    let mut reading_gates = false;

    let mut graph = GateGraph::default();

    input_file
        .read_to_string(&mut lines)
        .expect("Failed to read file to buffer");

    for line in lines.lines() {
        if line.is_empty() {
            reading_gates = true;
            continue;
        }

        if reading_gates {
            let split: Vec<&str> = line.split(" ").collect();
            let a = split[0];
            let b = split[2];

            let op = split[1];
            let name = split[4];

            match op {
                "OR" => graph.or(name, a, b),
                "AND" => graph.and(name, a, b),
                "XOR" => graph.xor(name, a, b),
                _ => panic!("Improperly Formated Input"),
            }
        } else {
            let split: Vec<&str> = line.split(": ").collect();
            graph.input(split[0], split[1] == "1");
        }
    }

    let mut out: u64 = 0;

    for i in 0..=45 {
        out <<= 1;
        let target = format!("z{:02}", 45 - i);
        let output = graph.eval(&target).expect("Graph not constructed properly");
        if output {
            out |= 1;
        }
    }

    println!("{out}")
}

pub enum Gate<'a> {
    Input(bool),
    Output(&'a str),
    And(&'a str, &'a str),
    Xor(&'a str, &'a str),
    Or(&'a str, &'a str),
}

#[derive(Default)]
pub struct GateGraph<'a> {
    gates: HashMap<&'a str, Gate<'a>>,
}

impl<'a> GateGraph<'a> {
    pub fn input(&mut self, name: &'a str, val: bool) {
        let input = Gate::Input(val);
        self.gates.insert(name, input);
    }

    pub fn output(&mut self, name: &'a str, dep: &'a str) {
        let output = Gate::Output(dep);
        self.gates.insert(name, output);
    }

    pub fn and(&mut self, name: &'a str, a: &'a str, b: &'a str) {
        let and = Gate::And(a, b);
        self.gates.insert(name, and);
    }

    pub fn or(&mut self, name: &'a str, a: &'a str, b: &'a str) {
        let or = Gate::Or(a, b);
        self.gates.insert(name, or);
    }

    pub fn xor(&mut self, name: &'a str, a: &'a str, b: &'a str) {
        let xor = Gate::Xor(a, b);
        self.gates.insert(name, xor);
    }

    pub fn eval(&self, name: &str) -> Option<bool> {
        let gate = self.gates.get(name)?;

        let val = match gate {
            Gate::And(a, b) => self.eval(a)? && self.eval(b)?,
            Gate::Or(a, b) => self.eval(a)? || self.eval(b)?,
            Gate::Xor(a, b) => self.eval(a)? ^ self.eval(b)?,
            Gate::Output(a) => self.eval(a)?,
            Gate::Input(b) => *b,
        };

        Some(val)
    }
}
