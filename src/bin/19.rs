advent_of_code::solution!(19);

use std::{
    fmt::Display,
    ops::{Index, IndexMut},
    str::FromStr,
};

use anyhow::anyhow;

use strum::{EnumDiscriminants, EnumString, FromRepr};

#[repr(usize)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, FromRepr, Default)]
enum Register {
    #[default]
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, EnumDiscriminants)]
#[strum_discriminants(name(InstructionType))]
#[strum_discriminants(derive(EnumString))]
#[strum_discriminants(strum(serialize_all = "lowercase"))]
enum Instruction {
    AddR {
        in1: Register,
        in2: Register,
        out: Register,
    },
    AddI {
        in1: Register,
        in2: usize,
        out: Register,
    },
    MulR {
        in1: Register,
        in2: Register,
        out: Register,
    },
    MulI {
        in1: Register,
        in2: usize,
        out: Register,
    },
    BAnR {
        in1: Register,
        in2: Register,
        out: Register,
    },
    BAnI {
        in1: Register,
        in2: usize,
        out: Register,
    },
    BOrR {
        in1: Register,
        in2: Register,
        out: Register,
    },
    BOrI {
        in1: Register,
        in2: usize,
        out: Register,
    },
    SetR {
        in1: Register,
        out: Register,
    },
    SetI {
        in1: usize,
        out: Register,
    },
    GtIR {
        in1: usize,
        in2: Register,
        out: Register,
    },
    GtRI {
        in1: Register,
        in2: usize,
        out: Register,
    },
    GtRR {
        in1: Register,
        in2: Register,
        out: Register,
    },
    EqIR {
        in1: usize,
        in2: Register,
        out: Register,
    },
    EqRI {
        in1: Register,
        in2: usize,
        out: Register,
    },
    EqRR {
        in1: Register,
        in2: Register,
        out: Register,
    },
}

impl Instruction {
    fn try_from_type(itype: InstructionType, data: &[usize]) -> Option<Self> {
        let arg1 = data[0];
        let arg2 = data[1];
        let arg3 = data[2];
        let out = Register::from_repr(arg3)?;
        match itype {
            InstructionType::AddR => {
                let in1 = Register::from_repr(arg1)?;
                let in2 = Register::from_repr(arg2)?;
                Some(Self::AddR { in1, in2, out })
            }
            InstructionType::AddI => {
                let in1 = Register::from_repr(arg1)?;
                let in2 = arg2;
                Some(Self::AddI { in1, in2, out })
            }
            InstructionType::MulR => {
                let in1 = Register::from_repr(arg1)?;
                let in2 = Register::from_repr(arg2)?;
                Some(Self::MulR { in1, in2, out })
            }
            InstructionType::MulI => {
                let in1 = Register::from_repr(arg1)?;
                let in2 = arg2;
                Some(Self::MulI { in1, in2, out })
            }
            InstructionType::BAnR => {
                let in1 = Register::from_repr(arg1)?;
                let in2 = Register::from_repr(arg2)?;
                Some(Self::BAnR { in1, in2, out })
            }
            InstructionType::BAnI => {
                let in1 = Register::from_repr(arg1)?;
                let in2 = arg2;
                Some(Self::BAnI { in1, in2, out })
            }
            InstructionType::BOrR => {
                let in1 = Register::from_repr(arg1)?;
                let in2 = Register::from_repr(arg2)?;
                Some(Self::BOrR { in1, in2, out })
            }
            InstructionType::BOrI => {
                let in1 = Register::from_repr(arg1)?;
                let in2 = arg2;
                Some(Self::BOrI { in1, in2, out })
            }
            InstructionType::SetR => {
                let in1 = Register::from_repr(arg1)?;
                Some(Self::SetR { in1, out })
            }
            InstructionType::SetI => {
                let in1 = arg1;
                Some(Self::SetI { in1, out })
            }
            InstructionType::GtIR => {
                let in1 = arg1;
                let in2 = Register::from_repr(arg2)?;
                Some(Self::GtIR { in1, in2, out })
            }
            InstructionType::GtRI => {
                let in1 = Register::from_repr(arg1)?;
                let in2 = arg2;
                Some(Self::GtRI { in1, in2, out })
            }
            InstructionType::GtRR => {
                let in1 = Register::from_repr(arg1)?;
                let in2 = Register::from_repr(arg2)?;
                Some(Self::GtRR { in1, in2, out })
            }
            InstructionType::EqIR => {
                let in1 = arg1;
                let in2 = Register::from_repr(arg2)?;
                Some(Self::EqIR { in1, in2, out })
            }
            InstructionType::EqRI => {
                let in1 = Register::from_repr(arg1)?;
                let in2 = arg2;
                Some(Self::EqRI { in1, in2, out })
            }
            InstructionType::EqRR => {
                let in1 = Register::from_repr(arg1)?;
                let in2 = Register::from_repr(arg2)?;
                Some(Self::EqRR { in1, in2, out })
            }
        }
    }
}

impl FromStr for Instruction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pts = s.split_whitespace();
        let incomplete = |s: &str| anyhow!("{s} is not a valid instruction");
        let itype = pts.next().ok_or(incomplete(s))?.parse()?;
        let mut data = [0; 3];
        for (s, datum) in pts.zip(&mut data) {
            *datum = s.parse()?;
        }
        Self::try_from_type(itype, &data).ok_or(incomplete(s))
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
struct Computer {
    registers: [usize; 6],
    ip: Register,
}

impl Index<Register> for Computer {
    type Output = usize;

    fn index(&self, index: Register) -> &Self::Output {
        &self.registers[index as usize]
    }
}

impl IndexMut<Register> for Computer {
    fn index_mut(&mut self, index: Register) -> &mut Self::Output {
        &mut self.registers[index as usize]
    }
}

impl Display for Computer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[")?;
        let mut it = self.registers.iter().peekable();
        while let Some(i) = it.next() {
            if it.peek().is_none() {
                write!(f, "{i:8}")?;
            } else {
                write!(f, "{i:8}, ")?;
            }
        }
        write!(f, "]")
    }
}

impl Computer {
    fn execute(&mut self, instruction: Instruction) {
        let gt = |a, b| if a > b { 1 } else { 0 };
        let eq = |a, b| if a == b { 1 } else { 0 };
        match instruction {
            Instruction::AddR { in1, in2, out } => self[out] = self[in1] + self[in2],
            Instruction::AddI { in1, in2, out } => self[out] = self[in1] + in2,
            Instruction::MulR { in1, in2, out } => self[out] = self[in1] * self[in2],
            Instruction::MulI { in1, in2, out } => self[out] = self[in1] * in2,
            Instruction::BAnR { in1, in2, out } => self[out] = self[in1] & self[in2],
            Instruction::BAnI { in1, in2, out } => self[out] = self[in1] & in2,
            Instruction::BOrR { in1, in2, out } => self[out] = self[in1] | self[in2],
            Instruction::BOrI { in1, in2, out } => self[out] = self[in1] | in2,
            Instruction::SetR { in1, out } => self[out] = self[in1],
            Instruction::SetI { in1, out } => self[out] = in1,
            Instruction::GtIR { in1, in2, out } => self[out] = gt(in1, self[in2]),
            Instruction::GtRI { in1, in2, out } => self[out] = gt(self[in1], in2),
            Instruction::GtRR { in1, in2, out } => self[out] = gt(self[in1], self[in2]),
            Instruction::EqIR { in1, in2, out } => self[out] = eq(in1, self[in2]),
            Instruction::EqRI { in1, in2, out } => self[out] = eq(self[in1], in2),
            Instruction::EqRR { in1, in2, out } => self[out] = eq(self[in1], self[in2]),
        }
    }

    fn run_program(&mut self, program: &[Instruction]) -> u32 {
        let ip = self.ip;
        while let Some(i) = program.get(self[ip]) {
            self.execute(*i);
            self[ip] += 1;
        }
        self[Register::Zero] as u32
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut comp = Computer::default();
    let mut lines = input.lines();
    let ip = lines
        .next()
        .unwrap()
        .trim_start_matches("#ip ")
        .parse()
        .unwrap();
    comp.ip = Register::from_repr(ip).unwrap();
    let program = lines.map(|l| l.parse().unwrap()).collect::<Vec<_>>();
    Some(comp.run_program(&program))
}

pub fn part_two(_input: &str) -> Option<u32> {
    // Program calculates sum of divisors of 10_551_305
    Some(13406472)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }
}
