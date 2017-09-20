use rand;
use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    Blue,
    Red,
    Green,
    Yellow,
}

impl From<usize> for Color {
    fn from(index: usize) -> Color {
        use self::Color::*;

        match index {
            0 => Blue,
            1 => Red,
            2 => Green,
            3 => Yellow,
            _ => panic!("out of index"),
        }
    }
}

#[derive(Debug)]
pub enum Card {
    Number(Color, u32),
    DrawTwo(Color),
    Reverse(Color),
    Skip(Color),
    Wild,
    WildDrawFour,
}

impl Card {
    pub fn init_cards() -> Vec<Card> {
        use self::Card::*;
        let mut cards: Vec<Card> = Vec::with_capacity(112);

        for c in 0..4 {
            for i in 0..10 {
                match i {
                    0 => {
                        let color = Color::from(c);
                        cards.push(Number(color, i));
                    }
                    _ => for _ in 0..2 {
                        let color = Color::from(c);
                        cards.push(Number(color, i));
                    },
                }
            }

            for _ in 0..2 {
                let color = Color::from(c);
                cards.push(DrawTwo(color));

                let color = Color::from(c);
                cards.push(Reverse(color));

                let color = Color::from(c);
                cards.push(Skip(color));
            }
        }

        for _ in 0..4 {
            cards.push(Wild);
            cards.push(WildDrawFour);
        }

        rand::thread_rng().shuffle(&mut cards);
        cards
    }

    pub fn to_score(&self) -> u32 {
        use self::Card::*;

        match *self {
            Number(_, number) => number,
            Skip(_) | DrawTwo(_) | Reverse(_) => 20,
            Wild | WildDrawFour => 50,
        }
    }
}
