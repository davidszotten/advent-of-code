use anyhow::{anyhow, Result};
use aoc2020::dispatch;
use std::collections::VecDeque;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

#[derive(Debug)]
struct Game {
    cups: VecDeque<usize>,
    // _current_index: usize,
}

fn round(game: Game) -> Game {
    let mut cups = game.cups;
    let len = cups.len();
    // current: cups[0]
    let current = cups.pop_front().expect("always have cups");
    cups.push_back(current);
    // cups.rotate_left(1);
    let mut pickup = vec![];
    // current now last, pick up first 3
    for _ in 0..3 {
        pickup.push(cups.pop_front().unwrap());
    }
    let sub_wrap = |n| {
        let s = (n + len - 1) % len;
        if s == 0 {
            len
        } else {
            s
        }
    };
    let mut destination = sub_wrap(current);
    while destination == pickup[0] || destination == pickup[1] || destination == pickup[2] {
        destination = sub_wrap(destination);
    }
    // dbg!(&cups);
    // dbg!(destination);
    let index = cups
        .iter()
        .enumerate()
        .filter(|(_, &val)| val == destination)
        .map(|(idx, _)| idx)
        .next()
        .expect("should find destination");
    // dbg!(&cups, destination, index);
    for _ in 0..3 {
        cups.insert(index + 1, pickup.pop().expect("have 3"));
    }
    let current_index = cups
        .iter()
        .enumerate()
        .filter(|(_, &val)| val == current)
        .map(|(idx, _)| idx)
        .next()
        .expect("should find destination");
    // dbg!(current_index, &cups);
    cups.rotate_left(current_index + 1);
    Game { cups }
}

fn run(mut game: Game, moves: usize) -> String {
    for _mv in 0..moves {
        // println!("Move {}\n{:?}\n", mv + 1, &game.cups);
        game = round(game);
    }

    let one_index = game
        .cups
        .iter()
        .enumerate()
        .filter(|(_, &val)| val == 1)
        .map(|(idx, _)| idx)
        .next()
        .expect("should find destination");
    game.cups.rotate_left(one_index);

    let mut res = "".to_string();
    for cup in game.cups.iter().skip(1) {
        res = format!("{}{}", res, cup);
    }
    res
}

fn parse(input: &str) -> Result<Game> {
    let cups = input
        .chars()
        .map(|c| {
            c.to_string()
                .parse()
                .map_err(|e| anyhow!("parse failure: {}", e))
        })
        .collect::<Result<VecDeque<usize>>>()?;
    Ok(Game { cups })
}

fn part1(input: &str) -> Result<String> {
    let game = parse(input)?;
    // println!("Move {}\n{:?}\n", 0 + 1, &game.cups);
    let res = run(game, 100);
    // println!("Move {}\n{:?}\n", "end", &game.cups);
    Ok(res)

    // 52863971 too high
}

fn part2(input: &str) -> Result<i32> {
    let game = parse(input)?;
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1("389125467")?, "67384529");
        Ok(())
    }

    #[test]
    fn test_run() -> Result<()> {
        let game = parse("389125467")?;
        let res = run(game, 10);
        assert_eq!(res, "92658374");
        Ok(())
    }
}
