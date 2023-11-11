use std::{cmp::Ordering, collections::BinaryHeap};

use anyhow::{Context, Result};
use aoc2015::dispatch;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum ImmediateSpell {
    Missile,
    Drain,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum EffectSpell {
    Shield,
    Poison,
    Recharge,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum Spell {
    Immediate(ImmediateSpell),
    Effect(EffectSpell),
}

impl Spell {
    fn mana(&self) -> i32 {
        match self {
            Spell::Immediate(ImmediateSpell::Missile) => 53,
            Spell::Immediate(ImmediateSpell::Drain) => 73,
            Spell::Effect(EffectSpell::Shield) => 113,
            Spell::Effect(EffectSpell::Poison) => 173,
            Spell::Effect(EffectSpell::Recharge) => 229,
        }
    }
}

const SPELLS: [Spell; 5] = [
    Spell::Immediate(ImmediateSpell::Missile),
    Spell::Immediate(ImmediateSpell::Drain),
    Spell::Effect(EffectSpell::Shield),
    Spell::Effect(EffectSpell::Poison),
    Spell::Effect(EffectSpell::Recharge),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Player {
    hp: i32,
    armour: i32,
    mana: i32,
}
impl Player {
    fn attacked_by(&mut self, boss: &Boss) {
        let damage = (boss.damage - self.armour).max(1);
        self.hp -= damage;
    }

    fn cast(&mut self, spell: ImmediateSpell, boss: &mut Boss) {
        match spell {
            ImmediateSpell::Missile => boss.hp -= 4,
            ImmediateSpell::Drain => {
                self.hp += 2;
                boss.hp -= 2
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Boss {
    hp: i32,
    damage: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Effect {
    spell: EffectSpell,
    timer: i8,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum EffectApplied {
    Done,
    Continues,
}

impl Effect {
    fn new(spell: EffectSpell) -> Self {
        let timer = match spell {
            EffectSpell::Shield | EffectSpell::Poison => 6,
            EffectSpell::Recharge => 5,
        };
        Self { spell, timer }
    }

    fn apply(&mut self, player: &mut Player, boss: &mut Boss) -> EffectApplied {
        match self.spell {
            EffectSpell::Shield => player.armour = 7,
            EffectSpell::Poison => boss.hp -= 3,
            EffectSpell::Recharge => player.mana += 101,
        }
        self.timer -= 1;
        if self.timer <= 0 {
            EffectApplied::Done
        } else {
            EffectApplied::Continues
        }
    }

    fn apply_effects(effects: Vec<Effect>, player: &mut Player, boss: &mut Boss) -> Vec<Effect> {
        let mut new = vec![];
        player.armour = 0;
        for mut effect in effects {
            if effect.apply(player, boss) == EffectApplied::Continues {
                new.push(effect);
            }
        }
        new
    }
}

fn check_result(player: &Player, boss: &Boss) -> Outcome {
    if player.hp <= 0 {
        Outcome::Lose
    } else if boss.hp <= 0 {
        Outcome::Win
    } else {
        Outcome::Undecided
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Outcome {
    Undecided,
    Win,
    Lose,
}

fn parse_line(line: &str) -> Result<i32> {
    let (_, raw) = line.split_once(": ").context("split failed")?;
    raw.parse().context("nan")
}

fn parse(s: &str) -> Result<Boss> {
    let mut lines = s.split('\n').map(parse_line);
    Ok(Boss {
        hp: lines.next().context("no hp line")??,
        damage: lines.next().context("no hp line")??,
    })
}

#[derive(Clone, Eq, PartialEq)]
struct State {
    player: Player,
    boss: Boss,
    effects: Vec<Effect>,
    spent: i32,
}
// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .spent
            .cmp(&self.spent)
            .then_with(|| self.player.cmp(&other.player))
            .then_with(|| self.boss.cmp(&other.boss))
            .then_with(|| self.effects.cmp(&other.effects))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn run(input: &str, hard: bool) -> Result<i32> {
    let boss = parse(input)?;
    let player = Player {
        hp: 50,
        armour: 0,
        mana: 500,
    };
    let effects = vec![];
    let mut queue = BinaryHeap::new();
    queue.push(State {
        player,
        boss,
        effects,
        spent: 0,
    });
    while let Some(State {
        mut player,
        mut boss,
        effects,
        spent,
    }) = queue.pop()
    {
        if hard {
            player.hp -= 1;
            if check_result(&player, &boss) == Outcome::Lose {
                continue;
            }
        }
        let effects = Effect::apply_effects(effects, &mut player, &mut boss);
        if check_result(&player, &boss) == Outcome::Win {
            return Ok(spent);
        }
        if player.mana < 0 {
            continue;
        }
        for spell in SPELLS {
            if spell.mana() > player.mana {
                continue;
            }
            let mut effects = effects.clone();
            let mut player = player;
            let mut boss = boss;
            if let Spell::Effect(espell) = spell {
                if effects.iter().any(|e| e.spell == espell) {
                    continue;
                }
                effects.push(Effect::new(espell));
            } else if let Spell::Immediate(ispell) = spell {
                player.cast(ispell, &mut boss);
                if check_result(&player, &boss) == Outcome::Win {
                    return Ok(spent + spell.mana());
                }
            }

            player.mana -= spell.mana();

            let effects = Effect::apply_effects(effects, &mut player, &mut boss);
            if check_result(&player, &boss) == Outcome::Win {
                return Ok(spent + spell.mana());
            }
            player.attacked_by(&boss);
            if check_result(&player, &boss) == Outcome::Lose {
                continue;
            }
            queue.push(State {
                player,
                boss,
                effects,
                spent: spent + spell.mana(),
            });
        }
    }
    Ok(-1)
}

fn part1(input: &str) -> Result<i32> {
    run(input, false)
}

fn part2(input: &str) -> Result<i32> {
    run(input, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "";

    #[test]
    fn test_part1() -> Result<()> {
        let mut player = Player {
            hp: 10,
            armour: 0,
            mana: 250,
        };
        let mut boss = Boss { hp: 13, damage: 8 };
        let effects = vec![];

        // p1
        dbg!(player, boss);
        let mut effects = Effect::apply_effects(effects, &mut player, &mut boss);
        effects.push(Effect::new(EffectSpell::Poison));
        player.mana -= 173;

        // b1
        dbg!(player, boss);
        let effects = Effect::apply_effects(effects, &mut player, &mut boss);
        player.attacked_by(&boss);

        // p2
        dbg!(player, boss);
        let effects = Effect::apply_effects(effects, &mut player, &mut boss);
        player.cast(ImmediateSpell::Missile, &mut boss);

        // b2
        dbg!(player, boss);
        let effects = Effect::apply_effects(effects, &mut player, &mut boss);

        dbg!(player, boss);

        Ok(())
    }
}
