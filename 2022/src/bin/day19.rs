use anyhow::{Context, Error, Result};
use aoc2022::dispatch;
use std::collections::{HashSet, VecDeque};
use std::ops::{Add, Mul, Sub};

fn main() -> Result<()> {
    dispatch(part1, part2)
}

#[derive(Debug, Default, Clone, Copy, Hash, Eq, PartialEq)]
struct MaterialSet {
    ore: i32,
    clay: i32,
    obsidian: i32,
    geode: i32,
}

impl MaterialSet {
    fn max(&self, other: &MaterialSet) -> MaterialSet {
        MaterialSet {
            ore: self.ore.max(other.ore),
            clay: self.clay.max(other.clay),
            obsidian: self.obsidian.max(other.obsidian),
            geode: self.geode.max(other.geode),
        }
    }
}

impl Add for MaterialSet {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        MaterialSet {
            ore: self.ore + other.ore,
            clay: self.clay + other.clay,
            obsidian: self.obsidian + other.obsidian,
            geode: self.geode + other.geode,
        }
    }
}

impl Sub for MaterialSet {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        MaterialSet {
            ore: self.ore - other.ore,
            clay: self.clay - other.clay,
            obsidian: self.obsidian - other.obsidian,
            geode: self.geode - other.geode,
        }
    }
}

impl Mul<MaterialSet> for i32 {
    type Output = MaterialSet;
    fn mul(self, other: MaterialSet) -> MaterialSet {
        MaterialSet {
            ore: other.ore * self,
            clay: other.clay * self,
            obsidian: other.obsidian * self,
            geode: other.geode * self,
        }
    }
}

#[derive(Debug, Clone)]
struct Blueprint {
    index: i32,
    ore_robot: MaterialSet,
    clay_robot: MaterialSet,
    obsidian_robot: MaterialSet,
    geode_robot: MaterialSet,
}

impl std::str::FromStr for Blueprint {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        let parts: Vec<_> = s
            .split(|c: char| !c.is_numeric())
            .filter(|&s| !s.is_empty())
            .collect::<Vec<_>>();

        let parts = parts
            .iter()
            .map(|n| n.parse::<i32>().context("failed to parse number"))
            .collect::<Result<Vec<i32>>>()?;
        let [index, ore_ore, clay_ore, obsidian_ore, obsidian_clay, geode_ore, geode_obsidian]: [i32; 7] =
        parts.try_into().unwrap();
        Ok(Blueprint {
            index,
            ore_robot: MaterialSet {
                ore: ore_ore,
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
            clay_robot: MaterialSet {
                ore: clay_ore,
                clay: 0,
                obsidian: 0,
                geode: 0,
            },
            obsidian_robot: MaterialSet {
                ore: obsidian_ore,
                clay: obsidian_clay,
                obsidian: 0,
                geode: 0,
            },
            geode_robot: MaterialSet {
                ore: geode_ore,
                clay: 0,
                obsidian: geode_obsidian,
                geode: 0,
            },
        })
    }
}

fn can_afford(recipe: MaterialSet, available: MaterialSet) -> bool {
    recipe.ore <= available.ore
        && recipe.clay <= available.clay
        && recipe.obsidian <= available.obsidian
        && recipe.geode <= available.geode
}

fn total_max_cost(blueprint: &Blueprint, total_minutes: i32, minutes_left: i32) -> MaterialSet {
    ((total_minutes - minutes_left) * blueprint.ore_robot)
        .max(&((total_minutes - minutes_left) * blueprint.clay_robot))
        .max(&((total_minutes - minutes_left) * blueprint.obsidian_robot))
        .max(&((total_minutes - minutes_left) * blueprint.geode_robot))
}

fn max_minute_spend_given(blueprint: &Blueprint, robots: MaterialSet) -> MaterialSet {
    let max_minute_spend = blueprint
        .ore_robot
        .max(&blueprint.clay_robot)
        .max(&blueprint.obsidian_robot)
        .max(&blueprint.geode_robot);
    if robots.ore >= max_minute_spend.ore {
        return blueprint
            .clay_robot
            .max(&blueprint.obsidian_robot)
            .max(&blueprint.geode_robot);
    }
    if robots.clay >= max_minute_spend.clay {
        return blueprint
            .ore_robot
            .max(&blueprint.obsidian_robot)
            .max(&blueprint.geode_robot);
    }
    max_minute_spend
}

fn test_blueprint(blueprint: Blueprint, minutes: i32) -> i32 {
    // ore, clay, obsidian, geode
    let available = MaterialSet::default();
    let robots = MaterialSet {
        ore: 1,
        ..Default::default()
    };
    let mut queue = VecDeque::from([(0, available, robots)]);
    let mut best = 0;
    let mut seen = HashSet::new();
    let mut furthest = 0;

    let max_minute_spend = blueprint
        .ore_robot
        .max(&blueprint.clay_robot)
        .max(&blueprint.obsidian_robot)
        .max(&blueprint.geode_robot);
    // dbg!(max_minute_spend);

    while let Some(next) = queue.pop_front() {
        let (elapsed, available, robots) = next;
        if seen.contains(&(available, robots)) {
            continue;
        }
        seen.insert((available, robots));
        if elapsed > furthest {
            furthest = elapsed;
            // dbg!(elapsed);
        }
        if elapsed == minutes {
            let prev = best;
            best = best.max(available.geode);
            if best == available.geode && best != prev {
                // dbg!(elapsed, best);
            }
            continue;
        }
        let total_max = total_max_cost(&blueprint, minutes, elapsed);
        if can_afford(blueprint.geode_robot, available) {
            let mut new_robots = robots;
            new_robots.geode += 1;
            queue.push_back((
                elapsed + 1,
                available + robots - blueprint.geode_robot,
                new_robots,
            ));
        }
        if available.obsidian <= total_max.obsidian
            && robots.obsidian < max_minute_spend.obsidian
            && can_afford(blueprint.obsidian_robot, available)
        {
            let mut new_robots = robots;
            new_robots.obsidian += 1;
            queue.push_back((
                elapsed + 1,
                available + robots - blueprint.obsidian_robot,
                new_robots,
            ));
        }
        if available.clay <= total_max.clay
            && robots.clay < max_minute_spend.clay
            && robots.clay < max_minute_spend_given(&blueprint, robots).clay
            && can_afford(blueprint.clay_robot, available)
        {
            let mut new_robots = robots;
            new_robots.clay += 1;
            queue.push_back((
                elapsed + 1,
                available + robots - blueprint.clay_robot,
                new_robots,
            ));
        }
        if available.ore <= total_max.ore
            && robots.ore < max_minute_spend.ore
            && can_afford(blueprint.ore_robot, available)
        {
            let mut new_robots = robots;
            new_robots.ore += 1;
            queue.push_back((
                elapsed + 1,
                available + robots - blueprint.ore_robot,
                new_robots,
            ));
        }
        if !(can_afford(blueprint.ore_robot, available)
            && can_afford(blueprint.clay_robot, available)
            && can_afford(blueprint.obsidian_robot, available)
            && can_afford(blueprint.geode_robot, available))
        {
            queue.push_back((elapsed + 1, available + robots, robots));
        }
    }
    // dbg!(best);
    best
}

fn part1(input: &str) -> Result<i32> {
    let blueprints = input
        .split('\n')
        .map(|l| l.parse::<Blueprint>())
        .collect::<Result<Vec<_>>>()?;
    Ok(blueprints
        .into_iter()
        .map(|b| b.index * test_blueprint(b, 24))
        .sum())
}

fn part2(input: &str) -> Result<i32> {
    let blueprints = input
        .split('\n')
        .map(|l| l.parse::<Blueprint>())
        .take(3)
        .collect::<Result<Vec<_>>>()?;
    Ok(blueprints
        .into_iter()
        .map(|b| test_blueprint(b, 32))
        .product())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(TEST_INPUT)?, 33);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(TEST_INPUT)?, 56 * 62);
        Ok(())
    }
}
