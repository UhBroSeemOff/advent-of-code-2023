use super::extract_numeric_at;

pub fn find_numbers_hard_way(input: &str) -> Option<u32> {
    let first_number = find_first_number(input, true).expect("number should exist");
    let last_number = find_first_number(input, false).expect("number should exist");

    let result = first_number * 10 + last_number;

    return Some(result);
}

const NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

#[derive(Clone, Copy)]
struct NumberInfo {
    value: u32,
    index: usize,
}

impl NumberInfo {
    fn extract_py_pattern(
        input: &str,
        pattern: &str,
        index: usize,
        positive_direction: bool,
    ) -> Option<Self> {
        let number = match positive_direction {
            true => input.find(pattern),
            false => input.rfind(pattern),
        };
        if let Some(number_index) = number {
            let index_of_pattern: u32 = index.try_into().unwrap();
            let value = NumberInfo {
                index: number_index,
                value: index_of_pattern + 1,
            };

            return Some(value);
        }

        return None;
    }
}

fn find_first_number(input: &str, positive_direction: bool) -> Option<u32> {
    let binding = vec![
        find_digit_number(input, positive_direction),
        find_word_number(input, positive_direction),
    ];
    let numbers = binding.iter().filter(|number| number.is_some());

    let result = match positive_direction {
        true => numbers.min_by_key(|number| number.unwrap().index)?,
        false => numbers.max_by_key(|number| number.unwrap().index)?,
    };

    return Some(result.unwrap().value);
}

fn find_digit_number(input: &str, positive_direction: bool) -> Option<NumberInfo> {
    let index = match positive_direction {
        true => input.find(char::is_numeric)?,
        false => input.rfind(char::is_numeric)?,
    };
    let value = extract_numeric_at(input, index)?;
    Some(NumberInfo { value, index })
}

fn find_all_numbers(input: &str, positive_direction: bool) -> Vec<NumberInfo> {
    return NUMBERS
        .iter()
        .enumerate()
        .filter_map(|(index, pattern)| {
            NumberInfo::extract_py_pattern(input, pattern, index, positive_direction)
        })
        .collect();
}

fn find_word_number(input: &str, positive_direction: bool) -> Option<NumberInfo> {
    let all_numbers = find_all_numbers(input, positive_direction);
    let result = match positive_direction {
        true => all_numbers.iter().min_by_key(|number| number.index)?,
        false => all_numbers.iter().max_by_key(|number| number.index)?,
    };

    Some(*result)
}
