use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::Read,
};

use slotmap::{new_key_type, SlotMap};

fn main() {
    let mut file = File::open("data/test_x_o").expect("Open data");
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
        let node_coords: Vec<_> = self
            .vals
            .iter() /*.map(|key| ctx[*key])*/
            .map(|node| {
                let (x, y) = self.coords[node];
                (x + 2, y + 2)
            })
            .collect();

        // Coords that belong to this garden (helps when creating the walk around path
        let my_coords = node_coords
            .iter()
            .map(|(x, y)| (*x, *y))
            .collect::<HashSet<(usize, usize)>>();

        let mut buffer_coords = HashSet::new();

        for (x, y) in node_coords {
            for dx in -1..=1 {
                let curr_x = x as isize + dx;
                for dy in -1..=1 {
                    let curr_y = y as isize + dy;

                    if !my_coords.contains(&(curr_x as usize, curr_y as usize)) {
                        buffer_coords.insert((curr_x as usize, curr_y as usize));
                    }
                }
            }
        }

        let mut corners = 0;

        buffer_coords.iter().for_each(|(x, y)| {
            let up = (*x, *y - 1);
            let down = (*x, *y + 1);
            let left = (*x - 1, *y);
            let right = (*x + 1, *y);
            let directions = [up, down, left, right];

            let matching = my_coords
                .iter()
                .filter(|coord| directions.contains(*coord))
                .count();

            let matching_ignore = buffer_coords
                .iter()
                .filter(|coord| directions.contains(*coord))
                .count();

            if matching == 0 || matching == 2 {
                corners += 1;
            } else if matching == 4 {
                corners += 4
            }
        });

        corners
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

#[cfg(test)]
mod tests {
    use std::{fs::File, io::Read};

    use crate::GardenGroups;
    #[test]
    fn test_1() {
        let mut file = File::open("data/test").expect("Open data");
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .expect("Failed to write to buffer");
        let chars: Vec<Vec<char>> = buf
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect();
        let garden_groups = GardenGroups::from_chars(&chars);

        let sides_sum: usize = garden_groups
            .get_gardens()
            .iter()
            .map(|garden| garden.area() * garden.num_sides(&garden_groups.graph))
            .sum();

        assert_eq!(sides_sum, 80)
    }

    #[test]
    /*
    EEEEE
    EXXXX
    EEEEE
    EXXXX
    EEEEE
    */
    fn test_2() {
        let mut file = File::open("data/test2").expect("Open data");
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .expect("Failed to write to buffer");
        let chars: Vec<Vec<char>> = buf
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect();
        let garden_groups = GardenGroups::from_chars(&chars);

        let sides_sum: usize = garden_groups
            .get_gardens()
            .iter()
            .map(|garden| garden.area() * garden.num_sides(&garden_groups.graph))
            .sum();

        assert_eq!(sides_sum, 236)
    }

    #[test]
    fn test_3() {
        let mut file = File::open("data/test3").expect("Open data");
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .expect("Failed to write to buffer");
        let chars: Vec<Vec<char>> = buf
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect();
        let garden_groups = GardenGroups::from_chars(&chars);

        let sides_sum: usize = garden_groups
            .get_gardens()
            .iter()
            .map(|garden| garden.area() * garden.num_sides(&garden_groups.graph))
            .sum();

        assert_eq!(sides_sum, 368)
    }

    #[test]
    fn x_o_test() {
        let mut file = File::open("data/test_x_o").expect("Open data");
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .expect("Failed to write to buffer");
        let chars: Vec<Vec<char>> = buf
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect();
        let garden_groups = GardenGroups::from_chars(&chars);

        let sides_sum: usize = garden_groups
            .get_gardens()
            .iter()
            .map(|garden| garden.area() * garden.num_sides(&garden_groups.graph))
            .sum();

        assert_eq!(sides_sum, 436)
    }

    #[test]
    fn test_big_example() {
        let mut file = File::open("data/big_example").expect("Open data");
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .expect("Failed to write to buffer");
        let chars: Vec<Vec<char>> = buf
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect();
        let garden_groups = GardenGroups::from_chars(&chars);

        let sides_sum: usize = garden_groups
            .get_gardens()
            .iter()
            .map(|garden| garden.area() * garden.num_sides(&garden_groups.graph))
            .sum();

        assert_eq!(sides_sum, 1206)
    }
}
