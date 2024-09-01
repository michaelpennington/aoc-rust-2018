use std::{
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fmt::Display,
    ops::{Add, Index, IndexMut},
    str::FromStr,
};

use anyhow::anyhow;

advent_of_code::solution!(15);

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
struct Pt(usize, usize);

impl PartialOrd for Pt {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Pt {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.1.cmp(&other.1).then_with(|| self.0.cmp(&other.0))
    }
}

impl Pt {
    fn h(&self, other: Pt) -> usize {
        self.0.abs_diff(other.0) + self.1.abs_diff(other.1)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, Default)]
enum Square {
    #[default]
    Open,
    Wall,
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Map(Vec<Vec<Square>>);

impl Index<Pt> for Map {
    type Output = Square;

    fn index(&self, index: Pt) -> &Self::Output {
        &self.0[index.1][index.0]
    }
}

impl IndexMut<Pt> for Map {
    fn index_mut(&mut self, index: Pt) -> &mut Self::Output {
        &mut self.0[index.1][index.0]
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum Race {
    Goblin,
    Elf,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
struct Player {
    race: Race,
    location: Pt,
    hp: u8,
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Battle {
    map: &'static Map,
    width: usize,
    height: usize,
    players: Vec<Player>,
    player_map: HashMap<Pt, usize>,
    killed_players: Vec<usize>,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
struct Node {
    p: Pt,
    score: usize,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.score.cmp(&other.score).reverse()
    }
}

impl Battle {
    fn connected(&self, p1: Pt, p2: Pt) -> bool {
        let mut to_do = VecDeque::new();
        let mut done = HashSet::new();
        to_do.push_back(p1);
        while let Some(p) = to_do.pop_back() {
            done.insert(p);
            for node in self.neighbors(p) {
                if node == p2 {
                    return true;
                }
                if !done.contains(&node) {
                    to_do.push_back(node);
                }
            }
        }
        false
    }

    fn neighbors(&self, pt: Pt) -> impl Iterator<Item = Pt> + '_ {
        (pt.0.saturating_sub(1)..=pt.0.add(1).min(self.height - 1))
            .flat_map(move |x| {
                (pt.1.saturating_sub(1)..=pt.1.add(1).min(self.width - 1)).map(move |y| Pt(x, y))
            })
            .filter(move |&p| {
                self.map[p] != Square::Wall
                    && !self.player_map.contains_key(&p)
                    && ((p.0 == pt.0 && p.1 != pt.1) || (p.1 == pt.1 && p.0 != pt.0))
            })
    }

    fn next_move(&self, player: Player) -> Option<Pt> {
        let target = match player.race {
            Race::Goblin => Race::Elf,
            Race::Elf => Race::Goblin,
        };
        self.players
            .iter()
            .enumerate()
            .filter(|(i, pl)| pl.race == target && !self.killed_players.contains(i))
            .flat_map(|pl| self.neighbors(pl.1.location))
            .filter_map(|pt| {
                self.neighbors(player.location)
                    .filter_map(|n| self.a_star(n, pt).map(|v| (n, pt, v)))
                    .min_by(|(n1, _, v1), (n2, _, v2)| {
                        v1.len().cmp(&v2.len()).then_with(|| n1.cmp(n2))
                    })
            })
            .min_by(|(_, pt1, v1), (_, pt2, v2)| v1.len().cmp(&v2.len()).then_with(|| pt1.cmp(pt2)))
            .map(|(n, _, _)| n)
    }

    fn neighboring_enemy(&self, player: Player) -> Option<Pt> {
        let target = match player.race {
            Race::Goblin => Race::Elf,
            Race::Elf => Race::Goblin,
        };
        (player.location.0.saturating_sub(1)..=player.location.0.add(1).min(self.height - 1))
            .flat_map(move |x| {
                (player.location.1.saturating_sub(1)..=player.location.1.add(1).min(self.width - 1))
                    .map(move |y| Pt(x, y))
            })
            .filter(move |&p| {
                ((p.0 == player.location.0 && p.1 != player.location.1)
                    || (p.1 == player.location.1 && p.0 != player.location.0))
                    && self
                        .player_map
                        .get(&p)
                        .is_some_and(|&pi| self.players[pi].race == target)
            })
            .min_by(|p1, p2| {
                self.players[self.player_map[p1]]
                    .hp
                    .cmp(&self.players[self.player_map[p2]].hp)
                    .then_with(|| p1.cmp(p2))
            })
    }

    fn a_star(&self, start: Pt, goal: Pt) -> Option<Vec<Pt>> {
        if start == goal {
            return Some(vec![goal]);
        }
        if !self.connected(start, goal) {
            return None;
        }
        let h = |p| goal.h(p);
        let mut open_set = BinaryHeap::new();
        open_set.push(Node {
            p: start,
            score: h(start),
        });
        let mut came_from: HashMap<Pt, Pt> = HashMap::new();
        let mut g_score = HashMap::new();
        g_score.insert(start, 0);
        while let Some(current) = open_set.pop() {
            if current.p == goal {
                let mut current = current.p;
                let mut out = vec![current];
                while let Some(new_current) = came_from.get(&current) {
                    out.push(*new_current);
                    current = *new_current;
                }
                out.reverse();
                return Some(out);
            }
            for neighbor in self.neighbors(current.p) {
                let tentative_g_score = g_score[&current.p] + 1;
                if tentative_g_score < *g_score.get(&neighbor).unwrap_or(&usize::MAX) {
                    came_from.insert(neighbor, current.p);
                    g_score.insert(neighbor, tentative_g_score);
                    open_set.push(Node {
                        p: neighbor,
                        score: tentative_g_score + h(neighbor),
                    });
                }
            }
        }
        None
    }

    fn check_if_done(&self) -> Option<(Race, u32)> {
        if self
            .players
            .iter()
            .enumerate()
            .filter(|(i, _)| !self.killed_players.contains(i))
            .all(|(_, p)| p.race == Race::Goblin)
        {
            println!("Goblins win!");
            Some((
                Race::Goblin,
                self.players
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| !self.killed_players.contains(i))
                    .map(|(_, p)| p.hp as u32)
                    .sum::<u32>(),
            ))
        } else if self
            .players
            .iter()
            .enumerate()
            .filter(|(i, _)| !self.killed_players.contains(i))
            .all(|(_, p)| p.race == Race::Elf)
        {
            println!("Elves win!");
            Some((
                Race::Elf,
                self.players
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| !self.killed_players.contains(i))
                    .map(|(_, p)| p.hp as u32)
                    .sum::<u32>(),
            ))
        } else {
            None
        }
    }

    fn play(&mut self) -> u32 {
        for round in 0.. {
            println!("Round {round}:\n{self}");
            let mut indices = (0..self.players.len())
                .filter(|i| !self.killed_players.contains(i))
                .collect::<Vec<_>>();
            indices.sort_by_key(|i| self.players[*i].location);
            for &i in &indices {
                if self.killed_players.contains(&i) {
                    continue;
                }
                if let Some((_, total_hp)) = self.check_if_done() {
                    return total_hp * round;
                }
                let player = &self.players[i];
                if let Some(neighbor) = self.neighboring_enemy(*player) {
                    let neighborp = &mut self.players[self.player_map[&neighbor]];
                    if let Some(hp) = neighborp.hp.checked_sub(3) {
                        neighborp.hp = hp;
                    } else {
                        neighborp.hp = 0;
                        self.killed_players
                            .push(self.player_map.remove(&neighbor).unwrap());
                        self.player_map.remove(&neighborp.location);
                    }
                } else if let Some(next_loc) = self.next_move(*player) {
                    self.player_map.remove(&self.players[i].location);
                    self.players[i].location = next_loc;
                    self.player_map.insert(next_loc, i);
                    let player = &self.players[i];
                    if let Some(neighbor) = self.neighboring_enemy(*player) {
                        let neighborp = &mut self.players[self.player_map[&neighbor]];
                        if let Some(hp) = neighborp.hp.checked_sub(3) {
                            neighborp.hp = hp;
                        } else {
                            self.killed_players
                                .push(self.player_map.remove(&neighbor).unwrap());
                            self.player_map.remove(&neighborp.location);
                        }
                    }
                }
            }
        }
        0
    }

    fn play_with_attack_power(&mut self, power: u8) -> Option<u32> {
        for round in 0.. {
            println!("Round {round}:\n{self}");
            let mut indices = (0..self.players.len())
                .filter(|i| !self.killed_players.contains(i))
                .collect::<Vec<_>>();
            indices.sort_by_key(|i| self.players[*i].location);
            for &i in &indices {
                if let Some((race, total_hp)) = self.check_if_done() {
                    match race {
                        Race::Goblin => return None,
                        Race::Elf => return Some(total_hp * round),
                    }
                }
                if self.killed_players.contains(&i) {
                    continue;
                }
                let player = &self.players[i];
                let power = if player.race == Race::Elf { power } else { 3 };
                if let Some(neighbor) = self.neighboring_enemy(*player) {
                    let neighborp = &mut self.players[self.player_map[&neighbor]];
                    if let Some(hp) = neighborp
                        .hp
                        .checked_sub(power)
                        .and_then(|hp| (hp != 0).then_some(hp))
                    {
                        neighborp.hp = hp;
                    } else {
                        if neighborp.race == Race::Elf {
                            return None;
                        }
                        neighborp.hp = 0;
                        self.killed_players
                            .push(self.player_map.remove(&neighbor).unwrap());
                        self.player_map.remove(&neighborp.location);
                    }
                } else if let Some(next_loc) = self.next_move(*player) {
                    self.player_map.remove(&self.players[i].location);
                    self.players[i].location = next_loc;
                    self.player_map.insert(next_loc, i);
                    let player = &self.players[i];
                    if let Some(neighbor) = self.neighboring_enemy(*player) {
                        let neighborp = &mut self.players[self.player_map[&neighbor]];
                        if let Some(hp) = neighborp
                            .hp
                            .checked_sub(power)
                            .and_then(|hp| (hp != 0).then_some(hp))
                        {
                            neighborp.hp = hp;
                        } else {
                            if neighborp.race == Race::Elf {
                                return None;
                            }
                            neighborp.hp = 0;
                            self.killed_players
                                .push(self.player_map.remove(&neighbor).unwrap());
                            self.player_map.remove(&neighborp.location);
                        }
                    }
                }
            }
        }
        None
    }
}

impl FromStr for Battle {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let height = s.lines().count();
        let width = s.lines().next().unwrap().len();
        let mut map = Box::new(Map(vec![Vec::with_capacity(width); height]));
        let mut players = Vec::new();
        for ((y, line), l) in s.lines().enumerate().zip(&mut map.0) {
            for (x, c) in line.char_indices() {
                match c {
                    '#' => l.push(Square::Wall),
                    '.' => l.push(Square::Open),
                    'G' => {
                        l.push(Square::Open);
                        players.push(Player {
                            race: Race::Goblin,
                            location: Pt(x, y),
                            hp: 200,
                        });
                    }
                    'E' => {
                        l.push(Square::Open);
                        players.push(Player {
                            race: Race::Elf,
                            location: Pt(x, y),
                            hp: 200,
                        });
                    }
                    _ => return Err(anyhow!("{c} is not a valid token")),
                }
            }
        }
        let map = Box::leak(map);
        let mut player_map = HashMap::with_capacity(players.len());
        for (index, player) in players.iter().enumerate() {
            player_map.insert(player.location, index);
        }
        let killed_players = Vec::new();
        Ok(Self {
            map,
            width,
            height,
            players,
            player_map,
            killed_players,
        })
    }
}

impl Display for Battle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (y, line) in self.map.0.iter().enumerate() {
            for (x, c) in line.iter().enumerate() {
                let c = if c == &Square::Wall {
                    '#'
                } else if let Some(p) = self
                    .players
                    .iter()
                    .enumerate()
                    .find(|(i, p)| p.location == Pt(x, y) && !self.killed_players.contains(i))
                {
                    match p.1.race {
                        Race::Goblin => 'G',
                        Race::Elf => 'E',
                    }
                } else {
                    '.'
                };
                write!(f, "{c}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut battle = input.trim().parse::<Battle>().unwrap();
    Some(battle.play())
}

pub fn part_two(input: &str) -> Option<u32> {
    let battle = input.trim().parse::<Battle>().unwrap();
    (4..).find_map(|i| battle.clone().play_with_attack_power(i))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(27730));
    }

    #[test]
    fn test_part_one_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(36334));
    }

    #[test]
    fn test_part_one_two() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(39514));
    }

    #[test]
    fn test_part_one_three() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(27755));
    }

    #[test]
    fn test_part_one_four() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(28944));
    }

    #[test]
    fn test_part_one_five() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 5,
        ));
        assert_eq!(result, Some(18740));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4988));
    }

    #[test]
    fn test_part_two_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(31284));
    }

    #[test]
    fn test_part_two_three() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 3,
        ));
        assert_eq!(result, Some(3478));
    }

    #[test]
    fn test_part_two_four() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 4,
        ));
        assert_eq!(result, Some(6474));
    }

    #[test]
    fn test_part_two_five() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 5,
        ));
        assert_eq!(result, Some(1140));
    }
}
