use std::fmt::Display;

use ndarray::Array2;

const WIDTH: usize = 40;
const HEIGHT: usize = 20;

#[derive(Debug, Clone)]
struct Turtle {
    x: usize,
    y: usize,

    direction: Direction,

    pen_down: bool,
}

impl Turtle {
    fn turn_right(&mut self) {
        self.direction = self.direction.next_right();
    }

    fn turn_left(&mut self) {
        self.direction = self.direction.next_left();
    }
}

impl Default for Turtle {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            direction: Direction::Right,
            pen_down: true,
        }
    }
}

/// The different directions a Turtle can face
#[derive(Debug, Clone, Copy)]
enum Direction {
    UpLeft = 0,
    Up = 1,
    UpRight = 2,
    Right = 3,
    DownRight = 4,
    Down = 5,
    DownLeft = 6,
    Left = 7,
}

impl From<u8> for Direction {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::UpLeft,
            1 => Self::Up,
            2 => Self::UpRight,
            3 => Self::Right,
            4 => Self::DownRight,
            5 => Self::Down,
            6 => Self::DownLeft,
            _ => Self::Left,
        }
    }
}

impl Direction {
    /// Returns the direction opposite to this one.
    /// For example Right is the opposite of Left.
    fn opposite(&self) -> Self {
        match self {
            Direction::UpLeft => Direction::DownRight,
            Direction::Up => Direction::Down,
            Direction::UpRight => Direction::DownLeft,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::DownLeft => Direction::UpRight,
            Direction::Down => Direction::Up,
            Direction::DownRight => Direction::UpLeft,
        }
    }

    /// Returns the direction to the right of this one
    fn next_right(&self) -> Self {
        Self::from(*self as u8 + 1)
    }

    /// Returns the direction to the left of this one
    fn next_left(&self) -> Self {
        if *self as u8 == 0 {
            Self::Left
        } else {
            Self::from(*self as u8 - 1)
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

impl Color {
    fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }
}

struct Board {
    contents: Array2<Color>,
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

fn main() {
    let mut t = Turtle {
        x: WIDTH / 2,
        y: HEIGHT / 2,
        ..Default::default()
    };

    let mut board: Array2<Color> =
        Array2::from_shape_simple_fn((WIDTH, HEIGHT), || Color::new(0, 0, 0));
}
