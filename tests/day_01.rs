use aoc_2022::day::day_01::get_day;

const INPUT: &str = r#"1000
2000
3000

4000

5000
6000

7000
8000
9000

10000"#;

#[test]
pub fn part1() {
    assert_eq!(get_day().part1(INPUT).expect("Error"), "24000".to_string());
}

#[test]
pub fn part2() {
    assert_eq!(get_day().part2(INPUT).expect("Error"), "45000".to_string());
}
