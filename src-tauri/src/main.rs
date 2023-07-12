#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use app::{Blackjack, Roulette, Slots, Player, Casino};
use tauri::State;
use std::sync::Mutex;

//use once_cell::sync::Lazy;
//static mut CASINO: Casino = Casino::new();
//static mut CASINO: Casino = Casino::new();
// lazy_static! {
//   static ref CASINO: Casino = Casino::new();
// }

#[tauri::command]
fn greet(name: String) -> String {
  
   format!("Hello i, {}!", name)
   
}
//For blackjack, I will need to return a blackjack object. That blackjack object 
//will then need to be put as an arugment any time a player takes an action

#[tauri::command]
fn start_blackjack(bet: String, casino: tauri::State<Casino>)  {
  casino.blackjack.lock().unwrap().start_game(bet.parse::<u32>().unwrap());
  //CASINO.blackjack.start_game(bet.parse::<u32>().unwrap());
  //return game;
  //return game.table.players[0].hand.iter().map(|card| card.to_string()).collect();
}

// #[tauri::command]
// fn create_blackjack() -> Blackjack {
//   let game = Blackjack::new();
//   return game;
// }

// #[tauri::command]
// fn end_game(mut game : Blackjack) -> Blackjack {
//   game.game_over();
//   return game;
// }

// #[tauri::command]
// fn deal_card(mut game: Blackjack) -> Blackjack {
//   game.deal_card();
//   return game;
// }

// #[tauri::command] 
// fn play_roulette(bets: String, placement: String) -> Roulette {
//   let bet = bets.parse::<u32>().unwrap();
//   let mut game = Roulette::new();
//   game.start_game(bet, placement);
//   //let serialized_data = serde_json::to_string(&game);
//   return game; 
// }

// #[tauri::command]
// fn play_slots(bets: String) -> Slots {
//   let bet = bets.parse::<u32>().unwrap();
//   let mut game = Slots::new();
//   game.start_game(bet);
//   return game;
// }
// fn main() {
//   tauri::Builder::default()
//     .run(tauri::generate_context!())
//     .expect("error while running tauri application");
// }

fn main() {
  tauri::Builder::default()
    .manage(Casino::new())
    //.invoke_handler(tauri::generate_handler![greet, start_blackjack, play_roulette, create_blackjack,  deal_card, end_game, play_slots])
    .invoke_handler(tauri::generate_handler![greet, start_blackjack])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}