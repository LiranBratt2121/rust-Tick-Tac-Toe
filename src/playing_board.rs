use crate::player::Player;

#[derive(Debug)]
pub struct Board {
    pub board: [[Option<Player>; 3]; 3],
}

impl Board {
    pub fn new() -> Board {
        let board: [[Option<Player>; 3]; 3] = Default::default();
        Board { board }
    }

    pub fn display(&self) -> () {
        println!();
        
        for i in 0..3 {
            for j in 0..3 {
                let symbol = match &self.board[i][j] {
                    Some(player) => player.symbol(),
                    None => ' ',
                };

                print!("{symbol} ");
            }
            println!();
        }
    }

    fn has_a_winner(&self, player: &Player) -> Option<Player> {
        let board = &self.board;
        let symbol = player.symbol();

        let matches =
            |x: usize, y: usize| board[x][y].as_ref().is_some_and(|p| p.symbol() == symbol);

        // Check rows
        for row in 0..3 {
            if (0..3).all(|col| matches(row, col)) {
                return Some(player.clone());
            }
        }

        // Check columns
        for col in 0..3 {
            if (0..3).all(|row| matches(row, col)) {
                return Some(player.clone());
            }
        }

        // Check diagonals
        if (0..3).all(|i| matches(i, i)) || (0..3).all(|i| matches(i, 2 - i)) {
            return Some(player.clone());
        }

        None
    }

    pub fn check_win(&self, player: &Player, winner_ref: &mut Option<Player>) -> () {
        self.has_a_winner(player)
            .map(|winner| {
                Player::display_winner(&winner);
                *winner_ref = Some(winner.clone());
            });
    }
}
