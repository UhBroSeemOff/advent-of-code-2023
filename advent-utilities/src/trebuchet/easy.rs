use std::{fs, path::PathBuf};

pub fn solve_easy_trebuchet_task(file_path: PathBuf) -> Option<()> {
    let file_content = fs::read_to_string(file_path).expect("file should exist");

    let lines = file_content.lines();
    let result: u32 = lines
        .map(|line| find_numbers(line).expect("number should be parsed"))
        .sum();

    println!("Result: {result}");
    Some(())
}

fn extract_numeric_at(source: &str, index: usize) -> Option<u32> {
    let number_as_char = source.chars().nth(index).expect("char should exist");
    let result = number_as_char
        .to_digit(10)
        .expect("char should be convertable to numeric");

    return Some(result);
}

fn find_numbers(input: &str) -> Option<u32> {
    let first_number_index = input
        .find(char::is_numeric)
        .expect("should be at least one numeric");

    let last_number_index = input
        .rfind(char::is_numeric)
        .expect("should be at least one numeric");

    let first_number = extract_numeric_at(input, first_number_index)?;
    let second_number = extract_numeric_at(input, last_number_index)?;
    let result_number = first_number * 10 + second_number;

    return Some(result_number);
}
