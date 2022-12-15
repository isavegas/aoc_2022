#[allow(unused_imports)]
use aoc_core::{bail, ensure, parse, AoCDay, ErrorWrapper};

pub struct Day07;

impl AoCDay for Day07 {
    fn day(&self) -> usize {
        07
    }
    fn expected(&self) -> (Option<&'static str>, Option<&'static str>) {
        (None, None)
    }
    fn part1(&self, _input: &str) -> Result<String, ErrorWrapper> {
        Err(ErrorWrapper::NotImplemented)
    }
    fn part2(&self, _input: &str) -> Result<String, ErrorWrapper> {
        Err(ErrorWrapper::NotImplemented)
    }
}

pub fn get_day() -> Box<dyn AoCDay> {
    Box::new(Day07)
}
