advent_of_code::solution!(4);

use std::{collections::HashMap, str::FromStr};

use chrono::{Days, NaiveDate, NaiveDateTime, NaiveTime, Timelike};

#[derive(Clone, PartialEq, Eq, Debug)]
struct Guard {
    date: NaiveDate,
    sleep_times: Vec<(u32, u32)>,
    id: u32,
}

impl FromStr for Guard {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sleep_times = Vec::new();
        let (time, next) = s.split_at(18);
        let time = time.trim();
        let next = next.trim();
        let date = NaiveDateTime::parse_from_str(time, "[%Y-%m-%d %H:%M]")?;
        let date = if date.time() > NaiveTime::from_hms_opt(23, 0, 0).unwrap() {
            (date + Days::new(1)).date()
        } else {
            date.date()
        };
        let id = next
            .split_whitespace()
            .nth(1)
            .unwrap()
            .trim_start_matches('#')
            .parse()?;
        Ok(Self {
            date,
            sleep_times,
            id,
        })
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Guards {
    map: HashMap<u32, Vec<Guard>>,
}

impl Guard {
    fn add(&mut self, s: &str) {
        let (time, next) = s.split_at(18);
        let time = time.trim();
        let datetime = NaiveDateTime::parse_from_str(time, "[%Y-%m-%d %H:%M]").unwrap();
        assert!(datetime.date() == self.date);
        if next.contains("wakes up") {
            self.sleep_times.last_mut().unwrap().1 = datetime.minute();
        } else {
            self.sleep_times.push((datetime.minute(), 0))
        }
    }

    fn time_asleep(&self) -> u32 {
        self.sleep_times.iter().map(|s| s.1 - s.0).sum()
    }

    fn asleep_at(&self, time: u32) -> bool {
        self.sleep_times.iter().any(|s| s.0 <= time && time < s.1)
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut lines = input.lines().collect::<Vec<_>>();
    lines.sort();
    let mut guards = Guards {
        map: HashMap::new(),
    };
    let mut new_guard: Option<Guard> = None;
    for line in &lines {
        if line.contains('#') {
            if let Some(guard) = new_guard {
                if let Some(gs) = guards.map.get_mut(&guard.id) {
                    gs.push(guard);
                } else {
                    guards.map.insert(guard.id, vec![guard]);
                }
            }
            new_guard = Some(Guard::from_str(line).unwrap());
        } else if let Some(g) = &mut new_guard {
            g.add(line);
        }
    }
    if let Some(guard) = new_guard {
        if let Some(gs) = guards.map.get_mut(&guard.id) {
            gs.push(guard);
        } else {
            guards.map.insert(guard.id, vec![guard]);
        }
    }
    let most_asleep = guards
        .map
        .iter()
        .map(|(id, gs)| (id, gs.iter().map(|g| g.time_asleep()).sum::<u32>()))
        .max_by_key(|(_, gs)| *gs)
        .unwrap()
        .0;
    let max_time = (0..60)
        .map(|t| {
            (
                t,
                guards.map[most_asleep]
                    .iter()
                    .filter(|g| g.asleep_at(t))
                    .count(),
            )
        })
        .max_by_key(|(_, ts)| *ts)
        .unwrap()
        .0;
    Some(most_asleep * max_time)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut lines = input.lines().collect::<Vec<_>>();
    lines.sort();
    let mut guards = Guards {
        map: HashMap::new(),
    };
    let mut new_guard: Option<Guard> = None;
    for line in &lines {
        if line.contains('#') {
            if let Some(guard) = new_guard {
                if let Some(gs) = guards.map.get_mut(&guard.id) {
                    gs.push(guard);
                } else {
                    guards.map.insert(guard.id, vec![guard]);
                }
            }
            new_guard = Some(Guard::from_str(line).unwrap());
        } else if let Some(g) = &mut new_guard {
            g.add(line);
        }
    }
    if let Some(guard) = new_guard {
        if let Some(gs) = guards.map.get_mut(&guard.id) {
            gs.push(guard);
        } else {
            guards.map.insert(guard.id, vec![guard]);
        }
    }
    let (id, max_time) = guards
        .map
        .iter()
        .flat_map(|(id, gs)| {
            (0..60).map(move |t| (id, t, gs.iter().filter(|g| g.asleep_at(t)).count()))
        })
        .max_by_key(|(_, _, ts)| *ts)
        .map(|(id, t, _)| (id, t))
        .unwrap();
    Some(id * max_time)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(240));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4455));
    }
}
