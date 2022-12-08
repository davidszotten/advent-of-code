use anyhow::{Context, Error, Result};
use aoc2022::coor::Coor;
use aoc2022::dispatch;
use std::collections::HashMap;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

#[derive(Debug)]
struct Map {
    heights: HashMap<Coor, i32>,
    max: usize,
}

impl Map {
    fn visible(&self, coor: Coor) -> Result<bool> {
        assert!(coor.x > 0);
        assert!(coor.x < self.max as i64);
        assert!(coor.y > 0);
        assert!(coor.y < self.max as i64);

        let height = self.heights.get(&coor).context("unmapped coordinate")?;
        let mut visible = false;

        for direction in [
            Coor::new(-1, 0),
            Coor::new(1, 0),
            Coor::new(0, -1),
            Coor::new(0, 1),
        ] {
            visible |= (1..)
                .map(|n| coor + direction * n)
                .map_while(|c| self.heights.get(&c))
                .all(|h| h < height)
        }
        Ok(visible)
    }

    fn count(&self) -> Result<usize> {
        let mut sum = 0;
        for x in 1..self.max {
            for y in 1..self.max {
                if self.visible(Coor::new(x as i64, y as i64))? {
                    sum += 1;
                }
            }
        }
        Ok(sum + (self.max + 1) * (self.max + 1) - (self.max - 1) * (self.max - 1))
    }

    fn view_distance(&self, coor: Coor) -> Result<usize> {
        assert!(coor.x > 0);
        assert!(coor.x < self.max as i64);
        assert!(coor.y > 0);
        assert!(coor.y < self.max as i64);

        let height = self.heights.get(&coor).context("unmapped coordinate")?;
        let mut visible = 1;

        for direction in [
            Coor::new(0, -1),
            Coor::new(-1, 0),
            Coor::new(0, 1),
            Coor::new(1, 0),
        ] {
            let it = (1..)
                .map(|n| coor + direction * n)
                .map_while(|c| self.heights.get(&c));
            let mut dist = 0;
            for h in it {
                dist += 1;
                if h >= height {
                    break;
                }
            }
            visible *= dist;
        }
        Ok(visible)
    }

    fn best_view(&self) -> Result<usize> {
        let mut best = 0;
        for x in 1..self.max {
            for y in 1..self.max {
                let view = self.view_distance(Coor::new(x as i64, y as i64))?;
                best = best.max(view);
            }
        }
        Ok(best)
    }
}

impl std::str::FromStr for Map {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let mut heights = HashMap::new();
        let mut max = 0;
        for (y, line) in s.trim().lines().enumerate() {
            for (x, c) in line.chars().enumerate() {
                heights.insert(Coor::new(x as i64, y as i64), c as i32 - '0' as i32);
            }
            max = y;
        }
        Ok(Self { heights, max })
    }
}

fn part1(input: &str) -> Result<usize> {
    let map: Map = input.parse()?;
    map.count()
}

fn part2(input: &str) -> Result<usize> {
    let map: Map = input.parse()?;
    map.best_view()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "30373
25512
65332
33549
35390";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(TEST_INPUT)?, 21);
        Ok(())
    }
    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(TEST_INPUT)?, 8);
        Ok(())
    }
}
