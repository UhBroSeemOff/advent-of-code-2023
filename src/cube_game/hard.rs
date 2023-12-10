use super::data::{Game, GameSet};

pub fn find_minimal_cubes_quantities(games: &Vec<Game>) -> Option<u32> {
    let games_copy = &mut games.to_owned();
    let result = games_copy
        .iter_mut()
        .filter_map(|game| sum_cubes_in_subsets(&mut game.subsets))
        .map(|game| game.get_power())
        .sum();

    Some(result)
}

impl GameSet {
    fn merge(&mut self, source: GameSet) -> &Self {
        self.red = find_max(self.red, source.red);
        self.green = find_max(self.green, source.green);
        self.blue = find_max(self.blue, source.blue);

        return self;
    }

    fn get_power(self) -> u32 {
        self.red * self.green * self.blue
    }
}

fn find_max(default: u32, target: u32) -> u32 {
    return default.max(target);
}

fn sum_cubes_in_subsets(subsets: &mut Vec<GameSet>) -> Option<GameSet> {
    let result = subsets.iter_mut().reduce(|accumulator, subset| {
        accumulator.merge(*subset);
        return accumulator;
    })?;

    return Some(*result);
}
