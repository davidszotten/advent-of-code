use anyhow::Result;
use aoc2015::dispatch;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

fn part1(_input: &str) -> Result<usize> {
    Ok(0)
}

fn part2(_input: &str) -> Result<usize> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = ""

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(INPUT)?, 0);
        Ok(())
    }
}
