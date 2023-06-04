use std::fs;

fn main() {
    println!(
        "Current directory: {}",
        std::env::current_dir().unwrap().display()
    );

    let input = fs::read_to_string("/home/danyal/code/aoc-rust/day-1/src/input.txt")
        .expect("Unable to read the file");

    let max_count = input
        .split("\n\n")
        .map(|num_list| {
            num_list
                .lines()
                .map(|num_string| num_string.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap_or(0);

    println!("{}", max_count);
}
