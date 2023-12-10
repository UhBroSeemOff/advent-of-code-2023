use super::number_data::{find_numbers_in_string, NumberData};

pub fn find_symbol_adjacent_numbers(input: String) -> Option<u32> {
    let numbers: Vec<NumberData> = input
        .lines()
        .enumerate()
        .filter_map(|(index, line)| find_numbers_in_string(line, index))
        .flatten()
        .collect();

    let result: u32 = numbers.iter().map(|number| number.value).sum();
    Some(result)
}
