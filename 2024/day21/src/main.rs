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
    let mut file = File::open("data/input").expect("Open data");
    let mut buf = String::new();
    file.read_to_string(&mut buf)
        .expect("Failed to write to buffer");

    let codes: Vec<[KeypadDigits; 4]> = buf.lines().map(|code| line_to_digits(code)).collect();
    println!("{codes:?}");
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

pub struct Robot<CONTROLLING: Controllable, TYPE = Middleman> {
    keypad: &'static [[Option<KeypadDigits>; 3]],
    key_ptr: (usize, usize),
    controlling: CONTROLLING,
    r#type: PhantomData<TYPE>,
}

impl<CONTROLLING: Controllable> Robot<CONTROLLING> {
    pub fn new(keypad: &'static [[Option<KeypadDigits>; 3]], controlling: CONTROLLING) -> Self {
        Self {
            key_ptr: (keypad.len() - 1, 2),
            keypad,
            controlling,
            r#type: PhantomData::default(),
        }
    }

    pub fn get_key(&self) -> KeypadDigits {
        let (x, y) = self.key_ptr;
        self.keypad[y][x].unwrap()
    }

    /// Returns how up and how left you need to go to get to the keypad if possible. Avoids "None"
    /// digits
    pub fn path_to(&self, to: KeypadDigits) -> Option<(isize, isize)> {
        todo!()
    }
}

pub trait Controllable {
    fn submit_key(&mut self, key: KeypadDigits);
}

impl<CONTROLLING: Controllable> Controllable for Robot<CONTROLLING, Middleman> {
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
}
