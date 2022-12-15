#[allow(unused_imports)]
use aoc_core::{
    bail, ensure, parse, parse_lines, parse_lines_with, parse_with, AoCDay, ErrorWrapper,
};

use std::str::FromStr;

pub struct Day05;

type Stacks = Vec<Vec<char>>;

#[allow(dead_code)]
fn print_stacks(stacks: &Stacks) {
    for (i, s) in stacks.iter().enumerate() {
        println!("{} :: {}", i, s.iter().collect::<String>());
    }
}

fn parse_stacks(s: &str) -> Stacks {
    let mut stacks: Stacks = s
        .lines()
        .map(|l| {
            l.chars()
                .enumerate()
                .filter_map(|(i, c)| match i {
                    _ if i >= 1 && (i - 1) % 4 == 0 => Some(c),
                    _ => None,
                })
                .collect::<Vec<char>>()
        })
        .collect();
    let _ = stacks.pop(); // Remove number line
    stacks.reverse();
    let mut rotated: Stacks = vec![];
    for i in 0..stacks.iter().map(|s| s.len()).max().unwrap() {
        let mut new_vec = vec![];
        for v in &stacks {
            if let Some(c) = v.get(i) {
                if c != &' ' {
                    new_vec.push(*c);
                }
            }
        }
        rotated.push(new_vec);
    }
    rotated
}

#[allow(dead_code)]
struct Instruction {
    pub source: usize,
    pub amount: usize,
    pub destination: usize,
}

impl Instruction {
    fn apply(&self, stacks: &mut Stacks, enhanced: bool) -> Result<(), ErrorWrapper> {
        if enhanced {
            let source = &mut stacks[self.source];
            let mut temp = source.split_off(source.len() - self.amount);
            stacks[self.destination].append(&mut temp);
        } else {
            for _ in 0..self.amount {
                match stacks[self.source].pop() {
                    Some(c) => stacks[self.destination].push(c),
                    None => bail!(format!("Unable to apply `{self}`")),
                }
            }
        }
        Ok(())
    }
}

impl std::fmt::Display for Instruction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} ({}) -> {}",
            self.source, self.amount, self.destination,
        )
    }
}

impl FromStr for Instruction {
    type Err = ErrorWrapper;
    fn from_str(s: &str) -> Result<Instruction, ErrorWrapper> {
        let parts = s.split(' ').collect::<Vec<&str>>();
        Ok(Instruction {
            source: parts[3].parse::<usize>()? - 1,
            amount: parts[1].parse()?,
            destination: parts[5].parse::<usize>()? - 1,
        })
    }
}

fn parse_input(input: &str) -> (Stacks, Vec<Instruction>) {
    let mut parts = input.split("\n\n");
    (
        parse_stacks(parts.next().unwrap()),
        parse_lines(parts.next().unwrap()).unwrap(),
    )
}

impl AoCDay for Day05 {
    fn day(&self) -> usize {
        05
    }
    fn expected(&self) -> (Option<&'static str>, Option<&'static str>) {
        (Some("QGTHFZBHV"), Some("MGDMPSZTM"))
    }
    fn part1(&self, input: &str) -> Result<String, ErrorWrapper> {
        let (mut stacks, instructions) = parse_input(input);
        for i in instructions {
            i.apply(&mut stacks, false)?;
        }
        Ok(stacks.iter_mut().map(|s| s.pop().unwrap_or(' ')).collect())
    }
    fn part2(&self, input: &str) -> Result<String, ErrorWrapper> {
        let (mut stacks, instructions) = parse_input(input);
        for i in instructions {
            i.apply(&mut stacks, true)?;
        }
        Ok(stacks.iter_mut().map(|s| s.pop().unwrap_or(' ')).collect())
    }
}

pub fn get_day() -> Box<dyn AoCDay> {
    Box::new(Day05)
}
