use anyhow::{bail, Context, Error, Result};
use aoc2022::dispatch;
use std::cmp::Ordering;
use std::iter::Peekable;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

#[derive(Debug, PartialEq, Eq, Clone)]
enum Packet {
    List(Vec<Packet>),
    Integer(i32),
}

fn parse_number<I>(chars: &mut Peekable<I>) -> Result<i32>
where
    I: Iterator<Item = char>,
{
    let mut digits = String::new();
    while let Some(c) = chars.next_if(|c| ('0'..='9').contains(c)) {
        digits.push(c);
    }
    digits
        .parse()
        .with_context(|| format!("invalid number `{}`", digits))
}

fn parse<I>(chars: &mut Peekable<I>) -> Result<Packet>
where
    I: Iterator<Item = char>,
{
    let mut list = vec![];
    assert_eq!(chars.next(), Some('['));
    while let Some(c) = chars.peek() {
        match c {
            '[' => {
                list.push(parse(chars)?);
            }
            '0'..='9' => {
                list.push(Packet::Integer(parse_number(chars)?));
            }
            ',' => {
                chars.next();
            }
            ']' => {
                chars.next();
                break;
            }
            _ => bail!("Invalid package character `{}`", c),
        }
    }
    Ok(Packet::List(list))
}

impl std::str::FromStr for Packet {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        parse(&mut s.chars().peekable())
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::List(self_list), Packet::List(other_list)) => {
                for (self_entry, other_entry) in self_list.iter().zip(other_list) {
                    match self_entry.cmp(other_entry) {
                        Ordering::Equal => continue,
                        o => return o,
                    }
                }
                self_list.len().cmp(&other_list.len())
            }

            (Packet::List(_), Packet::Integer(_)) => self.cmp(&Packet::List(vec![other.clone()])),
            (Packet::Integer(_), Packet::List(_)) => Packet::List(vec![self.clone()]).cmp(other),

            (Packet::Integer(self_int), Packet::Integer(other_int)) => self_int.cmp(other_int),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn part1(input: &str) -> Result<usize> {
    let mut sum = 0;
    for (idx, pair) in input.split("\n\n").enumerate() {
        let (a, b) = pair.split_once('\n').unwrap();
        let pa: Packet = a.parse()?;
        let pb: Packet = b.parse()?;
        if pa < pb {
            sum += idx + 1;
        }
    }
    Ok(sum)
}

fn part2(input: &str) -> Result<usize> {
    let mut packets = input
        .split('\n')
        .filter(|&l| !l.is_empty())
        .map(|l| l.parse::<Packet>().unwrap())
        .collect::<Vec<_>>();

    let d1: Packet = "[[2]]".parse().unwrap();
    let d2: Packet = "[[6]]".parse().unwrap();
    packets.push(d1.clone());
    packets.push(d2.clone());

    packets.sort();
    let i1 = packets.iter().position(|p| *p == d1).unwrap() + 1;
    let i2 = packets.iter().position(|p| *p == d2).unwrap() + 1;

    Ok(i1 * i2)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(TEST_INPUT)?, 13);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(TEST_INPUT)?, 140);
        Ok(())
    }

    #[test]
    fn test_parse1() -> Result<()> {
        assert_eq!(
            "[1]".parse::<Packet>()?,
            Packet::List(vec![Packet::Integer(1)])
        );
        Ok(())
    }

    #[test]
    fn test_parse2() -> Result<()> {
        assert_eq!("[]".parse::<Packet>()?, Packet::List(vec![]));
        Ok(())
    }

    #[test]
    fn test_parse3() -> Result<()> {
        assert_eq!(
            "[1,2]".parse::<Packet>()?,
            Packet::List(vec![Packet::Integer(1), Packet::Integer(2)])
        );
        Ok(())
    }

    #[test]
    fn test_parse4() -> Result<()> {
        assert_eq!(
            "[[1],[2,3,4]]".parse::<Packet>()?,
            Packet::List(vec![
                Packet::List(vec![Packet::Integer(1)]),
                Packet::List(vec![
                    Packet::Integer(2),
                    Packet::Integer(3),
                    Packet::Integer(4),
                ])
            ])
        );
        Ok(())
    }
}
