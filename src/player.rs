use rand;
use rand::Rng;
use card::{Card, Color};

#[derive(Debug)]
pub struct Player {
    name: String,
    cards: Vec<Card>,
}

impl Player {
    pub fn new(name: String) -> Player {
        Player {
            name: name,
            cards: Vec::new(),
        }
    }

    pub fn take_a_card(&mut self, card: Card) {
        self.cards.push(card);
    }

    pub fn put_down_a_card(&mut self, card: &Card, color: &Option<Color>) -> Option<Card> {
        let indexes = 0..self.cards.len();
        let availables: Vec<_> = match card {
            &Card::Number(color, number) => indexes
                .filter(|i| {
                    let card = &self.cards[*i];
                    match *card {
                        Card::Number(c, n) => {
                            if c == color {
                                return true;
                            } else if n == number {
                                return true;
                            }
                            return false;
                        }
                        Card::Reverse(c) => c == color,
                        Card::Skip(c) => c == color,
                        Card::DrawTwo(c) => c == color,
                        _ => true,
                    }
                })
                .collect(),
            &Card::Reverse(color) => indexes
                .filter(|i| {
                    let card = &self.cards[*i];
                    match *card {
                        Card::Number(c, _) => c == color,
                        Card::Skip(c) => c == color,
                        Card::DrawTwo(c) => c == color,
                        _ => true,
                    }
                })
                .collect(),
            &Card::Skip(color) => indexes
                .filter(|i| {
                    let card = &self.cards[*i];
                    match *card {
                        Card::Number(c, _) => c == color,
                        Card::Reverse(c) => c == color,
                        Card::DrawTwo(c) => c == color,
                        _ => true,
                    }
                })
                .collect(),
            &Card::DrawTwo(color) => indexes
                .filter(|i| {
                    let card = &self.cards[*i];
                    match *card {
                        Card::Number(c, _) => c == color,
                        Card::Reverse(c) => c == color,
                        Card::Skip(c) => c == color,
                        _ => true,
                    }
                })
                .collect(),
            _ => indexes.filter(|i| {
                let card = &self.cards[*i];
                let color = color.unwrap();
                match *card {
                    Card::Number(c, _) => c == color,
                    Card::Reverse(c) => c == color,
                    Card::Skip(c) => c == color,
                    Card::DrawTwo(c) => c == color,
                    _ => true,
                }
            }).collect(),
        };

        let len = availables.len();
        if len > 0 {
            let index = availables[rand::thread_rng().gen_range(0, len)];
            Some(self.cards.swap_remove(index))
        } else {
            None
        }
    }

    pub fn specify_color(&self) -> Color {
        let index = rand::thread_rng().gen_range(0, 4);
        Color::from(index)
    }

    pub fn get_len(&self) -> usize {
        self.cards.len()
    }

    pub fn get_name(&self) -> String {
        format!("{}", &self.name)
    }

    pub fn calculate_score(&self) -> u32 {
        self.cards.iter().map(|card| card.to_score()).sum()
    }

    // pub fn print_cards(&self) {
    //     println!("{:?}", self.cards);
    // }
}
