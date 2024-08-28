advent_of_code::solution!(9);

fn game(num_players: usize, last_marble: u32) -> u32 {
    let mut player_scores = vec![0; num_players];
    let mut marbles = vec![0];
    let mut current = 1;
    for marble in 1..=last_marble {
        if marble % 23 == 0 {
            let score = &mut player_scores[(marble as usize - 1) % num_players];
            current = (current + marbles.len() - 9) % marbles.len();
            *score += marble;
            *score += marbles.remove(current);
        } else {
            marbles.insert(current, marble);
        }
        current = ((current + 1) % marbles.len() + 1) % (marbles.len() + 1);
    }
    *player_scores.iter().max().unwrap()
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut pts = input.split_whitespace();
    let num_players = pts.next().unwrap().parse().unwrap();
    let last_marble = pts.nth(5).unwrap().parse().unwrap();
    Some(game(num_players, last_marble))
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut pts = input.split_whitespace();
    let num_players = pts.next().unwrap().parse().unwrap();
    let last_marble: u32 = pts.nth(5).unwrap().parse().unwrap();
    Some(game(num_players, last_marble * 100))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(32));
    }

    #[test]
    fn test_part_one_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(8317));
    }

    #[test]
    fn test_part_one_two() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(146373));
    }

    #[test]
    fn test_part_one_three() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(2764));
    }

    #[test]
    fn test_part_one_four() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(54718));
    }

    #[test]
    fn test_part_one_five() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 5,
        ));
        assert_eq!(result, Some(37305));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(22563));
    }
}
