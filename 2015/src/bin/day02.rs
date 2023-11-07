use anyhow::{anyhow, Error, Result};
use aoc2015::dispatch;
use std::convert::TryFrom;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

fn area(a: usize, b: usize) -> usize {
    a * b
}

struct Gift {
    w: usize,
    h: usize,
    l: usize,
}

impl Gift {
    fn smallest_sides(&self) -> (usize, usize) {
        let mut sides = [self.w, self.h, self.l];
        sides.sort();
        (sides[0], sides[1])
    }

    fn volume(&self) -> usize {
        self.w * self.h * self.l
    }
}

impl TryFrom<&str> for Gift {
    type Error = Error;
    fn try_from(s: &str) -> Result<Gift> {
        let mut numbers = s.split('x');
        Ok(Gift {
            w: numbers
                .next()
                .ok_or(anyhow!("First number missing"))?
                .parse()
                .map_err(|_| anyhow!("failed to parse first chunk"))?,
            h: numbers
                .next()
                .ok_or(anyhow!("Second number missing"))?
                .parse()
                .map_err(|_| anyhow!("failed to parse second chunk"))?,
            l: numbers
                .next()
                .ok_or(anyhow!("Third number missing"))?
                .parse()
                .map_err(|_| anyhow!("failed to parse third chunk"))?,
        })
    }
}

fn parse(input: &str) -> Result<Vec<Gift>> {
    input
        .split('\n')
        .map(Gift::try_from)
        .collect::<Result<Vec<_>>>()
}

fn part1(input: &str) -> Result<usize> {
    Ok(parse(input)?
        .iter()
        .map(|g| {
            2 * area(g.w, g.h)
                + 2 * area(g.w, g.l)
                + 2 * area(g.h, g.l)
                + area(g.smallest_sides().0, g.smallest_sides().1)
        })
        .sum())
}

fn part2(input: &str) -> Result<usize> {
    Ok(parse(input)?
        .iter()
        .map(|g| 2 * (g.smallest_sides().0 + g.smallest_sides().1) + g.volume())
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2x3x4\n1x1x10";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1("2x3x4")?, 58);
        assert_eq!(part1(INPUT)?, 58 + 43);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2("2x3x4")?, 34);
        assert_eq!(part2(INPUT)?, 34 + 14);
        Ok(())
    }
}
