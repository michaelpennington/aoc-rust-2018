use std::{fmt::Display, str::FromStr};

use anyhow::anyhow;

advent_of_code::solution!(12);

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
enum Pot {
    Plant,
    #[default]
    Empty,
}

impl Display for Pot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Pot::Plant => '#',
            Pot::Empty => '.',
        };
        write!(f, "{c}")
    }
}

impl TryFrom<char> for Pot {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Self::Plant),
            '.' => Ok(Self::Empty),
            _ => Err(anyhow!("{value} is not a valid pot")),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Default)]
struct Row {
    pots: Vec<(isize, Pot)>,
    rules: Vec<Rule>,
}

impl FromStr for Row {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let row = lines.next().unwrap().split_whitespace().nth(2).unwrap();
        let mut pots = Vec::with_capacity(row.len() * 6);
        for i in -(row.len() as isize)..=-1 {
            pots.push((i, Pot::default()));
        }
        for (i, c) in row.char_indices() {
            pots.push((i as isize, c.try_into()?));
        }
        for i in row.len()..row.len() * 4 {
            pots.push((i as isize, Pot::default()));
        }
        let mut rules = Vec::with_capacity(s.lines().count() - 2);
        for line in lines.skip(1) {
            if line.ends_with('#') {
                rules.push(line.parse()?);
            }
        }
        Ok(Self { pots, rules })
    }
}

impl Display for Row {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in &self.pots {
            write!(f, "{}", c.1)?;
        }
        Ok(())
    }
}

impl Row {
    fn next_gen(&mut self) {
        let mut new = self.pots.clone();
        for (i, w) in self.pots.windows(5).enumerate().map(|(i, w)| (i + 2, w)) {
            new[i] = if self.rules.iter().any(|r| r.matches(w)) {
                (w[2].0, Pot::Plant)
            } else {
                (w[2].0, Pot::Empty)
            };
        }
        self.pots = new;
    }

    fn total(&self) -> isize {
        self.pots
            .iter()
            .filter(|&p| p.1 == Pot::Plant)
            .map(|p| p.0)
            .sum()
    }

    fn is_previous_shifted(&self, prev: &[(isize, Pot)]) -> bool {
        self.pots[1..].iter().zip(prev).all(|(p1, p2)| p1.1 == p2.1)
    }

    fn plant_count(&self) -> isize {
        self.pots.iter().filter(|&p| p.1 == Pot::Plant).count() as isize
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Default)]
struct Rule([Pot; 5]);

impl Rule {
    fn matches(&self, pots: &[(isize, Pot)]) -> bool {
        (0..5).all(|i| self.0[i] == pots[i].1)
    }
}

impl FromStr for Rule {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut out = Self::default();
        for (c, p) in s.chars().zip(&mut out.0) {
            *p = c.try_into()?;
        }
        Ok(out)
    }
}

pub fn part_one(input: &str) -> Option<isize> {
    let mut row = input.parse::<Row>().unwrap();
    for _ in 0..20 {
        row.next_gen();
    }
    Some(row.total())
}

pub fn part_two(input: &str) -> Option<isize> {
    let mut row = input.parse::<Row>().unwrap();
    let mut prev = row.pots.clone();
    for i in 1..=50_000_000_000isize {
        row.next_gen();
        // println!("{row}");
        if row.is_previous_shifted(&prev) {
            return Some(row.total() + row.plant_count() * (50_000_000_000 - i));
        }
        prev = row.pots.clone();
    }
    Some(row.total())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(325));
    }
}
