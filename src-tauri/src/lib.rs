pub mod game_components; //Defines a module
pub use game_components::{Card, Deck, Player, Table};

pub mod games {
    pub mod blackjack;
    pub mod roulette;
    pub mod slots;
}



pub use games::blackjack::Blackjack;
pub use games::roulette::Roulette;
pub use games::slots::Slots;

pub mod casino;
pub use casino::Casino;