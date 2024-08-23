use std::{collections::HashMap, str::FromStr};

advent_of_code::solution!(2);

#[derive(Debug, Clone, PartialEq, Eq)]
struct Id(HashMap<char, u32>);

impl FromStr for Id {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = HashMap::with_capacity(s.len());
        for c in s.chars() {
            map.entry(c).and_modify(|e| *e += 1).or_insert(1);
        }
        Ok(Self(map))
    }
}

impl Id {
    fn has_double(&self) -> bool {
        self.0.values().any(|v| *v == 2)
    }

    fn has_triple(&self) -> bool {
        self.0.values().any(|v| *v == 3)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (doubles, triples) = input
        .lines()
        .map(|l| l.parse::<Id>().unwrap())
        .map(|l| (l.has_double(), l.has_triple()))
        .fold((0, 0), |(acc0, acc1), (ds, ts)| {
            let acc0 = if ds { acc0 + 1 } else { acc0 };
            let acc1 = if ts { acc1 + 1 } else { acc1 };
            (acc0, acc1)
        });
    Some(doubles * triples)
}

pub fn part_two(input: &str) -> Option<String> {
    let lines = input.lines().collect::<Vec<_>>();
    for (i, line1) in lines.iter().enumerate() {
        for line2 in lines[i + 1..].iter() {
            for i in 0..line1.len() {
                if line1[..i] == line2[..i] && line1[i + 1..] == line2[i + 1..] {
                    return Some(format!("{}{}", &line1[..i], &line1[i + 1..]));
                }
            }
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
        assert_eq!(result, Some(12));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some("fgij".into()));
    }
}
