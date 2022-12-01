use anyhow::{Context, Result};
use aoc2022::dispatch;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

fn parse(input: &str) -> Result<Vec<usize>> {
    input
        .split("\n\n")
        .map(|elf| {
            elf.split('\n')
                .map(|e| e.parse::<usize>().context("parse error"))
                .sum()
        })
        .collect()
}

fn part1(input: &str) -> Result<usize> {
    parse(input)?.into_iter().max().context("no max")
}

fn part2(input: &str) -> Result<usize> {
    let mut entries = parse(input)?;
    entries.sort();
    Ok(entries.iter().rev().take(3).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "10

20

30
40
50

60";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(TEST_INPUT)?, 120);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(TEST_INPUT)?, 200);
        Ok(())
    }
}
