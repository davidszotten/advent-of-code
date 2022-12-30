use anyhow::{Context, Result};
use aoc2022::dispatch;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

fn parse(input: &str) -> Result<Vec<i64>> {
    input
        .split('\n')
        .map(|l| l.parse().context("failed to parse int"))
        .collect()
}

fn part1(input: &str) -> Result<i64> {
    let values = parse(input)?;
    let len = values.len() as i64;
    let mut indexes: Vec<usize> = (0..values.len()).collect();
    for (index, value) in values.iter().enumerate() {
        let pos = indexes
            .iter()
            .position(|&i| i == index)
            .context("index not found")?;
        let mut new_pos: i64 = pos as i64 + value;
        new_pos = new_pos.rem_euclid(len - 1);
        let new_pos = new_pos as usize;
        indexes.remove(pos);
        indexes.insert(new_pos, index);
    }
    let pos0_original = values
        .iter()
        .position(|&i| i == 0)
        .context("0 pos not found")?;
    let pos0_final = indexes
        .iter()
        .position(|&i| i == pos0_original)
        .context("final 0 pos not found")?;
    Ok([1000, 2000, 3000]
        .iter()
        .map(|i| (values[indexes[((i + pos0_final) % values.len())]]))
        .sum())
}

fn part2(input: &str) -> Result<i64> {
    let values = parse(input)?;
    let values: Vec<_> = values.iter().map(|n| n * 811589153).collect();
    let len = values.len() as i64;
    let mut indexes: Vec<usize> = (0..values.len()).collect();
    for _ in 0..10 {
        for (index, value) in values.iter().enumerate() {
            let pos = indexes
                .iter()
                .position(|&i| i == index)
                .context("pos not found")?;
            let mut new_pos: i64 = pos as i64 + value;

            new_pos = new_pos.rem_euclid(len - 1);

            let new_pos = new_pos as usize;
            indexes.remove(pos);
            indexes.insert(new_pos, index);
        }
    }
    let pos0_original = values
        .iter()
        .position(|&i| i == 0)
        .context("original 0 pos not found")?;
    let pos0_final = indexes
        .iter()
        .position(|&i| i == pos0_original)
        .context("final 0 pos not found")?;
    Ok([1000, 2000, 3000]
        .iter()
        .map(|i| (values[indexes[((i + pos0_final) % values.len())]]))
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "1
2
-3
3
-2
0
4";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(TEST_INPUT)?, 3);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(TEST_INPUT)?, 1623178306);
        Ok(())
    }
}
