use anyhow::{Context, Error, Result};
use aoc2022::coor::Coor;
use aoc2022::dispatch;
use std::collections::HashMap;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

struct Reading {
    sensor: Coor,
    beacon: Coor,
}

impl std::str::FromStr for Reading {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let rest = s.strip_prefix("Sensor at ").context("start missing")?;
        let (raw_sensor, raw_beacon) = rest
            .split_once(": closest beacon is at ")
            .context("middle missing")?;
        Ok(Reading {
            sensor: raw_sensor
                .replace("x=", "")
                .replace(" y=", "")
                .parse()
                .context(format!("sensor? `{}`", raw_sensor))?,
            beacon: raw_beacon.replace("x=", "").replace(" y=", "").parse()?,
        })
    }
}

fn parse(s: &str) -> Result<Vec<Reading>> {
    s.split('\n').map(|l| l.parse::<Reading>()).collect()
}

fn manhattan(&c1: &Coor, &c2: &Coor) -> i64 {
    let delta = c2 - c1;
    delta.x.abs() + delta.y.abs()
}

fn find_segment(reading: &Reading, row: i64) -> Option<(i64, i64)> {
    let distance = manhattan(&reading.sensor, &reading.beacon);
    let y_distance = (reading.sensor.y - row).abs();
    if distance < y_distance {
        return None;
    }
    let segment_offset = distance - y_distance;
    Some((
        reading.sensor.x - segment_offset,
        reading.sensor.x + segment_offset,
    ))
}

fn count_row(readings: Vec<Reading>, row: i64) -> i64 {
    let mut segments = vec![];
    for reading in readings {
        if let Some(segment) = find_segment(&reading, row) {
            segments.push(segment);
        }
    }
    let mut points: HashMap<i64, Vec<i64>> = HashMap::new();
    for (start, end) in segments {
        points.entry(start).or_default().push(1);
        points.entry(end).or_default().push(-1);
    }
    let mut sorted: Vec<_> = points.keys().collect();
    sorted.sort();
    let mut total = 0;
    let mut prev = sorted[0];
    let mut inside = 1;
    for &current in &sorted[1..] {
        if inside > 0 {
            total += current - prev;
        }
        inside += points.get(current).unwrap_or(&vec![]).iter().sum::<i64>();
        prev = current;
    }
    total
}

fn find_hole(readings: &Vec<Reading>, row: i64) -> Option<i64> {
    let mut segments = vec![];
    for reading in readings {
        if let Some(segment) = find_segment(reading, row) {
            segments.push(segment);
        }
    }
    let mut points: HashMap<i64, Vec<i64>> = HashMap::new();
    for (start, end) in segments {
        points.entry(start).or_default().push(1);
        points.entry(end).or_default().push(-1);
    }
    let mut sorted: Vec<_> = points.keys().collect();
    sorted.sort();

    let get = |current| points.get(current).unwrap_or(&vec![]).iter().sum::<i64>();

    let mut prev = sorted[0];
    let mut inside = get(prev);
    for &current in &sorted[1..] {
        if inside <= 0 {
            return Some(prev + 1);
        }
        inside += get(current);
        prev = current;
    }
    None
}

fn find_signal(readings: Vec<Reading>, max: i64) -> Coor {
    for row in 0..max {
        if let Some(hole) = find_hole(&readings, row) {
            return Coor::new(hole, row);
        }
    }
    unreachable!();
}

fn part1(input: &str) -> Result<i64> {
    let readings = parse(input)?;
    Ok(count_row(readings, 2_000_000))
}

fn part2(input: &str) -> Result<i64> {
    let readings = parse(input)?;
    let signal = find_signal(readings, 4_000_000);
    Ok(signal.x * 4_000_000 + signal.y)
}

#[cfg(test)]
mod tests {
    use super::*;
    // 7, 1, 3, 4, 4, 5, 9
    const TEST_INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn test_find_segment() -> Result<()> {
        let readings = parse("Sensor at x=8, y=7: closest beacon is at x=2, y=10")?;
        assert_eq!(find_segment(&readings[0], 10), Some((2, 14)));
        assert_eq!(find_segment(&readings[0], 16), Some((8, 8)));
        assert_eq!(find_segment(&readings[0], -2), Some((8, 8)));
        assert_eq!(find_segment(&readings[0], -1), Some((7, 9)));
        Ok(())
    }

    #[test]
    fn test_find_signal() -> Result<()> {
        let readings = parse(TEST_INPUT)?;
        assert_eq!(find_signal(readings, 20), Coor::new(14, 11));
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<()> {
        let readings = parse(TEST_INPUT)?;
        assert_eq!(count_row(readings, 10), 26);
        Ok(())
    }
}
