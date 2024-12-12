use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::Read,
};

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
    let garden_groups = GardenGroups::from_chars(&chars);
    let totals: usize = garden_groups
        .get_gardens()
        .iter()
        .map(|garden| garden.area() * garden.perimeter(&garden_groups.graph))
        .sum();

    let sides_sum: usize = garden_groups
        .get_gardens()
        .iter()
        .map(|garden| garden.area() * garden.num_sides(&garden_groups.graph))
        .sum();

    println!("Total price of all gardens is ${}", totals);
    println!(
        "Total price of all gardens with respect to number of sides ${}",
        sides_sum
    );
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

impl Direction {
    pub const VARIANTS: [Direction; 4] = [
        Direction::Up,
        Direction::Left,
        Direction::Right,
        Direction::Down,
    ];
}

#[derive(Debug, Clone, Copy)]
pub struct GardenNode {
    pub val: char,
    directions: [Option<GraphKey>; 4],
}

impl PartialEq for GardenNode {
    fn eq(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

impl From<char> for GardenNode {
    fn from(value: char) -> Self {
        Self {
            val: value,
            directions: [None; 4],
        }
    }
}

impl GardenNode {
    pub fn connect(&mut self, node: GraphKey, direction: Direction) {
        self.directions[direction as usize] = Some(node)
    }
}

#[derive(Debug, Default)]
pub struct Garden {
    vals: Vec<GraphKey>,
    coords: HashMap<GraphKey, (usize, usize)>,
}

impl Garden {
    pub fn from_vec(vals: Vec<GraphKey>, coords: HashMap<GraphKey, (usize, usize)>) -> Self {
        Self { vals, coords }
    }

    pub fn num_sides(&self, ctx: &SlotMap<GraphKey, GardenNode>) -> usize {
        let mut horizontal_edges: HashSet<(usize, usize)> = HashSet::new(); // (y, x)
        let mut vertical_edges: HashSet<(usize, usize)> = HashSet::new(); // (x, y)

        for &key in &self.vals {
            let &(x, y) = self.coords.get(&key).unwrap();
            let node = ctx[key];

            if let Some(up_key) = node.directions[Direction::Up as usize] {
                if !self.vals.contains(&up_key) || ctx[up_key].val != node.val {
                    horizontal_edges.insert((y, x));
                }
            } else {
                horizontal_edges.insert((y, x));
            }

            if let Some(down_key) = node.directions[Direction::Down as usize] {
                if !self.vals.contains(&down_key) || ctx[down_key].val != node.val {
                    horizontal_edges.insert((y + 1, x));
                }
            } else {
                horizontal_edges.insert((y + 1, x));
            }

            if let Some(left_key) = node.directions[Direction::Left as usize] {
                if !self.vals.contains(&left_key) || ctx[left_key].val != node.val {
                    vertical_edges.insert((x, y));
                }
            } else {
                vertical_edges.insert((x, y));
            }

            if let Some(right_key) = node.directions[Direction::Right as usize] {
                if !self.vals.contains(&right_key) || ctx[right_key].val != node.val {
                    vertical_edges.insert((x + 1, y));
                }
            } else {
                vertical_edges.insert((x + 1, y));
            }
        }

        let hor_map: HashMap<usize, Vec<usize>> =
            horizontal_edges
                .iter()
                .fold(HashMap::new(), |mut acc, &(y, x)| {
                    acc.entry(y).or_default().push(x);
                    acc
                });

        let ver_map: HashMap<usize, Vec<usize>> =
            vertical_edges
                .iter()
                .fold(HashMap::new(), |mut acc, &(x, y)| {
                    acc.entry(x).or_default().push(y);
                    acc
                });

        fn count_groups(edge_map: &HashMap<usize, Vec<usize>>) -> usize {
            edge_map
                .values()
                .map(|positions| {
                    let mut sorted = positions.clone();
                    sorted.sort_unstable();
                    let mut groups = 0;
                    let mut prev = None;

                    for &pos in &sorted {
                        if let Some(prev_pos) = prev {
                            if pos != prev_pos + 1 {
                                groups += 1;
                            }
                        } else {
                            groups += 1;
                        }
                        prev = Some(pos);
                    }

                    groups
                })
                .sum()
        }

        let horizontal_groups = count_groups(&hor_map);
        let vertical_groups = count_groups(&ver_map);

        horizontal_groups + vertical_groups
    }

    pub fn area(&self) -> usize {
        self.vals.len()
    }

    pub fn perimeter(&self, ctx: &SlotMap<GraphKey, GardenNode>) -> usize {
        self.vals
            .iter()
            .map(|node| {
                let curr_node = ctx[*node];
                let mut curr_sum = 0;
                for direction in curr_node.directions {
                    if let Some(val) = direction {
                        if ctx[val].val != curr_node.val {
                            curr_sum += 1
                        }
                    } else {
                        curr_sum += 1
                    }
                }

                curr_sum
            })
            .sum()
    }
}

#[derive(Default, Debug)]
pub struct GardenGroups {
    pub graph: SlotMap<GraphKey, GardenNode>,
    pub coords: HashMap<GraphKey, (usize, usize)>,
}

impl GardenGroups {
    pub fn from_chars(from: &Vec<Vec<char>>) -> Self {
        let mut map = SlotMap::default();
        let mut coords = HashMap::new();
        let keys: Vec<Vec<GraphKey>> = from
            .iter()
            .map(|row| {
                row.iter()
                    .map(|character| {
                        let node: GardenNode = (*character).into();
                        map.insert(node)
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        for (row_idx, row) in keys.iter().enumerate() {
            for (col_idx, &key) in row.iter().enumerate() {
                coords.insert(key, (col_idx, row_idx));
                if row_idx > 0 {
                    let up_key = keys[row_idx - 1][col_idx];
                    map[key].connect(up_key, Direction::Up);
                    map[up_key].connect(key, Direction::Down);
                }
                if col_idx > 0 {
                    let left_key = row[col_idx - 1];
                    map[key].connect(left_key, Direction::Left);
                    map[left_key].connect(key, Direction::Right);
                }
            }
        }

        Self { graph: map, coords }
    }

    pub fn get_gardens(&self) -> Vec<Garden> {
        let mut visited: HashSet<GraphKey> = HashSet::new();
        let mut gardens = vec![];

        for key in self.graph.keys() {
            if !visited.contains(&key) {
                let garden = self.get_one_garden(key, &mut visited, &self.coords);
                gardens.push(garden);
            }
        }

        gardens
    }

    fn get_one_garden(
        &self,
        start: GraphKey,
        visited: &mut HashSet<GraphKey>,
        coords: &HashMap<GraphKey, (usize, usize)>,
    ) -> Garden {
        let mut stack = vec![start];
        let mut garden_nodes = vec![];
        let mut points = HashMap::new();

        while let Some(current) = stack.pop() {
            if visited.contains(&current) {
                continue;
            }

            visited.insert(current);
            garden_nodes.push(current);
            points.insert(current, coords[&current]);

            let current_node = self.graph[current];

            for &direction in &Direction::VARIANTS {
                if let Some(neighbor) = current_node.directions[direction as usize] {
                    if !visited.contains(&neighbor) && self.graph[neighbor].val == current_node.val
                    {
                        stack.push(neighbor);
                    }
                }
            }
        }

        Garden::from_vec(garden_nodes, points)
    }
}

