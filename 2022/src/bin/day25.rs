use anyhow::Result;
use aoc2022::dispatch;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

fn to_decimal(snafu: &str) -> i64 {
    snafu.chars().fold(0, |acc, c| {
        acc * 5
            + match c {
                '0'..='2' => c as i64 - '0' as i64,
                '-' => -1,
                '=' => -2,
                _ => unreachable!(),
            }
    })
}

fn to_snafu(n: i64) -> String {
    if n == 0 {
        return "".to_string();
    }
    let r = n.rem_euclid(5);
    match r {
        0 | 1 | 2 => to_snafu(n / 5) + &format!("{}", r),
        3 => to_snafu((n + 2) / 5) + "=",
        4 => to_snafu((n + 2) / 5) + "-",
        _ => unreachable!(),
    }
}

fn part1(input: &str) -> Result<String> {
    Ok(to_snafu(input.split('\n').map(to_decimal).sum()))
}

fn part2(_input: &str) -> Result<i64> {
    Ok(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";

    #[test]
    fn test_to_snafu() {
        assert_eq!(to_snafu(1), "1");
        assert_eq!(to_snafu(2), "2");
        assert_eq!(to_snafu(3), "1=");
        assert_eq!(to_snafu(4), "1-");
        assert_eq!(to_snafu(5), "10");
        assert_eq!(to_snafu(6), "11");
        assert_eq!(to_snafu(7), "12");
        assert_eq!(to_snafu(8), "2=");
        assert_eq!(to_snafu(9), "2-");
        assert_eq!(to_snafu(10), "20");
        assert_eq!(to_snafu(15), "1=0");
        assert_eq!(to_snafu(20), "1-0");
        assert_eq!(to_snafu(2022), "1=11-2");
        assert_eq!(to_snafu(12345), "1-0---0");
        assert_eq!(to_snafu(314159265), "1121-1110-1=0");
    }

    #[test]
    fn test_to_decimal() {
        assert_eq!((1), to_decimal("1"));
        assert_eq!((2), to_decimal("2"));
        assert_eq!((3), to_decimal("1="));
        assert_eq!((4), to_decimal("1-"));
        assert_eq!((5), to_decimal("10"));
        assert_eq!((6), to_decimal("11"));
        assert_eq!((7), to_decimal("12"));
        assert_eq!((8), to_decimal("2="));
        assert_eq!((9), to_decimal("2-"));
        assert_eq!((10), to_decimal("20"));
        assert_eq!((15), to_decimal("1=0"));
        assert_eq!((20), to_decimal("1-0"));
        assert_eq!((2022), to_decimal("1=11-2"));
        assert_eq!((12345), to_decimal("1-0---0"));
        assert_eq!((314159265), to_decimal("1121-1110-1=0"));
    }

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(TEST_INPUT)?, "2=-1=0");
        Ok(())
    }
}
