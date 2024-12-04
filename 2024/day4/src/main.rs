use std::{fs::File, io::Read};

use slotmap::{new_key_type, SlotMap};

fn main() {
    let mut file = File::open("data/input").expect("Open data");
    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .expect("Failed to write to buffer");
    let chars: Vec<Vec<char>> = buf
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let char_graph = WordSearchGraph::from_char_grid(chars);
    let count = char_graph.find_all_word_instances(&['M', 'A', 'S']) / 2;
    println!("X-MAS found: {count}")
}

new_key_type! {
    pub struct WordSearchKey;
}

#[repr(usize)]
#[derive(Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
    UpperDiagonalLeft,
    UpperDiagonalRight,
    LowerDiagonalLeft,
    LowerDiagonalRight,
}

#[derive(Debug, Clone, Copy)]
pub struct CharNode {
    val: char,
    directions: [Option<WordSearchKey>; 8],
}

impl CharNode {
    pub fn new(val: char) -> Self {
        Self {
            val,
            directions: [None; 8],
        }
    }

    pub fn register_direction(&mut self, direction: Direction, node: WordSearchKey) {
        self.directions[direction as usize] = Some(node)
    }

    pub fn value(&self) -> char {
        self.val
    }

    pub fn get(&self, direction: Direction) -> Option<WordSearchKey> {
        self.directions[direction as usize]
    }
}

#[derive(Debug)]
pub struct WordSearchGraph {
    nodes: SlotMap<WordSearchKey, CharNode>,
}

impl WordSearchGraph {
    pub fn from_char_grid(grid: Vec<Vec<char>>) -> Self {
        let mut graph = SlotMap::default();
        let key_grid: Vec<Vec<WordSearchKey>> = grid
            .iter()
            .map(|row| {
                row.iter()
                    .map(|val| {
                        let node = CharNode::new(*val);
                        graph.insert(node)
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        for y in 0..key_grid.len() {
            for x in 0..key_grid[y].len() {
                let curr_key = key_grid[y][x];
                let curr_node = &mut graph[curr_key];

                if y > 0 {
                    curr_node.register_direction(Direction::Up, key_grid[y - 1][x]);
                    if x > 0 {
                        curr_node.register_direction(
                            Direction::UpperDiagonalLeft,
                            key_grid[y - 1][x - 1],
                        );
                    }

                    if x < key_grid[y - 1].len() - 1 {
                        curr_node.register_direction(
                            Direction::UpperDiagonalRight,
                            key_grid[y - 1][x + 1],
                        );
                    }
                }

                if y < key_grid.len() - 1 {
                    curr_node.register_direction(Direction::Down, key_grid[y + 1][x]);
                    if x > 0 {
                        curr_node.register_direction(
                            Direction::LowerDiagonalLeft,
                            key_grid[y + 1][x - 1],
                        );
                    }

                    if x < key_grid[y + 1].len() - 1 {
                        curr_node.register_direction(
                            Direction::LowerDiagonalRight,
                            key_grid[y + 1][x + 1],
                        );
                    }
                }

                if x > 0 {
                    curr_node.register_direction(Direction::Left, key_grid[y][x - 1]);
                }

                if x < key_grid[y].len() - 1 {
                    curr_node.register_direction(Direction::Right, key_grid[y][x + 1]);
                }
            }
        }

        Self { nodes: graph }
    }

    pub fn find_all_word_instances(&self, word: &[char]) -> i32 {
        let search = [
            Direction::UpperDiagonalLeft,
            Direction::UpperDiagonalRight,
            Direction::LowerDiagonalLeft,
            Direction::LowerDiagonalRight,
        ];

        let mut count = 0;
        for (_, char_node) in &self.nodes {
            if char_node.value() == word[0] {
                for direction in search {
                    let mut curr_place = 1;
                    let mut prev_node = *char_node;
                    let mut curr_node = *char_node;
                    while let Some(next_node) = curr_node.get(direction) {
                        prev_node = curr_node;
                        curr_node = self.nodes[next_node];
                        if curr_node.value() == word[curr_place] {
                            curr_place += 1
                        } else {
                            break;
                        }
                        if curr_place == word.len() {
                            // MAS found, now check diagonals for X-mas
                            match direction {
                                Direction::UpperDiagonalLeft | Direction::LowerDiagonalRight => {
                                    if let Some(upper_right) =
                                        prev_node.get(Direction::UpperDiagonalRight)
                                    {
                                        let upper_right = self.nodes[upper_right];
                                        if let Some(lower_left) =
                                            prev_node.get(Direction::LowerDiagonalLeft)
                                        {
                                            let lower_left = self.nodes[lower_left];
                                            if (upper_right.value() == word[0]
                                                && lower_left.value() == word[word.len() - 1])
                                                || (upper_right.value() == word[word.len() - 1]
                                                    && lower_left.value() == word[0])
                                            {
                                                count += 1;
                                            }
                                            break;
                                        }
                                    }
                                }
                                Direction::UpperDiagonalRight | Direction::LowerDiagonalLeft => {
                                    if let Some(upper_left) =
                                        prev_node.get(Direction::UpperDiagonalLeft)
                                    {
                                        let upper_left = self.nodes[upper_left];
                                        if let Some(lower_right) =
                                            prev_node.get(Direction::LowerDiagonalRight)
                                        {
                                            let lower_right = self.nodes[lower_right];
                                            if (upper_left.value() == word[0]
                                                && lower_right.value() == word[word.len() - 1])
                                                || (upper_left.value() == word[word.len() - 1]
                                                    && lower_right.value() == word[0])
                                            {
                                                count += 1;
                                            }
                                            break;
                                        }
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
            }
        }
        count
    }
}
