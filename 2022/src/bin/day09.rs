use anyhow::{bail, Context, Error, Result};
use aoc2022::coor::Coor;
use aoc2022::dispatch;
use std::collections::HashSet;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

struct Move {
    direction: Coor,
    distance: usize,
}

impl std::str::FromStr for Move {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let (raw_dir, raw_dist) = s.split_once(' ').context("malformed instruction")?;
        let direction = match raw_dir {
            "R" => Coor::new(1, 0),
            "L" => Coor::new(-1, 0),
            "U" => Coor::new(0, 1),
            "D" => Coor::new(0, -1),
            _ => bail!("unknown direction"),
        };
        Ok(Move {
            direction,
            distance: raw_dist.parse()?,
        })
    }
}

fn follow(head: Coor, tail: Coor) -> Coor {
    let delta = head - tail;
    if delta.x.abs() <= 1 && delta.y.abs() <= 1 {
        return tail;
    }
    tail + Coor::new(delta.x.signum(), delta.y.signum())
}

fn part1(input: &str) -> Result<usize> {
    run(input, 2)
}

fn run(input: &str, length: usize) -> Result<usize> {
    let mut knots = vec![];
    for _ in 0..length {
        knots.push(Coor::new(0, 0));
    }
    let mut positions = HashSet::new();
    positions.insert(knots[length - 1]);
    for instruction in input.split('\n').map(|r| r.parse::<Move>()) {
        let instruction = instruction?;
        for _ in 0..instruction.distance {
            knots[0] += instruction.direction;
            for k in 1..length {
                knots[k] = follow(knots[k - 1], knots[k]);
            }
            positions.insert(knots[length - 1]);
        }
    }
    Ok(positions.len())
}

fn part2(input: &str) -> Result<usize> {
    run(input, 10)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(TEST_INPUT)?, 13);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(TEST_INPUT)?, 1);
        Ok(())
    }

    #[test]
    fn test_part2b() -> Result<()> {
        assert_eq!(
            part2(
                "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20"
            )?,
            36
        );
        Ok(())
    }
}
