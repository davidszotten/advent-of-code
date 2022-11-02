use anyhow::Result;
use aoc2015::dispatch;
use std::collections::HashMap;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

fn has_vowels(s: &str) -> bool {
    s.chars()
        .filter(|&c| c == 'a' || c == 'e' || c == 'i' || c == 'o' || c == 'u')
        .count()
        >= 3
}

fn has_double(s: &str) -> bool {
    for win in s.chars().collect::<Vec<_>>().windows(2) {
        if win[0] == win[1] {
            return true;
        }
    }
    false
}

fn contains_unwanted(s: &str) -> bool {
    s.contains("ab") || s.contains("cd") || s.contains("pq") || s.contains("xy")
}

fn nice(s: &str) -> bool {
    has_vowels(s) && has_double(s) && !contains_unwanted(s)
}

fn contains_pair(s: &str) -> bool {
    let mut seen = HashMap::new();
    for (idx, win) in s.chars().collect::<Vec<_>>().windows(2).enumerate() {
        if let Some(start) = seen.get(&win) {
            if idx > start + 1 {
                return true;
            }
        } else {
            seen.insert(win, idx);
        }
    }
    false
}

fn repeats_with_pair(s: &str) -> bool {
    for win in s.chars().collect::<Vec<_>>().windows(3) {
        if win[0] == win[2] {
            return true;
        }
    }
    false
}

fn nice2(s: &str) -> bool {
    contains_pair(s) && repeats_with_pair(s)
}

fn part1(input: &str) -> Result<usize> {
    Ok(input.split('\n').map(nice).filter(|&x| x).count())
}

fn part2(input: &str) -> Result<usize> {
    Ok(input.split('\n').map(nice2).filter(|&x| x).count())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "";

    #[test]
    fn test_nice() {
        assert!(nice("ugknbfddgicrmopn"));
        assert!(nice("aaa"));
        assert!(!nice("jchzalrnumimnmhp"));
        assert!(!nice("haegwjzuvuyypxyu"));
        assert!(!nice("dvszwmarrgswjxmb"));
    }

    #[test]
    fn test_contains_pair() {
        assert!(contains_pair("xyxy"));
        assert!(!contains_pair("aaa"));
    }
    #[test]
    fn test_repeats_with_pair() {
        assert!(repeats_with_pair("xyx"));
        assert!(repeats_with_pair("abcdefeghi"));
        assert!(repeats_with_pair("aaa"));
    }
    #[test]
    fn test_nice2() {
        assert!(nice2("qjhvhtzxzqqjkmpb"));
        assert!(nice2("xxyxx"));
        assert!(!nice2("uurcxstgmygtbstg"));
        assert!(!nice2("ieodomkazucvgmuy"));
    }

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(INPUT)?, 0);
        Ok(())
    }
}
