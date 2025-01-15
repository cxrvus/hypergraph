use std::{thread, time::Duration};

pub use map::*;
pub use vec2::*;

pub fn sleep(secs: f32) {
    thread::sleep(Duration::from_secs_f32(secs));
}

pub mod vec2 {
    use std::ops;

    #[derive(Debug, Default, Copy, Clone, PartialEq)]
    pub struct Vec2 {
        pub x: i32,
        pub y: i32,
    }

    #[derive(Debug, Default, Copy, Clone, PartialEq)]
    pub struct Vec2u {
        pub x: usize,
        pub y: usize,
    }

    impl Vec2 {
        pub fn unsign(self) -> Option<Vec2u> {
            let Self { x, y } = self;
            if x >= 0 && y >= 0 {
                Some(Vec2u {
                    x: x as usize,
                    y: y as usize,
                })
            } else {
                None
            }
        }

        pub fn cardinal() -> [Vec2; 4] {
            [-Vec2::Y, Vec2::X, Vec2::Y, -Vec2::X]
        }

        pub fn as_str(&self) -> &str {
            let Self { x, y } = self;
            match (x, y) {
                (0, 0) => "o",
                (0, -1) => "^",
                (1, 0) => ">",
                (0, 1) => "v",
                (-1, 0) => "<",
                _ => "*",
            }
        }

        pub const X: Self = Self { x: 1, y: 0 };
        pub const Y: Self = Self { x: 0, y: 1 };
        pub const ZERO: Self = Self { x: 0, y: 0 };
    }

    impl Vec2u {
        pub fn sign(self) -> Vec2 {
            let Self { x, y } = self;
            Vec2 {
                x: x as i32,
                y: y as i32,
            }
        }
    }

    impl ops::Add<Vec2u> for Vec2u {
        type Output = Self;

        fn add(self, other: Vec2u) -> Self::Output {
            Self {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    impl ops::Mul<usize> for Vec2u {
        type Output = Self;

        fn mul(self, scalar: usize) -> Self::Output {
            Self {
                x: self.x * scalar,
                y: self.y * scalar,
            }
        }
    }

    impl ops::Add<Vec2> for Vec2 {
        type Output = Self;

        fn add(self, other: Vec2) -> Self::Output {
            Self {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    impl ops::Sub<Vec2> for Vec2 {
        type Output = Self;

        fn sub(self, other: Vec2) -> Self::Output {
            Self {
                x: self.x - other.x,
                y: self.y - other.y,
            }
        }
    }

    impl ops::Mul<i32> for Vec2 {
        type Output = Self;

        fn mul(self, scalar: i32) -> Self::Output {
            Self {
                x: self.x * scalar,
                y: self.y * scalar,
            }
        }
    }

    impl ops::Rem<Vec2> for Vec2 {
        type Output = Self;

        fn rem(self, other: Vec2) -> Self::Output {
            Self {
                x: self.x.rem_euclid(other.x),
                y: self.y.rem_euclid(other.y),
            }
        }
    }

    impl ops::Neg for Vec2 {
        type Output = Self;

        fn neg(self) -> Self::Output {
            Self {
                x: -self.x,
                y: -self.y,
            }
        }
    }
}

pub mod map {
    use crate::util::vec2::*;
    use std::fmt::Debug;

    #[derive(Debug, Clone)]
    pub struct Map<T: Debug + PartialEq> {
        pub width: usize,
        pub height: usize,
        pub values: Vec<T>,
    }

    impl<T: Debug + PartialEq> Map<T> {
        pub fn in_bounds(&self, pos: &Vec2) -> bool {
            let Vec2 { x, y } = *pos;
            x >= 0 && y >= 0 && y < self.height as i32 && x < self.width as i32
        }

        pub fn at(&self, pos: &Vec2) -> Option<&T> {
            let Vec2u { x, y } = pos.unsign()?;
            if self.in_bounds(pos) {
                Some(&self.values[y * self.width + x])
            } else {
                None
            }
        }

        pub fn set_at(&mut self, pos: &Vec2, value: T) {
            if self.in_bounds(pos) {
                let Vec2u { x, y } = pos.unsign().unwrap();
                self.values[y * self.width + x] = value;
            } else {
                panic!("map index is out of range: {:?} = {:?}", pos, value)
            }
        }

        pub fn find_all(&self, target: T) -> Vec<Vec2u> {
            self.values
                .iter()
                .enumerate()
                .filter(|(_, value)| **value == target)
                .map(|(i, _)| self.get_pos(i).unwrap())
                .collect()
        }

        pub fn get_pos(&self, i: usize) -> Option<Vec2u> {
            self.values.get(i)?;
            Some(Vec2u {
                x: (i % self.width),
                y: (i / self.width),
            })
        }

        pub fn dimensions(&self) -> Vec2u {
            Vec2u {
                x: self.width,
                y: self.height,
            }
        }
    }

    #[derive(Debug)]
    pub struct ProxyMap {
        pub width: usize,
        pub height: usize,
        pub string: String,
    }

    impl ProxyMap {
        pub fn convert<T: Debug + PartialEq>(self, parser: fn(String) -> Vec<T>) -> Map<T> {
            Map {
                width: self.width,
                height: self.height,
                values: parser(self.string),
            }
        }
    }

    impl From<&str> for ProxyMap {
        fn from(value: &str) -> Self {
            let lines = value.trim().lines();

            Self {
                height: lines.clone().count(),
                width: lines.clone().next().unwrap().len(),
                string: lines.collect::<Vec<&str>>().join(""),
            }
        }
    }
}
