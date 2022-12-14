use anyhow::{bail, Result};
use aoc2022::coor::Coor;
use aoc2022::dispatch;
use std::collections::HashSet;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

#[derive(Debug)]
struct HLine {
    x_start: i64,
    x_end: i64,
    y: i64,
}

#[derive(Debug)]
struct VLine {
    x: i64,
    y_start: i64,
    y_end: i64,
}

#[derive(Debug)]
struct Map {
    horizontal: Vec<HLine>,
    vertical: Vec<VLine>,
    sand: HashSet<Coor>,
    bottom: i64,
    floor: bool,
}

impl Map {
    fn is_rock(&self, coor: &Coor) -> bool {
        for hline in &self.horizontal {
            if hline.x_start <= coor.x && coor.x <= hline.x_end && coor.y == hline.y {
                return true;
            }
        }
        for vline in &self.vertical {
            if vline.y_start <= coor.y && coor.y <= vline.y_end && coor.x == vline.x {
                return true;
            }
        }
        false
    }

    fn is_free(&self, coor: &Coor) -> bool {
        if self.floor && coor.y == self.bottom + 2 {
            return false;
        }
        !self.sand.contains(coor) && !self.is_rock(coor)
    }

    fn next(&self, pos: Coor) -> Option<Coor> {
        for offset in [Coor::new(0, 1), Coor::new(-1, 1), Coor::new(1, 1)] {
            let next = pos + offset;
            if self.is_free(&next) {
                return Some(next);
            }
        }
        None
    }

    fn drop_settled(&mut self) -> bool {
        let start = Coor::new(500, 0);
        let mut pos = start;
        while let Some(next) = self.next(pos) {
            pos = next;
            if !self.floor && pos.y > self.bottom {
                return false;
            }
        }
        if pos == start {
            return false;
        }

        self.sand.insert(pos);
        true
    }
}

fn parse(s: &str) -> Result<Map> {
    // 503,4 -> 502,4 -> 502,9 -> 494,9
    let mut horizontal = vec![];
    let mut vertical = vec![];
    let mut bottom = 0;
    let paths = s
        .split('\n')
        .map(|line| {
            line.split(" -> ")
                .map(|pair| pair.parse::<Coor>())
                .collect::<Result<Vec<Coor>>>()
        })
        .collect::<Result<Vec<Vec<Coor>>>>()?;
    for path in paths {
        for (coor1, coor2) in path.iter().zip(path.iter().skip(1)) {
            bottom = bottom.max(coor1.y).max(coor2.y);
            if coor1.x == coor2.x {
                vertical.push(VLine {
                    x: coor1.x,
                    y_start: coor1.y.min(coor2.y),
                    y_end: coor1.y.max(coor2.y),
                });
            } else if coor1.y == coor2.y {
                horizontal.push(HLine {
                    x_start: coor1.x.min(coor2.x),
                    x_end: coor1.x.max(coor2.x),
                    y: coor1.y,
                });
            } else {
                bail!("invalid path segment")
            }
        }
    }
    Ok(Map {
        horizontal,
        vertical,
        sand: HashSet::new(),
        bottom,
        floor: false,
    })
}

fn part1(input: &str) -> Result<i32> {
    let mut map = parse(input)?;
    let mut count = 0;
    while map.drop_settled() {
        count += 1
    }
    Ok(count)
}

fn part2(input: &str) -> Result<i32> {
    let mut map = parse(input)?;
    map.floor = true;
    let mut count = 0;
    while map.drop_settled() {
        count += 1
    }
    Ok(count + 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(TEST_INPUT)?, 24);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(TEST_INPUT)?, 93);
        Ok(())
    }
}
