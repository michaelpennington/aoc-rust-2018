use std::{collections::LinkedList, fmt::Display};

advent_of_code::solution!(5);

#[derive(Clone, PartialEq, Eq, Debug, Default)]
struct Polymer(LinkedList<char>);

impl From<&str> for Polymer {
    fn from(value: &str) -> Self {
        Self(value.chars().collect())
    }
}

impl Display for Polymer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in &self.0 {
            write!(f, "{c}")?;
        }
        Ok(())
    }
}

impl Polymer {
    fn dedup(&mut self) -> usize {
        let mut to_remove = Vec::new();
        let mut last_c = ' ';
        let mut just_removed = false;
        for (i, c) in self.0.iter().enumerate() {
            if just_removed {
                just_removed = false;
                last_c = *c;
                continue;
            }
            if (c.to_ascii_uppercase() == last_c && c.is_ascii_lowercase())
                || (c.to_ascii_lowercase() == last_c && c.is_ascii_uppercase())
            {
                to_remove.push(i);
                just_removed = true;
            }
            last_c = *c;
        }
        to_remove.sort_by_key(|i| std::cmp::Reverse(*i));
        for i in &to_remove {
            let mut rest = self.0.split_off(i + 1);
            self.0.pop_back();
            self.0.pop_back();
            self.0.append(&mut rest);
        }
        to_remove.len() * 2
    }

    fn remove(&self, c: char) -> Self {
        let mut v = self.0.iter().copied().collect::<Vec<_>>();
        v.retain(|&vc| vc != c && vc.to_ascii_lowercase() != c);
        Self(v.iter().copied().collect())
    }

    fn reduce(&mut self) -> usize {
        while self.dedup() > 0 {}
        self.len()
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut p: Polymer = input.trim().into();
    while p.dedup() > 0 {}
    Some(p.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let p: Polymer = input.trim().into();
    ('a'..='z')
        .map(|c| p.remove(c).reduce())
        .min()
        .map(|u| u as u32)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(10));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
