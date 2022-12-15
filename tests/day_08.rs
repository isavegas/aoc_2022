use aoc_2022::day::day_08::get_day;

const INPUT: &str = r#"30373
25512
65332
33549
35390"#;

#[test]
pub fn part1() {
    assert_eq!(get_day().part1(INPUT).expect("Error"), "21".to_string());
}

#[test]
pub fn part2() {
    assert_eq!(get_day().part2(INPUT).expect("Error"), "8".to_string());
}
