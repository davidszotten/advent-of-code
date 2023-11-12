use anyhow::{Context, Result};
use aoc2015::dispatch;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

fn parse(input: &str) -> Result<(usize, usize)> {
    let mut it = input.split_whitespace().flat_map(|s| s.trim_end_matches(|c| c == ',' || c == '.').parse::<usize>());
    Ok((it.next().context("row missing")?, it.next().context("col missing")?))
}

fn next(code: usize) -> usize {
    (code * 252533) % 33554393
}

fn next_coor(x: usize, y: usize) -> (usize, usize) {
    if y == 1 {
        (1, x+1)
    } else {
        (x+1, y-1)
    }
}

fn part1(input: &str) -> Result<usize> {
    let (row, col) = parse(input)?;
    let mut code = 20151125;
    let mut x = 1;
    let mut y = 1;
    while x != col || y != row {
        (x,y) = next_coor(x,y);
        code = next(code)
    }
    Ok(code)
}

fn part2(_input: &str) -> Result<usize> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "To continue, please consult the code grid in the manual.  Enter the code at row 1, column 2";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(INPUT)?, 0);
        Ok(())
    }
}
