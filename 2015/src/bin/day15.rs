use anyhow::{Context, Error, Result};
use aoc2015::dispatch;
use std::convert::TryFrom;

struct Spoon {
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

impl TryFrom<&str> for Spoon {
    type Error = Error;
    fn try_from(s: &str) -> Result<Spoon> {
        let mut parts = s.split(' ');
        parts.next(); // name
        parts.next(); // capacity
        let capacity = parts
            .next()
            .context("capacity")?
            .strip_suffix(',')
            .context("comma")?
            .parse()?;
        parts.next(); // durability
        let durability = parts
            .next()
            .context("durability")?
            .strip_suffix(',')
            .context("comma")?
            .parse()?;
        parts.next(); // flavor
        let flavor = parts
            .next()
            .context("flavor")?
            .strip_suffix(',')
            .context("comma")?
            .parse()?;
        parts.next(); // texture
        let texture = parts
            .next()
            .context("texture")?
            .strip_suffix(',')
            .context("comma")?
            .parse()?;
        parts.next(); // calories
        let calories = parts.next().context("calories")?.parse()?;
        Ok(Spoon {
            capacity,
            durability,
            flavor,
            texture,
            calories,
        })
    }
}

fn main() -> Result<()> {
    dispatch(part1, part2)
}

fn part1(input: &str) -> Result<i64> {
    let spoons: Vec<_> = input
        .split('\n')
        .map(Spoon::try_from)
        .collect::<Result<_>>()?;

    let mut max = 0;
    // TODO: how to make this generic over len?
    for x in 1..100 {
        for y in 1..100 {
            for z in 1..100 {
                for w in 1..100 {
                    if x + y + z + w != 100 {
                        continue;
                    }

                    let score = (spoons[0].capacity * x
                        + spoons[1].capacity * y
                        + spoons[2].capacity * z
                        + spoons[3].capacity * w)
                        .max(0)
                        * (spoons[0].durability * x
                            + spoons[1].durability * y
                            + spoons[2].durability * z
                            + spoons[3].durability * w)
                            .max(0)
                        * (spoons[0].flavor * x
                            + spoons[1].flavor * y
                            + spoons[2].flavor * z
                            + spoons[3].flavor * w)
                            .max(0)
                        * (spoons[0].texture * x
                            + spoons[1].texture * y
                            + spoons[2].texture * z
                            + spoons[3].texture * w)
                            .max(0);
                    max = max.max(score);
                }
            }
        }
    }

    Ok(max)
}

fn part2(input: &str) -> Result<i64> {
    let spoons: Vec<_> = input
        .split('\n')
        .map(Spoon::try_from)
        .collect::<Result<_>>()?;

    let mut max = 0;
    // TODO: how to make this generic over len?
    for x in 1..100 {
        for y in 1..100 {
            for z in 1..100 {
                for w in 1..100 {
                    if x + y + z + w != 100 {
                        continue;
                    }

                    if (spoons[0].calories * x
                        + spoons[1].calories * y
                        + spoons[2].calories * z
                        + spoons[3].calories * w)
                        != 500
                    {
                        continue;
                    }
                    let score = (spoons[0].capacity * x
                        + spoons[1].capacity * y
                        + spoons[2].capacity * z
                        + spoons[3].capacity * w)
                        .max(0)
                        * (spoons[0].durability * x
                            + spoons[1].durability * y
                            + spoons[2].durability * z
                            + spoons[3].durability * w)
                            .max(0)
                        * (spoons[0].flavor * x
                            + spoons[1].flavor * y
                            + spoons[2].flavor * z
                            + spoons[3].flavor * w)
                            .max(0)
                        * (spoons[0].texture * x
                            + spoons[1].texture * y
                            + spoons[2].texture * z
                            + spoons[3].texture * w)
                            .max(0);
                    max = max.max(score);
                }
            }
        }
    }

    Ok(max)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(INPUT)?, 0);
        Ok(())
    }
}
