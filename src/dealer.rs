use card::{Card, Color};
use player::Player;
use game::Game;

#[derive(Debug)]
pub struct Dealer {
    players: Vec<Player>,
    mountain: Vec<Card>,
    fields: Vec<Card>,
}

impl Dealer {
    pub fn new(mountain: Vec<Card>) -> Dealer {
        Dealer {
            players: Vec::new(),
            mountain: mountain,
            fields: Vec::new(),
        }
    }

    pub fn add_player(&mut self, name: String) {
        self.players.push(Player::new(name));
    }

    pub fn distribute_cards(&mut self, cards: usize) {
        for player in &mut self.players {
            for i in 0..cards {
                let card = self.mountain.remove(i);
                player.take_a_card(card);
            }
        }
    }

    pub fn start_game(&mut self) {
        let mut game = Game::new(self.players.len());
        self.initialize_game(&mut game);

        let mut count = 0;

        loop {
            count += 1;
            let index = game.current_player;
            if let Some(card) = self.players[index].put_down_a_card(&self.fields[0], &game.color) {
                match card {
                    Card::Number(color, _) => {
                        game.specify_color(color);
                    }
                    Card::Reverse(color) => {
                        game.specify_color(color);
                        if game.players > 2 {
                            game.reverse_order();
                        } else {
                            game.skip_player();
                        }
                    }
                    Card::Skip(color) => {
                        game.specify_color(color);
                        game.skip_player();
                    }
                    Card::DrawTwo(color) => {
                        game.specify_color(color);
                        let next = game.get_next_player();

                        for _ in 0..2 {
                            let card = self.draw_from_mountain();
                            let next_player = &mut self.players[next];
                            next_player.take_a_card(card);
                        }
                    }
                    Card::Wild => {
                        let player = &self.players[game.current_player];
                        let color = player.specify_color();
                        game.specify_color(color);
                    }
                    Card::WildDrawFour => {
                        {
                            let player = &self.players[game.current_player];
                            let color = player.specify_color();
                            game.specify_color(color);
                        }

                        let next = game.get_next_player();
                        for _ in 0..2 {
                            let card = self.draw_from_mountain();
                            let next_player = &mut self.players[next];
                            next_player.take_a_card(card);
                        }
                    }
                }

                self.fields.insert(0, card);

                let player = &self.players[game.current_player];
                if player.get_len() == 1 {
                    println!("{}, UNO!", &player.get_name());
                }
            } else {
                let card = self.draw_from_mountain();
                let player = &mut self.players[game.current_player];
                player.take_a_card(card);
            }

            let player = &self.players[game.current_player];
            if player.get_len() == 0 {
                println!("winner: {}", player.get_name());
                println!("count: {}", count);
                break;
            } else {
                game.change_player();
            }
        }
    }

    fn initialize_game(&mut self, game: &mut Game) {
        let card = self.draw_from_mountain();
        self.fields.insert(0, card);

        match self.fields[0] {
            Card::Number(_, _) => {}
            Card::Wild => {
                game.specify_color(Color::from(0));
            }
            Card::Reverse(_) => {
                game.reverse_order();
            }
            Card::Skip(_) => {
                game.skip_player();
            }
            Card::DrawTwo(_) => for _ in 0..2 {
                let card = self.draw_from_mountain();
                let player = &mut self.players[game.current_player];
                player.take_a_card(card);
            },
            Card::WildDrawFour => {
                let card = self.fields.remove(0);
                self.mountain.push(card);
                self.initialize_game(game);
            }
        };
    }

    fn draw_from_mountain(&mut self) -> Card {
        if self.mountain.len() == 0 {}
        return self.mountain.swap_remove(0);
    }

    pub fn show_scores(&self) {
        for player in &self.players {
            let score = player.calculate_score();
            let name = player.get_name();

            println!("{}: {}", name, score);
            // player.print_cards();
        }
    }
}
