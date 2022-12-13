use anyhow::{Context, Error, Result};
use aoc2022::coor::Coor;
use aoc2022::dispatch;
use std::collections::{HashMap, HashSet, VecDeque};

fn main() -> Result<()> {
    dispatch(part1, part2)
}

#[derive(Debug)]
struct Map {
    start: Coor,
    end: Coor,
    heights: HashMap<Coor, i32>,
}

impl std::str::FromStr for Map {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let mut heights = HashMap::new();
        let mut start = None;
        let mut end = None;
        for (y, line) in s.trim().lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                let pos = Coor::new(x as i64, y as i64);
                let height = if c == 'S' {
                    start = Some(pos);
                    'a'
                } else if c == 'E' {
                    end = Some(pos);
                    'z'
                } else {
                    c
                } as i32
                    - 'a' as i32;

                heights.insert(pos, height);
            }
        }
        let start = start.context("start missing")?;
        let end = end.context("end missing")?;
        Ok(Self {
            heights,
            start,
            end,
        })
    }
}

fn solve(map: &Map, start: Coor) -> Option<i32> {
    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    queue.push_back((start, 0));
    while let Some(entry) = queue.pop_front() {
        let (pos, dist) = entry;
        if pos == map.end {
            return Some(dist);
        }
        for offset in [
            Coor::new(-1, 0),
            Coor::new(1, 0),
            Coor::new(0, -1),
            Coor::new(0, 1),
        ] {
            let next = pos + offset;
            if seen.contains(&next) {
                continue;
            }
            if let (Some(next_height), Some(pos_height)) =
                (map.heights.get(&next), map.heights.get(&pos))
            {
                if *next_height <= pos_height + 1 {
                    seen.insert(next);
                    queue.push_back((next, dist + 1));
                }
            }
        }
    }
    None
}

fn part1(input: &str) -> Result<i32> {
    let map: Map = input.parse()?;
    solve(&map, map.start).context("failed to solve")
}

fn part2(input: &str) -> Result<i32> {
    let map: Map = input.parse()?;
    let best = map
        .heights
        .iter()
        .filter(|&(_, height)| *height == 0)
        .filter_map(|(&start, _)| solve(&map, start))
        .min();

    best.context("missing")
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(TEST_INPUT)?, 31);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(TEST_INPUT)?, 29);
        Ok(())
    }
}
