use std::convert::TryFrom;

use anyhow::{Context, Error, Result};
use aoc2015::dispatch;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

#[derive(Debug, Copy, Clone)]
struct Player {
    hit_points: i32,
    damage: i32,
    armour: i32,
}

impl Player {
    fn equip(&mut self, eq: &Equipment) {
        self.damage += eq.damage;
        self.armour += eq.armour;
    }
    fn attacked_by(&mut self, other: &Player) {
        let damage = (other.damage - self.armour).max(1);
        self.hit_points -= damage;
    }
}

#[derive(Debug)]
struct Equipment {
    cost: i32,
    damage: i32,
    armour: i32,
}

impl TryFrom<&str> for Equipment {
    type Error = Error;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut it = s.split_whitespace();
        let _name = it.next();
        Ok(Equipment {
            cost: it.next().context("no hp line")?.parse()?,
            damage: it.next().context("no hp line")?.parse()?,
            armour: it.next().context("no hp line")?.parse()?,
        })
    }
}

fn weapons() -> Result<Vec<Equipment>> {
    "Dagger        8     4       0
Shortsword   10     5       0
Warhammer    25     6       0
Longsword    40     7       0
Greataxe     74     8       0"
        .split('\n')
        .map(Equipment::try_from)
        .collect()
}
fn armour() -> Result<Vec<Equipment>> {
    "Leather      13     0       1
Chainmail    31     0       2
Splintmail   53     0       3
Bandedmail   75     0       4
Platemail   102     0       5"
        .split('\n')
        .map(Equipment::try_from)
        .collect()
}
fn rings() -> Result<Vec<Equipment>> {
    "Damage+1    25     1       0
Damage+2    50     2       0
Damage+3   100     3       0
Defense+1   20     0       1
Defense+2   40     0       2
Defense+3   80     0       3"
        .split('\n')
        .map(Equipment::try_from)
        .collect()
}

fn parse_line(line: &str) -> Result<i32> {
    let (_, raw) = line.split_once(": ").context("split failed")?;
    raw.parse().context("nan")
}

fn parse(s: &str) -> Result<Player> {
    let mut lines = s.split('\n').map(parse_line);
    Ok(Player {
        hit_points: lines.next().context("no hp line")??,
        damage: lines.next().context("no hp line")??,
        armour: lines.next().context("no hp line")??,
    })
}

fn player_win(player: Player, boss: Player) -> bool {
    let mut player = player;
    let mut boss = boss;

    loop {
        boss.attacked_by(&player);
        if boss.hit_points <= 0 {
            return true;
        }

        player.attacked_by(&boss);
        if player.hit_points <= 0 {
            return false;
        }
    }
}

fn part1(input: &str) -> Result<i32> {
    let boss = parse(input)?;
    let player_template = Player {
        hit_points: 100,
        damage: 0,
        armour: 0,
    };
    let weapons = weapons()?;
    let armours = armour()?;
    let rings = rings()?;
    let mut cheapest_min = 9999;
    for weapon in 0..weapons.len() {
        for armour in 0..=armours.len() {
            for ring1 in 0..=rings.len() {
                for ring2 in 0..=rings.len() {
                    if ring1 == ring2 {
                        continue;
                    }
                    let mut player = player_template;
                    let mut cost = 0;
                    if let Some(weapon) = weapons.get(weapon) {
                        player.equip(weapon);
                        cost += weapon.cost;
                    }
                    if let Some(armour) = armours.get(armour) {
                        player.equip(armour);
                        cost += armour.cost;
                    }
                    if let Some(ring1) = rings.get(ring1) {
                        player.equip(ring1);
                        cost += ring1.cost;
                    }
                    if let Some(ring2) = rings.get(ring2) {
                        player.equip(ring2);
                        cost += ring2.cost;
                    }
                    if player_win(player, boss) {
                        cheapest_min = cheapest_min.min(cost);
                    }
                }
            }
        }
    }
    Ok(cheapest_min)
}

fn part2(input: &str) -> Result<i32> {
    let boss = parse(input)?;
    let player_template = Player {
        hit_points: 100,
        damage: 0,
        armour: 0,
    };
    let weapons = weapons()?;
    let armours = armour()?;
    let rings = rings()?;
    let mut dearest_max = 0;
    for weapon in 0..weapons.len() {
        for armour in 0..=armours.len() {
            for ring1 in 0..=rings.len() {
                for ring2 in 0..=rings.len() {
                    if ring1 == ring2 {
                        continue;
                    }
                    let mut player = player_template;
                    let mut cost = 0;
                    if let Some(weapon) = weapons.get(weapon) {
                        player.equip(weapon);
                        cost += weapon.cost;
                    }
                    if let Some(armour) = armours.get(armour) {
                        player.equip(armour);
                        cost += armour.cost;
                    }
                    if let Some(ring1) = rings.get(ring1) {
                        player.equip(ring1);
                        cost += ring1.cost;
                    }
                    if let Some(ring2) = rings.get(ring2) {
                        player.equip(ring2);
                        cost += ring2.cost;
                    }
                    if !player_win(player, boss) {
                        dearest_max = dearest_max.max(cost);
                    }
                }
            }
        }
    }
    Ok(dearest_max)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(INPUT)?, 0);
        Ok(())
    }
}
