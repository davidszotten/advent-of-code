use aoc2018::{dispatch, Result};

fn main() {
    dispatch(&part1, &part2)
}

fn part1(_input: &str) -> Result<u32> {
    Ok(1)
}

fn part2(_input: &str) -> Result<u32> {
    Ok(2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() -> Result<()> {
        Ok(assert_eq!(part1("")?, 1))
    }
}
