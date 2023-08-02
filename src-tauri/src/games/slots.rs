#[derive(serde::Serialize)]
pub struct Slots {
    bet : u32, 
    pub slot1: u8, 
    pub slot2: u8,
    pub slot3: u8,
    pub net: u64,
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
    pub fn start_game(&mut self, bets:u32) -> String {
        
        let end_string:String;
        self.bet = bets;
        self.slot1 = rand::random::<u8>() % 3;
        self.slot2 = rand::random::<u8>() % 3;
        self.slot3 = rand::random::<u8>() % 3;
        if self.slot1 == self.slot2 && self.slot2 == self.slot3 {
            self.net = (self.bet * 3) as u64;
            end_string = format!("You won {} dollars!", self.net);
        }
        else if self.slot1 == self.slot2 || self.slot2 == self.slot3 || self.slot1 == self.slot3 {
            self.net = (self.bet * 2) as u64;
            end_string = format!("You won {} dollars!", self.net);
        }
        else {
            self.net = 0;
            end_string = format!("You lost {} dollars!", self.bet);
        }
        print!("{}", end_string);
        return end_string;
    }
}