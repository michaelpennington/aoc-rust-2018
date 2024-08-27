advent_of_code::solution!(8);

fn get_meta(nums: &[u32]) -> (u32, u32, &[u32]) {
    // dbg!(&nums);
    let num_children = nums[0];
    let num_meta = nums[1];
    let mut data = &nums[2..];
    let mut totals = 0;
    let mut total;
    let mut scores = Vec::new();
    let mut score;
    for _ in 0..num_children {
        (total, score, data) = get_meta(data);
        totals += total;
        scores.push(score);
    }
    totals += data[..num_meta as usize].iter().sum::<u32>();
    if num_children == 0 {
        (
            totals,
            data[..num_meta as usize].iter().sum(),
            &data[num_meta as usize..],
        )
    } else {
        (
            totals,
            data[..num_meta as usize]
                .iter()
                .filter(|&&k| k > 0 && k as usize <= scores.len())
                .map(|&k| scores[k as usize - 1])
                .sum(),
            &data[num_meta as usize..],
        )
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let nums = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>();
    let (total, _, _) = get_meta(&nums);
    Some(total)
}

pub fn part_two(input: &str) -> Option<u32> {
    let nums = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<_>>();
    let (_, score, _) = get_meta(&nums);
    Some(score)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(138));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(66));
    }
}
