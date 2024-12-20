use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap, HashSet},
    fs::File,
    io::Read,
};

fn main() {
    let mut file = File::open("data/input").expect("Open data");
    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .expect("Failed to write to buffer");

    let mut player = (0, 0);
    let mut goal = (0, 0);

    let height = buf.lines().count();
    let width = buf.lines().next().unwrap().chars().count();

    println!("{height}x{width} grid");

    let obstacles: HashSet<(usize, usize)> = buf
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(|(x, charcode)| match charcode {
                    '#' => Some((x, y)),
                    'S' => {
                        player = (x, y);
                        None
                    }
                    'E' => {
                        goal = (x, y);
                        None
                    }
                    _ => None,
                })
                .collect::<Vec<_>>()
        })
        .collect();

    let baseline = astar(player, goal, &obstacles, width, height)
        .expect("There should be a solution to a default maze");
    let baseline_score = baseline.len() - 1;

    println!("Score to beat {baseline_score}");

    let mut cheats: HashMap<usize, usize> = HashMap::new();
    for i in 0..baseline.len() {
        for j in (i + 1)..baseline.len() {
            let dist = manhattan_dist(baseline[i], baseline[j]);
            if dist < 21 && (j - i) > dist {
                *cheats.entry((j - i) - dist).or_default() += 1;
            }
        }
    }

    let count: usize = cheats
        .iter()
        .filter(|(picosec, _)| *picosec >= &100)
        .map(|(_, combos)| combos)
        .sum();

    println!("Number of cheated scores that beat by at least 100 {count}");
}

#[derive(Copy, Clone, Eq, PartialEq)]
struct Node {
    position: (usize, usize),
    cost: usize,
}

pub fn manhattan_dist(a: (usize, usize), b: (usize, usize)) -> usize {
    let (x1, y1) = (a.0 as isize, a.1 as isize);
    let (x2, y2) = (b.0 as isize, b.1 as isize);

    ((x2 - x1).abs() + (y2 - y1).abs()) as usize
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

fn heuristic(a: (usize, usize), b: (usize, usize)) -> usize {
    ((a.0 as isize - b.0 as isize).abs() + (a.1 as isize - b.1 as isize).abs()) as usize
}

fn astar(
    start: (usize, usize),
    goal: (usize, usize),
    obstacles: &HashSet<(usize, usize)>,
    width: usize,
    height: usize,
) -> Option<Vec<(usize, usize)>> {
    let mut open_set = BinaryHeap::new();
    let mut came_from = HashMap::new();
    let mut g_score: HashMap<(usize, usize), usize> = HashMap::new();
    let mut f_score: HashMap<(usize, usize), usize> = HashMap::new();

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
            if neighbor.0 > width || neighbor.1 > height || obstacles.contains(&neighbor) {
                continue;
            }

            let tentative_g_score = g_score.get(&position).unwrap_or(&usize::MAX) + 1;

            if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&usize::MAX) {
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
