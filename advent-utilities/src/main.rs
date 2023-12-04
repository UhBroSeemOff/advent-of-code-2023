use clap::Parser;
use human_panic::{self, setup_panic};
use std::fs;

#[derive(Parser)]
struct Options {
    file: std::path::PathBuf,
}

fn main() {
    setup_panic!();
    let options = Options::parse();
    let file_content = fs::read_to_string(options.file).expect("file should exist");

    let mut accumulator = 0;
    for line in file_content.lines() {
        let first_number_index = line
            .find(char::is_numeric)
            .expect("should be at least one numeric");

        let last_number_index = line
            .rfind(char::is_numeric)
            .expect("should be at least one numeric");

        let first_number =
            extract_numeric_at(line, first_number_index).expect("line should contain number");
        let second_number =
            extract_numeric_at(line, last_number_index).expect("line should contain number");
        let result_number = first_number * 10 + second_number;

        accumulator += result_number;
    }

    println!("Result: {}", accumulator);
}

fn extract_numeric_at(source: &str, index: usize) -> Option<u32> {
    let number_as_char = source.chars().nth(index).expect("char should exist");
    let result = number_as_char
        .to_digit(10)
        .expect("char should be convertable to numeric");

    return Some(result);
}
