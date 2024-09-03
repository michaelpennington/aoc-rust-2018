use std::{
    collections::HashMap,
    ops::{Add, Index, IndexMut},
    str::FromStr,
};

use anyhow::anyhow;

advent_of_code::solution!(18);

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum Acre {
    Open,
    Trees,
    Yard,
}

impl Acre {
    fn next(&self, counts: NeighborCounts) -> Self {
        match (self, counts) {
            (Self::Open, NeighborCounts { trees, .. }) if trees >= 3 => Self::Trees,
            (Self::Trees, NeighborCounts { yard, .. }) if yard >= 3 => Self::Yard,
            (Self::Yard, NeighborCounts { yard, trees, .. }) if yard == 0 || trees == 0 => {
                Self::Open
            }
            _ => *self,
        }
    }
}

impl TryFrom<char> for Acre {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Open),
            '|' => Ok(Self::Trees),
            '#' => Ok(Self::Yard),
            _ => Err(anyhow!("{value} is not a valid acre")),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Pt {
    x: usize,
    y: usize,
}

impl Index<Pt> for LumberYard {
    type Output = Acre;

    fn index(&self, index: Pt) -> &Self::Output {
        assert!(index.y < self.height && index.x < self.width);
        &self.yard[index.y][index.x]
    }
}

impl IndexMut<Pt> for LumberYard {
    fn index_mut(&mut self, index: Pt) -> &mut Self::Output {
        assert!(index.y < self.height && index.x < self.width);
        &mut self.yard[index.y][index.x]
    }
}

#[derive(Clone, PartialEq, Eq, Debug, Hash)]
struct LumberYard {
    yard: Vec<Vec<Acre>>,
    width: usize,
    height: usize,
}

impl FromStr for LumberYard {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut yard = Vec::with_capacity(50);
        let mut width = None;
        for line in s.lines() {
            let mut row = Vec::with_capacity(50);
            for c in line.chars() {
                row.push(c.try_into()?);
            }
            if let Some(w) = width {
                assert_eq!(w, row.len());
            } else {
                width = Some(row.len());
            }
            yard.push(row);
        }

        let height = yard.len();
        Ok(Self {
            yard,
            width: width.unwrap(),
            height,
        })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
struct NeighborCounts {
    open: u8,
    trees: u8,
    yard: u8,
}

impl LumberYard {
    fn neighbors(&self, pt: Pt) -> impl Iterator<Item = &Acre> + '_ {
        assert!(pt.y < self.height && pt.x < self.width);
        (pt.y.saturating_sub(1)..pt.y.add(2).min(self.height))
            .flat_map(move |y| {
                (pt.x.saturating_sub(1)..pt.x.add(2).min(self.width)).map(move |x| Pt { x, y })
            })
            .filter(move |&p| p.x != pt.x || p.y != pt.y)
            .map(|pt| &self[pt])
    }

    fn neighbor_count(&self, pt: Pt) -> NeighborCounts {
        let mut counts = NeighborCounts::default();
        for n in self.neighbors(pt) {
            match n {
                Acre::Open => counts.open += 1,
                Acre::Trees => counts.trees += 1,
                Acre::Yard => counts.yard += 1,
            }
        }
        counts
    }

    fn process_steps(&mut self, num: usize) {
        let mut temporary = self.clone();
        for _ in 0..num {
            for (pt, a) in (0..self.height)
                .flat_map(|y| (0..self.width).map(move |x| Pt { x, y }))
                .map(|p| (p, self[p]))
            {
                temporary[pt] = a.next(self.neighbor_count(pt));
            }
            std::mem::swap(self, &mut temporary);
        }
    }

    fn process_steps_smarter(&mut self, num: usize) {
        let mut temporary = self.clone();
        let mut seen = HashMap::new();
        for i in 0..num {
            if let Some(already) = seen.insert(self.yard.clone(), i) {
                let remaining = num - i;
                let cycle_len = i - already;
                self.process_steps(remaining % cycle_len);
                return;
            }
            for (pt, a) in (0..self.height)
                .flat_map(|y| (0..self.width).map(move |x| Pt { x, y }))
                .map(|p| (p, self[p]))
            {
                temporary[pt] = a.next(self.neighbor_count(pt));
            }
            std::mem::swap(self, &mut temporary);
        }
    }

    fn resource_value(&self) -> u32 {
        (self
            .yard
            .iter()
            .flat_map(|i| i.iter())
            .filter(|&&a| a == Acre::Trees)
            .count()
            * self
                .yard
                .iter()
                .flat_map(|i| i.iter())
                .filter(|&&a| a == Acre::Yard)
                .count()) as u32
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut yard = input.parse::<LumberYard>().unwrap();
    yard.process_steps(10);
    Some(yard.resource_value())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut yard = input.parse::<LumberYard>().unwrap();
    yard.process_steps_smarter(1_000_000_000);
    Some(yard.resource_value())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1147));
    }
}
