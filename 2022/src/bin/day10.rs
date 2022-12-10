use anyhow::{bail, Context, Error, Result};
use aoc2022::dispatch;
use std::fmt::Write;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

enum Instruction {
    Noop,
    AddX(i32),
}

impl std::str::FromStr for Instruction {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if s == "noop" {
            return Ok(Self::Noop);
        } else if let Some(digits) = s.strip_prefix("addx ") {
            return Ok(Self::AddX(digits.parse()?));
        }
        bail!("invalid string");
    }
}

fn parse(input: &str) -> Result<Vec<Instruction>> {
    input.split('\n').map(|l| l.parse()).collect()
}

fn strength(cycle: i32, x: i32) -> i32 {
    if (cycle - 20) % 40 == 0 {
        x * cycle
    } else {
        0
    }
}

fn part1(input: &str) -> Result<i32> {
    let instructions = parse(input)?;
    let mut cycle = 1;
    let mut x = 1;
    let mut total_signal = 0;
    for instruction in instructions {
        total_signal += strength(cycle, x);
        cycle += 1;
        if let Instruction::AddX(value) = instruction {
            total_signal += strength(cycle, x);
            cycle += 1;
            x += value;
        }
    }
    Ok(total_signal)
}

fn draw(buf: &mut String, cycle: i32, x: i32) -> Result<()> {
    let sprite_pos = (cycle - 1) % 40;
    if sprite_pos == 0 {
        writeln!(buf)?;
    }
    if (x - sprite_pos).abs() <= 1 {
        write!(buf, "#").context("write failed")
    } else {
        write!(buf, ".").context("write failed")
    }
}

fn part2(input: &str) -> Result<String> {
    let instructions = parse(input)?;
    let mut buf = String::new();
    let mut cycle = 1;
    let mut x = 1;
    for instruction in &instructions {
        draw(&mut buf, cycle, x)?;
        cycle += 1;
        if let Instruction::AddX(value) = instruction {
            draw(&mut buf, cycle, x)?;
            cycle += 1;
            x += value;
        }
    }
    Ok(buf)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(TEST_INPUT)?, 13140);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(
            part2(TEST_INPUT)?,
            "
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."
        );
        Ok(())
    }
}
