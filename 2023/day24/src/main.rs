use std::{fs::File, io::Read, ops};

fn main() {
    let mut file = File::open("data/test").expect("Open data");
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

    println!("{hailstones:?}")
}

impl ops::Add<Velocity> for Position {
    type Output = Position;
    fn add(self, rhs: Velocity) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
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

impl Hailstone {
    pub fn tick(&mut self) {
        self.position = self.position + self.velocity
    }

    pub fn position(&self) -> Position {
        self.position
    }
}

pub struct Storm {
    pub hailstones: Vec<Hailstone>,
}

impl Storm {
    pub fn new(hailstones: Vec<Hailstone>) -> Self {
        Self { hailstones }
    }
}
