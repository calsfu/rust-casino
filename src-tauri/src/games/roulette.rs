use rand::prelude::*;


pub fn find_color(num:u8) -> char {
    if num == 0 || num == 37 {
        return 'G';
    }
    if num <= 10 {
        if num % 2 == 0 {
            return 'B';
        }
        else {
            return 'R';
        }
    }
    else if num <= 18 {
        if num % 2 == 0 {
            return 'R';
        }
        else {
            return 'B';
        }
    }
    else if num <= 28 {
        if num % 2 == 0 {
            return 'B';
        }
        else {
            return 'R';
        }
    }
    else if num <= 36 {
        if num % 2 == 0 {
            return 'R';
        }
        else {
            return 'B';
        }
    }
    else {
        return 'z';
    }
}
#[derive(serde::Serialize)]
pub struct Roulette {
    //pub table: Table,
    pub bets: Vec<String>,
    money_bets: Vec<u32>,
    color: char,
    pub net: u64,
    number: u8, //0-37 where 37 is 00
}

impl Roulette { //Will roll a number, then a number will be selected
    pub fn new() -> Roulette {
        Roulette {
            //table : Table::new(),
            bets : Vec::new(),
            money_bets : Vec::new(),
            color : 'z',
            number : 0,
            net : 0,
        }
    }
    
    pub fn check_win(&self, _bets:Vec<String>) -> bool {
    
        return false;
    }

    pub fn start_game(&mut self, bets:u32 , place:String ) -> String {
        let temp:f32 = rand::thread_rng().gen();
        let mut net: u64 = 0;
        self.number = (38.0 * temp) as u8;
        self.color = find_color(self.number);
        //let mut counter = 0;
        //for place in placements {
            if place == "R" || place == "B" {
                if place == self.color.to_string() {
                    net = net + (bets* 2 as u32) as u64;
                }
            }   
            else if place == "G" && self.color.to_string() == "G" {
                net = net + (bets * 14 as u32) as u64
            }
            else if place[0..1].chars().all(char::is_numeric) == true {
                if place == self.number.to_string() || place == "00" && self.number == 37 {
                    net = net + (bets * 36 as u32) as u64;
                } 
            }
        // counter += 1;
    // }
        println!("The number is {} and the color is {}", self.number, self.color);
        self.net = net;
        let outcome_string: String = format!("The ball landed on {} which is {}\nYou won {} dollars!", self.number, self.color, self.net);
        return outcome_string;
    }
}

