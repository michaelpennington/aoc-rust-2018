use std::fmt::Display;

use strum::{Display, EnumString, FromRepr};

advent_of_code::solution!(7);

#[derive(Clone, Copy, PartialEq, Eq, EnumString, Hash, FromRepr, Debug, Display)]
enum Step {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    F = 5,
    G = 6,
    H = 7,
    I = 8,
    J = 9,
    K = 10,
    L = 11,
    M = 12,
    N = 13,
    O = 14,
    P = 15,
    Q = 16,
    R = 17,
    S = 18,
    T = 19,
    U = 20,
    V = 21,
    W = 22,
    X = 23,
    Y = 24,
    Z = 25,
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Requirements {
    requires: [Vec<Step>; 26],
    len: usize,
}

const NEW_VEC: Vec<Step> = Vec::new();

impl Default for Requirements {
    fn default() -> Self {
        Self {
            requires: [NEW_VEC; 26],
            len: 0,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Steps(Vec<Step>);

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
struct Worker {
    step: Step,
    time: u32,
}

impl Worker {
    fn step(&mut self) -> Option<Step> {
        if self.time == 0 {
            Some(self.step)
        } else {
            self.time -= 1;
            None
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
struct Workers([Option<Worker>; 5]);

impl Requirements {
    fn add(&mut self, s: &str) -> anyhow::Result<()> {
        let mut pts = s.split_whitespace();
        let requisite = pts.nth(1).unwrap().parse()?;
        let rfor: Step = pts.nth(5).unwrap().parse()?;
        self.requires[rfor as usize].push(requisite);
        self.len = self.len.max(requisite as usize + 1);
        Ok(())
    }

    fn compute(&mut self) -> Steps {
        let mut out = Vec::with_capacity(26);
        while let Some(next) = self.requires[..self.len]
            .iter()
            .enumerate()
            .find(|(i, ss)| ss.is_empty() && !out.contains(&Step::from_repr(*i).unwrap()))
            .map(|(i, _)| Step::from_repr(i).unwrap())
        {
            for ss in &mut self.requires {
                ss.retain(|&s| s != next);
            }
            out.push(next);
        }
        Steps(out)
    }

    fn compute_v2(&mut self) -> u32 {
        let mut done: Vec<Step> = Vec::with_capacity(26);
        let mut workers = Workers::default();
        for time in 0.. {
            if done.len() == self.len {
                return time - 1;
            }
            for worker in workers.0.iter_mut() {
                if let Some(w) = worker {
                    let res = w.step();
                    if let Some(st) = res {
                        for ss in &mut self.requires {
                            ss.retain(|&s| s != st);
                        }
                        done.push(st);
                        *worker = None
                    }
                }
            }
            loop {
                if !workers.0.iter().all(|w| w.is_some()) {
                    if let Some(next) = self.requires[..self.len]
                        .iter()
                        .enumerate()
                        .map(|(i, ss)| (Step::from_repr(i).unwrap(), ss))
                        .find(|(s, ss)| {
                            ss.is_empty()
                                && !done.contains(s)
                                && !workers.0.iter().flatten().any(|w| w.step == *s)
                        })
                        .map(|(s, _)| s)
                    {
                        let worker = workers.0.iter_mut().find(|w| w.is_none()).unwrap();
                        *worker = Some(Worker {
                            step: next,
                            time: next as u32 + 60,
                        })
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
        }
        0
    }
}

impl Display for Steps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for c in &self.0 {
            write!(f, "{c}")?;
        }
        Ok(())
    }
}

impl Display for Workers {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        for (i, w) in self.0.iter().enumerate() {
            if let Some(w) = w {
                write!(f, "{}: {:02}", w.step, w.time)?;
            } else {
                write!(f, ".:  0")?
            }
            if i <= 3 {
                write!(f, ", ")?;
            }
        }
        write!(f, "]")
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let mut reqs = Requirements::default();
    for line in input.lines() {
        reqs.add(line).unwrap();
    }
    Some(format!("{}", reqs.compute()))
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut reqs = Requirements::default();
    for line in input.lines() {
        reqs.add(line).unwrap();
    }
    Some(reqs.compute_v2())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some("CABDFE".into()));
    }
}
