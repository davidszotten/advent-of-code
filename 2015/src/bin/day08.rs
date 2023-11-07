use anyhow::Result;
use aoc2015::dispatch;
use multipeek::multipeek;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

fn count(s: &str) -> usize {
    let mut res = 2; // quotes
    let mut it = multipeek(s.chars());
    while let Some(c) = it.next() {
        if c == '\\' {
            if it.peek() == Some(&'x') {
                for _ in 0..2 {
                    it.next();
                }
                res += 3;
            } else {
                res += 1;
                it.next();
            }
        }
    }
    res
}

fn count_back(s: &str) -> usize {
    s.chars()
        .map(|c| match c {
            '"' => 1,
            '\\' => 1,
            _ => 0,
        })
        .sum::<usize>()
        + 2
}

fn part1(input: &str) -> Result<usize> {
    Ok(input.split('\n').map(count).sum())
}

fn part2(input: &str) -> Result<usize> {
    Ok(input.split('\n').map(count_back).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = r#"""
"abc"
"aaa\"aaa"
"\x27""#;

    #[test]
    fn test_count() {
        assert_eq!(count(r#""""#), 2);
        assert_eq!(count(r#"abc"#), 2);
        assert_eq!(count(r#"aaa\"aaa"#), 3);
        assert_eq!(count(r#"\x27"#), 5);
    }

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(INPUT)?, 12);
        Ok(())
    }

    #[test]
    fn test_count_back() {
        assert_eq!(count_back(r#""""#), 4);
        assert_eq!(count_back(r#""abc""#), 4);
        assert_eq!(count_back(r#""aaa\"aaa""#), 6);
        assert_eq!(count_back(r#""\x27""#), 5);
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(INPUT)?, 19);
        Ok(())
    }
}
