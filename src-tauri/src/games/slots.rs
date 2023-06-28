#[derive(serde::Serialize)]
pub struct Slots {
    bet : u32, 
    slot1: u8, 
    slot2: u8,
    slot3: u8,
    net: u64,
}

impl Slots {
    pub fn new() -> Slots {
        Slots {
            bet: 0,
            slot1: 0,
            slot2: 0,
            slot3: 0,
            net: 0,
        }
    }
    pub fn start_game(&mut self, bets:u32) {
        self.bet = bets;
        self.slot1 = rand::random::<u8>() % 3;
        self.slot2 = rand::random::<u8>() % 3;
        self.slot3 = rand::random::<u8>() % 3;
        if self.slot1 == self.slot2 && self.slot2 == self.slot3 {
            self.net = (self.bet * 3) as u64;
        }
        else if self.slot1 == self.slot2 || self.slot2 == self.slot3 || self.slot1 == self.slot3 {
            self.net = (self.bet * 2) as u64;
        }
        else {
            self.net = 0;
        }
    }
}