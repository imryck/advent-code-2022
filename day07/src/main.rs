use std::collections::HashMap;
use common;

const TOTAL_DISK_SIZE: i32 = 70000000;
const MAJ_SIZE_REQUIREMENT: i32 = 30000000;

fn main() {
    let cli_args = common::parse_cli_args();
    let lines = common::read_lines(&cli_args.input_path).unwrap();
    let directories = parse_instructions(lines);
    dbg!(directories.clone());
    part_1(directories.clone());
    part_2(directories.clone());
}

fn part_1(directories: HashMap<String, i32>) {
    println!("{:?}", directories.values().filter(|&sum| *sum <= 100000).sum::<i32>());
}

fn part_2(directories: HashMap<String, i32>) {
    let missing_space = directories["/"] + MAJ_SIZE_REQUIREMENT - TOTAL_DISK_SIZE;
    let directory_size_to_delete = *directories
        .values()
        .min_by_key(|f| {
            dbg!(f, missing_space);
            f.abs_diff(missing_space)
        })
        .unwrap();
    println!("{:?}", directory_size_to_delete);
}

fn parse_instructions(lines: Vec<String>) -> HashMap<String, i32> {
    let mut directories: HashMap<String, i32> = HashMap::new();
    let mut current_directory: Vec<String> = Vec::new();
    lines.into_iter()
        .map(|line| line.split(" ").map(str::to_string).collect::<Vec<String>>())
        .map(|parts| {
            if parts.len() > 2 {
                (parts[1].clone(), parts[2].clone())
            } else {
                (parts[0].clone(), parts[1].clone())
            }
        })
        .for_each(|(part1, part2)| {
            match (part1.as_str(), part2.as_str()) {
                ("cd", "/") => {
                    current_directory.clear();
                    current_directory.push("/".to_string())
                }
                ("cd", "..") => {
                    current_directory.pop();
                }
                ("cd", dirname) => {
                    let last_dir = current_directory.last().unwrap();
                    current_directory.push(format!("{last_dir}{dirname}/"));
                }
                (size, _file) => {
                    if let Ok(file_size) = size.parse::<i32>() {
                        current_directory.iter().cloned().for_each(|dir| {
                            directories.entry(dir)
                                .and_modify(|s| *s += file_size)
                                .or_insert(file_size);
                        });
                    }
                }
            }
        });
    directories
}
