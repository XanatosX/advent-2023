use crate::model::game_round::GameRound;

#[derive(Debug, Clone)]
pub struct Game {
    id: u32,
    rounds: Vec<GameRound>
}

impl Game {
    pub fn new(id: u32) -> Game {
        Game{id, rounds: Vec::new()}
    }

    pub fn get_id(&self) -> &u32 {
        &self.id
    }

    pub fn add_round(&mut self, game_round: GameRound) {
        self.rounds.push(game_round);
    }
    
    pub fn get_highest_red_value(&self) -> u32 {
        let default_return: u32 = 0;
        self.rounds.iter().map(|round| round.get_red().clone()).max().unwrap_or(default_return)
    }

    pub fn get_highest_green_value(&self) -> u32 {
        let default_return: u32 = 0;
        self.rounds.iter().map(|round| round.get_green().clone()).max().unwrap_or(default_return)
    }

    pub fn get_highest_blue_value(&self) -> u32 {
        let default_return: u32 = 0;
        self.rounds.iter().map(|round| round.get_blue().clone()).max().unwrap_or(default_return)
    }
}