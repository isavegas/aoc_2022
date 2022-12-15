use aoc_core::{bail, parse_lines_with, parse_with, AoCDay, ErrorWrapper};
use std::ops::Range;

pub struct Day04;

type Num = u64;

fn fully_contains((a, b): &(Range<Num>, Range<Num>)) -> bool {
    (a.start >= b.start && a.end <= b.end) || (a.start <= b.start && a.end >= b.end)
}

fn overlaps((a, b): &(Range<Num>, Range<Num>)) -> bool {
    (a.end >= b.start && a.start <= b.end) || (a.end <= b.start && a.start >= b.end)
}

fn parse_range(s: &str) -> Result<Range<Num>, ErrorWrapper> {
    if let [start, end] = s
        .split('-')
        .map(|p| p.parse().unwrap())
        .collect::<Vec<Num>>()[..]
    {
        Ok(Range { start, end })
    } else {
        bail!("Invalid range")
    }
}

fn parse_pair(line: &str) -> Result<(Range<Num>, Range<Num>), ErrorWrapper> {
    match &parse_with(line, ",", parse_range)?[..] {
        [a, b, ..] => Ok((a.clone(), b.clone())),
        _ => bail!("Invalid length"),
    }
}

impl AoCDay for Day04 {
    fn day(&self) -> usize {
        04
    }
    fn expected(&self) -> (Option<&'static str>, Option<&'static str>) {
        (Some("485"), Some("857"))
    }
    fn part1(&self, input: &str) -> Result<String, ErrorWrapper> {
        let pairs: Vec<(Range<Num>, Range<Num>)> = parse_lines_with(input, parse_pair)?;
        Ok(pairs
            .iter()
            .filter(|p| fully_contains(p))
            .count()
            .to_string())
    }
    fn part2(&self, input: &str) -> Result<String, ErrorWrapper> {
        let pairs: Vec<(Range<Num>, Range<Num>)> = parse_lines_with(input, parse_pair)?;
        Ok(pairs.iter().filter(|p| overlaps(p)).count().to_string())
    }
}

pub fn get_day() -> Box<dyn AoCDay> {
    Box::new(Day04)
}
