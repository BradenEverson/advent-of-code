pub struct EngineGraph {
    pub nodes: Vec<EngineNode>,
}

pub struct EngineNode {
    pub ty: Symbol,
    pub adjacent: Vec<Symbol>,
}

pub enum Symbol {
    Symbol(char),
    Empty,
    Number(i32),
}

fn main() {}
