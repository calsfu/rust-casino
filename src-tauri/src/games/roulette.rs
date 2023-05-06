use crate::game_components::{Table};
use std::io;
use std::time::Duration;
use std::{thread, time};
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
    moneyBets: Vec<u32>,
    color: char,
    net: u64,
    number: u8, //0-37 where 37 is 00
}

impl Roulette { //Will roll a number, then a number will be selected
    pub fn new() -> Roulette {
        Roulette {
            //table : Table::new(),
            bets : Vec::new(),
            moneyBets : Vec::new(),
            color : 'z',
            number : 0,
            net : 0,
        }
    }
    
    pub fn check_win(&self, bets:Vec<String>) -> bool {
        let mut counter:u8 = 0;
        for bets in bets.iter() {
            
            counter += 1;
        }
        return false;
    }
//     pub fn start_game(&mut self, bets:Vec<u32> , placements:Vec<String> ) -> u64 {
//         let temp:f32 = rand::thread_rng().gen();
//         let mut net: u64 = 0;
//         self.number = (38.0 * temp) as u8;
//         self.color = find_color(self.number);
//         let mut counter = 0;
//         for place in placements {
//             if place == "R" || place == "B" {
//                 if place == self.color.to_string() {
//                     net = net + (bets[counter]* 2 as u32) as u64;
//                 }
//             }
//             else if place == "G" && self.color.to_string() == "G" {
//                 net = net + (bets[counter] * 2 as u32) as u64
//             }
//             else if place[0..1].chars().all(char::is_numeric) == true {
//                 if place == self.number.to_string() || place == "00" && self.number == 37 {
//                     net = net + (bets[counter] * 36 as u32) as u64;
//                 } 
//             }
//             counter += 1;
//         }
//         return net;
//     }
// }
pub fn start_game(&mut self, bets:u32 , place:String ) {
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
}
}

// for players in self.table.players.iter_mut() {

        
//     loop {
//         println!("Enter your bet (num 00 - 36, R, B, G, O, E, F, S, T, FH, SH, FR, SR, TR, or H for help");
//         let mut input:String = String::new();
        
//             io::stdin()
//                 .read_line(&mut input)
//                 .expect("Failed to read line");
//         let input: String = input.trim().parse().expect("Please type a String!");
//         match input.as_str()  {
//             "h" => {
//                 println!("test");
//             }
//             _=> {
//                 println!("Please enter a valid bet");
//             }
//         }
//         self.bets.push(input);
//         loop {
//             println!("Enter your bet");
//             let mut bet:String = String::new();
//             io::stdin()
//                 .read_line(&mut bet)
//                 .expect("Failed to read line");
//             let bet: u32 = bet.trim().parse().expect("Please type a number!");
//             if bet > self.table.max_bet {
//                 println!("Bet is higher than the tables max bet. Please enter a lower bet");
//             }
//             else {
//                 self.moneyBets.push(bet);
//                 break;
//             }
//         }
        
//     }
// }
// let one_sec:Duration = time::Duration::from_secs(1);
// print!("Rolling");
// thread::sleep(one_sec);
// print!(".");
// thread::sleep(one_sec);
// print!(".");
// thread::sleep(one_sec);
// println!(".");
// thread::sleep(one_sec);
// let temp:f32 = rand::thread_rng().gen();
// //self.number = (38 * temp) as u8;
// self.color = find_color(self.number);
// for players in self.table.players.iter_mut() {

// }
