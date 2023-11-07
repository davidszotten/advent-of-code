use anyhow::{bail, Result};
use aoc2015::dispatch;
use permutohedron::LexicalPermutation;
use regex::Regex;
use std::collections::{HashMap, HashSet};

fn main() -> Result<()> {
    dispatch(part1, part2)
}

fn parse(s: &str) -> Result<HashMap<(String, String), i64>> {
    let mut res = HashMap::new();
    let re = Regex::new(r"(?<name1>\w+) would (?<action>gain|lose) (?<amount>\d+) happiness units by sitting next to (?<name2>\w+).").unwrap();
    for line in s.split('\n') {
        let Some(caps) = re.captures(line) else {
            bail!("no regex match for `{}`", line);
        };
        let dir = if &caps["action"] == "gain" { 1 } else { -1 };
        res.insert(
            (caps["name1"].to_string(), caps["name2"].to_string()),
            dir * caps["amount"].parse::<i64>()?,
        );
    }
    Ok(res)
}

fn part1(input: &str) -> Result<i64> {
    let rules = parse(input)?;
    let rules: HashMap<(&str, &str), i64> = rules
        .iter()
        .map(|((s1, s2), v)| ((s1.as_str(), s2.as_str()), *v))
        .collect();
    let names = rules
        .keys()
        .map(|(n, _)| n)
        .chain(rules.keys().map(|(_, n)| n))
        .collect::<HashSet<_>>();
    let mut names = names.iter().collect::<Vec<_>>();

    let mut max = 0;

    loop {
        let mut score = 0;
        let last = names.last().expect("names empty");
        for (n1, n2) in names.iter().zip(names.iter().skip(1)) {
            score += rules.get(&(*n1, *n2)).expect("score missing");
            score += rules.get(&(*n2, *n1)).expect("score missing");
        }
        score += rules.get(&(**last, *names[0])).expect("score missing 2");
        score += rules.get(&(*names[0], **last)).expect("score missing 2");
        max = max.max(score);

        if !names.next_permutation() {
            break;
        }
    }

    Ok(max)
}

fn part2(input: &str) -> Result<i64> {
    let rules = parse(input)?;
    let rules: HashMap<(&str, &str), i64> = rules
        .iter()
        .map(|((s1, s2), v)| ((s1.as_str(), s2.as_str()), *v))
        .collect();
    let names = rules
        .keys()
        .map(|(n, _)| n)
        .chain(rules.keys().map(|(_, n)| n))
        .collect::<HashSet<_>>();
    let mut names = names.iter().collect::<Vec<_>>();
    names.push(&&"me");

    let mut max = 0;

    loop {
        let mut score = 0;
        let last = names.last().expect("names empty");
        for (n1, n2) in names.iter().zip(names.iter().skip(1)) {
            score += rules.get(&(*n1, *n2)).unwrap_or(&0);
            score += rules.get(&(*n2, *n1)).unwrap_or(&0);
        }
        score += rules.get(&(**last, *names[0])).unwrap_or(&0);
        score += rules.get(&(*names[0], **last)).unwrap_or(&0);
        max = max.max(score);

        if !names.next_permutation() {
            break;
        }
    }

    Ok(max)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(INPUT)?, 330);
        Ok(())
    }
}
