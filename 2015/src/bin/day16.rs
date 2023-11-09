use std::collections::HashMap;

use anyhow::{Context, Result};
use aoc2015::dispatch;
use regex::Regex;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

fn sue(s: &str) -> Result<usize> {
    let re = Regex::new(r"Sue (\d+):").context("regex build")?;
    let cap = re.captures(s).context("regex match")?;
    cap[1].parse().context("not a number")
}

fn parse(s: &str) -> Result<HashMap<&str, usize>> {
    let mut res = HashMap::new();
    let re = Regex::new(r"(?<type>\w+): (?<count>\d+)").unwrap();
    for cap in re.captures_iter(s) {
        res.insert(
            cap.get(1).context("get 1")?.as_str(),
            cap[2].parse::<usize>()?,
        );
    }
    Ok(res)
}

fn part1(input: &str) -> Result<usize> {
    let target: HashMap<&str, usize> = [
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ]
    .iter()
    .cloned()
    .collect();
    'outer: for line in input.split('\n') {
        let parsed = parse(line)?;
        for key in parsed.keys() {
            if parsed.get(key) != target.get(key) {
                continue 'outer;
            }
        }
        return sue(line);
    }
    Ok(0)
}

fn part2(input: &str) -> Result<usize> {
    let target: HashMap<&str, usize> = [
        ("children", 3),
        ("cats", 7),
        ("samoyeds", 2),
        ("pomeranians", 3),
        ("akitas", 0),
        ("vizslas", 0),
        ("goldfish", 5),
        ("trees", 3),
        ("cars", 2),
        ("perfumes", 1),
    ]
    .iter()
    .cloned()
    .collect();
    'outer: for line in input.split('\n') {
        let parsed = parse(line)?;
        for key in parsed.keys() {
            match *key {
                "cats" | "trees" => {
                    if parsed.get(key) <= target.get(key) {
                        continue 'outer;
                    }
                }
                "pomeranians" | "goldfish" => {
                    if parsed.get(key) >= target.get(key) {
                        continue 'outer;
                    }
                }
                _ => {
                    if parsed.get(key) != target.get(key) {
                        continue 'outer;
                    }
                }
            }
        }
        return sue(line);
    }
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Sue 17: akitas: 7, vizslas: 0, goldfish: 6
Sue 18: trees: 5, vizslas: 9, cars: 0";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(INPUT)?, 0);
        Ok(())
    }
}
