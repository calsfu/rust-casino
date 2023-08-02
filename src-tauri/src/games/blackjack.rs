
use crate::game_components::{Table, Player, Card};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Blackjack {
    pub table: Table,
    bet: u32,
    pub net: u64,
    pub player_win : bool,
}
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
        Blackjack { 
            table: temp_table,
            bet: 0, 
            net: 0,
            player_win : false,
        }
    }
    pub fn game_over(&mut self) {
        println!("Ending game...");
        self.table.dealer.hand[1].hidden = false;
        self.table.dealer.set_blackjack_total(); //change this later
        while self.table.dealer.get_blackjack_total() < 17 && self.table.players[0].is_bust() == false {
            self.table.dealer.hand.push(self.table.deck.deal());
            self.table.dealer.set_blackjack_total();
        }
        if self.table.players[0].get_blackjack_total() > 21 { //should never run
            println!("You lose");
        }
        else if self.table.dealer.get_blackjack_total() > 21 || self.table.dealer.get_blackjack_total() < self.table.players[0].get_blackjack_total() {
            self.net = self.bet as u64 * 2;
            self.player_win = true;
            self.table.dealer.set_bust(true);
            println!("You win {}", self.net);
        }
        else if self.table.dealer.get_blackjack_total() == self.table.players[0].get_blackjack_total() {
            self.net = self.bet.into();
            println!("Tie {}", self.net);
        }
        
        
    }
    pub fn deal_card(&mut self) {
        if self.table.players[0].is_bust() {
            println!("Cannot deal to a busted player");
        }
        else {
            self.table.players[0].hand.push(self.table.deck.deal());
            println!("Player 1 cards are {}", self.table.players[0].hand[1]);
            self.table.players[0].set_blackjack_total(); //change this later
            println!("Blackjack total is {}",  self.table.players[0].get_blackjack_total());
            if self.table.players[0].get_blackjack_total() > 21 {
                self.game_over();
                self.table.players[0].set_bust(true);
                println!("Bust:");
            }
        }
    }
    pub fn start_game(&mut self, bet: u32) {
        self.table.dealer.hand.clear();
        self.table.players[0].hand.clear();
        self.bet = bet;
        self.player_win = false;
        self.table.players[0].set_bust(false);
        for _i in 0..=1 {
            self.table.dealer.hand.push(self.table.deck.deal());
        }
        self.table.dealer.hand[1].hidden = true;
        //for player in self.table.players.iter_mut() {
            self.table.players[0].hand.push(self.table.deck.deal());
            self.table.players[0].hand.push(self.table.deck.deal());
            self.table.players[0].set_blackjack_total();
        //}
        println!("Player 1 cards are {}", self.table.players[0].hand[0]);
        println!("Player 1 cards are {}", self.table.players[0].hand[1]);
      
    }
}