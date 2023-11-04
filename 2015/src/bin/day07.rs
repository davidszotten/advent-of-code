use anyhow::{Context, Error, Result};
use aoc2015::dispatch;
use std::collections::HashMap;
use std::convert::TryFrom;

type Wire = String;
type Wires = HashMap<Wire, u16>;

#[derive(Debug)]
enum Source {
    Value(u16),
    Variable(Wire),
}

impl Source {
    fn value(&self, wires: &Wires) -> Option<u16> {
        match self {
            Source::Value(v) => Some(*v),
            Source::Variable(w) => wires.get(&*w).map(|v| *v),
        }
    }
}

impl From<&str> for Source {
    fn from(s: &str) -> Source {
        if let Ok(n) = s.parse() {
            Source::Value(n)
        } else {
            Source::Variable(s.into())
        }
    }
}

#[derive(Debug)]
enum Input {
    Direct(Source),
    And(Source, Source),
    Or(Source, Source),
    Not(Source),
    Lshift(Source, Source),
    Rshift(Source, Source),
}

#[derive(Debug)]
struct Op {
    input: Input,
    target: Wire,
}

fn combine(wires: &Wires, a: &Source, b: &Source, f: fn(u16, u16) -> u16) -> Option<u16> {
    match (a.value(&wires), b.value(&wires)) {
        (Some(a), Some(b)) => Some(f(a, b)),
        _ => None,
    }
}

impl Op {
    fn apply(&self, wires: &Wires) -> Option<u16> {
        use Input::*;
        match &self.input {
            Direct(s) => s.value(&wires),
            And(s1, s2) => combine(&wires, &s1, &s2, |s1, s2| s1 & s2),
            Or(s1, s2) => combine(&wires, &s1, &s2, |s1, s2| s1 | s2),
            Lshift(s1, s2) => combine(&wires, &s1, &s2, |s1, s2| s1 << s2),
            Rshift(s1, s2) => combine(&wires, &s1, &s2, |s1, s2| s1 >> s2),
            Not(s) => s.value(&wires).map(|s| !s),
        }
    }
}

impl TryFrom<&str> for Op {
    type Error = Error;
    fn try_from(s: &str) -> Result<Op> {
        let (source, target) = s.split_once(" -> ").context("no arrow")?;
        let input = if let Some((left, right)) = source.split_once(" AND ") {
            Input::And(left.into(), right.into())
        } else if let Some((left, right)) = source.split_once(" OR ") {
            Input::Or(left.into(), right.into())
        } else if let Some((left, right)) = source.split_once(" LSHIFT ") {
            Input::Lshift(left.into(), right.into())
        } else if let Some((left, right)) = source.split_once(" RSHIFT ") {
            Input::Rshift(left.into(), right.into())
        // } else if let Ok(n) = source.parse::<u16>() {
        //     Input::Direct(n)
        } else if let Some(value) = source.strip_prefix("NOT ") {
            Input::Not(value.into())
        } else {
            Input::Direct(source.into())
            // bail!("failed to parse: `{}`", s)
        };
        Ok(Op {
            input,
            target: target.into(),
        })
    }
}

fn main() -> Result<()> {
    dispatch(part1, part2)
}

fn find(ops: &[Op], target: Wire) -> u16 {
    let mut wires = HashMap::new();
    while !wires.contains_key(&target) {
        for op in ops {
            if let Some(value) = op.apply(&wires) {
                wires.insert(op.target.clone(), value);
            }
        }
    }
    wires[&target]
}

fn parse(input: &str) -> Result<Vec<Op>> {
    input.split('\n').map(Op::try_from).collect()
}

fn part1(input: &str) -> Result<u16> {
    let ops = parse(input)?;
    Ok(find(&ops, "a".into()))
}

fn part2(input: &str) -> Result<u16> {
    let ops = parse(input)?;
    let new_b = find(&ops, "a".into());
    let ops = ops
        .into_iter()
        .map(|o| match o {
            Op {
                input: Input::Direct(_),
                target: b,
            } if b == "b".to_string() => Op {
                input: Input::Direct(Source::Value(new_b)),
                target: "b".into(),
            },
            op => op,
        })
        .collect::<Vec<_>>();
    Ok(find(&ops, "a".into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i";

    #[test]
    fn test_parse() -> Result<()> {
        let ops = parse(&INPUT);
        Ok(())
    }

    #[test]
    fn test_find() -> Result<()> {
        assert_eq!(find(&parse(INPUT)?, "d".into()), 72);
        assert_eq!(find(&parse(INPUT)?, "e".into()), 507);
        assert_eq!(find(&parse(INPUT)?, "f".into()), 492);
        assert_eq!(find(&parse(INPUT)?, "g".into()), 114);
        assert_eq!(find(&parse(INPUT)?, "h".into()), 65412);
        assert_eq!(find(&parse(INPUT)?, "i".into()), 65079);
        assert_eq!(find(&parse(INPUT)?, "x".into()), 123);
        assert_eq!(find(&parse(INPUT)?, "y".into()), 456);
        Ok(())
    }
}
