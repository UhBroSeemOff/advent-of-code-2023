use super::{data::*, Colors};

pub trait Parser {
    fn parse(input: &str) -> Self;
}

impl Parser for Game {
    fn parse(input: &str) -> Self {
        let (game, subsets_record) = input.split_once(':').expect("game record should exist");
        let id_string = game
            .split_whitespace()
            .last()
            .expect("id of the game should be separated");
        let id: u32 = id_string.parse().expect("id should be parsable");

        Game {
            id,
            subsets: subsets_record
                .split(";")
                .map(|subset: &str| GameSet::parse(subset))
                .collect(),
        }
    }
}

impl Parser for GameSet {
    fn parse(input: &str) -> Self {
        let color_records: Vec<&str> = input.split(",").collect();

        let mut result = GameSet::default();

        for color in color_records {
            let color_data = parse_color(color).expect("colors should be available to parse");
            match color_data {
                Colors::Red(quantity) => result.red = quantity,
                Colors::Green(quantity) => result.green = quantity,
                Colors::Blue(quantity) => result.blue = quantity,
            }
        }

        return result;
    }
}

fn parse_color(input: &str) -> Option<Colors> {
    let (quantity, color) = input
        .trim()
        .split_once(" ")
        .expect("should be two parts in game set record");

    let parsed_quantity = quantity.parse::<u32>().expect("quantity should be parsed");

    let result = match color.trim() {
        "red" => Colors::Red(parsed_quantity),
        "green" => Colors::Green(parsed_quantity),
        "blue" => Colors::Blue(parsed_quantity),
        _ => return None,
    };

    return Some(result);
}
