use anyhow::Result;
use aoc2015::dispatch;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

fn part1(input: &str) -> Result<usize> {
    let target: usize = input.parse()?;

    let mut houses = [0; 1_000_000];
    for elf in 1..houses.len() {
        let mut house = elf;
        while house < houses.len() {
            houses[house] += elf * 10;
            if houses[house] >= target {
                return Ok(house);
            }
            house += elf;
        }
    }
    Ok(0)
}

fn part2(input: &str) -> Result<usize> {
    let target: usize = input.parse()?;

    let mut houses = [0; 1_000_000];
    for elf in 1..houses.len() {
        let mut house = elf;
        let mut count = 0;
        while house < houses.len() && count < 50 {
            houses[house] += elf * 11;
            if houses[house] >= target {
                return Ok(house);
            }
            house += elf;
            count += 1;
        }
    }
    Ok(0)
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
