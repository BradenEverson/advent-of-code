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

    let (topographic_map, start_points) = TopographicMap::new(&chars);
    let trails_sum: usize = start_points
        .iter()
        .filter_map(|start| topographic_map.generate_trailhead(*start))
        .map(|trail| trail.score)
        .sum();

    println!("Total Scores of all Trails: {trails_sum}")
}

new_key_type! {
    pub struct GraphKey;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(usize)]
pub enum Direction {
    Up,
    Left,
    Right,
    Down,
}

#[derive(Default, Clone, Copy)]
pub struct ElevationNode {
    pub elevation: i8,
    directions: [Option<GraphKey>; 4],
}

impl ElevationNode {
    pub fn from_elevation(elevation: i8) -> Self {
        Self {
            elevation,
            directions: [None; 4],
        }
    }

    pub fn connect(&mut self, node: GraphKey, direction: Direction) {
        self.directions[direction as usize] = Some(node)
    }

    pub fn elevated_directions(&self, map: &SlotMap<GraphKey, ElevationNode>) -> Vec<GraphKey> {
        self.directions
            .iter()
            .filter_map(|direction| *direction)
            .filter(|direction| map[*direction].elevation == self.elevation + 1)
            .collect()
    }
}

#[derive(Default, Debug)]
pub struct Trail {
    pub score: usize,
}

pub struct TopographicMap {
    elevations: SlotMap<GraphKey, ElevationNode>,
}

impl TopographicMap {
    pub fn new(chars: &Vec<Vec<char>>) -> (Self, Vec<GraphKey>) {
        let mut elevation_map = SlotMap::default();
        let mut start_points = vec![];
        let keys: Vec<Vec<GraphKey>> = chars
            .iter()
            .map(|row| {
                row.iter()
                    .map(|character| format!("{}", character).parse::<i8>().unwrap_or(-1))
                    .map(|elevation| {
                        let elevation_node = ElevationNode::from_elevation(elevation);
                        let key = elevation_map.insert(elevation_node);

                        if elevation == 0 {
                            start_points.push(key);
                        }

                        key
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        for (row_idx, row) in keys.iter().enumerate() {
            for (col_idx, &key) in row.iter().enumerate() {
                if row_idx > 0 {
                    let up_key = keys[row_idx - 1][col_idx];
                    elevation_map[key].connect(up_key, Direction::Up);
                    elevation_map[up_key].connect(key, Direction::Down);
                }
                if col_idx > 0 {
                    let left_key = row[col_idx - 1];
                    elevation_map[key].connect(left_key, Direction::Left);
                    elevation_map[left_key].connect(key, Direction::Right);
                }
            }
        }

        let map = Self {
            elevations: elevation_map,
        };
        (map, start_points)
    }

    pub fn generate_trailhead(&self, begin: GraphKey) -> Option<Trail> {
        let mut curr_trail = Trail::default();
        let mut seen = HashSet::new();
        self.get_all_trailends(begin, &mut curr_trail, &mut seen);
        if curr_trail.score == 0 {
            None
        } else {
            Some(curr_trail)
        }
    }

    fn get_all_trailends(
        &self,
        from: GraphKey,
        builder: &mut Trail,
        seen_peaks: &mut HashSet<GraphKey>,
    ) {
        let curr_node = self.elevations[from];
        let valid_directions = curr_node.elevated_directions(&self.elevations);

        valid_directions.iter().for_each(|node| {
            if self.elevations[*node].elevation == 9 && !seen_peaks.contains(node) {
                // Comment or uncomment this line for part 2 lol
                seen_peaks.insert(*node);
                builder.score += 1
            } else {
                self.get_all_trailends(*node, builder, seen_peaks);
            }
        });
    }
}
