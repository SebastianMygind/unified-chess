use crate::chess::ChessBoard;

impl ChessBoard {
    pub fn perft(&self, depth: i64) -> i64 {
        let mut count: i64 = 0;

        let legal_moves = self.legal_moves();

        if depth > 1 {
            for legal_move in legal_moves {
                let mut new_chessboard = self.clone();

                let _move_result = match new_chessboard.make_move(legal_move) {
                    Ok(_) => {}

                    Err(_error) => {
                        println!("{new_chessboard}");
                        println!("Illegal move: {}", legal_move);
                        return -1;
                    }
                };

                count += new_chessboard.perft(depth - 1);
            }
        } else if depth == 1 {
            return self.legal_moves().len() as i64;
        } else if depth < 1 {
            return -1;
        }
        count
    }
}
