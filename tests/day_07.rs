use aoc_2022::day::day_07::get_day;

const INPUT: &str = r#"$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k"#;

#[test]
pub fn part1() {
    assert_eq!(get_day().part1(INPUT).expect("Error"), "95437".to_string());
}

#[test]
pub fn part2() {
    assert_eq!(
        get_day().part2(INPUT).expect("Error"),
        "24933642".to_string()
    );
}
