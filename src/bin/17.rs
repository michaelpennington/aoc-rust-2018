use std::{collections::HashMap, fmt::Display, str::FromStr};

use anyhow::anyhow;

advent_of_code::solution!(17);

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
struct Pt {
    x: usize,
    y: usize,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
enum Soil {
    Clay,
    WaterDown,
    WaterSideways,
}

impl Soil {
    fn is_water(&self) -> bool {
        matches!(self, Soil::WaterDown | Soil::WaterSideways)
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Clay {
    map: HashMap<Pt, Soil>,
    min: Pt,
    max: Pt,
}

impl FromStr for Clay {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut map = HashMap::new();
        let err = |s: &str| anyhow!("Malformed input {s}");
        let mut min = Pt {
            x: usize::MAX,
            y: usize::MAX,
        };
        let mut max = Pt {
            x: usize::MIN,
            y: usize::MIN,
        };
        for line in s.lines() {
            let (pt1, pt2) = line.split_once(", ").ok_or(err(line))?;
            let (var, val) = pt1.split_once('=').ok_or(err(pt1))?;
            match var {
                "x" => {
                    let x = val.parse()?;
                    let (_, range) = pt2.split_once('=').ok_or(err(pt2))?;
                    let (from, to) = range
                        .split_once("..")
                        .ok_or(err(range))
                        .and_then(|(from, to)| Ok((from.parse()?, to.parse()?)))?;
                    for y in from..=to {
                        min.x = min.x.min(x);
                        min.y = min.y.min(y);
                        max.x = max.x.max(x);
                        max.y = max.y.max(y);
                        map.insert(Pt { x, y }, Soil::Clay);
                    }
                }
                "y" => {
                    let y = val.parse()?;
                    let (_, range) = pt2.split_once('=').ok_or(err(pt2))?;
                    let (from, to) = range
                        .split_once("..")
                        .ok_or(err(range))
                        .and_then(|(from, to)| Ok((from.parse()?, to.parse()?)))?;
                    for x in from..=to {
                        map.insert(Pt { x, y }, Soil::Clay);
                    }
                }
                _ => return Err(err(var)),
            }
        }
        Ok(Self { map, max, min })
    }
}

impl Display for Clay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in self.min.y..=self.max.y {
            for x in self.min.x..=self.max.x {
                let c = if (x, y) == (500, 0) {
                    '+'
                } else {
                    match self.map.get(&Pt { x, y }) {
                        Some(Soil::WaterDown) => '|',
                        Some(Soil::WaterSideways) => '~',
                        Some(Soil::Clay) => '#',
                        None => '.',
                    }
                };
                write!(f, "{c}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Bounded {
    BothSides,
    ToLeft,
    ToRight,
    Unbounded,
}

impl Clay {
    fn fill(&mut self) {
        self.map.insert(
            Pt {
                x: 500,
                y: self.min.y,
            },
            Soil::WaterDown,
        );
        let mut todo_down = vec![Pt {
            x: 500,
            y: self.min.y,
        }];
        let mut todo_sideways = Vec::new();
        loop {
            if todo_down.is_empty() {
                return;
            }
            for w in todo_down.drain(..) {
                let mut it = self.vertical_to(w).peekable();
                while let Some(y) = it.next() {
                    if it.peek().is_none() && y != self.max.y {
                        if self
                            .map
                            .get(&Pt { x: w.x, y: y + 1 })
                            .is_some_and(|&c| c == Soil::Clay)
                            || self.find_span(Pt { x: w.x, y }).0 == Bounded::BothSides
                        {
                            self.map.insert(Pt { x: w.x, y }, Soil::WaterSideways);
                            todo_sideways.push(Pt { x: w.x, y });
                        } else {
                            self.map.insert(Pt { x: w.x, y }, Soil::WaterDown);
                        }
                    } else {
                        self.map.insert(Pt { x: w.x, y }, Soil::WaterDown);
                    }
                }
            }
            loop {
                let mut new_todo_sideways = Vec::new();
                for h in todo_sideways.drain(..) {
                    let (bounded, from, to) = self.find_span(h);
                    match bounded {
                        Bounded::BothSides => {
                            new_todo_sideways.push(Pt { x: h.x, y: h.y - 1 });
                            self.map
                                .insert(Pt { x: h.x, y: h.y - 1 }, Soil::WaterSideways);
                            for x in from..=to {
                                self.map.insert(Pt { x, y: h.y }, Soil::WaterSideways);
                            }
                        }
                        Bounded::ToLeft => {
                            for x in from..=to {
                                self.map.insert(Pt { x, y: h.y }, Soil::WaterSideways);
                            }
                            self.map.insert(Pt { x: to, y: h.y }, Soil::WaterDown);
                            todo_down.push(Pt { x: to, y: h.y });
                        }
                        Bounded::ToRight => {
                            for x in from..=to {
                                self.map.insert(Pt { x, y: h.y }, Soil::WaterSideways);
                            }
                            self.map.insert(Pt { x: from, y: h.y }, Soil::WaterDown);
                            todo_down.push(Pt { x: from, y: h.y });
                        }
                        Bounded::Unbounded => {
                            for x in from..=to {
                                self.map.insert(Pt { x, y: h.y }, Soil::WaterSideways);
                            }
                            self.map.insert(Pt { x: from, y: h.y }, Soil::WaterDown);
                            todo_down.push(Pt { x: from, y: h.y });
                            self.map.insert(Pt { x: to, y: h.y }, Soil::WaterDown);
                            todo_down.push(Pt { x: to, y: h.y });
                        }
                    }
                }
                if new_todo_sideways.is_empty() {
                    break;
                }
                todo_sideways = new_todo_sideways;
            }
        }
    }

    fn find_span(&self, pt: Pt) -> (Bounded, usize, usize) /* (bounded, from, to) */ {
        let (from, left_bounded) = (1..)
            .map(|delta| pt.x - delta)
            .find_map(|x| {
                if self
                    .map
                    .get(&Pt { x, y: pt.y })
                    .is_some_and(|&c| c == Soil::Clay)
                {
                    Some((x + 1, true))
                } else if !self.supported(Pt { x, y: pt.y }) {
                    Some((x, false))
                } else {
                    None
                }
            })
            .unwrap();
        let (to, right_bounded) = (1..)
            .map(|delta| pt.x + delta)
            .find_map(|x| {
                if self
                    .map
                    .get(&Pt { x, y: pt.y })
                    .is_some_and(|&c| c == Soil::Clay)
                {
                    Some((x - 1, true))
                } else if !self.supported(Pt { x, y: pt.y }) {
                    Some((x, false))
                } else {
                    None
                }
            })
            .unwrap();
        let bounded = match (left_bounded, right_bounded) {
            (true, true) => Bounded::BothSides,
            (true, false) => Bounded::ToLeft,
            (false, true) => Bounded::ToRight,
            (false, false) => Bounded::Unbounded,
        };
        (bounded, from, to)
    }

    fn supported(&self, pt: Pt) -> bool {
        self.map.contains_key(&Pt {
            x: pt.x,
            y: pt.y + 1,
        })
    }

    fn vertical_to(&self, pt: Pt) -> impl Iterator<Item = usize> {
        pt.y + 1
            ..(pt.y + 1..)
                .find(|&y| self.map.contains_key(&Pt { x: pt.x, y }) || y > self.max.y)
                .unwrap_or(self.max.y + 1)
    }

    fn count_water(&self) -> u32 {
        self.map
            .iter()
            .filter(|(k, v)| (self.min.y..=self.max.y).contains(&k.y) && v.is_water())
            .count() as u32
    }

    fn start_stop(&self, line: usize) -> Vec<(usize, usize)> {
        let mut seen_water: Option<usize> = None;
        let mut out = Vec::new();
        for x in self.min.x..=self.max.x {
            match (self.map.get(&Pt { x, y: line }), seen_water) {
                (None | Some(Soil::Clay), Some(start)) => {
                    out.push((start, x));
                    seen_water = None;
                }
                (Some(Soil::WaterDown), _) => unreachable!(),
                (Some(Soil::WaterSideways), None) => {
                    seen_water = Some(x);
                }
                _ => {}
            }
        }
        out
    }

    fn drain(&mut self) {
        self.map.retain(|_, v| *v != Soil::WaterDown);
        for y in self.min.y..self.max.y {
            for (start, stop) in self.start_stop(y) {
                if self
                    .find_span(Pt {
                        x: (start + stop) / 2,
                        y,
                    })
                    .0
                    != Bounded::BothSides
                {
                    for x in start..stop {
                        if let Some((_, e)) = self.map.remove_entry(&Pt { x, y }) {
                            assert!(e.is_water());
                        }
                    }
                }
            }
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut clay = input.parse::<Clay>().unwrap();
    clay.fill();
    Some(clay.count_water())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut clay = input.parse::<Clay>().unwrap();
    clay.fill();
    clay.drain();
    Some(clay.count_water())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(57));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(29));
    }
}
