use anyhow::{Context, Result};
use aoc2015::dispatch;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

fn find(input: &str, sum: u32) -> Result<u32> {
    let containers: Vec<_> = input
        .split('\n')
        .map(|c| c.parse::<u32>().context("nan"))
        .collect::<Result<_>>()?;
    let mut count = 0;
    // dbg!(&containers);
    for n in 0..2_u32.pow(containers.len() as u32) {
        let total: u32 = containers
            .iter()
            .enumerate()
            .map(|(idx, val)| ((n >> idx) & 1) * val)
            .sum();
        // println!("{n:#b} {total}");
        if total == sum {
            count += 1;
        }
    }
    Ok(count)
}

fn find2(input: &str, sum: u32) -> Result<u32> {
    let containers: Vec<_> = input
        .split('\n')
        .map(|c| c.parse::<u32>().context("nan"))
        .collect::<Result<_>>()?;
    let mut min = 2_u32.pow(containers.len() as u32);
    for n in 0..2_u32.pow(containers.len() as u32) {
        let total: u32 = containers
            .iter()
            .enumerate()
            .map(|(idx, val)| ((n >> idx) & 1) * val)
            .sum();
        if total == sum {
            min = min.min(n.count_ones());
        }
    }

    let mut count = 0;
    for n in 0..2_u32.pow(containers.len() as u32) {
        let total: u32 = containers
            .iter()
            .enumerate()
            .map(|(idx, val)| ((n >> idx) & 1) * val)
            .sum();
        if total == sum && n.count_ones() == min {
            count += 1;
        }
    }
    Ok(count)
}

fn part1(input: &str) -> Result<u32> {
    find(input, 150)
}

fn part2(input: &str) -> Result<u32> {
    find2(input, 150)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "20
15
10
5
5";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(find(INPUT, 25)?, 4);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(find2(INPUT, 25)?, 3);
        Ok(())
    }
}
