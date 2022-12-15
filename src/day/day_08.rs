#[allow(unused_imports)]
use aoc_core::{bail, AoCDay, ErrorWrapper};

pub struct Day08;

type Grid = Vec<Vec<usize>>;

impl AoCDay for Day08 {
    fn day(&self) -> usize {
        08
    }
    fn expected(&self) -> (Option<&'static str>, Option<&'static str>) {
        (Some("1543"), Some("595080"))
    }
    fn part1(&self, input: &str) -> Result<String, ErrorWrapper> {
        let grid: Grid = input
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect::<Vec<usize>>()
            })
            .collect::<Grid>();
        let grid_height = grid.len();
        let grid_width = grid[0].len();
        let mut count = (grid_height * 2) + ((grid_width - 2) * 2);
        for y in 1..grid_height - 1 {
            for x in 1..grid_width - 1 {
                let height = grid[y][x];
                let shorter = |other: &usize| other < &height;
                if grid[y][x + 1..].iter().all(shorter)
                    || grid[y][..x].iter().all(shorter)
                    || grid[y + 1..].iter().map(|v| &v[x]).all(shorter)
                    || grid[..y].iter().map(|v| &v[x]).all(shorter)
                {
                    count += 1;
                }
            }
        }
        Ok(count.to_string())
    }
    fn part2(&self, input: &str) -> Result<String, ErrorWrapper> {
        let grid: Grid = input
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect::<Vec<usize>>()
            })
            .collect::<Grid>();
        let grid_height = grid.len();
        let grid_width = grid[0].len();
        let mut high_score = 0;
        for y in 0..grid_height {
            for x in 0..grid_width {
                let height = grid[y][x];
                let mut right = 0;
                for h in grid[y][x + 1..].iter() {
                    right += 1;
                    if h >= &height {
                        break;
                    }
                }
                let mut left = 0;
                for h in grid[y][..x].iter().rev() {
                    left += 1;
                    if h >= &height {
                        break;
                    }
                }
                let mut down = 0;
                for h in grid[y + 1..].iter().map(|v| &v[x]) {
                    down += 1;
                    if h >= &height {
                        break;
                    }
                }
                let mut up = 0;
                for h in grid[..y].iter().map(|v| &v[x]).rev() {
                    up += 1;
                    if h >= &height {
                        break;
                    }
                }
                let score = left * right * down * up;
                if score > high_score {
                    high_score = score;
                }
            }
        }
        Ok(high_score.to_string())
    }
}

pub fn get_day() -> Box<dyn AoCDay> {
    Box::new(Day08)
}
