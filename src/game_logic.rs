use crate::player::Player;
use crate::playing_board::Board;

#[derive(Debug)]
pub struct Game {
    pub board: Board,
    pub current_player: Player,
    pub other_player: Player,
    moves_made: u8,
    pub winner: Option<Player>,
}

impl Game {
    pub fn new(player1: Player, player2: Player) -> Self {
        Game {
            board: Board::new(),
            current_player: player1,
            other_player: player2,
            moves_made: 1,
            winner: None,
        }
    }

    pub fn increment_moves(&mut self) {
        self.moves_made += 1;
    }

    pub fn game_loop(&mut self) -> () {
        println!("Welcome to tic tac toe!");
        println!(
            "{} VS {}",
            self.current_player.name(),
            self.other_player.name()
        );

        self.board.display();

        loop {
            if self.moves_made == 9 && self.winner.is_none() {
                Player::display_tie();
                break;
            }

            // Handle turn for both players
            for player in [&self.current_player, &self.other_player] {
                while !player.place(&mut self.board) {}
                self.board.display();
                self.board.check_win(player, &mut self.winner);

                if self.winner.is_some() {
                    break;
                }
            }

            self.increment_moves();
        }

        println!("Game info {:#?}", self);
    }
}
