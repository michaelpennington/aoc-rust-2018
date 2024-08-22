use std::collections::HashSet;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<i32> {
    Some(input.lines().map(|l| l.parse::<i32>().unwrap()).sum())
}

pub fn part_two(input: &str) -> Option<i32> {
    let mut total = 0;
    let mut seen = HashSet::new();
    seen.insert(total);
    for num in input.lines().map(|l| l.parse::<i32>().unwrap()).cycle() {
        total += num;
        if !seen.insert(total) {
            return Some(total);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(3));
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
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_one_four() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(-6));
    }

    #[test]
    fn test_part_two_one() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two_five() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 5,
        ));
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_two_six() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 6,
        ));
        assert_eq!(result, Some(10));
    }

    #[test]
    fn test_part_two_seven() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 7,
        ));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two_eight() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 8,
        ));
        assert_eq!(result, Some(14));
    }
}
