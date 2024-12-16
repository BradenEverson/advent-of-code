use std::collections::{HashSet, VecDeque};
use std::{fs::File, io::Read};

fn main() {
    let mut file = File::open("data/input").expect("Failed to open file");
    let mut buf = String::new();
    file.read_to_string(&mut buf).expect("Failed to read file");
    let chars: Vec<Vec<char>> = buf.lines().map(|line| line.chars().collect()).collect();

    let mut player_pos = None;
    let mut grid = vec![];
    for (row_idx, row) in chars.iter().enumerate() {
        let mut grid_row = vec![];
        for (col_idx, &cell) in row.iter().enumerate() {
            let grid_type = match cell {
                '#' => GridType::Obstacle,
                'E' => GridType::Goal,
                'S' => {
                    player_pos = Some((row_idx, col_idx));
                    GridType::Empty
                }
                _ => GridType::Empty,
            };
            grid_row.push(grid_type);
        }
        grid.push(grid_row);
    }

    let player_pos = player_pos.expect("Player starting position ('S') not found");

    let directions = [
        (0, -1, Direction::North), // North
        (1, 0, Direction::East),   // East
        (0, 1, Direction::South),  // South
        (-1, 0, Direction::West),  // West
    ];

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut scores = vec![];

    queue.push_back(Player {
        curr_pos: player_pos,
        curr_direction: Direction::East,
        steps: 0,
    });
    visited.insert((player_pos, Direction::East));

    while let Some(player) = queue.pop_front() {
        let (row, col) = player.curr_pos;
        if grid[row][col] == GridType::Goal {
            scores.push(player.steps as usize);
            continue;
        }

        for &(dx, dy, dir) in &directions {
            let new_row = row.wrapping_add(dy as usize);
            let new_col = col.wrapping_add(dx as usize);

            if new_row < grid.len()
                && new_col < grid[new_row].len()
                && grid[new_row][new_col] != GridType::Obstacle
            {
                let mut new_player = player.clone();
                if dir != player.curr_direction {
                    new_player.steps += 1000; // Turning costs 1000
                }
                new_player.steps += 1; // Moving forward costs 1
                new_player.curr_pos = (new_row, new_col);
                new_player.curr_direction = dir;

                if !visited.contains(&(new_player.curr_pos, new_player.curr_direction)) {
                    visited.insert((new_player.curr_pos, new_player.curr_direction));
                    queue.push_back(new_player);
                }
            }
        }
    }

    if let Some(&lowest_score) = scores.iter().min() {
        println!("Lowest raindeer score: {}", lowest_score);
    } else {
        println!("No valid path to the goal found.");
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum GridType {
    Obstacle,
    Goal,
    Empty,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Direction {
    North,
    East,
    South,
    West,
}

#[derive(Clone, Copy, Debug)]
struct Player {
    curr_pos: (usize, usize),
    curr_direction: Direction,
    steps: i32,
}

