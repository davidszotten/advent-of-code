use anyhow::Result;
use aoc2022::dispatch;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

fn part1(input: &str) -> Result<usize> {
    Ok(input.split('\n').count())
}

fn part2(input: &str) -> Result<usize> {
    Ok(input.split('\n').count())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "1721
263";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(TEST_INPUT)?, 2);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(TEST_INPUT)?, 2);
        Ok(())
    }
}
