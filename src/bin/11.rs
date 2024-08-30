use std::{
    fmt::Display,
    ops::{Add, Index, IndexMut},
};

advent_of_code::solution!(11);

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Default)]
pub struct Pt {
    x: usize,
    y: usize,
}

impl Add<Pt> for Pt {
    type Output = Pt;

    fn add(self, rhs: Pt) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Default)]
pub struct PtN {
    x: usize,
    y: usize,
    n: usize,
}

impl Display for PtN {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.x, self.y, self.n)
    }
}

const THREEXTHREE: [Pt; 9] = [
    Pt { x: 0, y: 0 },
    Pt { x: 1, y: 0 },
    Pt { x: 2, y: 0 },
    Pt { x: 0, y: 1 },
    Pt { x: 1, y: 1 },
    Pt { x: 2, y: 1 },
    Pt { x: 0, y: 2 },
    Pt { x: 1, y: 2 },
    Pt { x: 2, y: 2 },
];

impl Pt {
    fn power_level(&self, serial_number: usize) -> isize {
        let rack_id = self.x + 10;
        let mut power_level = self.y * rack_id;
        power_level += serial_number;
        power_level *= rack_id;
        power_level = (power_level / 100) % 10;
        power_level as isize - 5
    }

    fn square(&self) -> impl Iterator<Item = Pt> + '_ {
        THREEXTHREE.iter().map(|inc| *self + *inc)
    }

    fn n_square(&self, n: usize) -> impl Iterator<Item = Pt> + '_ {
        (0..n)
            .flat_map(move |x| (0..n).map(move |y| Pt { x, y }))
            .map(|p| p + *self)
    }
}

impl Display for Pt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Grid(Vec<Vec<isize>>);

impl Default for Grid {
    fn default() -> Self {
        Self(vec![vec![0; 300]; 300])
    }
}

impl Index<Pt> for Grid {
    type Output = isize;

    fn index(&self, index: Pt) -> &Self::Output {
        &self.0[index.x - 1][index.y - 1]
    }
}

impl IndexMut<Pt> for Grid {
    fn index_mut(&mut self, index: Pt) -> &mut Self::Output {
        &mut self.0[index.x - 1][index.y - 1]
    }
}

impl Grid {
    fn new(serial_number: usize) -> Self {
        let mut out = Self::default();
        for pt in Self::pts_iter() {
            out[pt] = pt.power_level(serial_number);
        }
        out
    }

    fn pts_iter() -> impl Iterator<Item = Pt> {
        (1..=300).flat_map(|x| (1..=300).map(move |y| Pt { x, y }))
    }

    fn pts_iter_upper_left() -> impl Iterator<Item = Pt> {
        (1..=298).flat_map(|x| (1..=298).map(move |y| Pt { x, y }))
    }

    fn pts_iter_upper_n(n: usize) -> impl Iterator<Item = Pt> {
        (1..=n).flat_map(move |x| (1..=n).map(move |y| Pt { x, y }))
    }

    fn square_score(&self, pt: &Pt) -> isize {
        pt.square().map(|p| self[p]).sum()
    }

    fn n_square_score(&self, pt: &Pt, n: usize) -> isize {
        pt.n_square(n).map(|p| self[p]).sum()
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for line in &self.0 {
            for n in line {
                write!(f, "{n:2} ")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn part_one(input: &str) -> Option<Pt> {
    let serial_number = input.trim().parse().unwrap();
    let grid = Grid::new(serial_number);
    Grid::pts_iter_upper_left().max_by_key(|p| grid.square_score(p))
}

pub fn part_two(input: &str) -> Option<PtN> {
    let serial_number = input.trim().parse().unwrap();
    let grid = Grid::new(serial_number);
    (1..=300)
        .flat_map(|n| Grid::pts_iter_upper_n(300 - dbg!(n) + 1).zip(std::iter::repeat(n)))
        .max_by_key(|(p, n)| grid.n_square_score(p, *n))
        .map(|(p, n)| PtN { x: p.x, y: p.y, n })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(Pt { x: 33, y: 45 }));
    }

    #[test]
    fn test_part_one_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(Pt { x: 21, y: 61 }));
    }

    //  Too Slow::
    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(
    //         result,
    //         Some(PtN {
    //             x: 90,
    //             y: 269,
    //             n: 16
    //         })
    //     );
    // }
    //
    // #[test]
    // fn test_part_two_one() {
    //     let result = part_two(&advent_of_code::template::read_file_part(
    //         "examples", DAY, 1,
    //     ));
    //     assert_eq!(
    //         result,
    //         Some(PtN {
    //             x: 232,
    //             y: 251,
    //             n: 12
    //         })
    //     );
    // }
}
