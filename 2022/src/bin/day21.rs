use anyhow::{Error, Result};
use aoc2022::dispatch;
use itertools::Itertools;
use std::collections::HashMap;
use std::ops::{Add, Div, Mul, Sub};

fn main() -> Result<()> {
    dispatch(part1, part2)
}

type F = fraction::Fraction;

#[derive(Debug)]
enum Val<'a> {
    Num(F),
    Var(&'a str),
}

#[derive(Debug)]
enum Op<'a> {
    Val(Val<'a>),
    Add(Val<'a>, Val<'a>),
    Sub(Val<'a>, Val<'a>),
    Mul(Val<'a>, Val<'a>),
    Div(Val<'a>, Val<'a>),
    Eq(Val<'a>, Val<'a>),
}

impl<'a> TryFrom<&'a str> for Val<'a> {
    type Error = Error;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        s.parse::<F>()
            .map(|n| Ok(Val::Num(n)))
            .unwrap_or(Ok(Val::Var(s)))
    }
}

impl<'a> TryFrom<&'a str> for Op<'a> {
    type Error = Error;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        if let Ok(Val::Num(n)) = s.try_into() {
            return Ok(Op::Val(Val::Num(n)));
        }
        if let Some((l, r)) = s.split_once(" + ") {
            return Ok(Op::Add(l.try_into()?, r.try_into()?));
        }
        if let Some((l, r)) = s.split_once(" - ") {
            return Ok(Op::Sub(l.try_into()?, r.try_into()?));
        }
        if let Some((l, r)) = s.split_once(" * ") {
            return Ok(Op::Mul(l.try_into()?, r.try_into()?));
        }
        if let Some((l, r)) = s.split_once(" / ") {
            return Ok(Op::Div(l.try_into()?, r.try_into()?));
        }
        Ok(Op::Val(Val::Var(s)))
    }
}

fn resolve(name: &str, ops: &HashMap<&str, Op>) -> F {
    let op = ops.get(name).unwrap();
    use Val::*;
    let eq = |a, b| if a == b { F::from(0) } else { F::from(1) };
    match op {
        Op::Val(Num(n)) => *n,
        Op::Val(Var(v)) => resolve(v, ops),
        Op::Add(Num(l), Num(r)) => *l + *r,
        Op::Add(Num(l), Var(r)) => *l + resolve(r, ops),
        Op::Add(Var(l), Num(r)) => resolve(l, ops) + *r,
        Op::Add(Var(l), Var(r)) => resolve(l, ops) + resolve(r, ops),
        Op::Sub(Num(l), Num(r)) => *l - *r,
        Op::Sub(Num(l), Var(r)) => *l - resolve(r, ops),
        Op::Sub(Var(l), Num(r)) => resolve(l, ops) - *r,
        Op::Sub(Var(l), Var(r)) => resolve(l, ops) - resolve(r, ops),
        Op::Mul(Num(l), Num(r)) => *l * *r,
        Op::Mul(Num(l), Var(r)) => *l * resolve(r, ops),
        Op::Mul(Var(l), Num(r)) => resolve(l, ops) * *r,
        Op::Mul(Var(l), Var(r)) => resolve(l, ops) * resolve(r, ops),
        Op::Div(Num(l), Num(r)) => *l / *r,
        Op::Div(Num(l), Var(r)) => *l / resolve(r, ops),
        Op::Div(Var(l), Num(r)) => resolve(l, ops) / *r,
        Op::Div(Var(l), Var(r)) => resolve(l, ops) / resolve(r, ops),
        Op::Eq(Num(l), Num(r)) => eq(*l, *r),
        Op::Eq(Num(l), Var(r)) => eq(*l, resolve(r, ops)),
        Op::Eq(Var(l), Num(r)) => eq(resolve(l, ops), *r),
        Op::Eq(Var(l), Var(r)) => eq(resolve(l, ops), resolve(r, ops)),
    }
}

fn part1(input: &str) -> Result<F> {
    let ops = input
        .split('\n')
        .map(|l| {
            let (var, op) = l.split_once(": ").unwrap();
            Ok((var, op.try_into()?))
        })
        .collect::<Result<HashMap<&str, Op>>>()?;
    Ok(resolve("root", &ops))
}

#[derive(Debug, PartialEq)]
struct Poly(Vec<F>);

impl Poly {
    fn n(c: F) -> Self {
        Poly(vec![c])
    }
}

impl Add for Poly {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Poly(
            self.0
                .into_iter()
                .zip_longest(other.0.into_iter())
                .map(|aorb| aorb.or_default())
                .map(|(a, b)| a + b)
                .collect(),
        )
    }
}

impl Add<Poly> for F {
    type Output = Poly;

    fn add(self, other: Poly) -> Poly {
        Poly(other.0.into_iter().map(|a| a + self).collect())
    }
}
impl Add<F> for Poly {
    type Output = Self;

    fn add(self, other: F) -> Self {
        Poly(self.0.into_iter().map(|a| a + other).collect())
    }
}

impl Sub for Poly {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Poly(
            self.0
                .into_iter()
                .zip_longest(other.0.into_iter())
                .map(|aorb| aorb.or_default())
                .map(|(a, b)| a - b)
                .collect(),
        )
    }
}

impl Sub<F> for Poly {
    type Output = Self;

    fn sub(self, other: F) -> Self {
        self - Poly::n(other)
    }
}

impl Sub<Poly> for F {
    type Output = Poly;

    fn sub(self, other: Poly) -> Poly {
        Poly::n(self) - other
    }
}

impl Mul<F> for Poly {
    type Output = Self;

    fn mul(self, other: F) -> Self {
        // self * Poly::n(other)
        Poly(self.0.into_iter().map(|a| a * other).collect())
    }
}

impl Mul<Poly> for F {
    type Output = Poly;

    fn mul(self, other: Poly) -> Poly {
        other * self
    }
}

impl Mul for Poly {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        self.0
            .into_iter()
            .enumerate()
            .map(|(idx, n)| {
                let mut other = other.0.clone();
                for _ in 0..idx {
                    other.insert(0, F::from(0));
                }
                n * Poly(other)
            })
            .fold(Poly(vec![]), |acc, x| acc + x)
    }
}

impl Div for Poly {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        if other.0.len() == 1 {
            return self / other.0[0];
        }
        dbg!(self, other);
        panic!("div");
    }
}

impl Div<F> for Poly {
    type Output = Self;

    fn div(self, other: F) -> Self {
        Poly(self.0.into_iter().map(|a| a / other).collect())
    }
}

impl Div<Poly> for F {
    type Output = Poly;

    fn div(self, other: Poly) -> Poly {
        dbg!(self, other);
        panic!("div");
    }
}

fn resolve2(name: &str, ops: &HashMap<&str, Op>) -> Poly {
    let op = ops.get(name).unwrap();
    use Val::*;

    if name == "humn" {
        return Poly(vec![F::from(0), F::from(1)]);
    }

    fn eq(a: Poly, b: Poly) -> Poly {
        // c + ax = b
        // x = (b-c)/a
        Poly::n((b.0[0] - a.0[0]) / a.0[1])
    }

    match op {
        Op::Val(Num(n)) => Poly::n(*n),
        Op::Val(Var(v)) => resolve2(v, ops),
        Op::Add(Num(l), Num(r)) => Poly::n(*l + *r),
        Op::Add(Num(l), Var(r)) => *l + resolve2(r, ops),
        Op::Add(Var(l), Num(r)) => resolve2(l, ops) + *r,
        Op::Add(Var(l), Var(r)) => resolve2(l, ops) + resolve2(r, ops),
        Op::Sub(Num(l), Num(r)) => Poly::n(*l - *r),
        Op::Sub(Num(l), Var(r)) => *l - resolve2(r, ops),
        Op::Sub(Var(l), Num(r)) => resolve2(l, ops) - *r,
        Op::Sub(Var(l), Var(r)) => resolve2(l, ops) - resolve2(r, ops),
        Op::Mul(Num(l), Num(r)) => Poly::n(*l * *r),
        Op::Mul(Num(l), Var(r)) => *l * resolve2(r, ops),
        Op::Mul(Var(l), Num(r)) => resolve2(l, ops) * *r,
        Op::Mul(Var(l), Var(r)) => resolve2(l, ops) * resolve2(r, ops),
        Op::Div(Num(l), Num(r)) => Poly::n(*l / *r),
        Op::Div(Num(l), Var(r)) => *l / resolve2(r, ops),
        Op::Div(Var(l), Num(r)) => resolve2(l, ops) / *r,
        Op::Div(Var(l), Var(r)) => resolve2(l, ops) / resolve2(r, ops),
        Op::Eq(Num(l), Num(r)) => eq(Poly::n(*l), Poly::n(*r)),
        Op::Eq(Num(l), Var(r)) => eq(Poly::n(*l), resolve2(r, ops)),
        Op::Eq(Var(l), Num(r)) => eq(resolve2(l, ops), Poly::n(*r)),
        Op::Eq(Var(l), Var(r)) => eq(resolve2(l, ops), resolve2(r, ops)),
    }
}

fn part2(input: &str) -> Result<F> {
    let mut ops = input
        .split('\n')
        .map(|l| {
            let (var, op) = l.split_once(": ").unwrap();
            Ok((var, op.try_into()?))
        })
        .collect::<Result<HashMap<&str, Op>>>()?;
    if let Op::Add(Val::Var(l), Val::Var(r)) = ops.get("root").unwrap() {
        ops.insert("root", Op::Eq(Val::Var(l), Val::Var(r)));
    } else {
        panic!("oops")
    }
    Ok(resolve2("root", &ops).0[0])
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(TEST_INPUT)?, F::from(152));
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(TEST_INPUT)?, F::from(301));
        Ok(())
    }

    #[test]
    fn test_poly() {
        assert_eq!(
            Poly(vec![F::from(1), F::from(2)]) + Poly(vec![F::from(4)]),
            Poly(vec![F::from(5), F::from(2)])
        );
        // (1+2x)(3+4x) = 3+4x+6x+8xx
        assert_eq!(
            Poly(vec![F::from(1), F::from(2)]) * Poly(vec![F::from(3), F::from(4)]),
            Poly(vec![F::from(3), F::from(10), F::from(8)])
        );
    }
}
