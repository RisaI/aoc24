use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

use winnow::{
    ascii::{alpha1, digit1},
    PResult, Parser,
};

type Signal = u16;
type Wire = String;

enum Operation {
    Input(Signal),
    And(Wire, Wire),
    Or(Wire, Wire),
    LShift(Wire, u8),
    RShift(Wire, u8),
    Not(Wire),
}

fn parse_wire(input: &mut &str) -> PResult<Wire> {
    alpha1
        .verify(|v: &str| v.len() <= 2)
        .map(|v: &str| v.to_string())
        .parse_next(input)
}

fn parse_operation(input: &mut &str) -> PResult<Operation> {
    winnow::combinator::alt((
        (digit1.parse_to().map(Operation::Input)),
        (parse_wire, " AND ", parse_wire).map(|(a, _, b)| Operation::And(a, b)),
        (parse_wire, " OR ", parse_wire).map(|(a, _, b)| Operation::Or(a, b)),
        ("NOT ", parse_wire).map(|(_, a)| Operation::Not(a)),
        (parse_wire, " LSHIFT ", digit1.parse_to()).map(|(a, _, s)| Operation::LShift(a, s)),
        (parse_wire, " RSHIFT ", digit1.parse_to()).map(|(a, _, s)| Operation::RShift(a, s)),
    ))
    .parse_next(input)
}

fn main() -> anyhow::Result<()> {
    let reader = BufReader::new(File::open("data.txt")?);

    let mut signals = HashMap::<Wire, Signal>::default();

    for line in reader.lines() {
        let line = line?;

        if line.is_empty() {
            continue;
        }

        let (op, _, output) = (parse_operation, " -> ", parse_wire)
            .parse(&line)
            .map_err(|e| anyhow::format_err!("{e}"))?;

        let v = match op {
            Operation::Input(v) => v,
            Operation::And(a, b) => signals.get(&a).unwrap() & signals.get(&b).unwrap(),
            Operation::Or(a, b) => signals.get(&a).unwrap() | signals.get(&b).unwrap(),
            Operation::LShift(a, b) => signals.get(&a).unwrap() << b,
            Operation::RShift(a, b) => signals.get(&a).unwrap() >> b,
            Operation::Not(a) => !signals.get(&a).unwrap(),
        };

        signals.insert(output, v);
    }

    println!("a: {}", signals.get("a").unwrap());

    Ok(())
}
