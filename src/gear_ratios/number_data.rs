use super::point::Point;

#[derive(Clone)]
pub struct NumberData {
    pub value: u32,
    pub coordinates: Vec<Point>,
}

impl NumberData {
    pub fn new(value: u32, initial_point: Point) -> Self {
        NumberData {
            value,
            coordinates: vec![initial_point],
        }
    }

    pub fn expand_number(&mut self, value: u32, point: Point) -> &mut Self {
        let new_value = self.value * 10 + value;
        self.value = new_value;
        self.coordinates.push(point);

        self
    }
}

pub fn find_numbers_in_string(input: &str, height: usize) -> Option<Vec<NumberData>> {
    let mut result = vec![];
    let mut current_number: Option<NumberData> = None;
    for (index, char) in input.chars().enumerate() {
        if !char.is_numeric() {
            if current_number.is_some() {
                result.push(current_number?);
            }
            current_number = None;
            continue;
        }

        let point = Point::new(index, height);
        let number: u32 = char.to_digit(10)?;
        match current_number {
            Some(ref mut value) => {
                value.expand_number(number, point);
            }
            None => {
                let new_value = NumberData::new(number, point);
                current_number = Some(new_value);
            }
        };
    }

    Some(result)
}
