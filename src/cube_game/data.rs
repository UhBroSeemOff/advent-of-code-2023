#[derive(Debug, Clone)]
pub struct Game {
    pub id: u32,
    pub subsets: Vec<GameSet>,
}

#[derive(Default, Debug, Clone, Copy)]
pub struct GameSet {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}
