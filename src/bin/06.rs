use std::{collections::HashMap, str::FromStr};

use anyhow::anyhow;

advent_of_code::solution!(6);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default, Hash)]
struct Pt(i32, i32);

impl Pt {
    fn distance(&self, other: &Self) -> u32 {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }
}

impl FromStr for Pt {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .split_once(", ")
            .and_then(|(x, y)| Some((x.parse().ok()?, y.parse().ok()?)))
            .ok_or(anyhow!("{s} is not a valid point"))?;
        Ok(Pt(x, y))
    }
}

const WIDTH: i32 = 750;
const HEIGHT: i32 = 750;

#[derive(Clone, Debug, PartialEq, Eq, Default)]
struct Map {
    inner: HashMap<Pt, Option<Pt>>,
}

impl Map {
    fn generate(pts: &[Pt]) -> Self {
        let mut inner = HashMap::new();
        for x in (-WIDTH / 2)..=(WIDTH / 2) {
            for y in (-HEIGHT / 2)..=(HEIGHT / 2) {
                let p = Pt(x, y);
                let closest = *pts.iter().min_by_key(|pt| pt.distance(&p)).unwrap();
                let closest = if pts
                    .iter()
                    .filter(|&&pt| pt != closest)
                    .any(|pt| pt.distance(&p) == closest.distance(&p))
                {
                    None
                } else {
                    Some(closest)
                };
                inner.insert(p, closest);
            }
        }
        Self { inner }
    }

    fn has_pts_on_boundary(&self, pt: Pt) -> bool {
        self.inner
            .iter()
            .filter(|&(p, _)| p.0.abs() == WIDTH / 2 || p.1.abs() == HEIGHT / 2)
            .any(|(_, &to)| Some(pt) == to)
    }

    fn num_pts(&self, pts: &[Pt]) -> HashMap<Pt, u32> {
        let mut map = HashMap::with_capacity(pts.len());
        for to in self.inner.values().flatten().filter(|p| pts.contains(p)) {
            map.entry(*to).and_modify(|e| *e += 1).or_insert(1);
        }
        map
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut pts = input
        .lines()
        .map(|l| l.parse::<Pt>().unwrap())
        .collect::<Vec<_>>();
    let map = Map::generate(&pts);
    pts.retain(|p| !map.has_pts_on_boundary(*p));
    let num = map.num_pts(&pts);
    num.values().max().copied()
}

pub fn part_two(input: &str) -> Option<u32> {
    let pts = input
        .lines()
        .map(|l| l.parse::<Pt>().unwrap())
        .collect::<Vec<_>>();
    const RANGE: i32 = 342;
    Some(
        (-RANGE..=RANGE)
            .flat_map(|x| (-RANGE..=RANGE).map(move |y| Pt(x, y)))
            .map(|p0| pts.iter().map(|p1| p0.distance(p1)).sum::<u32>())
            .filter(|&s| s < 10000)
            .count() as u32,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(17));
    }
}
