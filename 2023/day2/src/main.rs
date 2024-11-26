use std::{error::Error, fs::File, io::Read};

pub const RED: usize = 12;
pub const GREEN: usize = 13;
pub const BLUE: usize = 14;

fn main() {
    let mut input_file = File::open("data/input.in").expect("Failed to open file");
    let mut lines = String::new();

    input_file
        .read_to_string(&mut lines)
        .expect("Failed to read file to buffer");

    let mut games = vec![];
    for line in lines.lines() {
        let game = Game::from_line(line).expect("Failed to create game");
        games.push(game);
    }

    let id_sum: usize = games
        .iter()
        .filter(|game| game.is_game_possible(RED, GREEN, BLUE))
        .map(|game| game.id)
        .sum();

    println!("{id_sum}");

    let power_sum: usize = games.iter().filter_map(|game| game.get_power()).sum();
    println!("{power_sum}")
}

#[derive(Debug)]
pub struct Game {
    pub id: usize,
    pub red: Vec<usize>,
    pub blue: Vec<usize>,
    pub green: Vec<usize>,
}

impl Game {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            red: vec![],
            blue: vec![],
            green: vec![],
        }
    }

    pub fn from_line(line: &str) -> Result<Self, Box<dyn Error>> {
        let substr: Vec<_> = line[5..].split(":").collect();
        let id: usize = substr[0].parse()?;
        let mut game = Game::new(id);

        for session in substr[1].trim().split(";") {
            for point in session.trim().split(",") {
                let point = point.trim();
                let point_color_split: Vec<_> = point.split(" ").collect();
                let points: usize = point_color_split[0].parse()?;
                match point_color_split[1].trim() {
                    "red" => game.red.push(points),
                    "blue" => game.blue.push(points),
                    "green" => game.green.push(points),
                    _ => {}
                }
            }
        }

        Ok(game)
    }

    pub fn is_game_possible(&self, red: usize, green: usize, blue: usize) -> bool {
        self.red.iter().filter(|count| *count > &red).count() == 0
            && self.green.iter().filter(|count| *count > &green).count() == 0
            && self.blue.iter().filter(|count| *count > &blue).count() == 0
    }

    pub fn get_power(&self) -> Option<usize> {
        Some(self.red.iter().max()? * self.blue.iter().max()? * self.green.iter().max()?)
    }
}
