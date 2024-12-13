use std::{fs::File, io::Read, str::FromStr};

fn main() {
    let mut file = File::open("data/input").expect("Unable to open input file");
    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .expect("Unable to read input file");

    let lines: Vec<&str> = buf.lines().collect();
    let mut machines = Vec::new();

    for chunk in lines.chunks(4) {
        if let Ok(machine) = chunk.join("\n").parse::<ClawMachine>() {
            machines.push(machine);
        }
    }

    let mut total_tokens = 0;
    let mut prizes_won = 0;

    for machine in &machines {
        if let Some(tokens) = machine.minimum_tokens() {
            total_tokens += tokens;
            prizes_won += 1;
        }
    }

    println!("Prizes won: {}", prizes_won);
    println!("Total tokens spent: {}", total_tokens);
}

#[derive(Debug)]
pub struct ClawMachine {
    button_a: (i64, i64),
    button_b: (i64, i64),
    prize: (i64, i64),
}

impl FromStr for ClawMachine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();

        let button_a = parse_button(lines[0])?;
        let button_b = parse_button(lines[1])?;
        let prize = parse_prize(lines[2])?;

        Ok(ClawMachine {
            button_a,
            button_b,
            prize,
        })
    }
}

fn parse_button(line: &str) -> Result<(i64, i64), ()> {
    let parts: Vec<&str> = line.split([':', '+', ','].as_ref()).collect();
    if parts.len() < 4 {
        return Err(());
    }

    let x = parts[2].parse().map_err(|_| ())?;
    let y = parts[4].parse().map_err(|_| ())?;
    Ok((x, y))
}

fn parse_prize(line: &str) -> Result<(i64, i64), ()> {
    let parts: Vec<&str> = line.split(['=', ',', 'X', 'Y', ' '].as_ref()).collect();
    if parts.len() < 4 {
        return Err(());
    }

    let x: i64 = parts[3].parse().map_err(|_| ())?;
    let y: i64 = parts[7].parse().map_err(|_| ())?;

    Ok((x + 10_000_000_000_000, y + 10_000_000_000_000))
}

impl ClawMachine {
    pub fn minimum_tokens(&self) -> Option<i64> {
        let a = [
            [self.button_a.0, self.button_b.0],
            [self.button_a.1, self.button_b.1],
        ];
        let y = [self.prize.0, self.prize.1];

        let det = a[0][0] * a[1][1] - a[0][1] * a[1][0];
        if det == 0 {
            return None;
        }

        let x1 = (y[0] * a[1][1] - y[1] * a[0][1]) as f64 / det as f64;
        let x2 = (a[0][0] * y[1] - a[1][0] * y[0]) as f64 / det as f64;

        let x1_rounded = x1.round();
        let x2_rounded = x2.round();

        if x1_rounded < 0.0 || x2_rounded < 0.0 {
            return None;
        }

        if (x1 - x1_rounded).abs() > 1e-8 || (x2 - x2_rounded).abs() > 1e-8 {
            return None;
        }

        let tokens_cost = (3 * x1_rounded as i64) + (1 * x2_rounded as i64);

        Some(tokens_cost)
    }
}
