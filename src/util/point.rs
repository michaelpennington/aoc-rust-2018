use std::{
    ops::{Add, AddAssign},
    str::FromStr,
};

use anyhow::anyhow;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Pt<T> {
    pub x: T,
    pub y: T,
}

impl<T> Add<Pt<T>> for Pt<T>
where
    T: Add<Output = T>,
{
    type Output = Pt<T>;

    fn add(self, rhs: Pt<T>) -> Self::Output {
        Pt {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T> AddAssign<Pt<T>> for Pt<T>
where
    T: AddAssign<T>,
{
    fn add_assign(&mut self, rhs: Pt<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T> From<(T, T)> for Pt<T> {
    fn from(value: (T, T)) -> Self {
        Self {
            x: value.0,
            y: value.1,
        }
    }
}

impl<T> From<Pt<T>> for (T, T) {
    fn from(value: Pt<T>) -> Self {
        (value.x, value.y)
    }
}

impl<T> FromStr for Pt<T>
where
    T: FromStr,
    <T as FromStr>::Err: std::error::Error + Send + Sync + 'static,
{
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (x, y) = s
            .trim()
            .trim_start_matches('(')
            .trim_end_matches(')')
            .split_once(',')
            .ok_or(anyhow!("{s} must include a comma to be parsed as Pt"))?;
        Ok(Self {
            x: x.parse()?,
            y: y.parse()?,
        })
    }
}
