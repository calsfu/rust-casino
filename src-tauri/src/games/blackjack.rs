use std::io;

use crate::game_components::{Table, Player, Card};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Blackjack {
    table: Table,
}

// trait BlackjackTrait {
//     fn new() -> Blackjack;
//     fn start_game(self);
// }
pub fn print_cards(n:&Vec<Card>) {
    for cards in n.iter() {
        println!("{}", cards);
    }
}

impl Blackjack {
    pub fn new() -> Blackjack {
        let mut temp_table =  Table::new();
        temp_table.deck.shuffle_deck();
        temp_table.dealer = Player::new();
        println!("Blackjack created");
        for _i in 0..=1 {
            temp_table.add_player();
        }
        Blackjack { table: temp_table }
    }
    pub fn deal_card(&mut self) {
        self.table.players[0].hand.push(self.table.deck.deal());
    }
    pub fn start_game(&mut self) {
        for _i in 0..=1 {
            self.table.dealer.hand.push(self.table.deck.deal());
        }
        self.table.dealer.hand[1].hidden = true;
        //for player in self.table.players.iter_mut() {
            self.table.players[0].hand.push(self.table.deck.deal());
            self.table.players[0].hand.push(self.table.deck.deal());
        //}
        println!("Player 1 cards are {}", self.table.players[0].hand[0]);
        println!("Player 1 cards are {}", self.table.players[0].hand[1]);
        // for player in self.table.players.iter_mut() {
        //     let mut split = false;
        //     println!("{}'s turn", player.name);
            // loop {
            //     println!("Dealer hand: "); 
            //     print_cards(&self.table.dealer.hand);
            //     println!("Player hand: {:?}", player.hand);
            //     print_cards(&player.hand);
            //     println!("Total is {}", player.get_blackjack_total());
            //     println!("What would you like to do?");
            //     println!("1. Hit");
            //     println!("2. Stand");
            //     println!("3. Double Down");
            //     if player.hand.len() == 2 && player.hand[0].value == player.hand[1].value {
            //         println!("4. Split");
            //         split = true;
            //     }
            //     let mut input = String::new();
            //     io::stdin()
            //         .read_line(&mut input)
            //         .expect("Failed to read line");
                // let input: u32 = input.trim().parse().expect("Please type a number!");
                // match input {
                //     1 => {
                //         player.hand.push(self.table.deck.deal());
                //         println!("Drew a {:?}", player.hand.last());
                //     }
                //     2 => {
                //         println!("Stand");
                //         break;
                //         },
                //     3 => {
                //         if player.money < self.table.max_bet as i64{
                //             println!("You don't have enough money to double down");
                //             continue;
                //         }
                //         else {
                //             println!("Double Down");
                //             player.money -= self.table.max_bet as i64;
                //             player.multiplier = 4;
                //             player.hand.push(self.table.deck.deal());
                //             println!("Drew a {:?}", player.hand.last());
                //             println!("Total is {}", player.get_blackjack_total());
                //             if player.get_blackjack_total() > 21 {
                //                 println!("Bust");
                //             }
                //             break;
                //         }
                        
                //     },
                //     4 => {
                //         if !split{
                //             println!("Invalid input");
                //             continue;
                //         }
                //         else if player.money < self.table.max_bet as i64{
                //             println!("You don't have enough money to split");
                //             continue;
                //         }
                //         else {
                //             player.money -= self.table.max_bet as i64;

                //             println!("Split")
                //         }

                        
                //     },
                //     _ => {
                //         println!("Invalid input")
                //     },
                // } 
    //             if player.get_blackjack_total() > 21 { //checks for bust
    //                 println!("Player hand: {:?}", player.hand);
    //                 println!("Bust with {}!", player.get_blackjack_total());
    //                 println!("Player {} loses", player.name);
    //                 break;
    //             }
    //         }
    //     }
    //     while self.table.dealer.get_blackjack_total() < 17 { //dealer hits under 17
    //         self.table.dealer.hand.push(self.table.deck.deal())
    //     } 
    //     let dealer_total:u8 = self.table.dealer.get_blackjack_total();
    //     println!("Dealer total: {}", dealer_total);
    //     for player in self.table.players.iter_mut() { //compares each player to see if they won
    //         let player_total = player.get_blackjack_total();
    //         println!("Player {} total: {}", player.name, player_total);
    //         if player_total == dealer_total {
    //             println!("Player {} ties", player.name);
    //             player.money += self.table.max_bet as i64;
    //         }
    //         if player_total > dealer_total || dealer_total > 21 {
    //             println!("Player {} wins {}", player.name, player.multiplier as i64 * self.table.max_bet as i64);
    //             player.money += player.multiplier as i64 * self.table.max_bet as i64;
    //         }
    //         else {
    //             println!("Player {} loses", player.name);
    //         }
    //     }
    // }
    }
}