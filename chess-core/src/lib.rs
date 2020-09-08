use std::fs::read_to_string;
use std::time::Instant;

#[derive(Clone, Copy, Debug)]
struct Coordinate {
    x: u8,
    y: u8,
}

#[derive(Clone, Debug)]
struct ChessPlayer {
    id: String,
    color: Color,
}

#[derive(Clone, Copy, Debug)]
enum Color {
    Black,
    White,
}
#[derive(Clone, Copy, Debug)]
enum PieceType {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

#[derive(Clone, Copy, Debug)]
struct ChessPiece {
    id: (u8, u8),
    coordinate: Coordinate,
    player: &'static ChessPlayer,
    piece_type: PieceType,
}
impl ChessPiece {
    pub fn new(x: u8, y: u8, player: &'static ChessPlayer, piece_type: PieceType) -> Self {
        ChessPiece {
            coordinate: Coordinate { x, y },
            id: (x, y),
            piece_type,
            player,
        }
    }
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

struct Tail {
    coordinate: Coordinate,
    color: Color,
}

struct ChessBoard {
    history: Vec<GameHistory>,
    current_state: GameHistory,
    pieces: Vec<ChessPiece>,
    tails: Vec<Tail>,
    current_player: &'static ChessPlayer,
}

impl ChessBoard {
    pub fn new(player_a: &'static ChessPlayer, player_b: &'static ChessPlayer) -> Self {
        ChessBoard {
            current_player: &player_a,
            current_state: GameHistory::new(),
            tails: init_tails(),
            history: Vec::new(),
            pieces: [init_pieces(&player_a, true), init_pieces(&player_b, false)].concat(),
        }
    }
}

fn init_tails() -> Vec<Tail> {
    let mut tails = Vec::with_capacity(64);
    for x in 0..8 {
        for y in 0..8 {
            tails.push(Tail {
                coordinate: Coordinate { x, y },
                color: if y % 2 == 0 {
                    Color::Black
                } else {
                    Color::White
                },
            })
        }
    }
    tails
}

fn init_pieces(player: &'static ChessPlayer, top: bool) -> Vec<ChessPiece> {
    let first_row_y = if top { 7 } else { 0 };
    let second_row_y = if top { 6 } else { 1 };
    let mut pieces = vec![
        ChessPiece::new(0, first_row_y, player, PieceType::Rook),
        ChessPiece::new(1, first_row_y, player, PieceType::Knight),
        ChessPiece::new(2, first_row_y, player, PieceType::Bishop),
        ChessPiece::new(3, first_row_y, player, PieceType::Queen),
        ChessPiece::new(4, first_row_y, player, PieceType::King),
        ChessPiece::new(5, first_row_y, player, PieceType::Bishop),
        ChessPiece::new(6, first_row_y, player, PieceType::Knight),
        ChessPiece::new(7, first_row_y, player, PieceType::Rook),
    ];

    for x in 0..8 {
        let piece = ChessPiece::new(x, second_row_y, &player, PieceType::Pawn);
        pieces.push(piece);
    }
    pieces
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
