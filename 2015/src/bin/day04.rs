use anyhow::Result;
use aoc2015::dispatch;
use md5::{Digest, Md5};

fn main() -> Result<()> {
    dispatch(part1, part2)
}

fn part1(input: &str) -> Result<usize> {
    let mut n = 1;
    Ok(loop {
        let hash = Md5::digest(format!("{}{}", input, n).as_bytes());
        if format!("{:x}", hash).starts_with("00000") {
            break n;
        }
        n += 1;
    })
}

fn part2(input: &str) -> Result<usize> {
    let mut n = 1;
    Ok(loop {
        let hash = Md5::digest(format!("{}{}", input, n).as_bytes());
        if format!("{:x}", hash).starts_with("000000") {
            break n;
        }
        n += 1;
    })
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
