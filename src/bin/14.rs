use std::{fmt::Display, ops::Add};

use anyhow::anyhow;

advent_of_code::solution!(14);

#[repr(u8)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Default, PartialOrd, Ord)]
enum Digit {
    #[default]
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
}

impl TryFrom<u8> for Digit {
    type Error = anyhow::Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Zero),
            1 => Ok(Self::One),
            2 => Ok(Self::Two),
            3 => Ok(Self::Three),
            4 => Ok(Self::Four),
            5 => Ok(Self::Five),
            6 => Ok(Self::Six),
            7 => Ok(Self::Seven),
            8 => Ok(Self::Eight),
            9 => Ok(Self::Nine),
            _ => Err(anyhow!("{value} is not a valid digit")),
        }
    }
}

impl TryFrom<char> for Digit {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '0' => Ok(Self::Zero),
            '1' => Ok(Self::One),
            '2' => Ok(Self::Two),
            '3' => Ok(Self::Three),
            '4' => Ok(Self::Four),
            '5' => Ok(Self::Five),
            '6' => Ok(Self::Six),
            '7' => Ok(Self::Seven),
            '8' => Ok(Self::Eight),
            '9' => Ok(Self::Nine),
            _ => Err(anyhow!("{value} is not a valid digit")),
        }
    }
}

impl Add<Digit> for Digit {
    type Output = (Digit, Option<Digit>);

    fn add(self, rhs: Digit) -> Self::Output {
        let (l, r) = if self > rhs { (rhs, self) } else { (self, rhs) };
        match (l, r) {
            (Digit::Zero, digit) => (digit, None),
            (Digit::One, Digit::One) => (Digit::Two, None),
            (Digit::One, Digit::Two) => (Digit::Three, None),
            (Digit::One, Digit::Three) => (Digit::Four, None),
            (Digit::One, Digit::Four) => (Digit::Five, None),
            (Digit::One, Digit::Five) => (Digit::Six, None),
            (Digit::One, Digit::Six) => (Digit::Seven, None),
            (Digit::One, Digit::Seven) => (Digit::Eight, None),
            (Digit::One, Digit::Eight) => (Digit::Nine, None),
            (Digit::One, Digit::Nine) => (Digit::One, Some(Digit::Zero)),
            (Digit::Two, Digit::Two) => (Digit::Four, None),
            (Digit::Two, Digit::Three) => (Digit::Five, None),
            (Digit::Two, Digit::Four) => (Digit::Six, None),
            (Digit::Two, Digit::Five) => (Digit::Seven, None),
            (Digit::Two, Digit::Six) => (Digit::Eight, None),
            (Digit::Two, Digit::Seven) => (Digit::Nine, None),
            (Digit::Two, Digit::Eight) => (Digit::One, Some(Digit::Zero)),
            (Digit::Two, Digit::Nine) => (Digit::One, Some(Digit::One)),
            (Digit::Three, Digit::Three) => (Digit::Six, None),
            (Digit::Three, Digit::Four) => (Digit::Seven, None),
            (Digit::Three, Digit::Five) => (Digit::Eight, None),
            (Digit::Three, Digit::Six) => (Digit::Nine, None),
            (Digit::Three, Digit::Seven) => (Digit::One, Some(Digit::Zero)),
            (Digit::Three, Digit::Eight) => (Digit::One, Some(Digit::One)),
            (Digit::Three, Digit::Nine) => (Digit::One, Some(Digit::Two)),
            (Digit::Four, Digit::Four) => (Digit::Eight, None),
            (Digit::Four, Digit::Five) => (Digit::Nine, None),
            (Digit::Four, Digit::Six) => (Digit::One, Some(Digit::Zero)),
            (Digit::Four, Digit::Seven) => (Digit::One, Some(Digit::One)),
            (Digit::Four, Digit::Eight) => (Digit::One, Some(Digit::Two)),
            (Digit::Four, Digit::Nine) => (Digit::One, Some(Digit::Three)),
            (Digit::Five, Digit::Five) => (Digit::One, Some(Digit::Zero)),
            (Digit::Five, Digit::Six) => (Digit::One, Some(Digit::One)),
            (Digit::Five, Digit::Seven) => (Digit::One, Some(Digit::Two)),
            (Digit::Five, Digit::Eight) => (Digit::One, Some(Digit::Three)),
            (Digit::Five, Digit::Nine) => (Digit::One, Some(Digit::Four)),
            (Digit::Six, Digit::Six) => (Digit::One, Some(Digit::Two)),
            (Digit::Six, Digit::Seven) => (Digit::One, Some(Digit::Three)),
            (Digit::Six, Digit::Eight) => (Digit::One, Some(Digit::Four)),
            (Digit::Six, Digit::Nine) => (Digit::One, Some(Digit::Five)),
            (Digit::Seven, Digit::Seven) => (Digit::One, Some(Digit::Four)),
            (Digit::Seven, Digit::Eight) => (Digit::One, Some(Digit::Five)),
            (Digit::Seven, Digit::Nine) => (Digit::One, Some(Digit::Six)),
            (Digit::Eight, Digit::Eight) => (Digit::One, Some(Digit::Six)),
            (Digit::Eight, Digit::Nine) => (Digit::One, Some(Digit::Seven)),
            (Digit::Nine, Digit::Nine) => (Digit::One, Some(Digit::Eight)),
            _ => unreachable!(),
        }
    }
}

impl Display for Digit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Digit::Zero => '0',
            Digit::One => '1',
            Digit::Two => '2',
            Digit::Three => '3',
            Digit::Four => '4',
            Digit::Five => '5',
            Digit::Six => '6',
            Digit::Seven => '7',
            Digit::Eight => '8',
            Digit::Nine => '9',
        };
        write!(f, "{c}")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Digits([Digit; 10]);

impl TryFrom<[u8; 10]> for Digits {
    type Error = anyhow::Error;

    fn try_from(value: [u8; 10]) -> Result<Self, Self::Error> {
        let mut out = Self::default();
        for (val, into) in value.into_iter().zip(&mut out.0) {
            *into = val.try_into()?;
        }
        Ok(out)
    }
}

impl TryFrom<&[u8]> for Digits {
    type Error = anyhow::Error;

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        let mut out = Self::default();
        for (&val, into) in value.iter().zip(&mut out.0) {
            *into = val.try_into()?;
        }
        Ok(out)
    }
}

impl Display for Digits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for d in &self.0 {
            write!(f, "{d}")?;
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Recipes {
    goal: usize,
    digits: Vec<Digit>,
    elf1: usize,
    elf2: usize,
}

impl Display for Recipes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for d in &self.digits {
            write!(f, "{d}")?;
        }
        Ok(())
    }
}

impl Recipes {
    fn with_goal(goal: usize) -> Self {
        let mut digits = Vec::with_capacity(goal + 20);
        digits.push(Digit::Three);
        digits.push(Digit::Seven);
        Self {
            goal,
            digits,
            elf1: 0,
            elf2: 1,
        }
    }

    fn cook(&mut self) -> Option<Digits> {
        let mut d_len = self.digits.len();
        while d_len < self.goal + 10 {
            let (d1, d2) = (self.digits[self.elf1], self.digits[self.elf2]);
            let (new, car) = d1 + d2;
            self.digits.push(new);
            if let Some(digi) = car {
                self.digits.push(digi);
            }
            d_len = self.digits.len();
            self.elf1 = (self.elf1 + d1 as usize + 1) % d_len;
            self.elf2 = (self.elf2 + d2 as usize + 1) % d_len;
        }
        self.digits[self.goal..self.goal + 10]
            .try_into()
            .map(Digits)
            .ok()
    }

    fn cook_v2(&mut self, looking_for: &[Digit]) -> usize {
        let mut d_len = self.digits.len();
        let llen = looking_for.len();
        loop {
            let (d1, d2) = (self.digits[self.elf1], self.digits[self.elf2]);
            let (new, car) = d1 + d2;
            self.digits.push(new);
            if d_len > llen
                && self.digits[self.digits.len() - llen..]
                    .iter()
                    .zip(looking_for)
                    .all(|(d1, d2)| d1 == d2)
            {
                return self.digits.len() - llen;
            }
            if let Some(digi) = car {
                self.digits.push(digi);
                if d_len > 6
                    && self.digits[self.digits.len() - llen..]
                        .iter()
                        .zip(looking_for)
                        .all(|(d1, d2)| d1 == d2)
                {
                    return self.digits.len() - llen;
                }
            }
            d_len = self.digits.len();
            self.elf1 = (self.elf1 + d1 as usize + 1) % d_len;
            self.elf2 = (self.elf2 + d2 as usize + 1) % d_len;
        }
    }
}

pub fn part_one(input: &str) -> Option<Digits> {
    let goal = input.trim().parse().unwrap();
    let mut recipes = Recipes::with_goal(goal);
    recipes.cook()
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut recipes = Recipes::with_goal(100_000);
    let goal = input
        .trim()
        .chars()
        .map(|c| c.try_into().unwrap())
        .collect::<Vec<_>>();
    Some(recipes.cook_v2(&goal))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(
            result,
            Some([5, 1, 5, 8, 9, 1, 6, 7, 7, 9].try_into().unwrap())
        );
    }

    #[test]
    fn test_part_one_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(
            result,
            Some([0, 1, 2, 4, 5, 1, 5, 8, 9, 1].try_into().unwrap())
        );
    }

    #[test]
    fn test_part_one_two() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(
            result,
            Some([9, 2, 5, 1, 0, 7, 1, 0, 8, 5].try_into().unwrap())
        );
    }

    #[test]
    fn test_part_one_three() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(
            result,
            Some([5, 9, 4, 1, 4, 2, 9, 8, 8, 2].try_into().unwrap())
        );
    }

    #[test]
    fn test_part_two_four() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(9));
    }

    #[test]
    fn test_part_two_five() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 5,
        ));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two_six() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 6,
        ));
        assert_eq!(result, Some(18));
    }

    #[test]
    fn test_part_two_seven() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 7,
        ));
        assert_eq!(result, Some(2018));
    }
}
