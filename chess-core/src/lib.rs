use std::time::Instant;

struct Coordinate {
    x: u8,
    y: u8,
}

struct ChessPlayer {
    id: String,
    color: Color,
}

enum Color {
    Black,
    White,
}
enum PieceType {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}
struct ChessPiece {
    id: u8,
    coordinate: Coordinate,
    player: ChessPlayer,
    piece_type: PieceType,
}

type Date = String;

struct GameState {
    piece_id: u8,
    coordinate: Coordinate,
}
impl GameState {
    pub fn new(coordinate: Coordinate, piece_id: u8) -> Self {
        GameState {
            coordinate,
            piece_id,
        }
    }
}
struct GameHistory {
    start_date: Date,
    end_date: Option<Date>,
    moves: Vec<GameState>,
}
impl GameHistory {
    pub fn new() -> Self {
        GameHistory {
            end_date: None,
            start_date: "now".to_string(),
            moves: Vec::new(),
        }
    }

    pub fn append_to_history(&mut self, game_state: GameState) {
        self.moves.push(game_state)
    }
}
struct ChessBoard {
    history: Vec<GameHistory>,
    current_state: GameHistory,
    pieces: Vec<ChessPiece>,
    current_player: ChessPlayer,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
