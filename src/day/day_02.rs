use aoc_core::{bail, parse_lines_with, AoCDay, ErrorWrapper};
use std::convert::TryFrom;

pub struct Day02;

type Num = u64;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Play {
    Win,
    Lose,
    Draw,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl Shape {
    fn value(&self) -> Num {
        match *self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }
    fn wins_against(&self) -> Self {
        match *self {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper,
        }
    }
    fn loses_to(&self) -> Self {
        match *self {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock,
        }
    }
}

impl Play {
    fn apply(&self, opponent: &Shape) -> Shape {
        match *self {
            Self::Win => opponent.loses_to(),
            Self::Lose => opponent.wins_against(),
            Self::Draw => *opponent,
        }
    }
}

impl TryFrom<char> for Shape {
    type Error = ErrorWrapper;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'A' | 'X' => Ok(Self::Rock),
            'B' | 'Y' => Ok(Self::Paper),
            'C' | 'Z' => Ok(Self::Scissors),
            _ => Err(ErrorWrapper::ParseError(format!("Unknown shape: {value}"))),
        }
    }
}

impl TryFrom<char> for Play {
    type Error = ErrorWrapper;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' => Ok(Self::Lose),
            'Y' => Ok(Self::Draw),
            'Z' => Ok(Self::Win),
            _ => Err(ErrorWrapper::ParseError(format!("Unknown shape: {value}"))),
        }
    }
}

fn score(opponent: &Shape, player: &Shape) -> Num {
    if player == opponent {
        return 3;
    }
    if (player == &Shape::Rock && opponent == &Shape::Scissors)
        || (player == &Shape::Paper && opponent == &Shape::Rock)
        || (player == &Shape::Scissors && opponent == &Shape::Paper)
    {
        6
    } else {
        0
    }
}

fn parse_match(line: &str) -> Result<(Shape, Shape), ErrorWrapper> {
    if line.trim().len() != 3 {
        bail!("Bad input line!");
    }
    // line is guaranteed to have exactly 3 chars in it
    Ok((
        Shape::try_from(line.chars().next().unwrap())?,
        Shape::try_from(line.chars().nth(2).unwrap())?,
    ))
}

fn parse_strategy(line: &str) -> Result<(Shape, Play), ErrorWrapper> {
    if line.trim().len() != 3 {
        bail!("Bad input line!");
    }
    // line is guaranteed to have exactly 3 chars in it
    Ok((
        Shape::try_from(line.chars().next().unwrap())?,
        Play::try_from(line.chars().nth(2).unwrap())?,
    ))
}

impl AoCDay for Day02 {
    fn day(&self) -> usize {
        2
    }
    fn expected(&self) -> (Option<&'static str>, Option<&'static str>) {
        (Some("12276"), Some("9975"))
    }
    fn part1(&self, input: &str) -> Result<String, ErrorWrapper> {
        let strategy = parse_lines_with(input, parse_match)?;
        let mut total_score = 0;
        for (opponent, player) in &strategy {
            total_score += score(opponent, player) + player.value();
        }
        Ok(total_score.to_string())
    }
    fn part2(&self, input: &str) -> Result<String, ErrorWrapper> {
        let strategy = parse_lines_with(input, parse_strategy)?;
        let mut total_score = 0;
        for (opponent, play) in &strategy {
            //println!("{opponent:?} x {player:?} -> {total_score}");
            let player = play.apply(opponent);
            total_score += score(opponent, &player) + player.value();
        }
        Ok(total_score.to_string())
    }
}

pub fn get_day() -> Box<dyn AoCDay> {
    Box::new(Day02)
}
