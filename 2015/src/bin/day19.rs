use anyhow::{anyhow, Error, Result};
use aoc2015::dispatch;
use std::collections::HashSet;
use std::convert::TryFrom;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

struct Recipe<'a> {
    from: &'a str,
    to: &'a str,
}

#[derive(Debug)]
struct MatchIterator<'a> {
    haystack: &'a str,
    needle: &'a str,
    start: usize,
}

impl<'a> MatchIterator<'a> {
    fn new(haystack: &'a str, needle: &'a str) -> Self {
        MatchIterator {
            haystack,
            needle,
            start: 0,
        }
    }
}

impl<'a> Iterator for MatchIterator<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(offset) = self.haystack[self.start..].find(self.needle) {
            let next = self.start + offset;
            self.start = next + 1;
            Some(next)
        } else {
            None
        }
    }
}

impl<'a> Recipe<'a> {
    fn matches(&self, target: &'a str) -> MatchIterator {
        MatchIterator::new(target, self.from)
    }

    fn replace_at(&self, target: &str, pos: usize) -> String {
        let mut s = target.to_string();
        s.replace_range(pos..pos + self.from.len(), self.to);
        s
    }

    fn matches_rev(&self, target: &'a str) -> MatchIterator {
        MatchIterator {
            haystack: target,
            needle: self.to,
            start: 0,
        }
    }

    fn replace_at_rev(&self, target: &str, pos: usize) -> String {
        let mut s = target.to_string();
        s.replace_range(pos..pos + self.to.len(), self.from);
        s
    }
}

impl<'a> TryFrom<&'a str> for Recipe<'a> {
    type Error = Error;
    fn try_from(s: &'a str) -> Result<Self> {
        let mut parts = s.split(" => ");
        let from = parts.next().ok_or(anyhow!("no first part"))?;
        let to = parts.next().ok_or(anyhow!("no second part"))?;
        Ok(Recipe { from, to })
    }
}

fn parse(input: &str) -> Result<(Vec<Recipe>, &str)> {
    let mut sections = input.split("\n\n");
    let recipes = sections
        .next()
        .ok_or(anyhow!("split has at least one part"))?;
    let molecule = sections.next().ok_or(anyhow!("should have 2 sections"))?;
    let recipes = recipes
        .split('\n')
        .map(Recipe::try_from)
        .collect::<Result<Vec<_>>>()?;
    Ok((recipes, molecule))
}

fn part1(input: &str) -> Result<usize> {
    let (recipes, molecule) = parse(input)?;
    let mut results = HashSet::new();
    for recipe in recipes {
        for pos in recipe.matches(molecule) {
            results.insert(recipe.replace_at(molecule, pos));
        }
    }
    Ok(results.len())
}

fn round(steps: usize, start: &str, recipes: &[Recipe]) -> Option<usize> {
    for recipe in recipes.iter() {
        for pos in recipe.matches_rev(start) {
            let next = recipe.replace_at_rev(start, pos);
            if next == "e" {
                return Some(steps);
            }
            if let Some(recurse) = round(steps + 1, &next, recipes) {
                return Some(recurse);
            }
        }
    }
    None
}

fn part2(input: &str) -> Result<usize> {
    let (mut recipes, molecule) = parse(input)?;
    recipes.sort_by_key(|r| -(r.to.len() as i32));

    let steps = round(1, molecule, &recipes).ok_or(anyhow!("failed"))?;
    Ok(steps)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "H => HO
H => OH
O => HH

HOH";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(INPUT)?, 4);
        assert_eq!(
            part1(
                "H => HO
H => OH
O => HH

HOHOHO"
            )?,
            7
        );
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(
            part2(
                "e => H
e => O
H => HO
H => OH
O => HH

HOH"
            )?,
            3
        );
        Ok(())
    }

    #[test]
    fn test_matches() {
        let recipe = Recipe {
            from: "H",
            to: "HO",
        };
        assert_eq!(recipe.matches("HOH").collect::<Vec<_>>(), vec![0, 2]);
    }

    #[test]
    fn test_replace_at() {
        let recipe = Recipe {
            from: "H",
            to: "HO",
        };
        assert_eq!(recipe.replace_at("HOH", 0), "HOOH".to_string());
        assert_eq!(recipe.replace_at("HOH", 2), "HOHO".to_string());
    }
}
