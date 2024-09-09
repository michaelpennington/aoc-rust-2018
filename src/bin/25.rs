use std::str::FromStr;

advent_of_code::solution!(25);

#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
struct Pt4([i32; 4]);

const RANGE: u32 = 3;

impl Pt4 {
    fn new(x: i32, y: i32, z: i32, t: i32) -> Self {
        Self([x, y, z, t])
    }

    fn d(&self, other: &Pt4) -> u32 {
        self.0[0].abs_diff(other.0[0])
            + self.0[1].abs_diff(other.0[1])
            + self.0[2].abs_diff(other.0[2])
            + self.0[3].abs_diff(other.0[3])
    }

    fn within_range(&self, others: &[Pt4]) -> bool {
        others.iter().any(|o| self.d(o) <= RANGE)
    }

    fn within_range_of(&self, others: &[Vec<Pt4>]) -> Vec<usize> {
        others
            .iter()
            .enumerate()
            .filter(move |(_, os)| self.within_range(os))
            .map(move |(i, _)| i)
            .collect()
    }
}

impl FromStr for Pt4 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pts = s.trim().split(',');
        let x = pts.next().unwrap().parse()?;
        let y = pts.next().unwrap().parse()?;
        let z = pts.next().unwrap().parse()?;
        let t = pts.next().unwrap().parse()?;
        Ok(Pt4::new(x, y, z, t))
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut pts = Vec::new();
    for pt in input.lines().map(|l| l.parse::<Pt4>().unwrap()) {
        let mut range = pt.within_range_of(&pts).into_iter();
        if let Some(first) = range.next() {
            pts[first].push(pt);
            for next in range.rev() {
                let cons = pts.remove(next);
                pts[first].extend(cons);
            }
        } else {
            pts.push(vec![pt]);
        }
    }
    Some(pts.len() as u32)
}

pub fn part_two(_input: &str) -> Option<u32> {
    Some(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_one_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(4));
    }

    #[test]
    fn test_part_one_two() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(3));
    }

    #[test]
    fn test_part_one_three() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(8));
    }
}
