use aoc_2022::day::day_05::get_day;

const INPUT: &str = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2"#;

#[test]
pub fn part1() {
    assert_eq!(get_day().part1(INPUT).expect("Error"), "CMZ".to_string());
}

#[test]
pub fn part2() {
    assert_eq!(get_day().part2(INPUT).expect("Error"), "MCD".to_string());
}
