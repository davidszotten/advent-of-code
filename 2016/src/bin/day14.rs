use aoc2016::{dispatch, Result};
use md5::{Digest, Md5};
use std::collections::HashMap;
// use hex_literal::hex;

fn main() -> Result<()> {
    dispatch(&part1, &part2)
}

fn find_run<T: Eq + Copy>(bytes: &[T], length: usize) -> Option<T> {
    for window in bytes.windows(length) {
        if window.windows(2).all(|w| w[0] == w[1]) {
            return Some(window[0]);
        }
    }
    None
}

fn has_n_run<T: Eq>(bytes: &[T], n: &T, length: usize) -> bool {
    for window in bytes.windows(length) {
        if window.iter().all(|c| c == n) {
            return true;
        }
    }
    false
}

fn hash1(input: &str) -> String {
    let mut hasher = Md5::new();
    hasher.update(input);
    let result = hasher.finalize();
    let hash = format!("{:x}", result);
    hash
}

fn hash2016(input: &str) -> String {
    let mut input = input.to_string();
    let mut hasher = Md5::new();
    for _ in 0..2017 {
        hasher.update(&input);
        let result = hasher.finalize_reset();
        input = format!("{:x}", result);
    }
    input
}

fn part1(input: &str) -> Result<usize> {
    let mut index = 0;
    let mut potential: HashMap<usize, _> = HashMap::new();
    let mut found = vec![];
    let mut last = None;
    loop {
        let key = format!("{}{}", input, index);
        let hash = hash1(&key);

        let mut remove = None;
        for (pot_index, pot_value) in &potential {
            if has_n_run(&hash.as_bytes(), pot_value, 5) {
                found.push(*pot_index);
                remove = Some(*pot_index);
                // dbg!(found.len(), &pot_index, *pot_value as char);
                if found.len() == 64 {
                    // found.sort();
                    // return Ok(found[found.len() - 1]);
                    last = Some(index + 1000);
                }
            }
        }
        if let Some(remove) = remove {
            potential.remove(&remove);
        }

        if let Some(n) = find_run(&hash.as_bytes(), 3) {
            potential.insert(index, n);
        }

        index += 1;
        if index >= 1000 {
            potential.remove(&(index - 1000));
        }
        if let Some(last) = last {
            // dbg!(index, last);
            if index > last {
                found.sort();
                return Ok(found[64 - 1]);
            }
        }
    }
}

fn part2(input: &str) -> Result<usize> {
    let mut index = 0;
    let mut potential: HashMap<usize, _> = HashMap::new();
    let mut found = vec![];
    let mut last = None;
    loop {
        let key = format!("{}{}", input, index);
        let hash = hash2016(&key);

        let mut remove = None;
        for (pot_index, pot_value) in &potential {
            if has_n_run(&hash.as_bytes(), pot_value, 5) {
                found.push(*pot_index);
                remove = Some(*pot_index);
                // dbg!(found.len(), &pot_index, *pot_value as char);
                if found.len() == 64 {
                    // found.sort();
                    // return Ok(found[found.len() - 1]);
                    last = Some(index + 1000);
                }
            }
        }
        if let Some(remove) = remove {
            potential.remove(&remove);
        }

        if let Some(n) = find_run(&hash.as_bytes(), 3) {
            potential.insert(index, n);
        }

        index += 1;
        if index >= 1000 {
            potential.remove(&(index - 1000));
        }
        if let Some(last) = last {
            if index > last {
                found.sort();
                return Ok(found[63]);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_run() {
        assert_eq!(find_run(&[1, 2, 3, 4], 2), None);
        assert_eq!(find_run(&[1, 2, 2, 4], 2), Some(2));
        assert_eq!(find_run(&[1, 2, 2, 4], 3), None);
        assert_eq!(find_run(&[1, 2, 2, 2], 3), Some(2));
    }

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1("abc")?, 22728);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2("abc")?, 22551);
        Ok(())
    }
}
