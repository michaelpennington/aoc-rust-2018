advent_of_code::solution!(22);

use std::collections::HashMap;

use advent_of_code::util::{graph::Graph, point::Pt};
use strum::FromRepr;

#[repr(usize)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash, FromRepr)]
enum RegionType {
    Rocky = 0,
    Wet = 1,
    Narrow = 2,
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Cave {
    map: HashMap<Pt<usize>, Region>,
    depth: usize,
}

fn erosion_level(depth: usize, geologic_index: usize) -> usize {
    const MODULO: usize = 20183;
    (geologic_index + depth) % MODULO
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, Hash)]
enum Tools {
    Neither,
    ClimbingGear,
    #[default]
    Torch,
}

impl Tools {
    fn other(&self, ty: RegionType) -> Self {
        use RegionType::*;
        use Tools::*;
        match (self, ty) {
            (Neither, Wet) | (Torch, Rocky) => ClimbingGear,
            (Neither, Narrow) | (ClimbingGear, Rocky) => Torch,
            (ClimbingGear, Wet) | (Torch, Narrow) => Neither,
            _ => {
                unreachable!()
            }
        }
    }

    fn compat(&self, ty: RegionType) -> bool {
        use RegionType::*;
        use Tools::*;
        matches!(
            (self, ty),
            (ClimbingGear | Torch, Rocky)
                | (ClimbingGear | Neither, Wet)
                | (Torch | Neither, Narrow)
        )
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, Hash)]
struct Node {
    loc: Pt<usize>,
    tool: Tools,
}

impl Graph for Cave {
    type Node = Node;

    fn neighbors(&self, node: Self::Node) -> impl Iterator<Item = (Self::Node, usize)> {
        std::iter::once((
            Node {
                loc: node.loc,
                tool: node.tool.other(self.map[&node.loc].ty),
            },
            7,
        ))
        .chain(
            (node.loc.y.saturating_sub(1)..=(node.loc.y + 1))
                .flat_map(move |y| {
                    (node.loc.x.saturating_sub(1)..=(node.loc.x + 1)).map(move |x| (x, y))
                })
                .filter(move |&(x, y)| {
                    (x == node.loc.x && y != node.loc.y) || (x != node.loc.x && y == node.loc.y)
                })
                .map(|(x, y)| Pt { x, y })
                .filter(move |pt| node.tool.compat(self.map[pt].ty))
                .map(move |loc| {
                    (
                        Node {
                            loc,
                            tool: node.tool,
                        },
                        1,
                    )
                }),
        )
    }

    fn h(from: Self::Node, to: Self::Node) -> usize {
        from.loc.x.abs_diff(to.loc.x) + from.loc.y.abs_diff(to.loc.y)
    }
}

impl Cave {
    fn new(depth: usize, target: Pt<usize>) -> Self {
        const X_MUL: usize = 16807;
        const Y_MUL: usize = 48271;

        let erosion_level = |index| erosion_level(depth, index);
        let map = HashMap::with_capacity(target.x * target.y * 16);
        let mut out = Self { map, depth };
        out.map.insert(
            (0, 0).into(),
            Region {
                geologic_index: 0,
                erosion_level: erosion_level(0),
                ty: RegionType::Rocky,
            },
        );
        for x in 1..=(target.x * 16) {
            let geologic_index = X_MUL * x;
            let erosion_level = erosion_level(geologic_index);
            out.map.insert(
                (x, 0).into(),
                Region {
                    geologic_index,
                    erosion_level,
                    ty: RegionType::from_repr(erosion_level % 3).unwrap(),
                },
            );
        }
        for y in 1..=(target.y * 2) {
            let geologic_index = Y_MUL * y;
            let erosion_level = erosion_level(geologic_index);
            out.map.insert(
                (0, y).into(),
                Region {
                    geologic_index,
                    erosion_level,
                    ty: RegionType::from_repr(erosion_level % 3).unwrap(),
                },
            );
        }
        for y in 1..=(target.y * 2) {
            for x in 1..=(target.x * 16) {
                let geologic_index = out.map[&Pt { x: x - 1, y }].erosion_level
                    * out.map[&Pt { x, y: y - 1 }].erosion_level;
                let erosion_level = erosion_level(geologic_index);
                if (Pt { x, y }) == target {
                    out.map.insert(
                        target,
                        Region {
                            geologic_index,
                            erosion_level,
                            ty: RegionType::Rocky,
                        },
                    );
                } else {
                    out.map.insert(
                        (x, y).into(),
                        Region {
                            geologic_index,
                            erosion_level,
                            ty: RegionType::from_repr(erosion_level % 3).unwrap(),
                        },
                    );
                }
            }
        }
        out
    }

    fn risk_level(&self, target: Pt<usize>) -> usize {
        self.map
            .iter()
            .filter(|&(k, _)| k.x <= target.x && k.y <= target.y)
            .map(|(_, v)| v.ty as usize)
            .sum()
    }

    fn dist(&self, target: Pt<usize>) -> usize {
        let target = Node {
            loc: target,
            tool: Tools::Torch,
        };
        self.a_star_distance(
            Node {
                loc: Pt { x: 0, y: 0 },
                tool: Tools::Torch,
            },
            target,
        )
        .unwrap()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
struct Region {
    erosion_level: usize,
    geologic_index: usize,
    ty: RegionType,
}

pub fn part_one(input: &str) -> Option<usize> {
    let mut lines = input.lines();
    let depth = lines
        .next()
        .unwrap()
        .trim_start_matches("depth: ")
        .parse()
        .unwrap();
    let target = lines
        .next()
        .unwrap()
        .trim_start_matches("target: ")
        .parse()
        .unwrap();
    let cave = Cave::new(depth, target);
    Some(cave.risk_level(target))
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut lines = input.lines();
    let depth = lines
        .next()
        .unwrap()
        .trim_start_matches("depth: ")
        .parse()
        .unwrap();
    let target = lines
        .next()
        .unwrap()
        .trim_start_matches("target: ")
        .parse()
        .unwrap();
    let cave = Cave::new(depth, target);
    Some(cave.dist(target))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(45));
    }
}
