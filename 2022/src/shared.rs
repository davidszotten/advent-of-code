use anyhow::{bail, Context, Result};
use clap::{command, Arg};
use std::fmt::Display;
use std::fs::File;
use std::io::{self, Read};

enum Part {
    Part1,
    Part2,
}

enum Source {
    Stdin,
    File(String),
}

struct Args {
    part: Part,
    source: Source,
}

fn read_stdin() -> Result<String> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;
    Ok(buffer)
}

fn read_file(filename: &str) -> Result<String> {
    let mut buffer = String::new();
    let mut handle = File::open(filename)?;

    handle.read_to_string(&mut buffer)?;
    Ok(buffer)
}

fn parse_input() -> Result<Args> {
    let matches = command!()
        .name("adventofcode")
        .arg(
            Arg::new("part")
                .short('p')
                .default_value("1")
                .value_parser(["1", "2"])
        )
        .arg(
            Arg::new("input")
                .help("Sets the input file to use, or `-` for stdin. Leave out to assume `input/<binary name>`")
                .index(1),
        )
        .get_matches();

    let part = match matches
        .get_one::<String>("part")
        .expect("part has a default")
        .as_str()
    {
        "1" => Part::Part1,
        "2" => Part::Part2,
        _ => bail!("Invalid part"),
    };
    let source = match matches.get_one::<String>("input").map(String::as_str) {
        Some("-") => Source::Stdin,
        Some(filename) => Source::File(filename.into()),
        None => {
            let filename: String = std::env::current_exe()?
                .file_name()
                .context("invalid current exe filename")?
                .to_str()
                .context("current exe filename not a str?")?
                .into();
            Source::File(format!("input/{}", filename))
        }
    };
    Ok(Args { part, source })
}

type DayFunc<T> = fn(&str) -> Result<T>;

fn run<S, T>(part1: &DayFunc<S>, part2: &DayFunc<T>) -> Result<String>
where
    S: Display,
    T: Display,
{
    let args = parse_input()?;
    let input = match args.source {
        Source::Stdin => read_stdin(),
        Source::File(filename) => read_file(&filename),
    }?;
    match args.part {
        Part::Part1 => part1(&input).map(|res| format!("{}", res)),
        Part::Part2 => part2(&input).map(|res| format!("{}", res)),
    }
}

pub fn dispatch<S, T>(part1: DayFunc<S>, part2: DayFunc<T>) -> Result<()>
where
    S: Display,
    T: Display,
{
    let result = run(&part1, &part2)?;
    println!("{}", result);
    Ok(())
}
