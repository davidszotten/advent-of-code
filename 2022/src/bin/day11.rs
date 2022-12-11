use anyhow::{bail, Context, Error, Result};
use aoc2022::dispatch;
use num::integer::lcm;
use std::collections::VecDeque;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

#[derive(Debug)]
enum Op {
    Add(i64),
    Mul(i64),
    Square,
}

impl Op {
    fn apply(&self, item: i64) -> i64 {
        match self {
            Self::Add(other) => item + other,
            Self::Mul(other) => item * other,
            Self::Square => item * item,
        }
    }

    fn factor(&self) -> i64 {
        match self {
            Self::Add(n) => *n,
            Self::Mul(n) => *n,
            Self::Square => 1,
        }
    }
}
#[derive(Debug)]
struct Monkey {
    items: VecDeque<i64>,
    op: Op,
    test: i64,
    if_true: usize,
    if_false: usize,
}

impl std::str::FromStr for Monkey {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        // Monkey 1:
        //   Starting items: 54, 65, 75, 74
        //   Operation: new = old + 6
        //   Test: divisible by 19
        //     If true: throw to monkey 2
        //     If false: throw to monkey 0
        let mut lines = s.split('\n');
        lines.next().context("empty monkey")?; // name
        let raw_lines = lines
            .next()
            .context("items missing")?
            .strip_prefix("  Starting items: ")
            .context("malformed items")?;
        let items = raw_lines
            .split(", ")
            .map(|n| n.parse().context("invalid int for item"))
            .collect::<Result<_>>()?;
        let raw_op = lines
            .next()
            .context("op missing")?
            .strip_prefix("  Operation: new = old ")
            .context("malformed op")?;
        let op = if raw_op == "* old" {
            Op::Square
        } else if let Some(raw_digit) = raw_op.strip_prefix("+ ") {
            Op::Add(raw_digit.parse()?)
        } else if let Some(raw_digit) = raw_op.strip_prefix("* ") {
            Op::Mul(raw_digit.parse()?)
        } else {
            bail!("invalid op")
        };
        let test = lines
            .next()
            .context("test missing")?
            .strip_prefix("  Test: divisible by ")
            .context("malformed test")?
            .parse()?;
        let if_true = lines
            .next()
            .context("if_true missing")?
            .strip_prefix("    If true: throw to monkey ")
            .context("if_true test")?
            .parse()?;
        let if_false = lines
            .next()
            .context("if_false missing")?
            .strip_prefix("    If false: throw to monkey ")
            .context("if_false test")?
            .parse()?;
        Ok(Monkey {
            items,
            op,
            test,
            if_true,
            if_false,
        })
    }
}

fn run(input: &str, div: i64, rounds: usize) -> Result<usize> {
    let mut monkeys: Vec<Monkey> = input
        .split("\n\n")
        .map(|s| s.parse())
        .collect::<Result<_>>()?;
    let mut inspections = vec![0; monkeys.len()];
    let lcm: i64 = monkeys
        .iter()
        .map(|m| lcm(m.test, m.op.factor()))
        .fold(1, lcm);
    for _round in 0..rounds {
        for idx in 0..monkeys.len() {
            while let Some(item) = monkeys[idx].items.pop_front() {
                let monkey = &monkeys[idx];
                inspections[idx] += 1;
                let item = (monkey.op.apply(item) / div) % lcm;
                let dest = if item % monkey.test == 0 {
                    monkey.if_true
                } else {
                    monkey.if_false
                };

                monkeys[dest].items.push_back(item);
            }
        }
    }
    inspections.sort();
    inspections.reverse();
    Ok(inspections[0] * inspections[1])
}

fn part1(input: &str) -> Result<usize> {
    run(input, 3, 20)
}

fn part2(input: &str) -> Result<usize> {
    run(input, 1, 10000)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(TEST_INPUT)?, 10605);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(TEST_INPUT)?, 2713310158);
        Ok(())
    }
}
