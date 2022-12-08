use aoc_2022::day::day_02::get_day;

const INPUT: &str = r#"A Y
B X
C Z"#;

#[test]
pub fn part1() {
    assert_eq!(get_day().part1(INPUT).expect("Error"), "15".to_string());
}

#[test]
pub fn part2() {
    assert_eq!(get_day().part2(INPUT).expect("Error"), "12".to_string());
}
