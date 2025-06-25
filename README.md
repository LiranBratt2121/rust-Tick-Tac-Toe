Psudo game logic.
   
   /*
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

~~Tried to use as much rust features as possible lol~~

Stuff to improve and learn:
Start using the `Result` enum instead of booleans for better error control and to remove these ugly `"ifs"` in the game_loop function.
