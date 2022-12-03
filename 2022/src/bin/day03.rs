use anyhow::{bail, Context, Result};
use aoc2022::dispatch;
use std::collections::HashSet;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

fn find_item(rucksack: &str) -> Result<char> {
    let left: HashSet<_> = rucksack.chars().take(rucksack.len() / 2).collect();
    let right: HashSet<_> = rucksack.chars().skip(rucksack.len() / 2).collect();
    let item = left
        .intersection(&right)
        .next()
        .context("intersection is empty")?;
    Ok(*item)
}

fn priority(item: char) -> Result<usize> {
    Ok(match item {
        'a'..='z' => item as usize - 'a' as usize + 1,
        'A'..='Z' => item as usize - 'A' as usize + 27,
        _ => bail!("invalid item"),
    })
}

fn part1(input: &str) -> Result<usize> {
    input.split('\n').map(|r| priority(find_item(r)?)).sum()
}

fn find_badge(rucksacks: &[&str]) -> Result<char> {
    let intersection: HashSet<char> = rucksacks
        .iter()
        .map(|rucksack| rucksack.chars().collect::<HashSet<char>>())
        .reduce(|accum, item| accum.intersection(&item).copied().collect())
        .context("no rucksacks")?;
    assert_eq!(intersection.len(), 1);
    intersection
        .into_iter()
        .next()
        .context("intersection is empty")
}

fn part2(input: &str) -> Result<usize> {
    input
        .split('\n')
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|c| priority(find_badge(c)?))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(TEST_INPUT)?, 157);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(TEST_INPUT)?, 70);
        Ok(())
    }
}
