use anyhow::{Context, Result};
use aoc2015::dispatch;
use itertools::Itertools;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

fn parse(input: &str) -> Result<Vec<usize>> {
    input
        .split('\n')
        .map(|l| l.parse::<usize>().context("nan"))
        .collect()
}

fn run(input: &str, groups: usize) -> Result<usize> {
    let numbers = parse(input)?;
    let sum: usize = numbers.iter().sum();
    let target = sum / groups;
    for count in 1..numbers.len() {
        let option = numbers
            .iter()
            .combinations(count)
            .filter(|c| c.iter().map(|n| **n).sum::<usize>() == target)
            .map(|c| c.iter().map(|n| **n).product::<usize>())
            .min();
        if let Some(option) = option {
            return Ok(option);
        }
    }

    Ok(0)
}

fn part1(input: &str) -> Result<usize> {
    run(input, 3)
}

fn part2(input: &str) -> Result<usize> {
    run(input, 4)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1
2
3
4
5
7
8
9
10
11";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(INPUT)?, 99);
        Ok(())
    }
}
