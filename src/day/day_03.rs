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

fn value_of(c: &char) -> Num {
    let u = *c as Num;
    match u {
        _ if (65..=90).contains(&u) => (u - 64) + 26,
        _ if (97..=122).contains(&u) => u - 96,
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

fn common_chars(vec_a: &[char], vec_b: &[char]) -> Vec<char> {
    let mut a = vec_a.to_owned();
    let mut b = vec_b.to_owned();
    a.sort();
    b.sort();
    a.dedup();
    b.dedup();
    a.extend(b);
    a.sort();
    let mut out = vec![];
    let mut last = &'\0';
    for c in &a {
        if last == c {
            out.push(*c);
        }
        last = c;
    }
    out
}

impl AoCDay for Day03 {
    fn day(&self) -> usize {
        03
    }
    fn expected(&self) -> (Option<&'static str>, Option<&'static str>) {
        (Some("7785"), Some("2633"))
    }
    fn part1(&self, input: &str) -> Result<String, ErrorWrapper> {
        let mut rucksacks = parse_lines_with(input, parse_rucksack)?;
        let mut score = 0;
        for sack in &mut rucksacks {
            score += value_of(common_chars(&sack.a, &sack.b).first().unwrap_or(&'\0'));
        }
        Ok(score.to_string())
    }
    fn part2(&self, input: &str) -> Result<String, ErrorWrapper> {
        Ok(input
            .lines()
            .array_chunks::<3>()
            .map(|[a, b, c]| {
                let common_a_b = common_chars(
                    &a.chars().collect::<Vec<char>>(),
                    &b.chars().collect::<Vec<char>>(),
                );
                let common_all = common_chars(&common_a_b, &c.chars().collect::<Vec<char>>());
                value_of(common_all.first().unwrap_or(&'\0'))
            })
            .sum::<Num>()
            .to_string())
    }
}

pub fn get_day() -> Box<dyn AoCDay> {
    Box::new(Day03)
}
