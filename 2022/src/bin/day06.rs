use anyhow::{bail, Result};
use aoc2022::dispatch;
use std::collections::HashSet;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

fn find(input: &str, length: usize) -> Result<usize> {
    for start in length..input.len() {
        if input[start - length..start]
            .chars()
            .collect::<HashSet<_>>()
            .len()
            == length
        {
            return Ok(start);
        }
    }
    bail!("not found")
}

fn part1(input: &str) -> Result<usize> {
    find(input, 4)
}

fn part2(input: &str) -> Result<usize> {
    find(input, 14)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1("mjqjpqmgbljsphdztnvjfqwrcgsmlb")?, 7);
        assert_eq!(part1("bvwbjplbgvbhsrlpgdmjqwftvncz")?, 5);
        assert_eq!(part1("nppdvjthqldpwncqszvftbrmjlhg")?, 6);
        assert_eq!(part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")?, 10);
        assert_eq!(part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")?, 11);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb")?, 19);
        assert_eq!(part2("bvwbjplbgvbhsrlpgdmjqwftvncz")?, 23);
        assert_eq!(part2("nppdvjthqldpwncqszvftbrmjlhg")?, 23);
        assert_eq!(part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")?, 29);
        assert_eq!(part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")?, 26);
        Ok(())
    }
}
