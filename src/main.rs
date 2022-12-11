#![feature(iter_array_chunks)]

use aoc_core::AoCProject;

pub fn main() {
    let project = AoCProject::new(
        2022,
        env!("CARGO_PKG_VERSION").to_string(),
        None,
        "isavegas".to_string(),
        None,
    );
    project.run(&aoc_2022::get_days(), &aoc_2022::get_inputs());
}
