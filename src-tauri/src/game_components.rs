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
    pub num: u8,
}


impl Card {
    fn new(suit: String, value: String) -> Card { //Creates a new card. Might not be used much
        Card {
            suit: suit,
            value: value,
            hidden: false,
            num: 0,
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
        // let mut counter = 0;
        // for card in cards {
        //     card.num = counter;
        //     counter += 1;
        // }
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
    pub money: u64,
    multiplier: u8,
    bust: bool,
    blackjack_total: u8,
}
impl Player {
    pub fn new() -> Player {
        Player {
            name: String::new(),
            hand: Vec::new(),
            money: 200,
            multiplier:2,
            bust : false,
            blackjack_total : 0,
        }
    }
    pub fn add_total(&mut self, amount: u64) {
        self.money += amount;
    }
    pub fn remove_total(&mut self, amount: u64) {
        self.money -= amount;
    }
    pub fn is_bust(&self) -> bool {
        return self.bust;
    }
    pub fn set_bust(&mut self, bust: bool) {
        self.bust = bust;
    }
    pub fn show_stats(&self) {
        println!("Name: {}", self.name);
        println!("Money: {}", self.money);
    }
    pub fn get_blackjack_total(&self) -> u8 { //calculates and returns blackjack value
        return self.blackjack_total
    }
    pub fn set_blackjack_total(&mut self) -> u8 { //calculates and returns blackjack value
        let mut blackjack_total = 0;
        let mut ace:bool = false;
        for cards in self.hand.iter() {
            if cards.get_blackjack_value() == 11 {
                blackjack_total += 11;
                ace = true;
            }
            else {
                blackjack_total += cards.get_blackjack_value();
            }
        }
        if ace && blackjack_total > 21 {
            blackjack_total -= 10;
        }
        self.blackjack_total = blackjack_total; //
        return 1;
    }
    pub fn get_money(&self) -> String {
        return format!("{}", self.money);
    }
}

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
