
pub mod shared;
use std::cmp::Ordering;

#[macro_export]
macro_rules! include_input {
    ($day:literal) => {
        include_str!(concat!("./input/", stringify!($day), ".txt"))
    };
}

pub fn input_as_lines(s: &str) -> Vec<String> {
    s.split("\r\n").map(|x| x.to_owned()).collect::<Vec<_>>()
}

pub fn input_as_ints(s: &str) -> Vec<i64> {
    let input_lines = input_as_lines(s);

    input_lines.iter()
        .map(|s| s.parse::<i64>().expect("Unable to convert line to i64"))
        .collect()
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, Default)]
pub struct Point32 {
    pub x: i32,
    pub y: i32,
}

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, Default)]
pub enum Compass {
    #[default]
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Compass {
    pub fn turn_left(&self, times: usize) -> Self {
        match times.cmp(&1) {
            Ordering::Greater => self.turn_left(times - 1).turn_left(1),
            Ordering::Equal => match self {
                Compass::North => Compass::NorthWest,
                Compass::NorthEast => Compass::North,
                Compass::East => Compass::NorthEast,
                Compass::SouthEast => Compass::East,
                Compass::South => Compass::SouthEast,
                Compass::SouthWest => Compass::South,
                Compass::West => Compass::SouthWest,
                Compass::NorthWest => Compass::West,
            },
            Ordering::Less => *self,
        }
    }

    pub fn turn_right(&self, times: usize) -> Self {
        match times.cmp(&1) {
            Ordering::Greater => self.turn_right(times - 1).turn_right(1),
            Ordering::Equal => match self {
                Compass::North => Compass::NorthEast,
                Compass::NorthEast => Compass::East,
                Compass::East => Compass::SouthEast,
                Compass::SouthEast => Compass::South,
                Compass::South => Compass::SouthWest,
                Compass::SouthWest => Compass::West,
                Compass::West => Compass::NorthWest,
                Compass::NorthWest => Compass::North,
            },
            Ordering::Less => *self,
        }
    }

    pub fn cardinal_left(&self) -> Self {
        self.turn_left(2)
    }
    pub fn cardinal_right(&self) -> Self {
        self.turn_right(2)
    }
}


#[derive(Debug)]
pub struct Board2D<T> {
    vec: Vec<T>,
    width: usize,
    height: usize,
}

impl<T> Board2D<T> {
    pub fn new(vec: Vec<T>, height: usize, width: usize) -> Self {
        assert!(vec.len() == width * height);
        Self { vec, width, height }
    }

    pub fn width(&self) -> usize { self.width }
    pub fn height(&self) -> usize { self.height }

    pub fn index(&self, x: i64, y: i64) -> Option<&T> {
        if y >= 0 && x >= 0 && (y as usize) < self.height && (x as usize) < self.width  {
            Some(&self.vec[((y as usize) * self.width) + (x as usize)])
        } else {
            None
        }
    }
}