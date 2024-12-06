use std::{collections::HashSet, fs::File, io::Read};

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

    let grid = ObstacleMap::from_char_vec(&chars).expect("Failed to find player location");

    let valid_positions = grid.find_loop_positions();
    println!("Number of valid positions: {}", valid_positions.len());
}

new_key_type! {
    pub struct PositionKey;
}

pub struct ObstacleMap {
    pub grid: SlotMap<PositionKey, GridEntity>,
    pub player: Player,
    pub key_grid: Vec<Vec<PositionKey>>,
}

impl ObstacleMap {
    pub fn from_char_vec(grid: &Vec<Vec<char>>) -> Option<Self> {
        let mut player_pos = None;
        let mut graph = SlotMap::default();
        let key_grid: Vec<Vec<PositionKey>> = grid
            .iter()
            .map(|row| {
                row.iter()
                    .map(|val| {
                        let node = GridEntity::from_char(*val);
                        let key = graph.insert(node);
                        if *val == '^' {
                            player_pos = Some(key);
                        }
                        key
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        for (row_idx, row) in key_grid.iter().enumerate() {
            for (col_idx, &key) in row.iter().enumerate() {
                if row_idx > 0 {
                    let up_key = key_grid[row_idx - 1][col_idx];
                    graph[key].connect(up_key, Direction::Up);
                    graph[up_key].connect(key, Direction::Down);
                }
                if col_idx > 0 {
                    let left_key = row[col_idx - 1];
                    graph[key].connect(left_key, Direction::Left);
                    graph[left_key].connect(key, Direction::Right);
                }
            }
        }

        Some(Self {
            grid: graph,
            player: Player::new(player_pos?),
            key_grid,
        })
    }

    pub fn find_loop_positions(&self) -> Vec<PositionKey> {
        let mut valid_positions = Vec::new();
        for (_, row) in self.key_grid.iter().enumerate() {
            for (_, &key) in row.iter().enumerate() {
                if self.grid[key].grid_type == GridType::Obstacle || key == self.player.curr_pos {
                    continue;
                }

                let mut temp_grid = self.clone();
                temp_grid.grid[key].grid_type = GridType::Obstacle;

                if temp_grid.simulate_until_loop() {
                    valid_positions.push(key);
                }
            }
        }
        valid_positions
    }

    pub fn simulate_until_loop(&mut self) -> bool {
        let mut visited = HashSet::new();

        while self.advance() {
            let state = (self.player.curr_pos, self.player.curr_direction);
            if !visited.insert(state) {
                return true;
            }
        }
        false
    }

    pub fn advance(&mut self) -> bool {
        let curr_player_position = &self.grid[self.player.curr_pos];
        if let Some(entity) =
            curr_player_position.directions[self.player.get_curr_direction() as usize]
        {
            let moving_into = &self.grid[entity];
            match moving_into.grid_type {
                GridType::Empty => self.player.move_into(entity),
                GridType::Obstacle => self.player.turn_right(),
            }
            true
        } else {
            false
        }
    }
}

impl Clone for ObstacleMap {
    fn clone(&self) -> Self {
        Self {
            grid: self.grid.clone(),
            player: self.player.clone(),
            key_grid: self.key_grid.clone(),
        }
    }
}

#[derive(Clone)]
pub struct GridEntity {
    pub grid_type: GridType,
    pub directions: [Option<PositionKey>; 4],
}

impl GridEntity {
    pub fn from_char(char_code: char) -> Self {
        let grid_type = match char_code {
            '#' => GridType::Obstacle,
            _ => GridType::Empty,
        };

        Self {
            grid_type,
            directions: [None; 4],
        }
    }

    pub fn connect(&mut self, grid_entity: PositionKey, direction: Direction) {
        self.directions[direction as usize] = Some(grid_entity);
    }
}

#[derive(Clone, PartialEq)]
pub enum GridType {
    Obstacle,
    Empty,
}

#[derive(Clone)]
pub struct Player {
    curr_pos: PositionKey,
    curr_direction: Direction,
    steps: i32,
}

impl Player {
    pub fn new(curr_pos: PositionKey) -> Self {
        Self {
            curr_pos,
            curr_direction: Direction::Up,
            steps: 0,
        }
    }

    pub fn get_curr_direction(&self) -> Direction {
        self.curr_direction
    }

    pub fn turn_right(&mut self) {
        self.curr_direction = self.curr_direction.turn_right()
    }

    pub fn move_into(&mut self, new_pos: PositionKey) {
        self.steps += 1;
        self.curr_pos = new_pos
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(usize)]
pub enum Direction {
    Up,
    Left,
    Right,
    Down,
}

impl Direction {
    pub fn turn_right(&self) -> Direction {
        match *self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}
