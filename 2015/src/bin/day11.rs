use anyhow::{Context, Result};
use aoc2015::dispatch;
use std::convert::TryInto;

type Password = [char; 8];

fn main() -> Result<()> {
    dispatch(part1, part2)
}

fn check_run(password: &Password) -> bool {
    password
        .windows(3)
        .any(|win| win[1] as u8 == win[0] as u8 + 1 && win[2] as u8 == win[1] as u8 + 1)
}

fn check_chars(password: &Password) -> bool {
    password.iter().all(|&c| c != 'i' && c != 'o' && c != 'l')
}

fn check_pairs(password: &Password) -> bool {
    let mut it = password.iter().peekable();
    let mut found = 0;
    while let Some(char) = it.next() {
        if it.peek() == Some(&char) {
            it.next();
            found += 1
        }
        if found == 2 {
            return true;
        }
    }
    false
}

fn to_password(s: &str) -> Result<Password> {
    s.chars().collect::<Vec<_>>()[..]
        .try_into()
        .context("not 8 chars")
}

fn from_password(password: &Password) -> String {
    password.iter().collect()
}

fn check(password: Password) -> bool {
    check_run(&password) && check_chars(&password) && check_pairs(&password)
}

fn next_char(c: &mut char) {
    *c = ((*c as u8) + 1) as char
}

fn next_pos(password: &mut Password, pos: usize) {
    next_char(&mut password[pos]);
    if password[pos] as u8 > b'z' {
        password[pos] = 'a';
        if pos == 0 {
            panic!("overflow");
        }
        next_pos(password, pos - 1)
    }
}

fn next(password: &mut Password) {
    next_pos(password, 7)
}

fn part1(input: &str) -> Result<String> {
    let mut password = to_password(input)?;
    while !check(password) {
        next(&mut password);
    }
    Ok(from_password(&password))
}

fn part2(input: &str) -> Result<String> {
    let mut password = to_password(input)?;
    for _ in 0..2 {
        next(&mut password);
        while !check(password) {
            next(&mut password);
        }
    }
    Ok(from_password(&password))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "abcdefgh";

    #[test]
    fn test_check_pairs() -> Result<()> {
        assert!(check_pairs(&to_password("aabbuvxy")?));
        assert!(check_pairs(&to_password("aaaauvxy")?));
        assert!(!check_pairs(&to_password("aaauvxyz")?));
        Ok(())
    }

    #[test]
    fn test_next() -> Result<()> {
        let mut password = to_password("aaaaaaxx")?;
        next(&mut password);
        assert_eq!(&from_password(&password), "aaaaaaxy");
        next(&mut password);
        assert_eq!(&from_password(&password), "aaaaaaxz");
        next(&mut password);
        assert_eq!(&from_password(&password), "aaaaaaya");
        next(&mut password);
        assert_eq!(&from_password(&password), "aaaaaayb");
        Ok(())
    }

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(INPUT)?, "abcdffaa".to_string());
        Ok(())
    }
}
