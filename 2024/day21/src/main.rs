use std::{fs::File, io::Read, marker::PhantomData};

pub const NUMERIC_KEYPAD: [[Option<KeypadDigits>; 3]; 4] = [
    [
        Some(KeypadDigits::Seven),
        Some(KeypadDigits::Eight),
        Some(KeypadDigits::Nine),
    ],
    [
        Some(KeypadDigits::Four),
        Some(KeypadDigits::Five),
        Some(KeypadDigits::Six),
    ],
    [
        Some(KeypadDigits::One),
        Some(KeypadDigits::Two),
        Some(KeypadDigits::Three),
    ],
    [None, Some(KeypadDigits::Zero), Some(KeypadDigits::A)],
];

pub const DIRECTIONAL_KEYPAD: [[Option<KeypadDigits>; 3]; 2] = [
    [None, Some(KeypadDigits::Up), Some(KeypadDigits::A)],
    [
        Some(KeypadDigits::Left),
        Some(KeypadDigits::Down),
        Some(KeypadDigits::Left),
    ],
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum KeypadDigits {
    Zero,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Up,
    Down,
    Left,
    Right,
    A,
}

impl KeypadDigits {
    pub fn from_char(character: char) -> Option<Self> {
        match character {
            'A' => Some(Self::A),
            '0' => Some(Self::Zero),
            '1' => Some(Self::One),
            '2' => Some(Self::Two),
            '3' => Some(Self::Three),
            '4' => Some(Self::Four),
            '5' => Some(Self::Five),
            '6' => Some(Self::Six),
            '7' => Some(Self::Seven),
            '8' => Some(Self::Eight),
            '9' => Some(Self::Nine),
            _ => None,
        }
    }
}

fn main() {
    let mut file = File::open("data/test").expect("Open data");
    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .expect("Failed to write to buffer");

    let codes: Vec<[KeypadDigits; 4]> = buf.lines().map(|code| line_to_digits(code)).collect();
    let mut robots = vec![
        Robot::new(&NUMERIC_KEYPAD),
        Robot::new(&DIRECTIONAL_KEYPAD),
        Robot::new(&DIRECTIONAL_KEYPAD),
        Robot::new(&DIRECTIONAL_KEYPAD),
    ];

    let mut final_instructions = vec![];

    for code in codes {
        let mut current = code.to_vec();
        for robot in &mut robots {
            current = robot.key_presses_to(&current);
        }
        println!("Final instruction length: {}", current.len());
        final_instructions.push(current);
    }
}

pub fn line_to_digits(line: &str) -> [KeypadDigits; 4] {
    let parsed = line
        .chars()
        .filter_map(|key| KeypadDigits::from_char(key))
        .collect::<Vec<_>>();
    [parsed[0], parsed[1], parsed[2], parsed[3]]
}

pub struct Keypad;
pub struct Middleman;

pub struct Robot<TYPE = Middleman> {
    keypad: &'static [[Option<KeypadDigits>; 3]],
    key_ptr: (usize, usize),
    //controlling: CONTROLLING,
    r#type: PhantomData<TYPE>,
}

impl Robot {
    pub fn new(keypad: &'static [[Option<KeypadDigits>; 3]], /*controlling: CONTROLLING*/) -> Self {
        let key_ptr = if keypad == &DIRECTIONAL_KEYPAD {
            (0, 2)
        } else {
            (3, 2)
        };
        Self {
            key_ptr,
            keypad,
            //controlling,
            r#type: PhantomData::default(),
        }
    }

    pub fn get_key(&self) -> KeypadDigits {
        let (x, y) = self.key_ptr;
        self.keypad[y][x].unwrap()
    }

    /// Gets the directional keypad presses and submits that must be sent to achieve the desired
    /// sequence of keypad buttons to be clicked
    pub fn key_presses_to(&mut self, combo: &[KeypadDigits]) -> Vec<KeypadDigits> {
        let mut presses = Vec::new();

        for &target in combo {
            if let Some((dx, dy)) = self.path_to(target) {
                presses.extend(std::iter::repeat(KeypadDigits::Right).take(dx.max(0) as usize));
                presses.extend(std::iter::repeat(KeypadDigits::Left).take(-dx.min(0) as usize));
                presses.extend(std::iter::repeat(KeypadDigits::Up).take(dy.max(0) as usize));
                presses.extend(std::iter::repeat(KeypadDigits::Down).take(-dy.min(0) as usize));

                self.key_ptr.0 = (self.key_ptr.0 as isize + dx) as usize;
                self.key_ptr.1 = (self.key_ptr.1 as isize + dy) as usize;

                presses.push(KeypadDigits::A);
            }
        }

        presses
    }

    /// Returns how up and how left you need to go to get to the keypad if possible. Avoids "None"
    /// digits
    pub fn path_to(&self, to: KeypadDigits) -> Option<(isize, isize)> {
        let mut shortest_path: Option<(isize, isize)> = None;

        for (y, row) in self.keypad.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                if cell == Some(to) {
                    let dx = x as isize - self.key_ptr.0 as isize;
                    let dy = y as isize - self.key_ptr.1 as isize;

                    // Check the validity of the path
                    let mut valid_path = true;
                    let mut cx = self.key_ptr.0 as isize;
                    let mut cy = self.key_ptr.1 as isize;

                    while cx != x as isize || cy != y as isize {
                        if cx != x as isize {
                            cx += if dx > 0 { 1 } else { -1 };
                        }
                        if cy != y as isize {
                            cy += if dy > 0 { 1 } else { -1 };
                        }

                        if cx < 0
                            || cx >= self.keypad[0].len() as isize
                            || cy < 0
                            || cy >= self.keypad.len() as isize
                            || self.keypad[cy as usize][cx as usize].is_none()
                        {
                            valid_path = false;
                            break;
                        }
                    }

                    if valid_path {
                        shortest_path = Some((dx, dy));
                    }
                }
            }
        }

        shortest_path
    }
}

pub trait Controllable {
    fn submit_key(&mut self, key: KeypadDigits);
}

#[derive(Default)]
pub struct Output {
    out: String,
}

impl Controllable for Output {
    fn submit_key(&mut self, key: KeypadDigits) {
        let addition = match key {
            KeypadDigits::Zero => "0",
            KeypadDigits::One => "1",
            KeypadDigits::Two => "2",
            KeypadDigits::Three => "3",
            KeypadDigits::Four => "4",
            KeypadDigits::Five => "5",
            KeypadDigits::Six => "6",
            KeypadDigits::Seven => "7",
            KeypadDigits::Eight => "8",
            KeypadDigits::Nine => "9",
            KeypadDigits::A => "A",
            _ => unreachable!(),
        };
        self.out += addition;
    }
}

/*impl<CONTROLLING: Controllable> Controllable for Robot<CONTROLLING, Middleman> {
    fn submit_key(&mut self, key: KeypadDigits) {
        let (x, y) = (&mut self.key_ptr.0, &mut self.key_ptr.1);
        match key {
            KeypadDigits::Up => *y -= 1,
            KeypadDigits::Down => *y += 1,
            KeypadDigits::Left => *x -= 1,
            KeypadDigits::Right => *x += 1,
            KeypadDigits::A => self.controlling.submit_key(self.get_key()),
            _ => unreachable!(),
        }
    }
}*/
