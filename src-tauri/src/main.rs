#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]
use app::{ Casino, Card};


#[derive(serde::Serialize)]
struct Result {
  total: String,
  outcome: String,
}
#[derive(serde::Serialize)]
struct BlackjackUpdate {
  player_cards: Vec<Card>,
  dealer_cards: Vec<Card>,
  game_over: bool,
  player_win: bool,
  no_money: bool,
}
#[derive(serde::Serialize)]
struct SlotResult {
  slot1: u8,
  slot2: u8,
  slot3: u8,
  total: String,
  outcome: String,
}


#[tauri::command]
fn greet(name: String) -> String {
  
   format!("Hello i, {}!", name)
   
}
//For blackjack, I will need to return a blackjack object. That blackjack object 
//will then need to be put as an arugment any time a player takes an action

#[tauri::command]
fn start_blackjack(bet: String, casino: tauri::State<Casino>) -> BlackjackUpdate {
  if bet > casino.player.lock().unwrap().get_money() {
    return BlackjackUpdate {
      player_cards: Vec::new(),
      dealer_cards: Vec::new(),
      game_over: false,
      player_win: false,
      no_money: true,
    };
  }
  casino.blackjack.lock().unwrap().start_game(bet.parse::<u32>().unwrap());
  casino.player.lock().unwrap().remove_total(bet.parse::<u32>().unwrap().into());
  let player_cards = casino.blackjack.lock().unwrap().table.players[0].hand.clone();
  let dealer_cards = casino.blackjack.lock().unwrap().table.dealer.hand.clone();
  let curr_update = BlackjackUpdate {
    player_cards,
    dealer_cards,
    game_over: false,
    player_win: false,
    no_money: false,
  };
  return curr_update;  
  //CASINO.blackjack.start_game(bet.parse::<u32>().unwrap());
  //return game;
  //return game.table.players[0].hand.iter().map(|card| card.to_string()).collect();
}

#[tauri::command]
fn end_game(casino:tauri::State<Casino>) -> BlackjackUpdate {
  casino.blackjack.lock().unwrap().game_over();
  let player_cards = casino.blackjack.lock().unwrap().table.players[0].hand.clone();
  let dealer_cards = casino.blackjack.lock().unwrap().table.dealer.hand.clone();
  let is_bust = casino.blackjack.lock().unwrap().table.players[0].is_bust();
  let player_win = casino.blackjack.lock().unwrap().player_win;
  let net = casino.blackjack.lock().unwrap().net;
  println!("Net is {}", net);
  casino.player.lock().unwrap().add_total(net);
  let curr_update = BlackjackUpdate {
    player_cards,
    dealer_cards,
    game_over: is_bust,
    player_win,
    no_money: false,
  };
  return curr_update;
}

#[tauri::command]
fn deal_card(casino: tauri::State<Casino>) -> BlackjackUpdate {
  casino.blackjack.lock().unwrap().deal_card();
  let player_cards = casino.blackjack.lock().unwrap().table.players[0].hand.clone();
  let dealer_cards = casino.blackjack.lock().unwrap().table.dealer.hand.clone();
  let is_bust = casino.blackjack.lock().unwrap().table.players[0].is_bust();
  let curr_update = BlackjackUpdate {
    player_cards,
    dealer_cards,
    game_over: is_bust,
    player_win: false,
    no_money: false,
  };
  return curr_update;  
  //game.table.players[0].hand.push(game.table.deck.deal());
  //return game;
}

#[tauri::command] 
fn play_roulette(bets: String, placement: String, casino: tauri::State<Casino>) -> Result {
  //game.start_game(bet, placement);
  if bets > casino.player.lock().unwrap().get_money() {
    return Result {
      total: casino.player.lock().unwrap().get_money(),
      outcome: String::from("You don't have enough money to make that bet!"),
    };
  }
  let return_str = casino.roulette.lock().unwrap().start_game(bets.parse::<u32>().unwrap(), placement);
  let net: u64 = casino.roulette.lock().unwrap().net;
  //remove_and_add_bet(casino, bets.parse::<u32>().unwrap().into(), net);
  casino.player.lock().unwrap().remove_total(bets.parse::<u32>().unwrap().into());
  casino.player.lock().unwrap().add_total(net);
  let curr_result = Result {
    total: casino.player.lock().unwrap().get_money(),
    outcome: return_str,
  };
  return curr_result;
  //need to return new total/net, the number/color it landed on. 
  //let serialized_data = serde_json::to_string(&game);
  
}

#[tauri::command]
fn update_total(casino: tauri::State<Casino>) -> String {
  let total = casino.player.lock().unwrap().get_money();
  print!("{}", total);
  return total;
}

#[tauri::command]
fn play_slots(bets: String, casino: tauri::State<Casino>) -> SlotResult {
  if bets > casino.player.lock().unwrap().get_money() {
    return SlotResult {
      slot1: 0,
      slot2: 0,
      slot3: 0,
      total: casino.player.lock().unwrap().get_money(),
      outcome: String::from("You don't have enough money to make that bet!"),
    };
  }
  println!("Running slots...");
  let bet = bets.parse::<u32>().unwrap();
  let end_string = casino.slots.lock().unwrap().start_game(bet);
  println!("Slots ran");
  let net: u64 = casino.slots.lock().unwrap().net;
  println!("Net is {}", net);
  //remove_and_add_bet(casino, bet.into(), net);
  casino.player.lock().unwrap().remove_total(bet.into());
  casino.player.lock().unwrap().add_total(net);
  println!("Money is {}", casino.player.lock().unwrap().get_money());
  println!("slot 1 is {}", casino.slots.lock().unwrap().slot1);
  println!("slot 2 is {}", casino.slots.lock().unwrap().slot2);
  println!("slot 3 is {}", casino.slots.lock().unwrap().slot3);


  let s1 = casino.slots.lock().unwrap().slot1;
  let s2 = casino.slots.lock().unwrap().slot2;
  let s3 = casino.slots.lock().unwrap().slot3;
  let curr_result = SlotResult {
    slot1: s1,
    slot2: s2,
    slot3: s3,
    total: casino.player.lock().unwrap().get_money(),
    outcome: end_string,
  };
  println!("returning slots");
  return curr_result;
}


fn main() {
  tauri::Builder::default()
    .manage(Casino::new())
    //.invoke_handler(tauri::generate_handler![greet, start_blackjack, play_roulette, create_blackjack,  deal_card, end_game, play_slots])
    .invoke_handler(tauri::generate_handler![greet, start_blackjack, play_roulette, update_total, deal_card, end_game, play_slots])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}