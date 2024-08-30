use std::{
    cmp::Reverse,
    collections::HashMap,
    fmt::Display,
    ops::{AddAssign, Not},
    str::FromStr,
};

use anyhow::anyhow;

advent_of_code::solution!(13);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
pub struct Pt(u32, u32);

impl PartialOrd for Pt {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Pt {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.1.cmp(&other.1).then_with(|| self.0.cmp(&other.0))
    }
}

impl Display for Pt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.0, self.1)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Track {
    Hor,
    Ver,
    NW,
    NE,
    Int,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Dir {
    N,
    S,
    E,
    W,
}

impl Dir {
    fn next_dir(&self, num_inters: u32) -> Self {
        match num_inters % 3 {
            0 => self.left(),
            1 => *self,
            2 => self.right(),
            _ => unreachable!(),
        }
    }

    fn left(&self) -> Self {
        match self {
            Dir::N => Dir::W,
            Dir::E => Dir::N,
            Dir::W => Dir::S,
            Dir::S => Dir::E,
        }
    }

    fn right(&self) -> Self {
        !self.left()
    }
}

impl Not for Dir {
    type Output = Self;

    fn not(self) -> Self {
        match self {
            Dir::N => Dir::S,
            Dir::S => Dir::N,
            Dir::E => Dir::W,
            Dir::W => Dir::E,
        }
    }
}

impl AddAssign<Dir> for Pt {
    fn add_assign(&mut self, rhs: Dir) {
        match rhs {
            Dir::N => self.1 -= 1,
            Dir::S => self.1 += 1,
            Dir::E => self.0 += 1,
            Dir::W => self.0 -= 1,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Cart {
    loc: Pt,
    dir: Dir,
    intersections_seen: u32,
}

impl Display for Track {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Track::Hor => '-',
            Track::Ver => '|',
            Track::NW => '\\',
            Track::NE => '/',
            Track::Int => '+',
        };
        write!(f, "{c}")
    }
}

impl TryFrom<char> for Track {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '-' => Ok(Track::Hor),
            '|' => Ok(Track::Ver),
            '\\' => Ok(Track::NW),
            '/' => Ok(Track::NE),
            '+' => Ok(Track::Int),
            _ => Err(anyhow!("{value} is not a valid track")),
        }
    }
}

impl PartialOrd for Cart {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Cart {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.loc.cmp(&other.loc)
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Mines {
    map: HashMap<Pt, Track>,
    carts: Vec<Cart>,
}

impl FromStr for Mines {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = HashMap::new();
        let mut carts = Vec::new();
        for (y, line) in s.lines().enumerate().map(|(y, l)| (y as u32, l)) {
            for (x, c) in line.chars().enumerate().map(|(x, c)| (x as u32, c)) {
                match c {
                    '>' => {
                        carts.push(Cart {
                            loc: Pt(x, y),
                            dir: Dir::E,
                            intersections_seen: 0,
                        });
                        map.insert(Pt(x, y), Track::Hor);
                    }
                    '^' => {
                        carts.push(Cart {
                            loc: Pt(x, y),
                            dir: Dir::N,
                            intersections_seen: 0,
                        });
                        map.insert(Pt(x, y), Track::Ver);
                    }
                    '<' => {
                        carts.push(Cart {
                            loc: Pt(x, y),
                            dir: Dir::W,
                            intersections_seen: 0,
                        });
                        map.insert(Pt(x, y), Track::Hor);
                    }
                    'v' => {
                        carts.push(Cart {
                            loc: Pt(x, y),
                            dir: Dir::S,
                            intersections_seen: 0,
                        });
                        map.insert(Pt(x, y), Track::Ver);
                    }
                    ' ' => {}
                    _ => {
                        map.insert(Pt(x, y), c.try_into()?);
                    }
                }
            }
        }
        Ok(Self { map, carts })
    }
}

impl Display for Mines {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let min_x = self.map.keys().map(|p| p.0).min().unwrap();
        let max_x = self.map.keys().map(|p| p.0).max().unwrap();
        let min_y = self.map.keys().map(|p| p.1).min().unwrap();
        let max_y = self.map.keys().map(|p| p.1).max().unwrap();
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let pt = Pt(x, y);
                if let Some(cart) = self.carts.iter().find(|cart| cart.loc == pt) {
                    match cart.dir {
                        Dir::N => write!(f, "^")?,
                        Dir::S => write!(f, "v")?,
                        Dir::E => write!(f, ">")?,
                        Dir::W => write!(f, "<")?,
                    }
                } else if let Some(tile) = self.map.get(&pt) {
                    write!(f, "{tile}")?
                } else {
                    write!(f, " ")?
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Mines {
    fn step(&mut self) -> Option<Pt> {
        self.carts.sort();
        for i in 0..self.carts.len() {
            let cart = &mut self.carts[i];
            cart.loc += cart.dir;
            match (self.map[&cart.loc], cart.dir) {
                (Track::Hor, Dir::N | Dir::S) | (Track::Ver, Dir::E | Dir::W) => unreachable!(),
                (Track::NW, Dir::N) => cart.dir = Dir::W,
                (Track::NW, Dir::S) => cart.dir = Dir::E,
                (Track::NW, Dir::E) => cart.dir = Dir::S,
                (Track::NW, Dir::W) => cart.dir = Dir::N,
                (Track::NE, Dir::N) => cart.dir = Dir::E,
                (Track::NE, Dir::S) => cart.dir = Dir::W,
                (Track::NE, Dir::E) => cart.dir = Dir::N,
                (Track::NE, Dir::W) => cart.dir = Dir::S,
                (Track::Int, dir) => {
                    cart.dir = dir.next_dir(cart.intersections_seen);
                    cart.intersections_seen += 1;
                }
                _ => {}
            }

            let cart = &self.carts[i];
            if self.is_collision(i, cart) {
                return Some(cart.loc);
            }
        }
        None
    }

    fn step_v2(&mut self) -> Option<Pt> {
        self.carts.sort();
        let mut indices_to_remove = Vec::new();
        for i in 0..self.carts.len() {
            let cart = &mut self.carts[i];
            cart.loc += cart.dir;
            match (self.map[&cart.loc], cart.dir) {
                (Track::Hor, Dir::N | Dir::S) | (Track::Ver, Dir::E | Dir::W) => unreachable!(),
                (Track::NW, Dir::N) => cart.dir = Dir::W,
                (Track::NW, Dir::S) => cart.dir = Dir::E,
                (Track::NW, Dir::E) => cart.dir = Dir::S,
                (Track::NW, Dir::W) => cart.dir = Dir::N,
                (Track::NE, Dir::N) => cart.dir = Dir::E,
                (Track::NE, Dir::S) => cart.dir = Dir::W,
                (Track::NE, Dir::E) => cart.dir = Dir::N,
                (Track::NE, Dir::W) => cart.dir = Dir::S,
                (Track::Int, dir) => {
                    cart.dir = dir.next_dir(cart.intersections_seen);
                    cart.intersections_seen += 1;
                }
                _ => {}
            }

            let cart = &self.carts[i];
            if let Some(j) = self.is_collision_v2(i, cart) {
                indices_to_remove.push(i);
                indices_to_remove.push(j);
            }
        }
        indices_to_remove.sort_by_key(|&i| Reverse(i));
        for i in indices_to_remove {
            self.carts.remove(i);
        }
        if self.carts.len() == 1 {
            return Some(self.carts[0].loc);
        }
        None
    }

    fn is_collision_v2(&self, index: usize, cart: &Cart) -> Option<usize> {
        self.carts
            .iter()
            .enumerate()
            .filter(|&(i, _)| i != index)
            .find_map(|(i, c)| (c.loc == cart.loc).then_some(i))
    }

    fn is_collision(&self, index: usize, cart: &Cart) -> bool {
        self.carts
            .iter()
            .enumerate()
            .filter(|&(i, _)| i != index)
            .any(|(_, c)| c.loc == cart.loc)
    }
}

pub fn part_one(input: &str) -> Option<Pt> {
    let mut mines = input.parse::<Mines>().unwrap();
    loop {
        if let Some(p) = mines.step() {
            return Some(p);
        }
    }
}

pub fn part_two(input: &str) -> Option<Pt> {
    let mut mines = input.parse::<Mines>().unwrap();
    loop {
        if let Some(p) = mines.step_v2() {
            return Some(p);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(Pt(7, 3)));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(Pt(6, 4)));
    }
}
