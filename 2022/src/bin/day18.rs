use anyhow::Result;
use aoc2022::coor3::Coor3;
use aoc2022::dispatch;
use std::collections::{HashMap, HashSet, VecDeque};

fn main() -> Result<()> {
    dispatch(part1, part2)
}

const DIRECTIONS: [Coor3; 6] = [
    Coor3::new(1, 0, 0),
    Coor3::new(-1, 0, 0),
    Coor3::new(0, 1, 0),
    Coor3::new(0, -1, 0),
    Coor3::new(0, 0, 1),
    Coor3::new(0, 0, -1),
];

fn parse(input: &str) -> Result<HashSet<Coor3>> {
    input
        .split('\n')
        .map(|l| l.parse::<Coor3>())
        .collect::<Result<HashSet<_>>>()
}

fn part1(input: &str) -> Result<i32> {
    let coors = parse(input)?;
    let mut sum = 0;
    for coor in &coors {
        for offset in DIRECTIONS {
            if !coors.contains(&(*coor + offset)) {
                sum += 1;
            }
        }
    }
    Ok(sum)
}

fn bounds(coors: &HashSet<Coor3>) -> ((i64, i64), (i64, i64), (i64, i64)) {
    let x_min = coors.iter().map(|c| c.x).min().unwrap();
    let x_max = coors.iter().map(|c| c.x).max().unwrap();
    let y_min = coors.iter().map(|c| c.y).min().unwrap();
    let y_max = coors.iter().map(|c| c.y).max().unwrap();
    let z_min = coors.iter().map(|c| c.z).min().unwrap();
    let z_max = coors.iter().map(|c| c.z).max().unwrap();
    ((x_min, x_max), (y_min, y_max), (z_min, z_max))
}

fn can_reach_outside(coors: &HashSet<Coor3>, coor: &Coor3, known: &HashMap<Coor3, bool>) -> bool {
    let ((x_min, x_max), (y_min, y_max), (z_min, z_max)) = bounds(coors);
    let mut seen = HashSet::new();
    let mut queue = VecDeque::from([*coor]);
    while let Some(coor) = queue.pop_front() {
        if let Some(val) = known.get(&coor) {
            return *val;
        }
        if coor.x < x_min
            || coor.x > x_max
            || coor.y < y_min
            || coor.y > y_max
            || coor.z < z_min
            || coor.z > z_max
        {
            return true;
        }
        for offset in DIRECTIONS {
            let next = coor + offset;
            if coors.contains(&next) {
                continue;
            };
            if seen.contains(&next) {
                continue;
            }
            seen.insert(next);
            queue.push_back(next);
        }
    }
    false
}

fn part2(input: &str) -> Result<i32> {
    let coors = parse(input)?;
    let x_min = coors.iter().map(|c| c.x).min().unwrap();
    let x_max = coors.iter().map(|c| c.x).max().unwrap();
    let y_min = coors.iter().map(|c| c.y).min().unwrap();
    let y_max = coors.iter().map(|c| c.y).max().unwrap();
    let z_min = coors.iter().map(|c| c.z).min().unwrap();
    let z_max = coors.iter().map(|c| c.z).max().unwrap();

    let mut empty = HashSet::new();
    for x in x_min..=x_max {
        for y in y_min..=y_max {
            for z in z_min..=z_max {
                let coor = Coor3::new(x, y, z);
                if !coors.contains(&coor) {
                    empty.insert(coor);
                }
            }
        }
    }
    let mut inside = HashSet::new();
    let mut known = HashMap::new();
    for e in empty {
        if can_reach_outside(&coors, &e, &known) {
            known.insert(e, true);
        } else {
            known.insert(e, false);
            inside.insert(e);
        }
    }

    let mut sum = 0;
    for coor in &coors {
        for offset in DIRECTIONS {
            let c = *coor + offset;
            if !coors.contains(&c) && !inside.contains(&c) {
                sum += 1;
            }
        }
    }
    Ok(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(TEST_INPUT)?, 64);
        Ok(())
    }

    #[test]
    fn test_can_reach() -> Result<()> {
        let coors = parse(TEST_INPUT)?;
        assert!(!can_reach_outside(
            &coors,
            &Coor3::new(2, 2, 5),
            &HashMap::new()
        ));
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(TEST_INPUT)?, 58);
        Ok(())
    }
}
