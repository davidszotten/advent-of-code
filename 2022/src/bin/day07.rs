use anyhow::{Context, Result};
use aoc2022::dispatch;
use std::collections::VecDeque;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

#[derive(Debug)]
enum Entry<'a> {
    Dir(&'a str, Vec<Entry<'a>>),
    File(&'a str, usize),
}

fn parse<'a>(dirname: &'a str, input: &mut impl Iterator<Item = &'a str>) -> Result<Entry<'a>> {
    let mut contents = vec![];

    assert_eq!(input.next(), Some("$ ls"));

    while let Some(line) = input.next() {
        if line == "$ cd .." {
            break;
        }
        if &line[..5] == "$ cd " {
            let dir = &line[5..];
            contents.push(parse(dir, input)?);
        } else if &line[..4] == "dir " {
        } else {
            // file
            let (raw_size, filename) = line.split_once(' ').context("malformed file line")?;
            contents.push(Entry::File(filename, raw_size.parse()?));
        }
    }
    Ok(Entry::Dir(dirname, contents))
}

fn sizes1(entries: Vec<Entry>) -> usize {
    let mut sum = 0;
    let mut entries = entries.iter().collect::<VecDeque<_>>();
    while let Some(entry) = entries.pop_front() {
        if let Entry::Dir(_, ref contents) = entry {
            let partial = size(entry);
            if partial <= 100000 {
                sum += partial;
            }
            for sub in contents {
                entries.push_back(sub)
            }
        }
    }
    sum
}

fn sizes2(total: usize, entries: Vec<Entry>) -> Result<usize> {
    let mut sizes = vec![];
    let mut entries = entries.iter().collect::<VecDeque<_>>();
    while let Some(entry) = entries.pop_front() {
        if let Entry::Dir(_, ref contents) = entry {
            let partial = size(entry);
            sizes.push(partial);
            for sub in contents {
                entries.push_back(sub)
            }
        }
    }
    sizes.sort();
    let disk = 70000000;
    let required = 30000000;
    sizes
        .into_iter()
        .find(|s| required <= disk - total + s)
        .context("no dir found")
}

fn size(entry: &Entry) -> usize {
    match entry {
        Entry::File(_, size) => *size,
        Entry::Dir(_, entries) => entries.iter().map(size).sum(),
    }
}

fn part1(input: &str) -> Result<usize> {
    let mut lines = input.split('\n');
    lines.next(); // cd /
    let tree = parse("/", &mut lines)?;
    Ok(sizes1(vec![tree]))
}

fn part2(input: &str) -> Result<usize> {
    let mut lines = input.split('\n');
    lines.next(); // cd /
    let tree = parse("/", &mut lines)?;
    sizes2(size(&tree), vec![tree])
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(TEST_INPUT)?, 95437);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(TEST_INPUT)?, 24933642);
        Ok(())
    }
}
