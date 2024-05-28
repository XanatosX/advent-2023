#[derive(Debug, Clone)]
pub struct GameRound {
    red: u32,
    green: u32,
    blue: u32
}

impl  GameRound {
    pub fn new(red: u32, green: u32, blue: u32) -> GameRound {
        GameRound { red, green, blue }
    }

    pub fn get_red(&self) -> &u32 {
        &self.red
    }

    pub fn get_green(&self) -> &u32 {
        &self.green
    }


    pub fn get_blue(&self) -> &u32 {
        &self.blue
    }
}