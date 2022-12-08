use aoc_core::{bail, parse_lines_with, AoCDay, ErrorWrapper};

pub struct Day03;

type Num = u64;

// A: 65 ... Z: 90
// a: 97 ... z: 122

#[derive(Debug, Clone)]
struct Rucksack {
    pub a: Vec<char>,
    pub b: Vec<char>,
}

fn value_of(c: &char) -> usize {
    let u = *c as usize;
    match u {
        _ if u >= 65 && u <= 90 => (u - 64) + 26,
        _ if u >= 97 && u <= 122 => u - 96,
        _ => 0,
    }
}

fn parse_rucksack(line: &str) -> Result<Rucksack, ErrorWrapper> {
    if line.len() % 2 != 0 {
        bail!("Invalid input!");
    }
    let middle = line.len() / 2;
    let (a, b) = line.split_at(middle);
    Ok(Rucksack {
        a: a.chars().collect(),
        b: b.chars().collect(),
    })
}

impl AoCDay for Day03 {
    fn day(&self) -> usize {
        03
    }
    fn expected(&self) -> (Option<&'static str>, Option<&'static str>) {
        (Some("7785"), None)
    }
    fn part1(&self, input: &str) -> Result<String, ErrorWrapper> {
        let mut rucksacks = parse_lines_with(input, parse_rucksack)?;
        let mut score = 0;
        for sack in &mut rucksacks {
            sack.a.sort();
            sack.a.dedup();
            sack.b.sort();
            sack.b.dedup();
            let mut total = sack.a.clone();
            total.extend(sack.b.clone());
            total.sort();
            let mut last = '\0';
            for c in &total {
                if last == *c {
                    score += value_of(c) as Num;
                }
                last = *c;
            }
        }
        Ok(score.to_string())
    }
    fn part2(&self, _input: &str) -> Result<String, ErrorWrapper> {
        Err(ErrorWrapper::NotImplemented)
    }
}

pub fn get_day() -> Box<dyn AoCDay> {
    Box::new(Day03)
}
