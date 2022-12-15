#[allow(unused_imports)]
use aoc_core::{AoCDay, ErrorWrapper, parse, bail, ensure};

pub struct Day@DAY@;

#[allow(dead_code)]
type Num = u64;

impl AoCDay for Day@DAY@ {
    fn day(&self) -> usize {
        @DAY@
    }
    fn expected(&self) -> (Option<&'static str>, Option<&'static str>) {
        (None, None)
    }
    #[allow(unused_variables)]
    fn part1(&self, input: &str) -> Result<String, ErrorWrapper> {
        Err(ErrorWrapper::NotImplemented)
    }
    #[allow(unused_variables)]
    fn part2(&self, input: &str) -> Result<String, ErrorWrapper> {
        Err(ErrorWrapper::NotImplemented)
    }
}

pub fn get_day() -> Box<dyn AoCDay> {
    Box::new(Day@DAY@)
}
