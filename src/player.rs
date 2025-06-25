use crate::playing_board::Board;
use std::io;

#[derive(Debug, Clone)]
pub enum Player {
    P1 { name: String, symbol: char },
    P2 { name: String, symbol: char },
}

impl Player {
    pub fn symbol(&self) -> char {
        match self {
            Player::P1 { symbol, .. } => *symbol,
            Player::P2 { symbol, .. } => *symbol,
        }
    }

    pub fn name(&self) -> String {
        match self {
            Player::P1 { name, .. } => name.clone(),
            Player::P2 { name, .. } => name.clone(),
        }
    }

    pub fn display_winner(winner: &Player) -> () {
        println!("{} WINS!", winner.name());
    }

    pub fn display_tie() -> () {
        println!("OH NO! It's A TIE!");
    }

    pub fn place(&self, board: &mut Board) -> bool {
        let mut buf = String::new();

        println!("Hey {}! Your turn.", self.name());

        if io::stdin().read_line(&mut buf).is_err() {
            return false;
        }

        let Ok(index) = buf.trim().parse::<usize>() else {
            return false;
        };

        if index > 8 {
            return false;
        }

        let board_array = &mut board.board;

        let row = index / 3;
        let col = index % 3;

        let desired_location = &board_array[row as usize][col as usize];

        if desired_location.is_some() {
            return false;
        }

        board_array[row as usize][col as usize] = Some(self.clone());
        true
    }
}
