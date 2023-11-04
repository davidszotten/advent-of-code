use permutohedron::LexicalPermutation;
use std::collections::{HashMap, HashSet};

use anyhow::{Context, Result};
use aoc2015::dispatch;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

fn parse(input: &str) -> Result<HashMap<(&str, &str), usize>> {
    let mut res = HashMap::new();
    for line in input.split('\n') {
        let (cities, distance) = line.split_once(" = ").context("eq sign")?;
        let (c1, c2) = cities.split_once(" to ").context("to")?;
        res.insert((c1, c2), distance.parse()?);
    }
    Ok(res)
}

fn run(input: &str) -> Result<(usize, usize)> {
    let distances = parse(input)?;
    let cities = distances
        .keys()
        .map(|(c, _)| c)
        .chain(distances.keys().map(|(_, c)| c))
        .collect::<HashSet<_>>();
    let mut cities = cities.iter().collect::<Vec<_>>();
    cities.sort();
    let mut min: Option<usize> = None;
    let mut max: Option<usize> = None;
    loop {
        let mut distance = 0;
        let mut fail = false;
        for (c1, c2) in cities.iter().zip(cities.iter().skip(1)) {
            let d1: Option<&usize> = distances.get(&(c1, c2));
            let d2: Option<&usize> = distances.get(&(c2, c1));
            match (d1, d2) {
                (Some(next_distance), _) => distance += next_distance,
                (_, Some(next_distance)) => distance += next_distance,
                (None, None) => fail = true,
            }
        }
        if !fail {
            min = Some(match min {
                None => distance,
                Some(d) => d.min(distance),
            });
            max = Some(match max {
                None => distance,
                Some(d) => d.max(distance),
            });
        }
        if !cities.next_permutation() {
            break;
        }
    }
    Ok((min.expect("empty"), max.expect("empty2")))
}

fn part1(input: &str) -> Result<usize> {
    run(input).map(|r| r.0)
}

fn part2(input: &str) -> Result<usize> {
    run(input).map(|r| r.1)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(INPUT)?, 605);
        Ok(())
    }
}
