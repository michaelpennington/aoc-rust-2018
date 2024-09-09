advent_of_code::solution!(24);

use std::{ops::Index, str::FromStr};

use anyhow::anyhow;
use bitflags::bitflags;
use smallvec::{smallvec, SmallVec};

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
    struct DamageTypes: u8 {
        const RADIATION = 1 << 0;
        const COLD = 1 << 1;
        const FIRE = 1 << 2;
        const SLASHING = 1 << 3;
        const BLUDGEONING = 1 << 4;
    }
}

impl FromStr for DamageTypes {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dtype = DamageType::from_str(s)?;
        Ok(dtype.into())
    }
}

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum DamageType {
    Radiation,
    Cold,
    Fire,
    Slashing,
    Bludgeoning,
}

impl From<DamageType> for DamageTypes {
    fn from(value: DamageType) -> Self {
        match value {
            DamageType::Radiation => Self::RADIATION,
            DamageType::Cold => Self::COLD,
            DamageType::Fire => Self::FIRE,
            DamageType::Slashing => Self::SLASHING,
            DamageType::Bludgeoning => Self::BLUDGEONING,
        }
    }
}

impl FromStr for DamageType {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "radiation" => Ok(Self::Radiation),
            "cold" => Ok(Self::Cold),
            "fire" => Ok(Self::Fire),
            "slashing" => Ok(Self::Slashing),
            "bludgeoning" => Ok(Self::Bludgeoning),
            _ => Err(anyhow!("{s} is not a valid damage type")),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Group {
    num_units: u32,
    hp: u32,
    weaknesses: DamageTypes,
    immunities: DamageTypes,
    attack_type: DamageType,
    attack_power: u32,
    initiative: u8,
}

impl Group {
    fn effective_power(&self) -> u32 {
        self.num_units * self.attack_power
    }

    fn damage_dealt(&self, target: &Group) -> u32 {
        if target.immunities.contains(self.attack_type.into()) {
            0
        } else if target.weaknesses.contains(self.attack_type.into()) {
            2 * self.effective_power()
        } else {
            self.effective_power()
        }
    }

    fn choose_target(&self, enemies: &[Group], already_chosen: &[Option<usize>]) -> Option<usize> {
        enemies
            .iter()
            .enumerate()
            .filter(|(i, e)| !already_chosen.contains(&Some(*i)) && e.num_units > 0)
            .max_by(|(_, e1), (_, e2)| {
                self.damage_dealt(e1)
                    .cmp(&self.damage_dealt(e2))
                    .then_with(|| e1.effective_power().cmp(&e2.effective_power()))
                    .then_with(|| e1.initiative.cmp(&e2.initiative))
            })
            .filter(|(_, e)| self.damage_dealt(e) != 0)
            .map(|(i, _)| i)
    }

    fn attack(&self, enemy: &mut Group) -> bool {
        let damage = self.damage_dealt(enemy);
        enemy.num_units = enemy.num_units.saturating_sub(damage / enemy.hp);
        damage / enemy.hp != 0
    }
}

impl FromStr for Group {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut words = s.splitn(8, |c: char| c.is_whitespace());
        let num_units = words.next().unwrap().parse()?;
        let hp = words.nth(3).unwrap().parse()?;
        let mut the_rest = words.last().unwrap();
        let mut weaknesses = DamageTypes::empty();
        let mut immunities = DamageTypes::empty();
        if let Some((buffs, rest)) = the_rest
            .split_once(") ")
            .map(|(b, r)| (b.trim_start_matches('('), r))
        {
            for part in buffs.split("; ") {
                if let Some(s) = part.strip_prefix("immune to ") {
                    for dtype in s.split(", ") {
                        immunities.insert(dtype.parse()?);
                    }
                } else if let Some(s) = part.strip_prefix("weak to ") {
                    for dtype in s.split(", ") {
                        weaknesses.insert(dtype.parse()?);
                    }
                }
            }
            the_rest = rest;
        }
        let mut words = the_rest.split_whitespace();
        let attack_power = words.nth(5).unwrap().parse()?;
        let attack_type = words.next().unwrap().parse()?;
        let initiative = words.last().unwrap().parse()?;

        Ok(Self {
            num_units,
            hp,
            weaknesses,
            immunities,
            attack_type,
            attack_power,
            initiative,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Army {
    groups: Vec<Group>,
}

impl FromStr for Army {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut groups = Vec::with_capacity(s.lines().count());
        for line in s.lines() {
            groups.push(line.parse()?);
        }
        Ok(Self { groups })
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum ArmyType {
    ImmuneSystem,
    Infection,
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Battle {
    immune_system: Army,
    is_choose_order: Vec<usize>,
    infection: Army,
    in_choose_order: Vec<usize>,
    attack_order: Vec<(ArmyType, usize)>,
}

impl Index<(ArmyType, usize)> for Battle {
    type Output = Group;

    fn index(&self, index: (ArmyType, usize)) -> &Self::Output {
        match index.0 {
            ArmyType::ImmuneSystem => &self.immune_system.groups[index.1],
            ArmyType::Infection => &self.infection.groups[index.1],
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Targets {
    immune_system: SmallVec<[Option<usize>; 16]>,
    infection: SmallVec<[Option<usize>; 16]>,
}

impl Default for Targets {
    fn default() -> Self {
        Self {
            immune_system: smallvec![None; 16],
            infection: smallvec![None; 16],
        }
    }
}

impl Battle {
    fn new(immune_system: Army, infection: Army) -> Self {
        let is_choose_order = (0..immune_system.groups.len()).collect();
        let in_choose_order = (0..infection.groups.len()).collect();
        let attack_order = (0..immune_system.groups.len())
            .map(|i| (ArmyType::ImmuneSystem, i))
            .chain((0..infection.groups.len()).map(|i| (ArmyType::Infection, i)))
            .collect();
        let mut out = Self {
            immune_system,
            is_choose_order,
            infection,
            in_choose_order,
            attack_order,
        };
        out.set_attack_order();
        out
    }

    fn set_choose_orders(&mut self) {
        let mut to_remove = SmallVec::<[usize; 8]>::new();
        for (i, &n) in self.is_choose_order.iter().enumerate().rev() {
            if self.immune_system.groups[n].num_units == 0 {
                to_remove.push(i);
            }
        }
        for i in to_remove {
            self.is_choose_order.remove(i);
        }
        let mut to_remove = SmallVec::<[usize; 8]>::new();
        for (i, &n) in self.in_choose_order.iter().enumerate().rev() {
            if self.infection.groups[n].num_units == 0 {
                to_remove.push(i);
            }
        }
        for i in to_remove {
            self.in_choose_order.remove(i);
        }
        self.is_choose_order.sort_by(|&i1, &i2| {
            let g1 = &self.immune_system.groups[i1];
            let g2 = &self.immune_system.groups[i2];
            g1.effective_power()
                .cmp(&g2.effective_power())
                .then_with(|| g1.initiative.cmp(&g2.initiative))
                .reverse()
        });
        self.in_choose_order.sort_by(|&i1, &i2| {
            let g1 = &self.infection.groups[i1];
            let g2 = &self.infection.groups[i2];
            g1.effective_power()
                .cmp(&g2.effective_power())
                .then_with(|| g1.initiative.cmp(&g2.initiative))
                .reverse()
        });
    }

    fn set_attack_order(&mut self) {
        self.attack_order.sort_by(|&(atype1, i1), &(atype2, i2)| {
            let g1 = match atype1 {
                ArmyType::ImmuneSystem => &self.immune_system.groups[i1],
                ArmyType::Infection => &self.infection.groups[i1],
            };
            let g2 = match atype2 {
                ArmyType::ImmuneSystem => &self.immune_system.groups[i2],
                ArmyType::Infection => &self.infection.groups[i2],
            };
            g1.initiative.cmp(&g2.initiative).reverse()
        })
    }

    fn prune_attack_order(&mut self) {
        let mut to_remove = SmallVec::<[usize; 8]>::new();
        for (i, &(atype, n)) in self.attack_order.iter().enumerate().rev() {
            match atype {
                ArmyType::ImmuneSystem if self.immune_system.groups[n].num_units == 0 => {
                    to_remove.push(i);
                }
                ArmyType::Infection if self.infection.groups[n].num_units == 0 => {
                    to_remove.push(i);
                }
                _ => {}
            }
        }
        for i in to_remove {
            self.attack_order.remove(i);
        }
    }

    fn choose_targets(&mut self) -> Targets {
        let mut targets = Targets::default();
        for &i in &self.is_choose_order {
            targets.immune_system[i] = self.immune_system.groups[i]
                .choose_target(&self.infection.groups, &targets.immune_system);
        }
        for &i in &self.in_choose_order {
            targets.infection[i] = self.infection.groups[i]
                .choose_target(&self.immune_system.groups, &targets.infection);
        }
        targets
    }

    fn has_won(&self) -> Option<ArmyType> {
        if self.immune_system.groups.iter().all(|g| g.num_units == 0) {
            Some(ArmyType::Infection)
        } else if self.infection.groups.iter().all(|g| g.num_units == 0) {
            Some(ArmyType::ImmuneSystem)
        } else {
            None
        }
    }

    fn fight(&mut self, boost: Option<u32>) -> (ArmyType, u32) {
        if let Some(boost) = boost {
            for gp in &mut self.immune_system.groups {
                gp.attack_power += boost;
            }
        }
        let winner = 'outer: loop {
            self.set_choose_orders();
            let targets = self.choose_targets();
            let mut damage_done = false;
            for &(atype, index) in &self.attack_order {
                if let Some(winner) = self.has_won() {
                    break 'outer winner;
                }
                if self[(atype, index)].num_units == 0 {
                    continue;
                }
                match atype {
                    ArmyType::ImmuneSystem => {
                        if let Some(target) = targets.immune_system[index] {
                            damage_done |= self.immune_system.groups[index]
                                .attack(&mut self.infection.groups[target]);
                        }
                    }
                    ArmyType::Infection => {
                        if let Some(target) = targets.infection[index] {
                            damage_done |= self.infection.groups[index]
                                .attack(&mut self.immune_system.groups[target]);
                        }
                    }
                }
            }
            if !damage_done {
                return (ArmyType::Infection, 0);
            }
            self.prune_attack_order();
        };
        match winner {
            ArmyType::ImmuneSystem => (
                ArmyType::ImmuneSystem,
                self.immune_system.groups.iter().map(|g| g.num_units).sum(),
            ),
            ArmyType::Infection => (
                ArmyType::Infection,
                self.infection.groups.iter().map(|g| g.num_units).sum(),
            ),
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let (immune_system, infection): (Army, Army) = input
        .split_once("\n\n")
        .map(|(im, inf)| {
            (
                im.trim_start_matches("Immune System:")
                    .trim()
                    .parse()
                    .unwrap(),
                inf.trim_start_matches("Infection:").trim().parse().unwrap(),
            )
        })
        .unwrap();
    let mut battle = Battle::new(immune_system, infection);

    Some(battle.fight(None).1)
}

pub fn part_two(input: &str) -> Option<u32> {
    let (immune_system, infection): (Army, Army) = input
        .split_once("\n\n")
        .map(|(im, inf)| {
            (
                im.trim_start_matches("Immune System:")
                    .trim()
                    .parse()
                    .unwrap(),
                inf.trim_start_matches("Infection:").trim().parse().unwrap(),
            )
        })
        .unwrap();
    let battle = Battle::new(immune_system, infection);
    for i in 0.. {
        let res = battle.clone().fight(Some(i));
        if res.0 == ArmyType::ImmuneSystem {
            return Some(res.1);
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5216));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(51));
    }
}
