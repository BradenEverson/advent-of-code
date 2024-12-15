use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

fn main() {
    let path = "data/input";
    let input = read_lines(path).expect("Could not read file");

    let mut warehouse = Vec::new();
    let mut moves = String::new();
    let mut reading_moves = false;

    for line in input {
        let line = line.expect("Error reading line");
        if line.trim().is_empty() {
            reading_moves = true;
        } else if reading_moves {
            moves.push_str(line.trim());
        } else {
            warehouse.push(line.chars().collect::<Vec<_>>());
        }
    }

    warehouse = scale_warehouse(&warehouse);
    print_warehouse(&warehouse);
    let mut robot_pos = find_robot(&warehouse);

    for m in moves.chars() {
        let (dx, dy) = match m {
            '^' => (-1, 0),
            'v' => (1, 0),
            '<' => (0, -1),
            '>' => (0, 1),
            _ => continue,
        };

        move_robot(&mut warehouse, &mut robot_pos, dx, dy);
    }

    let mut gps_sum = 0;
    for (row, line) in warehouse.iter().enumerate() {
        for (col, ch) in line.iter().enumerate() {
            if *ch == '[' {
                gps_sum += 100 * row + col;
            }
        }
    }
    print_warehouse(&warehouse);
    println!("Sum of GPS coordinates: {}", gps_sum);
}

fn find_robot(warehouse: &Vec<Vec<char>>) -> (usize, usize) {
    let mut pos = (0, 0);
    for (y, line) in warehouse.iter().enumerate() {
        for (x, val) in line.iter().enumerate() {
            if *val == '@' {
                pos = (y, x);
                break;
            }
        }
    }

    pos
}

fn print_warehouse(warehouse: &Vec<Vec<char>>) {
    for line in warehouse {
        println!("{}", line.iter().collect::<String>())
    }
    println!()
}

fn scale_warehouse(warehouse: &Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut scaled = Vec::new();
    for line in warehouse {
        let mut scaled_row = Vec::new();
        for ch in line {
            match ch {
                '#' => scaled_row.extend(vec!['#', '#']),
                'O' => scaled_row.extend(vec!['[', ']']),
                '.' => scaled_row.extend(vec!['.', '.']),
                '@' => scaled_row.extend(vec!['@', '.']),
                _ => (),
            }
        }
        scaled.push(scaled_row.clone());
    }
    scaled
}

fn move_robot(
    warehouse: &mut Vec<Vec<char>>,
    robot_pos: &mut (usize, usize),
    dx: isize,
    dy: isize,
) {
    let (x, y) = (robot_pos.0 as isize, robot_pos.1 as isize);
    let next_x = x + dx;
    let next_y = y + dy;

    if !is_within_bounds(warehouse, next_x, next_y)
        || warehouse[next_x as usize][next_y as usize] == '#'
    {
        return;
    }

    let box_pattern = ['[', ']'];

    if box_pattern.contains(&warehouse[next_x as usize][next_y as usize]) {
        let mut box_positions = Vec::new();
        let mut current_x = next_x;
        let mut current_y = next_y;

        let mut idx = 0;

        while is_within_bounds(warehouse, current_x, current_y)
            && box_pattern.contains(&warehouse[current_x as usize][current_y as usize])
        {
            let curr_box = if idx % 2 == 0 { ']' } else { '[' };
            box_positions.push((current_x, current_y, curr_box));
            current_x += dx;
            current_y += dy;
            idx += 1;
        }

        if is_within_bounds(warehouse, current_x, current_y)
            && warehouse[current_x as usize][current_y as usize] == '.'
        {
            while let Some((bx, by, character)) = box_positions.pop() {
                let (offset, other) = match character {
                    '[' => (1, ']'),
                    ']' => (-1, '['),
                    _ => unreachable!(),
                };
                let new_x = bx + dx;
                let new_y = by + dy;
                warehouse[new_x as usize][new_y as usize] = character;
                warehouse[bx as usize][by as usize] = '.';

                if dy == 0 {
                    warehouse[new_x as usize][(new_y + offset) as usize] = other;
                    warehouse[bx as usize][by as usize] = '.';
                }
            }
        } else {
            return;
        }
    }

    warehouse[robot_pos.0][robot_pos.1] = '.';
    warehouse[next_x as usize][next_y as usize] = '@';
    *robot_pos = (next_x as usize, next_y as usize);
}

fn is_within_bounds(warehouse: &Vec<Vec<char>>, x: isize, y: isize) -> bool {
    x >= 0 && x < warehouse.len() as isize && y >= 0 && y < warehouse[0].len() as isize
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
