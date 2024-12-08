use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::Read,
};

fn main() {
    let mut file = File::open("data/input").expect("Open file");
    let mut buf = String::new();
    file.read_to_string(&mut buf).expect("Read to string");

    let mut total_antinodes = HashSet::new();

    let mut antennas: HashMap<char, Vec<Point>> = HashMap::new();

    let grid: Vec<Vec<char>> = buf.lines().map(|line| line.chars().collect()).collect();
    for (y, row) in grid.iter().enumerate() {
        for (x, val) in row.iter().enumerate() {
            if *val != '.' {
                antennas
                    .entry(*val)
                    .or_insert(vec![])
                    .push(Point(x as f32, y as f32));
            }
        }
    }

    let points = grid_to_points(grid);

    for (_, antenna_points) in antennas.iter() {
        for (spot, antenna) in antenna_points.iter().enumerate() {
            let rest = &antenna_points[(spot + 1)..];
            for other in rest {
                // get all points "In line" of the two
                points
                    .iter()
                    .filter(|point| {
                        let slope_a = point.slope(antenna);
                        let slope_b = point.slope(other);

                        slope_a == slope_b || point == &antenna || point == &other
                    })
                    /*.filter(|point| {
                        antenna.dist(point) == other.dist(point) * 2f32
                            || other.dist(point) == antenna.dist(point) * 2f32
                    })*/
                    .for_each(|test_point| {
                        total_antinodes.insert((test_point.0 as i32, test_point.1 as i32));
                    });
            }
        }
    }
    println!("Total antinodes: {}", total_antinodes.len())
}

#[derive(Clone, Copy, PartialEq)]
struct Point(f32, f32);

impl Eq for Point {}

impl Point {
    pub fn dist(&self, other: &Point) -> f32 {
        f32::sqrt((self.0 - other.0).powi(2) + (self.1 - other.1).powi(2))
    }

    pub fn slope(&self, other: &Point) -> f32 {
        (other.1 - self.1) / (other.0 - self.0)
    }
}

fn grid_to_points(grid: Vec<Vec<char>>) -> Vec<Point> {
    let mut points = Vec::new();

    for (row, row_vec) in grid.iter().enumerate() {
        for (col, _) in row_vec.iter().enumerate() {
            points.push(Point(row as f32, col as f32));
        }
    }

    points
}
