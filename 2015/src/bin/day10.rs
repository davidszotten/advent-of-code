use anyhow::{Context, Result};
use aoc2015::dispatch;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

fn next(seq: &[usize]) -> Vec<usize> {
    let mut it = seq.iter().peekable();
    let mut res = vec![];
    while let Some(num) = it.next() {
        let mut count = 1;
        while it.peek() == Some(&num) {
            it.next();
            count += 1;
        }
        res.push(count);
        res.push(*num);
    }
    res
}

fn run(input: &str, rounds: usize) -> Result<usize> {
    let input: Vec<usize> = input
        .chars()
        .map(|c| c.to_string().parse().context("parse digit"))
        .collect::<Result<Vec<_>>>()?;
    let mut seq = input;
    for _ in 0..rounds {
        seq = next(&seq);
    }
    Ok(seq.len())
}

fn part1(input: &str) -> Result<usize> {
    run(input, 40)
}

fn part2(input: &str) -> Result<usize> {
    run(input, 50)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "";

    #[test]
    fn test_next() {
        assert_eq!(next(&vec![1]), vec![1, 1]);
        assert_eq!(next(&vec![1, 1]), vec![2, 1]);
        assert_eq!(next(&vec![2, 1]), vec![1, 2, 1, 1]);
    }

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(INPUT)?, 0);
        Ok(())
    }
}
