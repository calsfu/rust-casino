#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use app::{Blackjack, Deck, Roulette, Table};

use serde_json::Result;

#[tauri::command]
fn greet(name: String) -> String {
  
   format!("Hello i, {}!", name)
   
}
//For blackjack, I will need to return a blackjack object. That blackjack object 
//will then need to be put as an arugment any time a player takes an action

#[tauri::command]
fn start_blackjack(bet: &str) -> Blackjack {
  let mut game = Blackjack::new();
  game.start_game();

  return game;
  //return game.table.players[0].hand.iter().map(|card| card.to_string()).collect();
}

#[tauri::command]
fn create_blackjack() -> Blackjack {
  let mut game = Blackjack::new();
  return game;
}

#[tauri::command]
fn deal_card(mut game: Blackjack) -> Blackjack {
  game.deal_card();
  return game;
}

#[tauri::command] 
fn play_roulette(bets: String, placement: String) -> Roulette {
  let bet = bets.parse::<u32>().unwrap();
  let mut game = Roulette::new();
  game.start_game(bet, placement);
  //let serialized_data = serde_json::to_string(&game);
  return game; 
}

// fn main() {
//   tauri::Builder::default()
//     .run(tauri::generate_context!())
//     .expect("error while running tauri application");
// }

fn main() {
  
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet, start_blackjack, play_roulette, create_blackjack,  deal_card])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}