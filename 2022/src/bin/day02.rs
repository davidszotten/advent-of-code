use anyhow::Result;
use aoc2022::dispatch;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

fn part1(input: &str) -> Result<i32> {
    #[allow(clippy::identity_op)]
    Ok(input
        .split('\n')
        .map(|row| match row {
            "A X" => 1 + 3,
            "B X" => 1 + 0,
            "C X" => 1 + 6,
            "A Y" => 2 + 6,
            "B Y" => 2 + 3,
            "C Y" => 2 + 0,
            "A Z" => 3 + 0,
            "B Z" => 3 + 6,
            "C Z" => 3 + 3,
            _ => panic!("invalid entry"),
        })
        .sum())
}

fn part2(input: &str) -> Result<i32> {
    #[allow(clippy::identity_op)]
    Ok(input
        .split('\n')
        .map(|row| match row {
            "A X" => 3 + 0,
            "B X" => 1 + 0,
            "C X" => 2 + 0,
            "A Y" => 1 + 3,
            "B Y" => 2 + 3,
            "C Y" => 3 + 3,
            "A Z" => 2 + 6,
            "B Z" => 3 + 6,
            "C Z" => 1 + 6,
            _ => panic!("invalid entry"),
        })
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(TEST_INPUT)?, 15);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(TEST_INPUT)?, 12);
        Ok(())
    }
}
