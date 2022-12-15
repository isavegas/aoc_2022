use aoc_core::{bail, AoCDay, ErrorWrapper};
use std::collections::HashMap;

pub struct Day07;

#[derive(Debug, Clone)]
struct Directory {
    pub directories: HashMap<String, Directory>,
    pub files: HashMap<String, usize>,
}

impl Directory {
    pub fn new() -> Self {
        Self {
            directories: HashMap::new(),
            files: HashMap::new(),
        }
    }
    pub fn get_size(&self) -> usize {
        self.directories
            .values()
            .map(Directory::get_size)
            .sum::<usize>()
            + self.files.values().sum::<usize>()
    }
}

fn build_filesystem(log: &str) -> Result<Directory, ErrorWrapper> {
    let mut root = Directory::new();
    let mut dir = vec!["/".to_string()];
    for command_chunk in log.split("$ ").filter(|p| !p.is_empty()) {
        let mut lines = command_chunk.lines();
        let command_line = lines.next().unwrap().trim();
        let mut command_parts = command_line.split(' ');
        match command_parts.next().unwrap() {
            "ls" => {
                let mut work = &mut root;
                for d in dir.iter().skip(1) {
                    work = work
                        .directories
                        .entry(d.clone())
                        .or_insert(Directory::new());
                }
                for l in lines {
                    let parts = l.split(' ').collect::<Vec<&str>>();
                    if parts[0] != "dir" {
                        work.files
                            .insert(parts[1].to_string(), parts[0].parse().expect("Uh oh"));
                    }
                }
            }
            "cd" => {
                match command_parts.next().unwrap() {
                    "/" => dir = vec!["/".to_string()],
                    ".." => {
                        dir.pop();
                    }
                    d => dir.push(d.to_string()),
                }
                if command_parts.next().is_some() {
                    bail!("Too many parameters to `cd`");
                }
            }
            _ => bail!("Unknown command"),
        }
    }
    Ok(root)
}

impl AoCDay for Day07 {
    fn day(&self) -> usize {
        07
    }
    fn expected(&self) -> (Option<&'static str>, Option<&'static str>) {
        (Some("1989474"), Some("1111607"))
    }
    fn part1(&self, input: &str) -> Result<String, ErrorWrapper> {
        let filesystem = build_filesystem(input)?;
        fn recurse_dirs(dir: &Directory) -> usize {
            let mut total = 0;
            let size = dir.get_size();
            if size <= 100000 {
                total += size;
            }
            for d in dir.directories.values() {
                total += recurse_dirs(d);
            }
            total
        }
        Ok(recurse_dirs(&filesystem).to_string())
    }
    fn part2(&self, input: &str) -> Result<String, ErrorWrapper> {
        let filesystem = build_filesystem(input)?;
        fn recurse_dirs(dir: &Directory, min_size: usize, mut best_size: usize) -> usize {
            let size = dir.get_size();
            if size > min_size && size < best_size {
                best_size = size
            }
            for d in dir.directories.values() {
                let size = recurse_dirs(d, min_size, best_size);
                if size > min_size && size < best_size {
                    best_size = size
                }
            }
            best_size
        }
        Ok(recurse_dirs(&filesystem, filesystem.get_size() - 40000000, 30000000).to_string())
    }
}

pub fn get_day() -> Box<dyn AoCDay> {
    Box::new(Day07)
}
