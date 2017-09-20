extern crate rand;

mod card;
mod dealer;
mod player;
mod game;

use dealer::Dealer;
use card::Card;

fn main() {
    let mut cards = Vec::new();
    for _ in 0..1 {
        cards.extend(Card::init_cards());
    }
    let mut dealer = Dealer::new(cards);

    for i in 0..5 {
        let name = format!("Player{}", i);
        dealer.add_player(name);
    }

    dealer.distribute_cards(7);
    dealer.start_game();
    dealer.show_scores();
}
