use anyhow::{Context, Error, Result};
use aoc2022::dispatch;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

#[derive(Debug)]
struct Range {
    start: i32,
    end: i32,
}

impl Range {
    fn contains(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.start <= other.end && self.end >= other.start
    }
}

impl std::str::FromStr for Range {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (start, end) = s
            .split_once('-')
            .with_context(|| format!("no dash in `{}`", s))?;
        Ok(Range {
            start: start.parse()?,
            end: end.parse()?,
        })
    }
}

#[derive(Debug)]
struct Pair(Range, Range);

impl Pair {
    fn contains(&self) -> bool {
        self.0.contains(&self.1) || self.1.contains(&self.0)
    }

    fn overlaps(&self) -> bool {
        self.0.overlaps(&self.1)
    }
}

impl std::str::FromStr for Pair {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let raw = s
            .split_once(',')
            .with_context(|| format!("no comma in `{}`", s))?;
        Ok(Pair(raw.0.parse()?, raw.1.parse()?))
    }
}

fn parse(input: &str) -> Result<Vec<Pair>> {
    input
        .split('\n')
        .map(|r| r.parse::<Pair>())
        .collect::<Result<Vec<_>>>()
}

fn part1(input: &str) -> Result<usize> {
    Ok(parse(input)?.iter().filter(|p| p.contains()).count())
}

fn part2(input: &str) -> Result<usize> {
    Ok(parse(input)?.iter().filter(|p| p.overlaps()).count())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(TEST_INPUT)?, 2);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(TEST_INPUT)?, 4);
        Ok(())
    }
}
