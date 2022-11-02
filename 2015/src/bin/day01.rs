use anyhow::{anyhow, Result};
use aoc2015::dispatch;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

fn part1(input: &str) -> Result<i32> {
    Ok(input
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            c => unreachable!("bad input `{}`", c),
        })
        .sum())
}

fn part2(input: &str) -> Result<usize> {
    input
        .chars()
        .map(|c| match c {
            '(' => 1,
            ')' => -1,
            c => unreachable!("bad input `{}`", c),
        })
        .scan(0, |state, x| {
            *state += x;
            Some(*state)
        })
        .enumerate()
        .skip_while(|(_, v)| *v >= 0)
        .map(|(idx, _)| idx)
        .next()
        .ok_or(anyhow!("empty?"))
        .map(|s| s + 1) // 0 vs 1 indexing
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = ")())())";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(INPUT)?, -3);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(")")?, 1);
        Ok(())
    }
}
