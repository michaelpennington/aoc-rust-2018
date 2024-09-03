use std::{
    collections::HashMap,
    ops::{Add, Not},
};

use anyhow::anyhow;

advent_of_code::solution!(20);

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
struct Pt {
    x: isize,
    y: isize,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum Dir {
    N,
    S,
    E,
    W,
}

impl TryFrom<char> for Dir {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'N' => Ok(Self::N),
            'S' => Ok(Self::S),
            'E' => Ok(Self::E),
            'W' => Ok(Self::W),
            _ => Err(anyhow!("{value} is not a valid dir")),
        }
    }
}

#[allow(clippy::suspicious_arithmetic_impl)]
impl Add<Dir> for Pt {
    type Output = Pt;

    fn add(self, rhs: Dir) -> Self::Output {
        match rhs {
            Dir::N => Pt {
                x: self.x,
                y: self.y - 1,
            },
            Dir::S => Pt {
                x: self.x,
                y: self.y + 1,
            },
            Dir::E => Pt {
                x: self.x + 1,
                y: self.y,
            },
            Dir::W => Pt {
                x: self.x - 1,
                y: self.y,
            },
        }
    }
}

impl Not for Dir {
    type Output = Dir;

    fn not(self) -> Self::Output {
        match self {
            Dir::N => Dir::S,
            Dir::S => Dir::N,
            Dir::E => Dir::W,
            Dir::W => Dir::E,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Default)]
struct Map {
    distances: HashMap<Pt, u32>,
}

impl Map {
    const ORIGIN: Pt = Pt { x: 0, y: 0 };

    fn construct(s: &str) -> Self {
        let mut out = Self::default();
        let mut stack = Vec::new();
        let mut cur_pos = Self::ORIGIN;
        let mut previous_d = 0;
        out.distances.insert(Self::ORIGIN, 0);
        for c in s
            .trim()
            .trim_start_matches('^')
            .trim_end_matches('$')
            .chars()
        {
            match c {
                '(' => {
                    stack.push(cur_pos);
                }
                ')' => {
                    stack.pop();
                }
                '|' => {
                    cur_pos = *stack.last().unwrap();
                }
                'N' | 'S' | 'E' | 'W' => {
                    let dir: Dir = c.try_into().unwrap();
                    cur_pos = cur_pos + dir;
                    out.distances
                        .entry(cur_pos)
                        .and_modify(|d| *d = *d.min(&mut (previous_d + 1)))
                        .or_insert(previous_d + 1);
                }
                _ => unreachable!(),
            }
            previous_d = out.distances[&cur_pos];
        }
        out
    }

    fn longest_path_len(&self) -> u32 {
        *self.distances.values().max().unwrap()
    }

    fn num_paths_larger_than(&self, value: u32) -> u32 {
        self.distances.values().filter(|&&v| v >= value).count() as u32
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let map = Map::construct(input);
    Some(map.longest_path_len())
}

pub fn part_two(input: &str) -> Option<u32> {
    let map = Map::construct(input);
    Some(map.num_paths_larger_than(1000))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_one_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(10));
    }

    #[test]
    fn test_part_one_two() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_one_three() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(23));
    }

    #[test]
    fn test_part_one_four() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(31));
    }
}
