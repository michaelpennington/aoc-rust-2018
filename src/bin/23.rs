advent_of_code::solution!(23);

use std::{ops::Div, str::FromStr};

use advent_of_code::util::point::Pt3;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Nanobot {
    pos: Pt3<isize>,
    radius: usize,
}

fn points_around(o: Pt3<isize>) -> impl Iterator<Item = Pt3<isize>> {
    (o.x - 100..=o.x + 100).flat_map(move |x| {
        (o.y - 100..=o.y + 100)
            .flat_map(move |y| (o.z - 100..=o.z + 100).map(move |z| Pt3 { x, y, z }))
    })
}

impl Nanobot {
    fn contains(&self, pt: &Pt3<isize>) -> bool {
        self.pos.abs_norm(pt) <= self.radius
    }
}

impl Div<usize> for Nanobot {
    type Output = Nanobot;

    fn div(self, rhs: usize) -> Self::Output {
        Self {
            pos: self.pos / (rhs as isize),
            radius: self.radius / rhs,
        }
    }
}

impl FromStr for Nanobot {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (pt1, pt2) = s.split_once(", ").unwrap();
        let pos = pt1.trim_start_matches("pos=").parse()?;
        let radius = pt2.trim_start_matches("r=").parse()?;
        Ok(Self { pos, radius })
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let bots: Vec<Nanobot> = input.lines().map(|l| l.parse().unwrap()).collect();
    let largest = bots.iter().max_by_key(|b| b.radius).unwrap();
    Some(bots.iter().filter(|b| largest.contains(&b.pos)).count())
}

const ORIGIN: Pt3<isize> = Pt3 { x: 0, y: 0, z: 0 };

pub fn part_two(input: &str) -> Option<usize> {
    let bots: Vec<Nanobot> = input.lines().map(|l| l.parse().unwrap()).collect();
    let mut divisor = 1_000_000;
    let mut best_guess = ORIGIN;
    while divisor > 1 {
        let low_res = bots.iter().map(|b| *b / divisor).collect::<Vec<_>>();
        best_guess = points_around(best_guess)
            .max_by(|pt1, pt2| {
                low_res
                    .iter()
                    .filter(|b| b.contains(pt1))
                    .count()
                    .cmp(&low_res.iter().filter(|b| b.contains(pt2)).count())
                    .then_with(|| ORIGIN.abs_norm(pt1).cmp(&ORIGIN.abs_norm(pt2)).reverse())
            })
            .unwrap();
        divisor /= 10;
        best_guess *= 10;
    }
    best_guess = points_around(best_guess)
        .max_by(|pt1, pt2| {
            bots.iter()
                .filter(|b| b.contains(pt1))
                .count()
                .cmp(&bots.iter().filter(|b| b.contains(pt2)).count())
                .then_with(|| ORIGIN.abs_norm(pt1).cmp(&ORIGIN.abs_norm(pt2)).reverse())
        })
        .unwrap();
    Some(ORIGIN.abs_norm(&best_guess))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }
}
