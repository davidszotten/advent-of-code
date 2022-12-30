use anyhow::{bail, Error, Result};
use aoc2022::coor::Coor;
use aoc2022::dispatch;
use std::collections::{HashMap, HashSet, VecDeque};

fn main() -> Result<()> {
    dispatch(part1, part2)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Open,
    Wall,
    Blizzard(Direction),
}

impl TryFrom<char> for Tile {
    type Error = Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        Ok(match c {
            '.' => Tile::Open,
            '#' => Tile::Wall,
            '>' => Tile::Blizzard(Direction::Right),
            '<' => Tile::Blizzard(Direction::Left),
            'v' => Tile::Blizzard(Direction::Down),
            '^' => Tile::Blizzard(Direction::Up),
            _ => bail!("invalid tile"),
        })
    }
}

#[derive(Debug, Clone)]
struct Map {
    tiles: HashMap<Coor, Tile>,
    blizzards: Vec<(Coor, Direction)>,
    max_x: i64,
    max_y: i64,
}

impl Map {
    fn _print(&self, pos: Coor) {
        for y in 0..=self.max_y {
            for x in 0..=self.max_x {
                if pos.x == x && pos.y == y {
                    print!("E");
                    continue;
                }
                let blizzards = self
                    .blizzards
                    .iter()
                    .filter(|(c, _)| c.x == x && c.y == y)
                    .map(|(_, d)| d)
                    .collect::<Vec<_>>();
                if blizzards.is_empty() {
                    print!(
                        "{}",
                        match self.tiles.get(&Coor::new(x, y)).unwrap() {
                            Tile::Open => '.',
                            Tile::Wall => '#',
                            Tile::Blizzard(_) => unreachable!(),
                        }
                    );
                } else if blizzards.len() > 1 {
                    print!("{}", blizzards.len());
                } else {
                    print!(
                        "{}",
                        match blizzards[0] {
                            Direction::Right => '>',
                            Direction::Left => '<',
                            Direction::Up => '^',
                            Direction::Down => 'v',
                        }
                    );
                }
            }
            println!();
        }
        println!();
    }

    fn wrap(&self, coor: Coor, dir: Coor) -> Coor {
        if dir.x == 0 {
            Coor::new(coor.x, self.max_y - coor.y)
        } else {
            Coor::new(self.max_x - coor.x, coor.y)
        }
    }

    fn mv(&mut self) {
        let mut new_blizzards = vec![];
        for (coor, dir) in &self.blizzards {
            let coor_dir = match dir {
                Direction::Up => Coor::new(0, -1),
                Direction::Down => Coor::new(0, 1),
                Direction::Right => Coor::new(1, 0),
                Direction::Left => Coor::new(-1, 0),
            };
            let mut next = *coor + coor_dir;
            if *self.tiles.get(&next).unwrap() == Tile::Wall {
                next = self.wrap(*coor, coor_dir);
            }

            new_blizzards.push((next, *dir));
        }
        self.blizzards = new_blizzards;
    }
}

impl std::str::FromStr for Map {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let mut tiles = HashMap::new();
        let mut blizzards = vec![];
        let mut max_x = 0;
        let mut max_y = 0;
        for (y, line) in s.lines().enumerate() {
            let y = y as i64;
            for (x, c) in line.chars().enumerate() {
                let x = x as i64;
                let coor = Coor::new(x, y);
                let tile = c.try_into()?;
                match tile {
                    Tile::Open | Tile::Wall => {
                        tiles.insert(coor, tile);
                    }
                    Tile::Blizzard(direction) => {
                        tiles.insert(coor, Tile::Open);
                        blizzards.push((coor, direction));
                    }
                }
                max_x = max_x.max(x);
            }
            max_y = max_y.max(y);
        }
        Ok(Self {
            tiles,
            blizzards,
            max_x,
            max_y,
        })
    }
}

fn map_step(cache: &mut HashMap<usize, Map>, step: usize) -> Map {
    if let Some(map) = cache.get(&step) {
        return map.clone();
    }
    let mut map = map_step(cache, step - 1);
    map.mv();
    cache.insert(step, map.clone());
    map
}

fn part1(input: &str) -> Result<usize> {
    let map: Map = input.parse()?;
    let mut map_cache = HashMap::new();
    map_cache.insert(0, map.clone());
    let mut seen = HashSet::new();
    let start: Coor = map
        .tiles
        .iter()
        .filter(|(c, t)| c.y == 0 && **t == Tile::Open)
        .map(|(c, _)| *c)
        .next()
        .unwrap();
    let end: Coor = map
        .tiles
        .iter()
        .filter(|(c, t)| c.y == map.max_y && **t == Tile::Open)
        .map(|(c, _)| *c)
        .next()
        .unwrap();
    let mut queue = VecDeque::from([(start, 0)]);
    while let Some(next) = queue.pop_front() {
        let (pos, step) = next;
        if pos == end {
            return Ok(step);
        }
        let next_map = map_step(&mut map_cache, step + 1);
        for direction in [
            Coor::new(0, 1),
            Coor::new(0, -1),
            Coor::new(1, 0),
            Coor::new(-1, 0),
        ] {
            let next_pos = pos + direction;
            if seen.contains(&(next_pos, step + 1)) {
                continue;
            }
            if *next_map.tiles.get(&next_pos).unwrap_or(&Tile::Wall) != Tile::Open
                || next_map.blizzards.iter().any(|(c, _)| *c == next_pos)
            {
                continue;
            }
            seen.insert((next_pos, step + 1));
            queue.push_back((next_pos, step + 1));
        }
        if seen.contains(&(pos, step + 1)) {
            continue;
        }
        if next_map.blizzards.iter().any(|(c, _)| *c == pos) {
            continue;
        }
        seen.insert((pos, step + 1));
        queue.push_back((pos, step + 1));
    }
    Ok(0)
}

fn part2(input: &str) -> Result<usize> {
    let map: Map = input.parse()?;
    let mut map_cache = HashMap::new();
    map_cache.insert(0, map.clone());
    let mut seen = HashSet::new();
    let start: Coor = map
        .tiles
        .iter()
        .filter(|(c, t)| c.y == 0 && **t == Tile::Open)
        .map(|(c, _)| *c)
        .next()
        .unwrap();
    let end: Coor = map
        .tiles
        .iter()
        .filter(|(c, t)| c.y == map.max_y && **t == Tile::Open)
        .map(|(c, _)| *c)
        .next()
        .unwrap();
    // let mut queue = VecDeque::from([(start, 0, vec![])]);
    let mut queue = VecDeque::from([(start, 0, 0)]);
    while let Some(next) = queue.pop_front() {
        let (pos, step, mut phase) = next;
        // let (pos, step, path) = next;
        // let mut path = path.clone();
        // path.push((pos, map_step(&mut map_cache, step)));
        // dbg!(step);
        // println!("{:?} ({})", step, phase);
        // map_step(&mut map_cache, step)._print(pos);
        if pos == end && phase == 0 {
            // queue.push_back((pos, step, phase + 1));
            queue.clear();
            phase += 1;
        }
        if pos == start && phase == 1 {
            queue.clear();
            phase += 1;
            // queue.push_back((pos, step, phase + 1));
        }
        if pos == end && phase == 2 {
            return Ok(step);
        }
        // if (pos == end && phase == 0 || phase == 2) || (pos == start && phase == 1) {
        //     dbg!(phase);
        //     if phase == 2 {
        //         return Ok(step);
        //     } else if phase == 1{
        //         queue.push_back((pos, step, phase + 1));
        //     }
        //     // for (step, (pos, map)) in path.into_iter().enumerate() {
        //     // println!("{:?}", step);
        //     // map._print(pos);
        //     // }
        //     return Ok(step);
        // }
        let next_map = map_step(&mut map_cache, step + 1);
        for direction in [
            Coor::new(0, 1),
            Coor::new(0, -1),
            Coor::new(1, 0),
            Coor::new(-1, 0),
        ] {
            let next_pos = pos + direction;
            if seen.contains(&(next_pos, step + 1)) {
                continue;
            }
            if *next_map.tiles.get(&next_pos).unwrap_or(&Tile::Wall) != Tile::Open
                || next_map.blizzards.iter().any(|(c, _)| *c == next_pos)
            {
                continue;
            }
            seen.insert((next_pos, step + 1));
            queue.push_back((next_pos, step + 1, phase));
            // queue.push_back((next_pos, step + 1, path.clone()));
        }
        if seen.contains(&(pos, step + 1)) {
            continue;
        }
        if next_map.blizzards.iter().any(|(c, _)| *c == pos) {
            continue;
        }
        seen.insert((pos, step + 1));
        queue.push_back((pos, step + 1, phase));
        // queue.push_back((pos, step + 1, path.clone()));
    }
    // for _ in 0..5 {
    //     map._print();
    //     map.mv();
    // }
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const _TEST_INPUT: &str = "#.#####
#.....#
#.>...#
#.....#
#.....#
#...v.#
#####.#";

    const TEST_INPUT: &str = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";

    #[test]
    fn test_wrap() -> Result<()> {
        let map: Map = TEST_INPUT.parse()?;
        assert_eq!(map.wrap(Coor::new(4, 4), Coor::new(0, 1)), Coor::new(4, 1));
        assert_eq!(map.wrap(Coor::new(4, 1), Coor::new(0, -1)), Coor::new(4, 4));
        assert_eq!(map.wrap(Coor::new(6, 3), Coor::new(1, 0)), Coor::new(1, 3));
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(TEST_INPUT)?, 18);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(TEST_INPUT)?, 54);
        Ok(())
    }
}
