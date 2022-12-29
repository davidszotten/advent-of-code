use anyhow::Result;
use aoc2022::coor::Coor;
use aoc2022::dispatch;
use std::collections::{HashMap, HashSet};

fn main() -> Result<()> {
    dispatch(part1, part2)
}

type Map = HashSet<Coor>;

struct Rock {
    coors: Vec<Coor>,
    left: i64,
    bottom: i64,
}

impl Rock {
    fn new(coors: Vec<Coor>) -> Self {
        let left = coors.iter().map(|c| c.x).min().unwrap();
        let bottom = coors.iter().map(|c| c.y).min().unwrap();
        Rock {
            coors,
            left,
            bottom,
        }
    }
}

fn make_rocks() -> Vec<Rock> {
    vec![
        Rock::new(vec![
            Coor::new(0, 0),
            Coor::new(1, 0),
            Coor::new(2, 0),
            Coor::new(3, 0),
        ]),
        Rock::new(vec![
            Coor::new(-1, 0),
            Coor::new(0, 0),
            Coor::new(1, 0),
            Coor::new(0, -1),
            Coor::new(0, 1),
        ]),
        Rock::new(vec![
            Coor::new(2, 2),
            Coor::new(2, 1),
            Coor::new(0, 0),
            Coor::new(1, 0),
            Coor::new(2, 0),
        ]),
        Rock::new(vec![
            Coor::new(0, 3),
            Coor::new(0, 2),
            Coor::new(0, 1),
            Coor::new(0, 0),
        ]),
        Rock::new(vec![
            Coor::new(0, 0),
            Coor::new(1, 0),
            Coor::new(0, 1),
            Coor::new(1, 1),
        ]),
    ]
}

fn make_map() -> Map {
    let mut map = HashSet::new();
    for x in 0..7 {
        map.insert(Coor::new(x, 0));
    }
    map
}

fn get_jet_offset(jet: char) -> i64 {
    match jet {
        '>' => 1,
        '<' => -1,
        _ => panic!("invalid jet"),
    }
}

fn _compare(map: &Map, length: i64) -> bool {
    let max = map.iter().map(|c| c.y).max().unwrap();
    let map1 = map
        .iter()
        .copied()
        .filter(|c| c.y > max - length)
        .collect::<Map>();
    let map2 = map
        .iter()
        .map(|&c| Coor::new(c.x, c.y + length))
        .filter(|c| c.y > max - length && c.y <= max)
        .collect::<Map>();

    map1 == map2
}

fn _print(map: &Map, take: i64) {
    let x_min = map.iter().map(|c| c.x).min().unwrap();
    let x_max = map.iter().map(|c| c.x).max().unwrap();
    let y_min = map.iter().map(|c| c.y).min().unwrap();
    let y_max = map.iter().map(|c| c.y).max().unwrap();
    let mut take = take;
    if take == 0 {
        take = y_max - y_min;
    }
    for y in (y_min..=y_max).rev().take(take as usize) {
        for x in x_min..=x_max {
            print!(
                "{}",
                if map.contains(&Coor::new(x, y)) {
                    '#'
                } else {
                    ' '
                }
            );
        }
        println!();
    }
    for _ in x_min..=x_max {
        print!("-");
    }
    println!();
    println!();
}

fn _print_rock(map: &Map, rock: &Rock, x_offset: i64, y_offset: i64) {
    let mut map = map.clone();
    for &coor in &rock.coors {
        map.insert(Coor::new(coor.x + x_offset, coor.y + y_offset));
    }
    _print(&map, 0);
}

fn find_row(map: &Map, pattern: &str, from_top: i64) -> bool {
    let x_min = map.iter().map(|c| c.x).min().unwrap();
    let x_max = map.iter().map(|c| c.x).max().unwrap();
    let y_max = map.iter().map(|c| c.y).max().unwrap();
    let y = y_max - from_top;
    (x_min..=x_max)
        .enumerate()
        .all(|(idx, x)| !(map.contains(&Coor::new(x, y)) ^ (&pattern[idx..idx + 1] == "#")))
}

fn _find(map: &Map, pattern: &str) -> bool {
    (0..3).any(|from_top| find_row(map, pattern, from_top))
}

fn is_sealed(map: &Map) -> bool {
    find_row(map, "      #", 0)
        && find_row(map, " #    #", 1)
        && find_row(map, "### ###", 2)
        && find_row(map, " ##### ", 3)
}

fn drop(input: &str, n: usize) -> Result<i64> {
    let rocks = make_rocks();
    let mut map = make_map();
    let mut jets = input.chars().enumerate().cycle();

    let mut count_on_prev_pattern_repeats: HashMap<_, i64> = HashMap::new();
    let mut height = 0;
    let mut count_heights = HashMap::new();
    let mut _printed = false;
    let mut jet_index;

    for (count, rock) in rocks.iter().cycle().take(n).enumerate() {
        let count = count + 1;
        let mut x_offset = 2 - rock.left;
        let mut y_offset = 3 - rock.bottom + height + 1;
        // _print_rock(&map, rock, x_offset, y_offset);
        loop {
            let (idx, jet) = jets.next().unwrap();
            jet_index = idx;

            let jet_offset = get_jet_offset(jet);
            let can_jet = rock.coors.iter().all(|coor| {
                !map.contains(&Coor::new(
                    coor.x + x_offset + jet_offset,
                    coor.y + y_offset,
                )) && coor.x + x_offset + jet_offset >= 0
                    && coor.x + x_offset + jet_offset < 7
            });

            if can_jet {
                x_offset += jet_offset;
            }

            let drop_offset = -1;
            let can_drop = rock.coors.iter().all(|coor| {
                !map.contains(&Coor::new(
                    coor.x + x_offset,
                    coor.y + y_offset + drop_offset,
                ))
            });

            if can_drop {
                y_offset += drop_offset;
            } else {
                for &coor in &rock.coors {
                    map.insert(Coor::new(coor.x + x_offset, coor.y + y_offset));
                    height = height.max(coor.y + y_offset);
                }
                break;
            }
        }
        count_heights.insert(count as i64, height);

        if is_sealed(&map) {
            map.retain(|c| c.y > height - 5);
            let seal_key = (count % 5, jet_index);
            if let Some(count_on_prev_pattern_repeat) = count_on_prev_pattern_repeats.get(&seal_key)
            {
                let _height_of_repeated_pattern =
                    count_heights.get(count_on_prev_pattern_repeat).unwrap();

                let cycle_drop_length = count as i64 - count_on_prev_pattern_repeat;
                let target = n as i64 - count_on_prev_pattern_repeat;
                let rounds = target / cycle_drop_length as i64;
                let remainder = target % cycle_drop_length as i64;
                let height_on_prev_pattern_repeat =
                    count_heights.get(count_on_prev_pattern_repeat).unwrap();
                let height_of_repeated_pattern = height - height_on_prev_pattern_repeat;
                let remainder_height = count_heights
                    .get(&(remainder + count_on_prev_pattern_repeat))
                    .unwrap();

                return Ok(remainder_height + rounds * height_of_repeated_pattern);
            }
            count_on_prev_pattern_repeats.insert(seal_key, count as i64);
        }
    }
    Ok(height)
}

fn part1(input: &str) -> Result<i64> {
    drop(input, 2022)
}

fn part2(input: &str) -> Result<i64> {
    drop(input, 1000000000000)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(TEST_INPUT)?, 3068);
        Ok(())
    }

    #[test]
    fn test_compare() {
        let mut map = Map::new();
        map.insert(Coor::new(1, 1));
        map.insert(Coor::new(0, 4));
        map.insert(Coor::new(0, 7));
        assert!(_compare(&map, 3));
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(TEST_INPUT)?, 1514285714288);
        Ok(())
    }
}
