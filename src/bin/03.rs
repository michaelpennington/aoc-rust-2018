use std::{collections::HashMap, str::FromStr};

advent_of_code::solution!(3);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Claim {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    id: u32,
}

impl FromStr for Claim {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (id, rest) = s.split_once(" @ ").unwrap();
        let id = id.trim_start_matches('#').parse()?;
        let (loc, pos) = rest.split_once(": ").unwrap();
        let (x, y) = loc.split_once(',').unwrap();
        let (width, height) = pos.split_once('x').unwrap();
        let (x, y) = (x.parse()?, y.parse()?);
        let (width, height) = (width.parse()?, height.parse()?);
        Ok(Self {
            x,
            y,
            width,
            height,
            id,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
struct Map {
    map: HashMap<(usize, usize), u32>,
}

impl Map {
    fn add(&mut self, claim: Claim) {
        for x in claim.x..claim.x + claim.width {
            for y in claim.y..claim.y + claim.height {
                self.map.entry((x, y)).and_modify(|e| *e += 1).or_insert(1);
            }
        }
    }

    fn nonoverlapping(&self, claim: Claim) -> bool {
        self.map
            .iter()
            .filter(|&(&(x, y), _)| {
                claim.x <= x
                    && x < claim.x + claim.width
                    && claim.y <= y
                    && y < claim.y + claim.height
            })
            .all(|(_, &e)| e == 1)
    }

    fn count(&self) -> u32 {
        self.map.values().filter(|&&k| k >= 2).count() as u32
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut map = Map::default();
    for claim in input.lines().map(|l| l.parse::<Claim>().unwrap()) {
        map.add(claim);
    }
    Some(map.count())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut map = Map::default();
    let claims = input
        .lines()
        .map(|l| l.parse::<Claim>().unwrap())
        .collect::<Vec<_>>();
    for claim in &claims {
        map.add(*claim);
    }
    for claim in claims {
        if map.nonoverlapping(claim) {
            return Some(claim.id);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(3));
    }
}
