use std::{
    collections::HashMap,
    fmt::Display,
    ops::{Index, IndexMut},
    str::FromStr,
};

use strum::{EnumDiscriminants, EnumIter, FromRepr, IntoEnumIterator};

advent_of_code::solution!(16);

#[derive(Clone, Copy, PartialEq, Eq, Debug, EnumDiscriminants)]
#[strum_discriminants(derive(EnumIter, Hash))]
#[strum_discriminants(name(InstructionType))]
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
    fn try_from_type(itype: InstructionType, data: [usize; 4]) -> Option<Self> {
        let arg1 = data[1];
        let arg2 = data[2];
        let arg3 = data[3];
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

#[repr(usize)]
#[derive(Clone, Copy, PartialEq, Eq, Debug, FromRepr)]
enum Register {
    Zero = 0,
    One = 1,
    Two = 2,
    Three = 3,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
struct Computer {
    registers: [usize; 4],
}

impl FromStr for Computer {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim_start_matches('[').trim_end_matches(']');
        let mut out = [0; 4];
        for (num, reg) in s.split(',').map(|s| s.trim()).zip(&mut out) {
            *reg = num.parse()?;
        }
        Ok(Self { registers: out })
    }
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

impl Computer {
    fn execute(&self, instruction: Instruction) -> Self {
        let mut c = *self;
        let gt = |a, b| if a > b { 1 } else { 0 };
        let eq = |a, b| if a == b { 1 } else { 0 };
        match instruction {
            Instruction::AddR { in1, in2, out } => c[out] = c[in1] + c[in2],
            Instruction::AddI { in1, in2, out } => c[out] = c[in1] + in2,
            Instruction::MulR { in1, in2, out } => c[out] = c[in1] * c[in2],
            Instruction::MulI { in1, in2, out } => c[out] = c[in1] * in2,
            Instruction::BAnR { in1, in2, out } => c[out] = c[in1] & c[in2],
            Instruction::BAnI { in1, in2, out } => c[out] = c[in1] & in2,
            Instruction::BOrR { in1, in2, out } => c[out] = c[in1] | c[in2],
            Instruction::BOrI { in1, in2, out } => c[out] = c[in1] | in2,
            Instruction::SetR { in1, out } => c[out] = c[in1],
            Instruction::SetI { in1, out } => c[out] = in1,
            Instruction::GtIR { in1, in2, out } => c[out] = gt(in1, c[in2]),
            Instruction::GtRI { in1, in2, out } => c[out] = gt(c[in1], in2),
            Instruction::GtRR { in1, in2, out } => c[out] = gt(c[in1], c[in2]),
            Instruction::EqIR { in1, in2, out } => c[out] = eq(in1, c[in2]),
            Instruction::EqRI { in1, in2, out } => c[out] = eq(c[in1], in2),
            Instruction::EqRR { in1, in2, out } => c[out] = eq(c[in1], c[in2]),
        }
        c
    }

    fn run_program(&mut self, program: &Program) -> u32 {
        for i in &program.instructions {
            *self = self.execute(*i);
        }
        self[Register::Zero] as u32
    }
}

struct Program {
    instructions: Vec<Instruction>,
}

impl Display for Computer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{:5}, {:5}, {:5}, {:5}]",
            self.registers[0], self.registers[1], self.registers[2], self.registers[3]
        )
    }
}

impl Program {
    fn with_opcodes(op_codes: &HashMap<usize, InstructionType>, datas: &str) -> Self {
        let mut instructions = Vec::with_capacity(datas.lines().count());
        for line in datas.lines() {
            let mut data = [0; 4];
            for (num, data) in line
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .zip(&mut data)
            {
                *data = num
            }
            instructions.push(Instruction::try_from_type(op_codes[&data[0]], data).unwrap());
        }
        Self { instructions }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Sample {
    before: Computer,
    instruction: [usize; 4],
    after: Computer,
}

impl FromStr for Sample {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let before = lines
            .next()
            .unwrap()
            .trim_start_matches("Before: ")
            .trim()
            .parse()?;

        let mut instruction = [0; 4];
        for (n, i) in lines
            .next()
            .unwrap()
            .split_whitespace()
            .zip(&mut instruction)
        {
            *i = n.parse()?;
        }
        let after = lines
            .next()
            .unwrap()
            .trim_start_matches("After: ")
            .trim()
            .parse()?;
        Ok(Self {
            before,
            instruction,
            after,
        })
    }
}

impl Sample {
    fn possible_ops(&self) -> Vec<InstructionType> {
        InstructionType::iter()
            .filter_map(|itype| Instruction::try_from_type(itype, self.instruction))
            .filter(|&inst| self.before.execute(inst) == self.after)
            .map(InstructionType::from)
            .collect()
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .split_once("\n\n\n")
            .unwrap()
            .0
            .split("\n\n")
            .map(|eg| eg.parse::<Sample>().unwrap())
            .filter(|eg| eg.possible_ops().len() >= 3)
            .count() as u32,
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut potential_opcodes = HashMap::new();
    let all_types: Vec<_> = InstructionType::iter().collect();
    for i in 0..16 {
        potential_opcodes.insert(i, all_types.clone());
    }
    let (samples, program) = input.split_once("\n\n\n").unwrap();
    for sample in samples.split("\n\n").map(|s| s.parse::<Sample>().unwrap()) {
        let op = potential_opcodes.get_mut(&sample.instruction[0]).unwrap();
        let possible_ops = sample.possible_ops();
        op.retain(|ty| possible_ops.contains(ty));
    }
    let mut ops: HashMap<usize, InstructionType> = HashMap::new();
    loop {
        if ops.len() == 16 {
            break;
        }
        for (code, vals) in &potential_opcodes {
            if vals.len() == 1 && !ops.contains_key(code) {
                ops.insert(*code, vals[0]);
            }
        }
        for (code, val) in &ops {
            potential_opcodes.remove(code);
            for val2 in &mut potential_opcodes.values_mut() {
                val2.retain(|v| v != val);
            }
        }
    }
    let inst = Program::with_opcodes(&ops, program);
    let mut comp = Computer::default();
    Some(comp.run_program(&inst))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1));
    }
}
