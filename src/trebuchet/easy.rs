use super::extract_numeric_at;

pub fn find_numbers_easy_way(input: &str) -> Option<u32> {
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
