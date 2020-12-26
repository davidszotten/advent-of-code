use anyhow::{bail, Result};
use aoc2015::coor::Coor;
use aoc2015::dispatch;
use std::collections::HashSet;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

fn part1(input: &str) -> Result<usize> {
    let mut pos = Coor::new(0, 0);
    let mut seen = HashSet::new();
    seen.insert(pos.clone());
    for c in input.chars() {
        let offset = match c {
            '>' => Coor::new(1, 0),
            '<' => Coor::new(-1, 0),
            '^' => Coor::new(0, 1),
            'v' => Coor::new(0, -1),
            _ => bail!("invalid char `{}`", c),
        };
        pos += offset;
        seen.insert(pos.clone());
    }
    Ok(seen.len())
}

fn part2(input: &str) -> Result<usize> {
    let mut pos1 = Coor::new(1, 0);
    let mut pos2 = Coor::new(1, 0);
    let mut turn = true;
    let mut seen1 = HashSet::new();
    let mut seen2 = HashSet::new();
    seen1.insert(pos1.clone());
    seen2.insert(pos2.clone());
    for c in input.chars() {
        let offset = match c {
            '>' => Coor::new(1, 0),
            '<' => Coor::new(-1, 0),
            '^' => Coor::new(0, 1),
            'v' => Coor::new(0, -1),
            _ => bail!("invalid char `{}`", c),
        };
        if turn {
            pos1 += offset;
            seen1.insert(pos1.clone());
        } else {
            pos2 += offset;
            seen2.insert(pos2.clone());
        }
        turn = !turn;
    }
    Ok((seen1.union(&seen2)).count())
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(">")?, 2);
        assert_eq!(part1("^>v<")?, 4);
        assert_eq!(part1("^v^v^v^v^v")?, 2);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2("^v")?, 3);
        assert_eq!(part2("^>v<")?, 3);
        assert_eq!(part2("^v^v^v^v^v")?, 11);
        Ok(())
    }
}
