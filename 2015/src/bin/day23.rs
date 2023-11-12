use std::{convert::TryFrom, str::FromStr};

use anyhow::{bail, Context, Error, Result};
use aoc2015::dispatch;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

#[derive(Debug, Hash, PartialEq, Eq, Copy, Clone)]
enum Register {
    A,
    B,
}

impl FromStr for Register {
    type Err = Error;
    fn from_str(value: &str) -> Result<Self> {
        if value.starts_with('a') {
            Ok(Register::A)
        } else if value.starts_with('b') {
            Ok(Register::B)
        } else {
            bail!("no reg match: {}", value)
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Op {
    Half(Register),
    Triple(Register),
    Inc(Register),
    Jump(i64),
    JumpIfEven(Register, i64),
    JumpIfOne(Register, i64),
}

impl TryFrom<&str> for Op {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Some(rest) = value.strip_prefix("hlf ") {
            return Ok(Op::Half(rest.parse().context("no register")?));
        }
        if let Some(rest) = value.strip_prefix("tpl ") {
            return Ok(Op::Triple(rest.parse().context("no register")?));
        }
        if let Some(rest) = value.strip_prefix("inc ") {
            return Ok(Op::Inc(rest.parse().context("no register")?));
        }
        if let Some(rest) = value.strip_prefix("jmp ") {
            return Ok(Op::Jump(rest.parse().context("no offset")?));
        }
        if let Some(rest) = value.strip_prefix("jio ") {
            let (reg_raw, offset_raw) = rest.split_once(' ').context("no split")?;
            return Ok(Op::JumpIfOne(
                reg_raw.parse().context("bad reg")?,
                offset_raw.parse().context("bad offset")?,
            ));
        }
        if let Some(rest) = value.strip_prefix("jie ") {
            let (reg_raw, offset_raw) = rest.split_once(' ').context("no split")?;
            return Ok(Op::JumpIfEven(
                reg_raw.parse().context("bad reg")?,
                offset_raw.parse().context("bad offset")?,
            ));
        }
        bail!("no match: {}", value)
    }
}

#[derive(Debug, Default)]
struct Registers {
    a: i64,
    b: i64,
}

impl Registers {
    fn get(&self, register: Register) -> i64 {
        match register {
            Register::A => self.a,
            Register::B => self.b,
        }
    }

    fn set(&mut self, register: Register, value: i64) {
        match register {
            Register::A => self.a = value,
            Register::B => self.b = value,
        }
    }
}

fn run(ops: &[Op], registers: Registers) -> i64 {
    let mut pc = 0;
    let mut registers = registers;
    while let Some(op) = ops.get(pc) {
        // dbg!(pc, op, &registers);
        match op {
            Op::Half(reg) => registers.set(*reg, registers.get(*reg) / 2),
            Op::Triple(reg) => registers.set(*reg, registers.get(*reg) * 3),
            Op::Inc(reg) => registers.set(*reg, registers.get(*reg) + 1),
            Op::Jump(offset) => pc += *offset as usize - 1,
            Op::JumpIfEven(reg, offset) => {
                if registers.get(*reg) % 2 == 0 {
                    pc += *offset as usize - 1
                }
            }
            Op::JumpIfOne(reg, offset) => {
                if registers.get(*reg) == 1 {
                    pc += *offset as usize - 1
                }
            }
        }
        pc += 1;
    }
    registers.get(Register::B)
}

fn part1(input: &str) -> Result<i64> {
    let ops: Vec<_> = input.split('\n').map(Op::try_from).collect::<Result<_>>()?;
    let registers = Registers::default();
    Ok(run(&ops, registers))
}

fn part2(input: &str) -> Result<i64> {
    let ops: Vec<_> = input.split('\n').map(Op::try_from).collect::<Result<_>>()?;
    let mut registers = Registers::default();
    registers.set(Register::A, 1);
    Ok(run(&ops, registers))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "inc a
jio a, +2
tpl a
inc a";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(INPUT)?, 0);
        Ok(())
    }
}
