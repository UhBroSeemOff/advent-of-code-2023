use super::data::{Game, GameSet};

const PRESET: GameSet = GameSet {
    red: 12,
    green: 13,
    blue: 14,
};

pub fn find_possible_games_sum(games: &Vec<Game>) -> Option<u32> {
    let possible_games = games.iter().filter(|game| is_game_possible(game));
    let result: u32 = possible_games.map(|game| game.id).sum();
    return Some(result);
}

fn is_game_possible(input: &Game) -> bool {
    input.subsets.iter().all(is_set_possible)
}

fn is_set_possible(input: &GameSet) -> bool {
    input.blue <= PRESET.blue && input.green <= PRESET.green && input.red <= PRESET.red
}
