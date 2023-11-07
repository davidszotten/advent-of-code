use anyhow::{Context, Result};
use aoc2015::dispatch;
use serde_json::Value;

fn main() -> Result<()> {
    dispatch(part1, part2)
}

// fn has_red_key(value: &Value) -> bool {
// match value {
// serde_json::value::Value::Object(m) => m.keys().any(|k| k == "red"),
// _ => false,
// }
// }
//         _ => false,
// fn has_red_child(value: &Value) -> bool {
//     use serde_json::value::Value::*;
//     match value {
//         Array(v) => v.iter().any(has_red_child),
//         Object(m) => m.keys().any(|k| k == "red") || m.values().any(has_red_child),
//         _ => false,
//     }
// }

fn sum_unless_red(value: &Value) -> Result<i64> {
    use serde_json::value::Value::*;
    Ok(match value {
        Number(n) => n.as_i64().context("not an i64")?,
        Array(v) => v.iter().map(sum_unless_red).sum::<Result<_>>()?,
        Object(m) => {
            if m.values().any(|v| v == "red") {
                0
            } else {
                m.values().map(sum_unless_red).sum::<Result<_>>()?
            }
        }
        _ => 0,
    })
}

fn sum(value: &Value) -> Result<i64> {
    use serde_json::value::Value::*;
    Ok(match value {
        Number(n) => n.as_i64().context("not an i64")?,
        Array(v) => v.iter().map(sum).sum::<Result<_>>()?,
        Object(m) => m.values().map(sum).sum::<Result<_>>()?,
        _ => 0,
    })
}

fn part1(input: &str) -> Result<i64> {
    let value: Value = serde_json::from_str(input)?;
    sum(&value)
}

fn part2(input: &str) -> Result<i64> {
    let value: Value = serde_json::from_str(input)?;
    sum_unless_red(&value)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "{}";

    #[test]
    fn test_part1() -> Result<()> {
        assert_eq!(part1(INPUT)?, 0);
        assert_eq!(part1("[1,2,3]")?, 6);
        assert_eq!(part1(r#"{"a":{"b":4},"c":-1}"#)?, 3);
        Ok(())
    }

    #[test]
    fn test_part2() -> Result<()> {
        // assert_eq!(part2(INPUT)?, 0);
        // assert_eq!(part2("[1,2,3]")?, 6);
        assert_eq!(part2(r#"[1,{"c":"red","b":2},3]"#)?, 4);
        Ok(())
    }
}
