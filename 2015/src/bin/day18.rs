use std::collections::HashSet;

use anyhow::Result;
use aoc2015::{coor::Coor, dispatch};

struct Map {
    size: usize,
    coors: HashSet<Coor>,
    corners_always_on: bool,
}

fn main() -> Result<()> {
    dispatch(part1, part2)
}

fn maybe_set_corners(coors: &mut HashSet<Coor>, size: i64, corners_always_on: bool) {
    if corners_always_on {
        coors.insert(Coor::new(0, 0));
        coors.insert(Coor::new(size - 1, 0));
        coors.insert(Coor::new(0, size - 1));
        coors.insert(Coor::new(size - 1, size - 1));
    }
}

fn parse(s: &str, size: usize, corners_always_on: bool) -> Map {
    let mut coors = HashSet::new();
    for (y, line) in s.split('\n').enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                coors.insert(Coor::new(x as i64, y as i64));
            }
        }
    }
    maybe_set_corners(&mut coors, size as i64, corners_always_on);
    Map {
        coors,
        size,
        corners_always_on,
    }
}

fn neighbours(map: &Map, coor: Coor) -> usize {
    let mut count = 0;
    for x in [-1, 0, 1] {
        for y in [-1, 0, 1] {
            if x == 0 && y == 0 {
                continue;
            }
            let other = Coor::new(coor.x + x, coor.y + y);
            if map.coors.contains(&other) {
                count += 1
            }
        }
    }
    count
}

fn step(map: Map) -> Map {
    let mut new = HashSet::new();
    for x in 0..map.size {
        for y in 0..map.size {
            let coor = Coor::new(x as i64, y as i64);
            let n = neighbours(&map, coor);
            if map.coors.contains(&coor) && (n == 2 || n == 3) {
                new.insert(coor);
            }
            if !map.coors.contains(&coor) && n == 3 {
                new.insert(coor);
            }
        }
    }

    maybe_set_corners(&mut new, map.size as i64, map.corners_always_on);

    Map {
        size: map.size,
        coors: new,
        corners_always_on: map.corners_always_on,
    }
}

fn part1(input: &str) -> Result<usize> {
    let mut map = parse(input, 100, false);
    for _ in 0..100 {
        map = step(map);
    }
    Ok(map.coors.len())
}

fn part2(input: &str) -> Result<usize> {
    let mut map = parse(input, 100, true);
    for _ in 0..100 {
        map = step(map);
    }
    Ok(map.coors.len())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = ".#.#.#
...##.
#....#
..#...
#.#..#
####..";

    #[test]
    fn test_part1() {
        let mut map = parse(INPUT, 6, false);
        for _ in 0..4 {
            map = step(map);
        }
        assert_eq!(map.coors.len(), 4);
    }

    #[test]
    fn test_part2() {
        let mut map = parse(INPUT, 6, true);
        for _ in 0..5 {
            map = step(map);
        }
        assert_eq!(map.coors.len(), 17);
    }
}
