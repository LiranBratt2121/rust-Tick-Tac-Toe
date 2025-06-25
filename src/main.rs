pub mod player;
pub mod playing_board;
pub mod game_logic;

use crate::player::Player;
use crate::game_logic::Game;

fn main() {
    let p1 = Player::P1 { name: "p1".to_string(), symbol: 'X' };
    let p2 = Player::P2 { name: "p2".to_string(), symbol: 'O' };
    
    let mut game = Game::new(p1, p2);

    game.game_loop();

    println!("{game:#?}");

    /*
    LOGIC
        Create Game obj.
        Struct Game {
            Player1: Player,
            Player2: Player,
            Board: Board,
            Turn: 1,
            Winner: None

            fn game_loop() -> None
        }

        enum Player {
            Name: String,
            Symbol: Char

            fn place(&mut board: Board, index: u8) -> bool
        }

        struct Board {
            Board: Arr[3][3]

            fn check_win(Player) -> bool
        }
        
        game.game_loop():
            while Turn <= 9 && !game.winner -> Game contiue.
            p1_place
            check_win
            if win -> break
            p2_place
            check_win
            if win -> break
            Turn += 1
     */
}
