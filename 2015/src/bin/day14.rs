use anyhow::{Context, Error, Result};
use aoc2015::dispatch;
use regex::Regex;
use std::convert::TryFrom;

struct Deer {
    speed: u64,
    fly: u64,
    rest: u64,
}

impl TryFrom<&str> for Deer {
    type Error = Error;
    fn try_from(s: &str) -> Result<Deer> {
        // Rudolph can fly 22 km/s for 8 seconds, but then must rest for 165 seconds.
        let re = Regex::new(r"(?<name>\w+) can fly (?<speed>\d+) km/s for (?<fly>\d+) seconds, but then must rest for (?<rest>\d+) seconds.").unwrap();
        let caps = re.captures(s).context(format!("no regex match: {}", s))?;
        Ok(Deer {
            speed: caps["speed"].parse()?,
            fly: caps["fly"].parse()?,
            rest: caps["rest"].parse()?,
        })
    }
}

fn run(deer: &Deer, time: u64) -> u64 {
    let mut time = time;
    let mut distance = 0;
    let mut next = deer.fly;
    let mut fly = true;
    while time >= next {
        time -= next;
        if fly {
            distance += deer.speed * next;
            next = deer.rest;
        } else {
            next = deer.fly
        }
        fly = !fly;
    }
    if fly {
        distance += deer.speed * time;
    }
    distance
}

fn run_points(deer: &[Deer], end: u64) -> u64 {
    let mut points: Vec<_> = deer.iter().map(|_| 0).collect();
    for time in 1..=end {
        let distances: Vec<_> = deer.iter().map(|d| run(d, time)).collect();
        let max_time_distance = distances.iter().max().expect("no max");
        for idx in 0..deer.len() {
            if distances[idx] == *max_time_distance {
                points[idx] += 1;
            }
        }
    }
    points.iter().max().copied().expect("no end max")
}

fn main() -> Result<()> {
    dispatch(part1, part2)
}

fn part1(input: &str) -> Result<u64> {
    let deer: Vec<_> = input
        .split('\n')
        .map(Deer::try_from)
        .collect::<Result<_>>()?;
    deer.iter().map(|d| run(d, 2503)).max().context("no max")
}

fn part2(input: &str) -> Result<u64> {
    let deer: Vec<_> = input
        .split('\n')
        .map(Deer::try_from)
        .collect::<Result<_>>()?;
    Ok(run_points(&deer, 2503))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Rudolph can fly 22 km/s for 8 seconds, but then must rest for 165 seconds.
Dancer can fly 7 km/s for 20 seconds, but then must rest for 119 seconds.";

    #[test]
    fn test_run() {
        let comet = Deer {
            speed: 14,
            fly: 10,
            rest: 127,
        };
        assert_eq!(run(&comet, 1000), 1120);
        let dancer = Deer {
            speed: 16,
            fly: 11,
            rest: 162,
        };
        assert_eq!(run(&dancer, 1000), 1056);
    }

    #[test]
    fn test_run_points() {
        let comet = Deer {
            speed: 14,
            fly: 10,
            rest: 127,
        };
        let dancer = Deer {
            speed: 16,
            fly: 11,
            rest: 162,
        };
        assert_eq!(run_points(&[comet, dancer], 1000), 689);
    }

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(INPUT)?, 0);
        Ok(())
    }
}
