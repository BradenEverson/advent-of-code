use std::{fmt::Display, fs::File, io::Read};

pub const TEST_MIN: u64 = 7;
pub const TEST_MAX: u64 = 27;

pub const MIN: u64 = 200000000000000;
pub const MAX: u64 = 400000000000000;

fn main() {
    let mut file = File::open("data/input").expect("Open data");
    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .expect("Failed to write to buffer");
    let hailstones: Vec<_> = buf
        .lines()
        .map(|line| {
            let position_velocity_split = line.split("@").collect::<Vec<_>>();
            let position: Vec<f32> = position_velocity_split[0]
                .split(",")
                .into_iter()
                .filter_map(|pos_val| pos_val.trim().parse::<f32>().ok())
                .collect();
            let position = Position(position[0], position[1], position[2]);
            let velocity: Vec<f32> = position_velocity_split[1]
                .split(",")
                .into_iter()
                .filter_map(|pos_val| pos_val.trim().parse::<f32>().ok())
                .collect();

            let velocity = Velocity(velocity[0], velocity[1], velocity[2]);
            Hailstone { position, velocity }
        })
        .collect();

    let storm = Storm::new(hailstones);

    println!("{} collisions", storm.collisions_in_range(MIN, MAX))
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Position(pub f32, pub f32, pub f32);
#[derive(Default, Debug, Clone, Copy)]
pub struct Velocity(pub f32, pub f32, pub f32);

#[derive(Default, Debug, Clone, Copy)]
pub struct Hailstone {
    pub position: Position,
    pub velocity: Velocity,
}

impl Display for Hailstone {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, {}, {} @ {}, {}, {}",
            self.position.0,
            self.position.1,
            self.position.2,
            self.velocity.0,
            self.velocity.1,
            self.velocity.2,
        )
    }
}

impl Hailstone {
    pub fn collision(&self, other: &Hailstone) -> Option<Position> {
        let p1 = &self.position;
        let v1 = &self.velocity;
        let p2 = &other.position;
        let v2 = &other.velocity;

        let det = v1.0 * v2.1 - v1.1 * v2.0;

        if det.abs() < 1e-12 {
            return None;
        }

        let dx = p2.0 - p1.0;
        let dy = p2.1 - p1.1;

        let t = (dx * v2.1 - dy * v2.0) / det;
        let s = (dx * v1.1 - dy * v1.0) / det;

        if t >= 0.0 && s >= 0.0 {
            let x = p1.0 + v1.0 * t;
            let y = p1.1 + v1.1 * t;
            let z = p1.2 + v1.2 * t;

            // ignore z for now

            return Some(Position(x, y, z));
        }

        None
    }
}

pub struct Storm {
    pub hailstones: Vec<Hailstone>,
}

impl Storm {
    pub fn new(hailstones: Vec<Hailstone>) -> Self {
        Self { hailstones }
    }

    pub fn collisions_in_range(&self, min: u64, max: u64) -> u64 {
        let min = min as f32;
        let max = max as f32;

        let mut collisions = 0;
        for i in 0..self.hailstones.len() - 1 {
            let stone_a = self.hailstones[i];
            for stone_b in &self.hailstones[i + 1..] {
                if let Some(col) = stone_a.collision(stone_b) {
                    if col.0 >= min && col.0 <= max && col.1 >= min && col.1 <= max {
                        collisions += 1;
                    }
                }
            }
        }

        collisions
    }
}
