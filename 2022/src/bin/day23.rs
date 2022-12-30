use anyhow::{Error, Result};
use aoc2022::coor::Coor;
use aoc2022::dispatch;
use std::collections::{HashMap, HashSet, VecDeque};

fn main() -> Result<()> {
    dispatch(part1, part2)
}

#[derive(Debug)]
struct Map {
    elves: HashSet<Coor>,
    // go
    directions: VecDeque<Coor>,
}

impl Map {
    fn new(elves: HashSet<Coor>) -> Self {
        Map {
            elves,
            directions: VecDeque::from([
                Coor::new(0, -1),
                Coor::new(0, 1),
                Coor::new(-1, 0),
                Coor::new(1, 0),
            ]),
        }
    }
    fn _print(&self) {
        let x_min = self.elves.iter().map(|c| c.x).min().unwrap();
        let x_max = self.elves.iter().map(|c| c.x).max().unwrap();
        let y_min = self.elves.iter().map(|c| c.y).min().unwrap();
        let y_max = self.elves.iter().map(|c| c.y).max().unwrap();
        for y in y_min..=y_max {
            for x in x_min..=x_max {
                print!(
                    "{}",
                    if self.elves.contains(&Coor::new(x, y)) {
                        '#'
                    } else {
                        '.'
                    }
                );
            }
            println!();
        }
        println!();
    }

    fn count(&self) -> usize {
        let x_min = self.elves.iter().map(|c| c.x).min().unwrap();
        let x_max = self.elves.iter().map(|c| c.x).max().unwrap();
        let y_min = self.elves.iter().map(|c| c.y).min().unwrap();
        let y_max = self.elves.iter().map(|c| c.y).max().unwrap();
        let mut count = 0;
        for y in y_min..=y_max {
            for x in x_min..=x_max {
                if !self.elves.contains(&Coor::new(x, y)) {
                    count += 1;
                }
            }
        }
        count
    }

    fn alone(&self, pos: &Coor) -> bool {
        for direction in &self.directions {
            let sweep = Coor::new(-direction.y, direction.x);
            if [-1, 0, 1]
                .iter()
                .any(|offset| self.elves.contains(&(*pos + *direction + sweep * *offset)))
            {
                return false;
            }
        }
        true
    }

    fn free(&self, pos: &Coor, direction: &Coor) -> bool {
        let sweep = Coor::new(-direction.y, direction.x);
        [-1, 0, 1]
            .iter()
            .all(|offset| !self.elves.contains(&(*pos + *direction + sweep * *offset)))
    }

    fn mv(&mut self) -> bool {
        let mut moved = false;
        let mut suggestion: HashMap<Coor, Vec<Coor>> = HashMap::new();
        for elf in &self.elves {
            if self.alone(elf) {
                continue;
            }
            for direction in &self.directions {
                if self.free(elf, direction) {
                    suggestion.entry(*elf + *direction).or_default().push(*elf);
                    break;
                }
            }
        }
        for (new, old) in suggestion {
            if old.len() > 1 {
                continue;
            }
            moved = true;
            self.elves.remove(&old[0]);
            self.elves.insert(new);
        }
        let tmp = self.directions.pop_front().unwrap();
        self.directions.push_back(tmp);
        moved
    }
}

impl std::str::FromStr for Map {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let mut elves = HashSet::new();
        for (y, line) in s.lines().enumerate() {
            let y = y as i64;
            for (x, c) in line.chars().enumerate() {
                let x = x as i64;
                if c == '#' {
                    elves.insert(Coor::new(x, y));
                }
            }
        }
        Ok(Self::new(elves))
    }
}

fn part1(input: &str) -> Result<usize> {
    let mut map: Map = input.parse()?;
    // map._print();
    for _ in 0..10 {
        map.mv();
        // map._print();
    }
    // map._print();
    Ok(map.count())
}

fn part2(input: &str) -> Result<usize> {
    let mut map: Map = input.parse()?;
    let mut count = 1;
    while map.mv() {
        count += 1
    }
    Ok(count)
}

#[cfg(test)]
mod tests {
    use super::*;

    const _TEST_INPUT: &str = ".....
..##.
..#..
.....
..##.
.....";

    const TEST_INPUT: &str = "..............
..............
.......#......
.....###.#....
...#...#.#....
....#...##....
...#.###......
...##.#.##....
....#..#......
..............
..............
..............";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(TEST_INPUT)?, 110);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(TEST_INPUT)?, 20);
        Ok(())
    }
}
