use aoc_2022::day::day_06::get_day;

const INPUT: &str = r#"mjqjpqmgbljsphdztnvjfqwrcgsmlb"#;

#[test]
pub fn part1() {
    assert_eq!(get_day().part1(INPUT).expect("Error"), "7".to_string());
}

#[test]
pub fn part2() {
    assert_eq!(get_day().part2(INPUT).expect("Error"), "19".to_string());
}
