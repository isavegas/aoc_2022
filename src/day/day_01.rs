use aoc_core::{parse_lines, parse_with, AoCDay, ErrorWrapper};

pub struct Day01;

type Num = u64;

impl AoCDay for Day01 {
    fn day(&self) -> usize {
        01
    }
    fn expected(&self) -> (Option<&'static str>, Option<&'static str>) {
        (Some("72070"), Some("211805"))
    }
    fn part1(&self, input: &str) -> Result<String, ErrorWrapper> {
        let values: Vec<Vec<Num>> = parse_with(input, "\n\n", |value| parse_lines(value))?;
        Ok(values
            .iter()
            .map(|elf| elf.iter().sum::<Num>())
            .max()
            .unwrap()
            .to_string())
    }
    fn part2(&self, input: &str) -> Result<String, ErrorWrapper> {
        let values: Vec<Vec<Num>> = parse_with(input, "\n\n", |value| parse_lines(value))?;
        let mut elves: Vec<Num> = values.iter().map(|elf| elf.iter().sum::<Num>()).collect();
        elves.sort();
        Ok(elves.iter().rev().take(3).sum::<Num>().to_string())
    }
}

pub fn get_day() -> Box<dyn AoCDay> {
    Box::new(Day01)
}
