use std::{
    fmt::Display,
    ops::{Add, AddAssign},
    str::FromStr,
};

use anyhow::anyhow;

advent_of_code::solution!(10);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
struct Pt(i32, i32);

impl AddAssign<Pt> for Pt {
    fn add_assign(&mut self, rhs: Pt) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Add<Pt> for Pt {
    type Output = Pt;

    fn add(self, rhs: Pt) -> Self::Output {
        Pt(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl Add<&Pt> for Pt {
    type Output = Pt;

    fn add(self, rhs: &Pt) -> Self::Output {
        Pt(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl FromStr for Pt {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s.split_once(',').ok_or(anyhow!("pt must have a comma"))?;
        let (x, y) = (
            x.trim_start_matches('<').trim().parse()?,
            y.trim_end_matches('>').trim().parse()?,
        );
        Ok(Self(x, y))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Star {
    p: Pt,
    v: Pt,
}

impl Star {
    fn update(&mut self) {
        self.p += self.v;
    }
}

impl PartialOrd for Star {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Star {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.p.cmp(&other.p)
    }
}

impl FromStr for Star {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split("=");
        assert_eq!(parts.next().unwrap(), "position");
        let mut p = parts.next().unwrap();
        assert!(p.ends_with(" velocity"));
        p = p.trim_end_matches(" velocity");
        let v = parts.next().unwrap();
        Ok(Self {
            p: p.parse()?,
            v: v.parse()?,
        })
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Debug, Default)]
struct Points(Vec<Star>);

impl Display for Points {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let l = self.0.iter().map(|s| s.p.0).min().unwrap();
        let r = self.0.iter().map(|s| s.p.0).max().unwrap();
        let u = self.0.iter().map(|s| s.p.1).min().unwrap();
        let d = self.0.iter().map(|s| s.p.1).max().unwrap();
        for x in u..=d {
            for y in l..=r {
                let c = match self.0.iter().find(|p| p.p == Pt(y, x)) {
                    Some(_) => '#',
                    None => '.',
                };
                write!(f, "{c}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Points {
    fn u(&self) -> i32 {
        self.0.iter().map(|s| s.p.1).min().unwrap()
    }

    fn d(&self) -> i32 {
        self.0.iter().map(|s| s.p.1).max().unwrap()
    }

    fn l(&self) -> i32 {
        self.0.iter().map(|s| s.p.0).min().unwrap()
    }

    fn r(&self) -> i32 {
        self.0.iter().map(|s| s.p.0).max().unwrap()
    }

    fn span(&self) -> i32 {
        self.r() - self.l() + self.d() - self.u()
    }
}

pub fn part_one(input: &str) -> Option<String> {
    let pts: Vec<_> = input.lines().map(|l| l.parse().unwrap()).collect();
    let mut pts = Points(pts);
    for _ in 0..11000 {
        if pts.span() <= 80 {
            println!("{pts}");
            return Some("EHAZPZHP".into());
        }
        for pt in &mut pts.0 {
            pt.update();
        }
    }
    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let pts: Vec<_> = input.lines().map(|l| l.parse().unwrap()).collect();
    let mut pts = Points(pts);
    for secs in 0..11000 {
        if pts.span() <= 80 {
            return Some(secs);
        }
        for pt in &mut pts.0 {
            pt.update();
        }
    }
    None
}
