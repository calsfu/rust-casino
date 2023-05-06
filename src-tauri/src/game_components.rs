use std::fmt;
extern crate rand;
extern crate shuffle;
use crate::game_components::rand::prelude::SliceRandom;


//deserialize 

#[derive(Clone, Default, Debug, serde::Serialize, serde::Deserialize)]
pub struct Card {
    pub suit: String,
    pub value: String,
    pub hidden: bool,
}
// trait CardTrait {
//     fn new(suit: String, value: String) -> Card;
//     fn get_suit(&self) -> String;
//     fn get_value(&self) -> String;
//     fn get_blackjack_value(&self) -> u8;
// }

impl Card {
    fn new(suit: String, value: String) -> Card { //Creates a new card. Might not be used much
        Card {
            suit: suit,
            value: value,
            hidden: false,
        }
    }
    fn get_blackjack_value(&self) -> u8 { //May have player score as a parameter to decide whether to use 1 or 11. Can also do in blackjac function.
        //let my_int: i32 = self.value.parse().unwrap();
        if self.value.eq("A") {
            return 11;
        }
        if self.value.eq("J") || self.value.eq("Q") || self.value.eq("K") {
            return 10;
        }
        return self.value.parse().unwrap();
        //self.blackjack_value
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.hidden {
            write!(f, "\n _____\n|* * *|\n| * * |\n|* * *|\n|_*_*_|")
        } else {
            // let card_face = " _____";
            // card_face += "\n"
            write!(f, "\n _____\n|{0}    |\n|     |\n|     |\n|____{0}|", self.value)

        }
    }
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub struct Deck {
    pub cards: Vec<Card>,
}

// trait DeckTrait {
//     fn new() -> Deck;
//     fn get_cards(&self) -> Vec<Card>;
//     fn shuffle_deck(&mut self);
//     fn deal(&mut self) -> Card;
// }

impl Deck {
    pub fn new() -> Deck {
        let mut cards: Vec<Card> = Vec::new();
        let suits = vec!["Hearts", "Diamonds", "Spades", "Clubs"];
        let values = vec![
            "A", "2", "3", "4", "5", "6", "7", "8", "9", "10", "J", "Q", "K",
        ];
        for suit in suits {
            for value in &values {
                cards.push(Card::new(suit.to_string(), value.to_string()));
            }
        }
        Deck { cards: cards }
    }
    pub fn shuffle_deck(&mut self) {
        //let rng = StepRng::new(0, 1);
        //let mut irs = Irs::default();
        //irs.shuffle(&mut self.cards, &mut rng);
        self.cards.shuffle(&mut rand::thread_rng());
    }
    pub fn deal(&mut self) -> Card {
        self.cards.pop().unwrap()
    }
}
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Player {
    pub name: String,
    pub hand: Vec<Card>,
    pub money: i64,
    pub multiplier: u8,
}
impl Player {
    pub fn new() -> Player {
        Player {
            name: String::new(),
            hand: Vec::new(),
            money: 50000,
            multiplier:2,
        }
    }
    pub fn show_stats(&self) {
        println!("Name: {}", self.name);
        println!("Money: {}", self.money);
    }
    pub fn get_blackjack_total(&mut self) -> u8 { //calculates and returns blackjack value
        let mut blackjacktotal = 0;
        let mut ace:bool = false;
        for cards in self.hand.iter() {
            if cards.get_blackjack_value() == 11 {
                blackjacktotal += 11;
                ace = true;
            }
            else {
                blackjacktotal += cards.get_blackjack_value();
            }
        }
        if ace && blackjacktotal > 21 {
            blackjacktotal -= 10;
        }
        return blackjacktotal
    }
}
// trait PlayerTrait {
//     fn get_name(&self) -> String;
//     fn get_hand(&self) -> Vec<Card>;
//     fn get_blackjack_total(&mut self) -> u8; 
// }

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Table {
    pub number: u32,
    pub players: Vec<Player>,
    pub dealer: Player,
    pub deck: Deck,
    pub max_bet: u32,   
}
static mut TABLE_COUNT: u32 = 0;
impl Table {
    pub fn new() -> Table {
        let mut _table_count = 0;
        unsafe { //I know this isn't the best way. Will come back to later
            _table_count = TABLE_COUNT;
            TABLE_COUNT += 1;
        }
        Table {
            number: _table_count,
            players: Vec::new(),
            dealer: Player::new(),
            deck: Deck::new(),
            max_bet: 200,   
        }
    }
    pub fn add_player(&mut self) {
        self.players.push(Player::new());
    }
    pub fn remove_player(&mut self) {
        self.players.pop();
    }
}
// trait TableTrait {
//     fn add_player(&mut self);
//     fn remove_player(&mut self);
//     fn get_number(&self) -> u32;
// }