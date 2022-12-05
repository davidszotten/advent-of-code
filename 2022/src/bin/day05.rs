use anyhow::{Context, Error, Result};
use aoc2022::dispatch;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

type Stack = Vec<char>;

struct Move {
    count: usize,
    from: usize,
    to: usize,
}

impl std::str::FromStr for Move {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        // move 1 from 2 to 1
        let mut parts = s.split(' ');
        parts.next().context("'move' missing")?;
        let count = parts.next().context("move missing")?.parse()?;
        parts.next().context("'from' missing")?;
        let from = parts.next().context("move missing")?.parse()?;
        parts.next().context("'to' missing")?;
        let to = parts.next().context("to missing")?.parse()?;
        Ok(Move { count, from, to })
    }
}

fn parse_stacks(input: &str) -> Result<Vec<Stack>> {
    //     [D]
    // [N] [C]
    // [Z] [M] [P]
    //  1   2   3
    let mut stacks = vec![];
    for _ in 0..=input.split('\n').next().unwrap_or(input).len() / 4 {
        stacks.push(vec![]);
    }
    for line in input.split('\n').rev() {
        if &line[..2] == " 1" {
            continue;
        }
        for start in 0..=(line.len() / 4) {
            let char = line[start * 4 + 1..start * 4 + 2].as_bytes()[0] as char;
            if char != ' ' {
                stacks[start].push(char);
            }
        }
    }
    Ok(stacks)
}

fn parse_moves(input: &str) -> Result<Vec<Move>> {
    input.split('\n').map(|r| r.parse()).collect()
}

fn parse(input: &str) -> Result<(Vec<Stack>, Vec<Move>)> {
    let (raw_stacks, raw_moves) = input.split_once("\n\n").context("malformed input")?;
    Ok((parse_stacks(raw_stacks)?, parse_moves(raw_moves)?))
}

fn run(stacks: Vec<Vec<char>>, moves: Vec<Move>) -> Result<Vec<char>> {
    let mut stacks = stacks;
    for mv in moves {
        for _ in 0..mv.count {
            let crat = stacks[mv.from - 1].pop().context("nothing to move")?;
            stacks[mv.to - 1].push(crat);
        }
    }
    Ok(stacks.iter().map(|s| s[s.len() - 1]).collect())
}

fn run2(stacks: Vec<Vec<char>>, moves: Vec<Move>) -> Result<Vec<char>> {
    let mut stacks = stacks;
    for mv in moves {
        let mut temp = vec![];
        for _ in 0..mv.count {
            temp.push(stacks[mv.from - 1].pop().context("nothing to move")?);
        }
        for _ in 0..mv.count {
            stacks[mv.to - 1].push(temp.pop().context("nothing to move")?);
        }
    }
    Ok(stacks.iter().map(|s| s[s.len() - 1]).collect())
}

fn part1(input: &str) -> Result<String> {
    let (stacks, moves) = parse(input)?;
    let result = run(stacks, moves);
    Ok(result?.iter().collect())
}

fn part2(input: &str) -> Result<String> {
    let (stacks, moves) = parse(input)?;
    let result = run2(stacks, moves);
    Ok(result?.iter().collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    // use concat and literal '\n' to work around the trailing whitespace trim
    const TEST_INPUT: &str = concat!(
        "    [D]    \n",
        "[N] [C]    \n",
        "[Z] [M] [P]\n",
        " 1   2   3 \n",
        "
move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"
    );

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(TEST_INPUT)?, "CMZ".to_string());
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(TEST_INPUT)?, "MCD".to_string());
        Ok(())
    }

    #[test]
    fn test_parse_stacks() -> Result<()> {
        let (raw_stacks, _) = TEST_INPUT.split_once("\n\n").context("malformed input")?;
        let stacks = parse_stacks(raw_stacks)?;
        assert_eq!(stacks, vec![vec!['Z', 'N'], vec!['M', 'C', 'D'], vec!['P']]);
        Ok(())
    }
}
