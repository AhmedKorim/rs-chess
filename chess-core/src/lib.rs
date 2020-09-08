#[allow(unsed)]
use std::fs::read_to_string;
use std::time::Instant;

#[derive(Clone, Copy, Debug)]
struct Coordinate {
    x: u8,
    y: u8,
}

#[derive(Clone, Debug)]
pub struct ChessPlayer {
    id: String,
    color: Color,
}

#[derive(Clone, Copy, Debug)]
pub enum Color {
    Black,
    White,
}
#[derive(Clone, Copy, Debug)]
pub enum PieceType {
    King,
    Queen,
    Bishop,
    Knight,
    Rook,
    Pawn,
}

#[derive(Clone, Copy, Debug)]
pub struct ChessPiece<'a> {
    id: (u8, u8),
    coordinate: Coordinate,
    player: &'a ChessPlayer,
    piece_type: PieceType,
}
impl<'a> ChessPiece {
    pub fn new(x: u8, y: u8, player: &'a ChessPlayer, piece_type: PieceType) -> Self {
        ChessPiece {
            coordinate: Coordinate { x, y },
            id: (x, y),
            piece_type,
            player,
        }
    }
}

pub type Date = String;

pub struct GameState {
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

pub struct ChessBoard {
    history: Vec<GameHistory>,
    current_state: GameHistory,
    pieces: Vec<ChessPiece<'_>>,
    tails: Vec<Tail>,
    current_player: ChessPlayer,
}

impl ChessBoard {
    pub fn new(player_a: ChessPlayer, player_b: ChessPlayer) -> Self {
        ChessBoard {
            current_player: player_a,
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
    use super::*;
    use std::rc::Rc;

    #[test]
    fn game_board_has_64_tails() {
        let player_1 = ChessPlayer {
            id: "one".to_string(),
            color: Color::Black,
        };
        let player_2 = ChessPlayer {
            id: "two".to_string(),
            color: Color::White,
        };

        let game = ChessBoard::new(player_1.clone(), player_1.clone());

        assert_eq!(game.tails.len(), 64);
        dbg!(&player_1, &player_2);
    }
    fn game_board_has_16_pieces() {}
}
