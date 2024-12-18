use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
    fs::File,
    io::Read,
};

pub const SIZE: u16 = 70;
pub const TAKE: usize = 1024;

fn main() {
    let mut file = File::open("data/input").expect("Open data");
    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .expect("Failed to write to buffer");

    let obstacles: HashSet<(u16, u16)> = buf
        .lines()
        .filter_map(|line| {
            let lines = line.split(',').collect::<Vec<_>>();
            let x = lines[0].parse::<u16>().ok()?;
            let y = lines[1].parse::<u16>().ok()?;
            Some((x, y))
        })
        .take(TAKE)
        .collect();

    if let Some(path) = astar((0, 0), (SIZE, SIZE), &obstacles) {
        for y in 0..=SIZE {
            for x in 0..=SIZE {
                if obstacles.contains(&(x, y)) {
                    print!("#");
                } else if path.contains(&(x, y)) {
                    print!("O");
                } else {
                    print!(".");
                }
            }
            println!();
        }
        println!("Path length: {}", path.len() - 1);
    } else {
        println!("No path found");
    }

    let mut take = TAKE;
    loop {
        let obstacles: Vec<(u16, u16)> = buf
            .lines()
            .filter_map(|line| {
                let lines = line.split(',').collect::<Vec<_>>();
                let x = lines[0].parse::<u16>().ok()?;
                let y = lines[1].parse::<u16>().ok()?;
                Some((x, y))
            })
            .take(take)
            .collect();
        let new_addition = obstacles[take - 1];
        if let None = astar(
            (0, 0),
            (SIZE, SIZE),
            &obstacles.into_iter().collect::<HashSet<_>>(),
        ) {
            println!("Blocking point at {:?}", new_addition);
            break;
        }

        take += 1;
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    position: (u16, u16),
    cost: u16,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn heuristic(a: (u16, u16), b: (u16, u16)) -> u16 {
    ((a.0 as i16 - b.0 as i16).abs() + (a.1 as i16 - b.1 as i16).abs()) as u16
}

fn astar(
    start: (u16, u16),
    goal: (u16, u16),
    obstacles: &HashSet<(u16, u16)>,
) -> Option<Vec<(u16, u16)>> {
    let mut open_set = BinaryHeap::new();
    let mut came_from = HashMap::new();
    let mut g_score: HashMap<(u16, u16), u16> = HashMap::new();
    let mut f_score: HashMap<(u16, u16), u16> = HashMap::new();

    g_score.insert(start, 0);
    f_score.insert(start, heuristic(start, goal));
    open_set.push(Node {
        position: start,
        cost: f_score[&start],
    });

    while let Some(Node { position, .. }) = open_set.pop() {
        if position == goal {
            let mut path = vec![position];
            let mut current = position;
            while let Some(&prev) = came_from.get(&current) {
                path.push(prev);
                current = prev;
            }
            path.reverse();
            return Some(path);
        }

        let neighbors = [
            (position.0.wrapping_sub(1), position.1),
            (position.0 + 1, position.1),
            (position.0, position.1.wrapping_sub(1)),
            (position.0, position.1 + 1),
        ];

        for &neighbor in &neighbors {
            if neighbor.0 > SIZE || neighbor.1 > SIZE || obstacles.contains(&neighbor) {
                continue;
            }

            let tentative_g_score = g_score.get(&position).unwrap_or(&u16::MAX) + 1;

            if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&u16::MAX) {
                came_from.insert(neighbor, position);
                g_score.insert(neighbor, tentative_g_score);
                f_score.insert(neighbor, tentative_g_score + heuristic(neighbor, goal));

                if !open_set.iter().any(|n| n.position == neighbor) {
                    open_set.push(Node {
                        position: neighbor,
                        cost: f_score[&neighbor],
                    });
                }
            }
        }
    }

    None
}
