use anyhow::{bail, Context, Error, Result};
use aoc2015::coor::Coor;
use aoc2015::dispatch;
use std::convert::TryFrom;

#[derive(Debug, PartialEq)]
enum Action {
    TurnOn,
    TurnOff,
    Toggle,
}

#[derive(Debug, PartialEq)]
struct Rect {
    start: Coor,
    end: Coor,
}

#[derive(Debug, PartialEq)]
struct Instruction {
    action: Action,
    rect: Rect,
}

impl TryFrom<&str> for Instruction {
    type Error = Error;
    fn try_from(s: &str) -> Result<Instruction> {
        let (action, rest) = if let Some(rest) = s.strip_prefix("turn on ") {
            (Action::TurnOn, rest)
        } else if let Some(rest) = s.strip_prefix("turn off ") {
            (Action::TurnOff, rest)
        } else if let Some(rest) = s.strip_prefix("toggle ") {
            (Action::Toggle, rest)
        } else {
            bail!("invalid prefix {}", s)
        };
        let (start_raw, end_raw) = rest.split_once(" through ").context("no through")?;
        let (start_x_raw, start_y_raw) = start_raw.split_once(',').context("no start comma")?;
        let (end_x_raw, end_y_raw) = end_raw.split_once(',').context("no end comma")?;
        Ok(Instruction {
            action,
            rect: Rect {
                start: Coor::new(start_x_raw.parse()?, start_y_raw.parse()?),
                end: Coor::new(end_x_raw.parse()?, end_y_raw.parse()?),
            },
        })
    }
}

fn main() -> Result<()> {
    dispatch(part1, part2)
}

fn part1(input: &str) -> Result<usize> {
    let instructions = input
        .split('\n')
        .map(Instruction::try_from)
        .collect::<Result<Vec<_>>>()?;
    let mut lights = [false; 1000 * 1000];
    for instruction in instructions {
        let action = match instruction.action {
            Action::TurnOn => |_| true,
            Action::TurnOff => |_| false,
            Action::Toggle => |v: bool| !v,
        };
        let x_start = instruction.rect.start.x.min(instruction.rect.end.x);
        let x_end = instruction.rect.start.x.max(instruction.rect.end.x);
        let y_start = instruction.rect.start.y.min(instruction.rect.end.y);
        let y_end = instruction.rect.start.y.max(instruction.rect.end.y);
        for x in x_start..=x_end {
            for y in y_start..=y_end {
                lights[(y * 1000 + x) as usize] = action(lights[(y * 1000 + x) as usize]);
            }
        }
    }
    Ok(lights.iter().filter(|&v| *v).count())
}

fn part2(input: &str) -> Result<usize> {
    let instructions = input
        .split('\n')
        .map(Instruction::try_from)
        .collect::<Result<Vec<_>>>()?;
    let mut lights = [0; 1000 * 1000];
    for instruction in instructions {
        let action = match instruction.action {
            Action::TurnOn => |v| v + 1,
            Action::TurnOff => |v: usize| v.max(1) - 1,
            Action::Toggle => |v| v + 2,
        };
        let x_start = instruction.rect.start.x.min(instruction.rect.end.x);
        let x_end = instruction.rect.start.x.max(instruction.rect.end.x);
        let y_start = instruction.rect.start.y.min(instruction.rect.end.y);
        let y_end = instruction.rect.start.y.max(instruction.rect.end.y);
        for x in x_start..=x_end {
            for y in y_start..=y_end {
                lights[(y * 1000 + x) as usize] = action(lights[(y * 1000 + x) as usize]);
            }
        }
    }
    Ok(lights.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "turn on 47,124 through 598,880
toggle 828,303 through 979,770
turn off 174,272 through 280,311";

    #[test]
    fn test_parse() -> Result<()> {
        assert_eq!(
            INPUT
                .split('\n')
                .map(Instruction::try_from)
                .collect::<Result<Vec<_>>>()?,
            vec![
                Instruction {
                    action: Action::TurnOn,
                    rect: Rect {
                        start: Coor::new(47, 124),
                        end: Coor::new(598, 880)
                    }
                },
                Instruction {
                    action: Action::Toggle,
                    rect: Rect {
                        start: Coor::new(828, 303),
                        end: Coor::new(979, 770)
                    }
                },
                Instruction {
                    action: Action::TurnOff,
                    rect: Rect {
                        start: Coor::new(174, 272),
                        end: Coor::new(280, 311)
                    }
                },
            ]
        );
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(INPUT)?, 0);
        Ok(())
    }
}
