use aoc_2022::day::day_04::get_day;

const INPUT: &str = r#"2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8"#;

#[test]
pub fn part1() {
    assert_eq!(get_day().part1(INPUT).expect("Error"), "2".to_string());
}

#[test]
pub fn part2() {
    assert_eq!(get_day().part2(INPUT).expect("Error"), "4".to_string());
}
