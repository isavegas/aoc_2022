#[allow(unused_imports)]
use aoc_core::{bail, ensure, parse, AoCDay, ErrorWrapper};

pub struct Day06;

#[allow(dead_code)]
type Num = u64;

fn find_unique_series(s: &str, len: usize) -> Option<usize> {
    let mut buf = vec![];
    let mut sop = None;
    for i in 0..s.len() - len {
        buf.extend(s[i..i + len].chars());
        buf.sort();
        buf.dedup();
        if buf.len() == len {
            sop = Some(i + len);
            break;
        }
        buf = vec![];
    }
    sop
}

impl AoCDay for Day06 {
    fn day(&self) -> usize {
        06
    }
    fn expected(&self) -> (Option<&'static str>, Option<&'static str>) {
        (Some("1987"), Some("3059"))
    }
    fn part1(&self, input: &str) -> Result<String, ErrorWrapper> {
        Ok(find_unique_series(input, 4).unwrap().to_string())
    }
    fn part2(&self, input: &str) -> Result<String, ErrorWrapper> {
        Ok(find_unique_series(input, 14).unwrap().to_string())
    }
}

pub fn get_day() -> Box<dyn AoCDay> {
    Box::new(Day06)
}
