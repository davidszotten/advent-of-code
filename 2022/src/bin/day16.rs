use anyhow::{Context, Error, Result};
use aoc2022::dispatch;
use std::collections::{BTreeSet, HashMap, HashSet, VecDeque};

fn main() -> Result<()> {
    dispatch(part1, part2)
}

#[derive(Debug)]
struct Reading<'a> {
    valve: &'a str,
    rate: i64,
    tunnels: Vec<&'a str>,
}

impl<'a> TryFrom<&'a str> for Reading<'a> {
    type Error = Error;

    fn try_from(s: &'a str) -> Result<Self, Self::Error> {
        // Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
        let s = s
            .strip_prefix("Valve ")
            .context(format!("start missing `{}`", s))?;
        let (valve, s) = s.split_once(' ').context("valve missing")?;
        let s = s.strip_prefix("has flow rate=").context("flow missing")?;
        let (raw_rate, s) = if s.contains("tunnels") {
            s.split_once("; tunnels lead to valves ")
                .context("rate missing")?
        } else {
            s.split_once("; tunnel leads to valve ")
                .context("rate missing")?
        };
        let tunnels = s.split(", ").collect();
        Ok(Reading {
            valve,
            rate: raw_rate.parse()?,
            tunnels,
        })
    }
}

#[derive(Debug, Clone)]
struct State<'a> {
    pos: &'a str,
    steps: usize,
    visited: HashSet<&'a str>,
    open: HashSet<&'a str>,
    released: i64,
    rates: Vec<(usize, i64)>,
    path: Vec<&'a str>,
}

impl State<'_> {
    fn flow_rate(&self, map: &HashMap<&str, &Reading>) -> i64 {
        self.open.iter().map(|s| map.get(s).unwrap().rate).sum()
    }

    fn total(&self) -> i64 {
        let mut prev = 0;
        let mut last = 0;
        let mut total = 0;
        for &(step, rate) in &self.rates {
            total += (step - prev) as i64 * rate;
            prev = step;
            last = rate;
        }
        total += (29 - prev) as i64 * last;
        total
    }
}

fn shortest_paths<'a>(
    readings: &[Reading<'a>],
) -> HashMap<&'a str, HashMap<&'a str, (Vec<&'a str>, usize)>> {
    let mut paths: HashMap<&'a str, HashMap<&'a str, (Vec<&'a str>, usize)>> = HashMap::new();
    let map = readings
        .iter()
        .map(|r| (r.valve, r))
        .collect::<HashMap<&str, _>>();

    let endpoints = readings
        .iter()
        .filter(|r| r.valve == "AA" || r.rate > 0)
        .map(|r| r.valve)
        .collect::<Vec<_>>();

    for &start in &endpoints {
        'pair: for &end in &endpoints {
            if start == end {
                continue;
            }
            if paths.get(&start).and_then(|m| m.get(&end)).is_some() {
                continue;
            }
            let mut queue = VecDeque::from([(start, vec![], 0)]);
            let mut seen = HashSet::new();
            while let Some((next, passed, distance)) = queue.pop_front() {
                let location = map.get(next).unwrap();
                for &tunnel in &location.tunnels {
                    if tunnel == end {
                        paths
                            .entry(start)
                            .or_default()
                            .insert(end, (passed.clone(), distance + 1));
                        paths
                            .entry(end)
                            .or_default()
                            .insert(start, (passed.clone(), distance + 1));
                        continue 'pair;
                    }
                    if seen.contains(&tunnel) {
                        continue;
                    }
                    seen.insert(tunnel);
                    let mut passed = passed.clone();
                    if map.get(&tunnel).unwrap().rate > 0 {
                        passed.push(tunnel);
                    }
                    queue.push_back((tunnel, passed, distance + 1));
                }
            }
        }
    }
    paths
}

fn _shortest_direct_paths<'a>(
    readings: &[Reading<'a>],
) -> HashMap<&'a str, HashMap<&'a str, usize>> {
    let mut paths: HashMap<&'a str, HashMap<&'a str, usize>> = HashMap::new();
    let map = readings
        .iter()
        .map(|r| (r.valve, r))
        .collect::<HashMap<&str, _>>();

    let endpoints = readings
        .iter()
        .filter(|r| r.valve == "AA" || r.rate > 0)
        .map(|r| r.valve)
        .collect::<Vec<_>>();

    for &start in &endpoints {
        'pair: for &end in &endpoints {
            if start == end {
                continue;
            }
            if paths.get(&start).and_then(|m| m.get(&end)).is_some() {
                continue;
            }
            let mut queue = VecDeque::from([(start, 0)]);
            let mut seen = HashSet::new();
            while let Some((next, distance)) = queue.pop_front() {
                let location = map.get(next).unwrap();
                for &tunnel in &location.tunnels {
                    if tunnel == end {
                        paths.entry(start).or_default().insert(end, distance + 1);
                        paths.entry(end).or_default().insert(start, distance + 1);
                        continue 'pair;
                    }
                    if seen.contains(&tunnel) {
                        continue;
                    }
                    if endpoints.contains(&tunnel) {
                        continue;
                    }
                    seen.insert(tunnel);
                    if let Some(dst) = map.get(&tunnel) {
                        if dst.rate > 0 {
                            continue;
                        }
                    }
                    queue.push_back((tunnel, distance + 1));
                }
            }
        }
    }
    paths
}

fn part1(input: &str) -> Result<i64> {
    let readings = input
        .split('\n')
        .map(|l| l.try_into())
        .collect::<Result<Vec<Reading>>>()?;
    let paths = shortest_paths(&readings);
    let map = readings
        .iter()
        .map(|r| (r.valve, r))
        .collect::<HashMap<&str, _>>();
    let start = State {
        pos: "AA",
        steps: 0,
        visited: HashSet::from(["AA"]),
        open: HashSet::new(),
        released: 0,
        rates: vec![],
        path: vec![],
    };
    let mut best = 0;
    let mut queue = VecDeque::from([start]);
    while let Some(state) = queue.pop_front() {
        let location = map.get(state.pos).unwrap();
        if state.total() > best {
            best = best.max(state.total());
        }
        if state.path.len() > 30 {
            continue;
        };
        if !state.open.contains(state.pos) && location.rate > 0 {
            let mut new_state = state.clone();
            new_state.steps += 1;
            new_state.open.insert(state.pos);
            new_state.released += state.flow_rate(&map);
            new_state.path.push(state.pos);
            new_state
                .rates
                .push((new_state.steps, new_state.flow_rate(&map)));
            queue.push_back(new_state);
            continue;
        }
        for (tunnel, (_, distance)) in paths.get(state.pos).unwrap() {
            if state.visited.contains(tunnel) {
                continue;
            }
            let mut new_state = state.clone();
            new_state.pos = tunnel;
            new_state.visited.insert(tunnel);
            new_state.steps += distance;
            if new_state.steps >= 30 {
                continue;
            }
            new_state.released += state.flow_rate(&map);
            new_state.path.push(tunnel);
            new_state
                .rates
                .push((new_state.steps, new_state.flow_rate(&map)));
            queue.push_back(new_state);
        }
    }
    Ok(best)
}

fn _add_opens<'a>(path: &'_ Vec<(&'a str, usize)>) -> Vec<Vec<(&'a str, usize)>> {
    let mut counts: HashMap<&str, usize> = HashMap::new();
    let mut paths: Vec<Vec<(&'a str, usize)>> = vec![vec![path[0]]];
    for (n, _) in path {
        if n == &"AA" {
            continue;
        }
        *counts.entry(n).or_default() += 1;
    }
    for entry in &path[1..] {
        if *counts.get(&entry.0).unwrap_or(&0) > 1 {
            let mut new = vec![];
            for option1 in &mut paths {
                let mut option2 = option1.clone();
                option1.push(*entry);
                option1.push((" ", 1));
                option2.push(*entry);
                // NB: no ""
                new.push(option2);
            }
            paths.extend(new);
        } else {
            for option in &mut paths {
                option.push(*entry);
                if entry.0 != "AA" {
                    option.push((" ", 1));
                }
            }
        }
    }
    paths
}

fn add_opens2<'a>(
    path: &[(&'a str, usize)],
    other_path: &'_ Vec<(&'a str, usize)>,
) -> Vec<Vec<(&'a str, usize)>> {
    let mut counts: HashMap<&str, usize> = HashMap::new();
    let mut paths: Vec<Vec<(&'a str, usize)>> = vec![vec![path[0]]];
    for (n, _) in path.iter().chain(other_path) {
        if n == &"AA" {
            continue;
        }
        *counts.entry(n).or_default() += 1;
    }
    for entry in &path[1..] {
        if *counts.get(&entry.0).unwrap_or(&0) > 1 {
            let mut new = vec![];
            for option1 in &mut paths {
                let mut option2 = option1.clone();
                option1.push(*entry);
                option1.push((" ", 1));
                option2.push(*entry);
                // NB: no ""
                new.push(option2);
            }
            paths.extend(new);
        } else {
            for option in &mut paths {
                option.push(*entry);
                if entry.0 != "AA" {
                    option.push((" ", 1));
                }
            }
        }
    }
    paths
}

fn part2(input: &str) -> Result<i64> {
    let readings = input
        .split('\n')
        .map(|l| l.try_into())
        .collect::<Result<Vec<Reading>>>()?;
    let map = readings
        .iter()
        .map(|r| (r.valve, r))
        .collect::<HashMap<&str, _>>();
    let paths = _shortest_direct_paths(&readings);

    // a1, a2, visited, path
    let mut options = vec![];
    let start = (
        "AA",
        "AA",
        BTreeSet::<&str>::from(["AA"]),
        vec![("AA", 0)],
        vec![("AA", 0)],
    );
    let mut seen = HashSet::new();
    let mut queue = VecDeque::from([start]);
    let all = paths.keys().cloned().collect::<BTreeSet<_>>();
    while let Some((a1, a2, visited, path1, path2)) = queue.pop_front() {
        if visited == all {
            options.push((path1, path2));
            continue;
        }
        let location1 = paths.get(a1).unwrap();
        let location2 = paths.get(a2).unwrap();
        for (&tunnel1, &d1) in location1.iter() {
            for (&tunnel2, &d2) in location2.iter() {
                if seen.contains(&(visited.clone(), tunnel1, tunnel2))
                    || seen.contains(&(visited.clone(), tunnel2, tunnel1))
                {
                    continue;
                }
                seen.insert((visited.clone(), tunnel1, tunnel2));
                let mut path1 = path1.clone();
                path1.push((tunnel1, d1));
                let mut path2 = path2.clone();
                path2.push((tunnel2, d2));
                let mut visited = visited.clone();
                visited.insert(tunnel1);
                visited.insert(tunnel2);
                queue.push_back((tunnel1, tunnel2, visited, path1, path2));
            }
        }
    }

    let mut possible = vec![];
    for (path1, path2) in options {
        let paths1 = add_opens2(&path1, &path2);
        let paths2 = add_opens2(&path2, &path1);
        for option1 in &paths1 {
            let _label1 = &option1.iter().map(|(v, _)| v).cloned().collect::<String>();
            for option2 in &paths2 {
                let _label2 = &option2.iter().map(|(v, _)| v).cloned().collect::<String>();
                let mut rates = [0; 26];
                let mut open = HashSet::new();
                for player in [option1, option2] {
                    let mut time = 1;
                    let mut rate = 0;
                    let mut prev_time = 0;
                    let mut prev_pos = "AA";
                    for (pos, dist) in player {
                        time += dist;
                        rate += if let Some(location1) = map.get(prev_pos) {
                            if pos == &" " && !open.contains(&prev_pos) {
                                open.insert(prev_pos);
                                location1.rate
                            } else {
                                0
                            }
                        } else {
                            0
                        };
                        if prev_time >= 26 {
                            break;
                        }
                        for entry in rates.iter_mut().take(time.min(26)).skip(prev_time) {
                            *entry += rate;
                        }
                        prev_time = time;
                        prev_pos = pos;
                    }
                    for entry in rates.iter_mut().skip(prev_time) {
                        *entry += rate;
                    }
                }
                possible.push(rates.iter().sum());
            }
        }
    }

    Ok(*possible.iter().max().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(TEST_INPUT)?, 1651);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        assert_eq!(part2(TEST_INPUT)?, 1707);
        Ok(())
    }
}
