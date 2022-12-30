use anyhow::{bail, Error, Result};
use aoc2022::coor::Coor;
use aoc2022::coor3::Coor3;
use aoc2022::dispatch;
use std::collections::HashMap;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Blank,
    Open,
    Wall,
}

impl TryFrom<char> for Tile {
    type Error = Error;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        Ok(match c {
            ' ' => Tile::Blank,
            '.' => Tile::Open,
            '#' => Tile::Wall,
            _ => bail!("invalid tile"),
        })
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Local {
    u: Coor3,
    v: Coor3,
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
struct Side(i64, i64);

impl Local {
    fn new(u: Coor3, v: Coor3) -> Self {
        Self { u, v }
    }

    fn axis(&self) -> Coor3 {
        self.u.cross(self.v)
    }

    // ↑
    // v
    // u →
    //
    //    y
    //    ↑
    //  ↙︎  → x
    // z
    // (u, v)

    fn left(&self) -> Self {
        Self::new(-(-self.u).cross(self.v), self.v)
    }

    fn right(&self) -> Self {
        Self::new(-self.u.cross(self.v), self.v)
    }

    fn down(&self) -> Self {
        Self::new(self.u, (-self.v).cross(self.u))
    }

    fn up(&self) -> Self {
        Self::new(self.u, self.v.cross(self.u))
    }

    fn mv(&self, direction: Coor) -> Self {
        match (direction.x, direction.y) {
            (1, 0) => self.right(),
            (-1, 0) => self.left(),
            (0, 1) => self.up(),
            (0, -1) => self.down(),
            _ => panic!("invalid mv direction"),
        }
    }

    fn v_to_global(&self, direction: Coor) -> Coor3 {
        direction.x * self.u + direction.y * self.v
    }

    fn v_to_local(&self, direction: Coor3) -> Coor {
        Coor::new(direction.dot(self.u), direction.dot(self.v))
    }
}

#[derive(Debug)]
struct Map {
    tiles: HashMap<Coor, Tile>,
    min_x: i64,
    max_x: i64,
    min_y: i64,
    max_y: i64,
}

impl Map {
    fn side_coors(&self) -> (HashMap<Side, Local>, HashMap<Coor3, Side>) {
        let start = self.start();

        let start_side = self.coor_to_side(start);
        let mut side_coors: HashMap<Side, Local> = HashMap::from([(
            start_side,
            Local::new(Coor3::new(1, 0, 0), Coor3::new(0, 1, 0)),
        )]);
        let right = |s: Side| Side(s.0 + 1, s.1);
        let left = |s: Side| Side(s.0 - 1, s.1);
        let up = |s: Side| Side(s.0, s.1 + 1);
        for _ in 0..16 {
            for side_y in 0..4 {
                for side_x in 0..4 {
                    let side = Side(side_x, -side_y);
                    if side_coors.contains_key(&side) {
                        continue;
                    }

                    match *self
                        .tiles
                        .get(&self.side_top_left(side))
                        .unwrap_or(&Tile::Blank)
                    {
                        Tile::Blank => continue,
                        Tile::Open | Tile::Wall => {}
                    }
                    if let Some(prev) = side_coors.get(&(right(side))) {
                        side_coors.insert(side, prev.left());
                    }
                    if let Some(prev) = side_coors.get(&(left(side))) {
                        side_coors.insert(side, prev.right());
                    }
                    if let Some(prev) = side_coors.get(&(up(side))) {
                        side_coors.insert(side, prev.down());
                    }
                }
            }
        }
        let coor_sides: HashMap<Coor3, Side> = side_coors
            .iter()
            .map(|(side, local)| (local.axis(), *side))
            .collect();
        (side_coors, coor_sides)
    }

    fn start(&self) -> Coor {
        *self
            .tiles
            .iter()
            .filter(|&(c, t)| *t == Tile::Open && c.y == 0)
            .min_by_key(|&(c, _)| c.x)
            .unwrap()
            .0
    }

    fn wrap(&self, coor: Coor) -> Coor {
        let mut x = coor.x;
        let mut y = coor.y;
        let x_size = self.max_x - self.min_x;
        let y_size = self.max_y - self.min_y;
        while x < self.min_x {
            x += x_size;
        }
        while x >= self.max_x {
            x -= x_size;
        }
        while y <= self.min_y {
            y += y_size;
        }
        while y > self.max_y {
            y -= y_size;
        }
        Coor::new(x, y)
    }

    fn wrap_cube(&self, coor: Coor, direction: Coor) -> (Coor, Coor) {
        let side = self.coor_to_side(coor);
        let (side_coors, coor_sides) = self.side_coors();
        let side_local = side_coors.get(&side).unwrap();
        let next = side_local.mv(direction);
        let destination_side = *coor_sides.get(&next.axis()).unwrap();
        let destination_local = side_coors.get(&destination_side).unwrap();
        let destination_direction = destination_local.v_to_local(next.v_to_global(direction));
        let global = self.c_to_global(coor);
        let top_left = self.side_top_left(destination_side);

        let find = |target: Coor3| -> Coor {
            for x in [
                0,
                self.side_size() - 1,
                target.x,
                self.side_size() - 1 - target.x,
                target.y,
                self.side_size() - 1 - target.y,
                target.z,
                self.side_size() - 1 - target.z,
            ] {
                for y in [
                    0,
                    self.side_size() - 1,
                    target.x,
                    self.side_size() - 1 - target.x,
                    target.y,
                    self.side_size() - 1 - target.y,
                    target.z,
                    self.side_size() - 1 - target.z,
                ] {
                    let test = top_left + Coor::new(x, -y);
                    if self.c_to_global(test) == target {
                        return test;
                    }
                }
            }
            panic!("not found");
        };
        let destination_local = find(global);

        (destination_local, destination_direction)
    }

    fn find_opposite(&self, pos: Coor, dir: Coor) -> Coor {
        let mut next = self.wrap(pos + dir);
        while *self.tiles.get(&next).unwrap_or(&Tile::Blank) == Tile::Blank {
            next = self.wrap(next + dir);
        }
        next
    }

    fn c_to_global(&self, global_coor: Coor) -> Coor3 {
        let coor = self.global_to_local(Coor::new(global_coor.x, -global_coor.y));
        let side = self.coor_to_side(global_coor);
        let (side_coors, _coor_sides) = self.side_coors();
        let side_local = side_coors.get(&side).unwrap();
        let axis = side_local.axis();

        let side_size = self.side_size();
        let _mul = |n: i64, vec: Coor3| {
            // n * vec
            if vec.x + vec.y + vec.z < 0 {
                (side_size - 1 - n) * -vec
            } else {
                n * vec
            }
        };
        _mul(coor.x, side_local.u) + _mul(coor.y, side_local.v) + _mul(side_size - 1, axis)
    }

    fn side_size(&self) -> i64 {
        let x_size = self.max_x - self.min_x;
        let y_size = self.max_y - self.min_y;
        (x_size.max(y_size) + 1) / 4
    }

    fn coor_to_side(&self, coor: Coor) -> Side {
        let side = coor / self.side_size();
        Side(side.x, side.y)
    }

    fn global_to_local(&self, coor: Coor) -> Coor {
        let side_size = self.side_size();
        Coor::new(coor.x % side_size, side_size - 1 - (coor.y % side_size))
    }

    fn side_top_left(&self, side: Side) -> Coor {
        let x_size = self.max_x - self.min_x;
        let y_size = self.max_y - self.min_y;
        let side_size = (x_size.max(y_size) + 1) / 4;
        Coor::new(side.0, side.1) * side_size
    }

    fn _find_edge(&self, pos: Coor, dir: Coor) -> Coor {
        let mut pos = pos;
        while *self.tiles.get(&(pos + dir)).unwrap_or(&Tile::Blank) == Tile::Blank {
            pos += dir;
        }
        pos
    }
}

impl std::str::FromStr for Map {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let mut tiles = HashMap::new();
        let mut min_x = i64::MAX;
        let mut max_x = i64::MIN;
        let mut min_y = i64::MAX;
        let mut max_y = i64::MIN;
        for (y, line) in s.lines().enumerate() {
            let y = -(y as i64);
            for (x, c) in line.chars().enumerate() {
                let x = x as i64;
                tiles.insert(Coor::new(x, y), c.try_into()?);
                min_x = min_x.min(x);
                max_x = max_x.max(x);
            }
            min_y = min_y.min(y);
            max_y = max_y.max(y);
        }
        Ok(Self {
            tiles,
            min_x,
            max_x,
            max_y,
            min_y,
        })
    }
}

fn parse_moves(input: &str) -> (Vec<i64>, Vec<bool>) {
    let moves: Vec<_> = input
        .split(|c: char| !c.is_numeric())
        .filter(|&s| !s.is_empty())
        .map(|s| s.parse::<i64>().unwrap())
        .collect::<Vec<_>>();
    let turns: Vec<_> = input
        .split(|c: char| c.is_numeric())
        .filter(|&s| !s.is_empty())
        .map(|s| match s {
            "R" => true,
            "L" => false,
            _ => unreachable!("invalid turn"),
        })
        .collect::<Vec<_>>();
    (moves, turns)
}

fn part1(input: &str) -> Result<i64> {
    let (raw_map, raw_moves) = input.split_once("\n\n").unwrap();
    let map: Map = raw_map.parse()?;
    let (moves, turns) = parse_moves(raw_moves);

    let mut pos = map.start();

    let mut dir = Coor::new(1, 0);
    for (&mv, &turn) in moves.iter().zip(turns.iter()) {
        for _ in 0..mv {
            let next = pos + dir;
            match *map.tiles.get(&next).unwrap_or(&Tile::Blank) {
                Tile::Open => pos = next,
                Tile::Wall => break,
                Tile::Blank => {
                    let next = map.find_opposite(pos, dir);
                    if *map.tiles.get(&next).unwrap_or(&Tile::Blank) == Tile::Open {
                        pos = next;
                    }
                }
            }
        }
        if turn {
            // R
            // [0  -1]
            // [1 0]
            dir = Coor::new(dir.y, -dir.x);
        } else {
            // L
            // [0 1]
            // [-1  0]
            dir = Coor::new(-dir.y, dir.x);
        }
    }

    for _ in 0..moves[moves.len() - 1] {
        let next = pos + dir;
        match *map.tiles.get(&next).unwrap_or(&Tile::Blank) {
            Tile::Open => pos = next,
            Tile::Wall => break,
            Tile::Blank => {
                let next = map.find_opposite(pos, dir);
                if *map.tiles.get(&next).unwrap_or(&Tile::Blank) == Tile::Open {
                    pos = next;
                }
            }
        }
    }
    let dir_score = match (dir.x, dir.y) {
        (1, 0) => 0,
        (0, -1) => 1,
        (-1, 0) => 2,
        (0, 1) => 3,
        _ => unreachable!(),
    };
    Ok((-pos.y + 1) * 1000 + (pos.x + 1) * 4 + dir_score)
}

fn part2(input: &str) -> Result<i64> {
    let (raw_map, raw_moves) = input.split_once("\n\n").unwrap();
    let map: Map = raw_map.parse()?;
    let (moves, turns) = parse_moves(raw_moves);

    let mut pos = map.start();

    let mut dir = Coor::new(1, 0);
    for (&mv, &turn) in moves.iter().zip(turns.iter()) {
        for _ in 0..mv {
            let next = pos + dir;
            match *map.tiles.get(&next).unwrap_or(&Tile::Blank) {
                Tile::Open => pos = next,
                Tile::Wall => break,
                Tile::Blank => {
                    let (next, next_dir) = map.wrap_cube(pos, dir);
                    if *map.tiles.get(&next).unwrap_or(&Tile::Blank) == Tile::Open {
                        pos = next;
                        dir = next_dir;
                    }
                }
            }
        }
        if turn {
            // R
            // [0  -1]
            // [1 0]
            dir = Coor::new(dir.y, -dir.x);
            // dir =
            // directions[(directions.iter().position(|d| d == &dir).unwrap() + 1).rem_euclid(4)];
        } else {
            // L
            // [0 1]
            // [-1  0]
            dir = Coor::new(-dir.y, dir.x);
            // dir =
            // directions[(directions.iter().position(|d| d == &dir).unwrap() + 3).rem_euclid(4)];
        }
    }

    for _ in 0..moves[moves.len() - 1] {
        let next = pos + dir;
        match *map.tiles.get(&next).unwrap_or(&Tile::Blank) {
            Tile::Open => pos = next,
            Tile::Wall => break,
            Tile::Blank => {
                let next = map.find_opposite(pos, dir);
                if *map.tiles.get(&next).unwrap_or(&Tile::Blank) == Tile::Open {
                    pos = next;
                }
            }
        }
    }
    let dir_score = match (dir.x, dir.y) {
        (1, 0) => 0,
        (0, -1) => 1,
        (-1, 0) => 2,
        (0, 1) => 3,
        _ => unreachable!(),
    };
    Ok((-pos.y + 1) * 1000 + (pos.x + 1) * 4 + dir_score)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

    #[test]
    fn test_parse_moves() {
        assert_eq!(parse_moves("10R5L3"), (vec![10, 5, 3], vec![true, false]));
    }

    // (map.find_opposite(Coor::new(5, 4), Coor::new(0, -1)));
    // (map.find_opposite(Coor::new(0, 6), Coor::new(-1, 0)));

    #[test]
    fn test_axes() {
        let x = Coor3::new(1, 0, 0);
        let y = Coor3::new(0, 1, 0);
        let z = Coor3::new(0, 0, 1);
        let front = Local { u: x, v: y };
        let right = Local { u: -z, v: y };
        let left = Local { u: z, v: y };
        let down = Local { u: x, v: z };
        let up = Local { u: x, v: -z };
        assert_eq!(front.right(), right);
        assert_eq!(front.left(), left);
        assert_eq!(front.down(), down);
        assert_eq!(front.up(), up);
    }

    #[test]
    fn test_wrap_cube() -> Result<()> {
        let (raw_map, _raw_moves) = TEST_INPUT.split_once("\n\n").unwrap();
        let map: Map = raw_map.parse()?;

        assert_eq!(
            map.wrap_cube(Coor::new(11, -5), Coor::new(1, 0)),
            (Coor::new(14, -8), Coor::new(0, -1))
        );
        assert_eq!(
            map.wrap_cube(Coor::new(10, -11), Coor::new(0, -1)),
            (Coor::new(1, -7), Coor::new(0, 1))
        );
        assert_eq!(
            map.wrap_cube(Coor::new(6, -4), Coor::new(0, 1)),
            (Coor::new(8, -2), Coor::new(1, 0))
        );
        Ok(())
    }

    #[test]
    fn test_c_to_global() -> Result<()> {
        let (raw_map, _raw_moves) = TEST_INPUT.split_once("\n\n").unwrap();
        let map: Map = raw_map.parse()?;

        assert_eq!(map.c_to_global(Coor::new(8, 0)), Coor3::new(0, 3, 3));
        assert_eq!(map.c_to_global(Coor::new(11, -3)), Coor3::new(3, 0, 3));
        assert_eq!(map.c_to_global(Coor::new(11, -4)), Coor3::new(3, 0, 3));
        assert_eq!(map.c_to_global(Coor::new(11, -5)), Coor3::new(3, 0, 2));
        assert_eq!(map.c_to_global(Coor::new(14, -8)), Coor3::new(3, 0, 2));
        assert_eq!(map.c_to_global(Coor::new(10, -11)), Coor3::new(2, 3, 0));
        assert_eq!(map.c_to_global(Coor::new(4, -5)), Coor3::new(0, 3, 2));
        assert_eq!(map.c_to_global(Coor::new(3, -5)), Coor3::new(0, 3, 2));
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(TEST_INPUT)?, 6032);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(TEST_INPUT)?, 5031);
        Ok(())
    }
}
