use crate::games::{blackjack::Blackjack, roulette::Roulette, slots::Slots};
use crate::game_components::Player;
use std::sync::Mutex;

//#[derive()]
pub struct Casino {
    pub blackjack: Mutex<Blackjack>,
    pub roulette: Mutex<Roulette>,
    pub slots: Mutex<Slots>,
    pub player: Mutex<Player>,
  }
  impl Casino {
    pub fn new() -> Casino {
        Casino {
            blackjack: Mutex::new(Blackjack::new()),
            roulette: Mutex::new(Roulette::new()),
            slots: Mutex::new(Slots::new()),
            player: Mutex::new(Player::new()),
        }
    }

}