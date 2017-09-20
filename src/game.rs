use card::Color;

#[derive(Debug)]
pub enum Direction {
    Forward,
    Backward,
}

#[derive(Debug)]
pub struct Game {
    pub direction: Direction,
    pub color: Option<Color>,
    pub current_player: usize,
    pub players: usize
}

impl Game {
    pub fn new(players: usize) -> Game {
        Game {
            direction: Direction::Forward,
            color: None,
            current_player: 0,
            players: players,
        }
    }

    pub fn specify_color(&mut self, color: Color) {
        self.color = Some(color);
    }

    pub fn reverse_order(&mut self) {
        use self::Direction::*;
        self.direction = match self.direction {
            Forward => Backward,
            Backward => Forward,
        }
    }

    pub fn skip_player(&mut self) {
        self.change_player();
    }

    pub fn change_player(&mut self) {
        self.current_player = self.get_next_player();
    }

    pub fn get_next_player(&self) -> usize {
        let len = self.players;
        use self::Direction::*;

        match self.direction {
            Forward => {
                let index = self.current_player + 1;
                if index == len {
                    return 0;
                } else {
                    return index;
                }
            },
            Backward => {
                let index = self.current_player;
                if index == 0 {
                    return len - 1;
                } else {
                    return index - 1;
                }
            },
        }
    }
}