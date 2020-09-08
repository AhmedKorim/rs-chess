#![feature(in_band_lifetimes)]

#[derive(Clone, Copy, Debug)]
pub struct Coordinate {
    x: u8,
    y: u8,
}

impl Coordinate {
    pub fn new(x: u8, y: u8) -> Self {
        Coordinate { x, y }
    }
    pub fn create_right(&self) -> Self {
        Coordinate {
            x: self.x + 1,
            ..self.clone()
        }
    }
    pub fn create_left(&self) -> Self {
        Coordinate {
            x: self.x - 1,
            ..self.clone()
        }
    }
    pub fn create_top(&self) -> Self {
        Coordinate {
            y: self.y + 1,
            ..self.clone()
        }
    }
    pub fn create_bottom(&self) -> Self {
        Coordinate {
            y: self.y - 1,
            ..self.clone()
        }
    }
    pub fn create_bottom_left(&self) -> Self {
        Coordinate {
            x: self.x - 1,
            y: self.y - 1,
        }
    }
    pub fn create_bottom_right(&self) -> Self {
        Coordinate {
            x: self.x + 1,
            y: self.y - 1,
        }
    }
    pub fn create_top_left(&self) -> Self {
        Coordinate {
            x: self.x - 1,
            y: self.y + 1,
        }
    }
    pub fn create_top_right(&self) -> Self {
        Coordinate {
            x: self.x + 1,
            y: self.y + 1,
        }
    }
    /// lines
    pub fn line_x(&self) -> Vec<Self> {
        let mut line = Vec::new();
        for x in 0..8 {
            line.push(Coordinate::new(x, self.y))
        }
        line
    }

    pub fn line_x_left(&self) -> Vec<Self> {
        let mut line = Vec::new();
        for x in 0..(slef.x + 1) {
            line.push(Coordinate::new(x, self.y))
        }
        line
    }
    pub fn line_x_right(&self) -> Vec<Self> {
        let mut line = Vec::new();
        for x in slef.x..8 {
            line.push(Coordinate::new(x, self.y))
        }
        line
    }
    pub fn line_y(&self) -> Vec<Self> {
        let mut line = Vec::new();
        for y in 0..8 {
            line.push(Coordinate::new(self.x, y))
        }
        line
    }

    pub fn line_y_top(&self) -> Vec<Self> {
        let mut line = Vec::new();
        for y in self.y..8 {
            line.push(Coordinate::new(self.x, y))
        }
        line
    }
    pub fn line_y_bottom(&self) -> Vec<Self> {
        let mut line = Vec::new();
        for y in 0..(self.y + 1) {
            line.push(Coordinate::new(self.x, y))
        }
        line
    }

    pub fn line_p45(&self) -> Vec<Self> {
        Vec::new()
    }
    pub fn line_p45_top(&self) -> Vec<Self> {
        Vec::new()
    }
    pub fn line_p45_bottom(&self) -> Vec<Self> {
        Vec::new()
    }
    pub fn line_n45(&self) -> Vec<Self> {
        Vec::new()
    }
    pub fn line_n45_top(&self) -> Vec<Self> {
        Vec::new()
    }
    pub fn line_n45_bottom(&self) -> Vec<Self> {
        Vec::new()
    }
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
pub struct ChessPiece<'player> {
    id: (u8, u8),
    coordinate: Coordinate,
    player: &'player ChessPlayer,
    piece_type: PieceType,
}

impl<'player> ChessPiece<'player> {
    pub fn new(x: u8, y: u8, player: &'player ChessPlayer, piece_type: PieceType) -> Self {
        ChessPiece {
            coordinate: Coordinate { x, y },
            id: (x, y),
            piece_type,
            player,
        }
    }
    pub fn next_moves(&self) -> Vec<Coordinate> {
        let mut next_moves = Vec::new();
        let Coordinate { x, y } = self.coordinate;
        let can_go_up = y != 7;
        let can_go_down = y != 0;
        let can_go_right = x != 7;
        let can_go_left = x != 0;

        match self.piece_type {
            PieceType::King => {
                if can_go_up {
                    next_moves.push(self.coordinate.crate_top())
                }
                if can_go_down {
                    next_moves.push(self.coordinate.create_bottom());
                }
                if can_go_left {
                    next_moves.push(self.coordinate.create_left())
                }
                if can_go_right {
                    next_moves.push(self.coordinate.create_right())
                }

                if can_go_right && can_go_up {
                    next_moves.push(self.coordinate.create_top_right())
                }
                if can_go_right && can_go_down {
                    next_moves.push(self.coordinate.create_bottom_right())
                }
                if can_go_left && can_go_up {
                    next_moves.push(self.coordinate.create_top_left())
                }
                if can_go_left && can_go_down {
                    next_moves.push(self.coordinate.create_bottom_left())
                }
            }
            PieceType::Queen => {
                if can_go_up {
                    next_moves.push(self.coordinate.line_y_top())
                }
                if can_go_down {
                    next_moves.push(self.coordinate.line_y_bottom());
                }
                if can_go_left {
                    next_moves.push(self.coordinate.line_x_left())
                }
                if can_go_right {
                    next_moves.push(self.coordinate.line_x_right())
                }

                if can_go_right && can_go_up {
                    next_moves.push(self.coordinate.line_p45_top())
                }
                if can_go_right && can_go_down {
                    next_moves.push(self.coordinate.line_p45_bottom())
                }
                if can_go_left && can_go_up {
                    next_moves.push(self.coordinate.line_n45_top())
                }
                if can_go_left && can_go_down {
                    next_moves.push(self.coordinate.line_n45_bottom())
                }
            }
            PieceType::Bishop => {
                if can_go_right && can_go_up {
                    next_moves.push(self.coordinate.line_p45_top())
                }
                if can_go_right && can_go_down {
                    next_moves.push(self.coordinate.line_p45_bottom())
                }
                if can_go_left && can_go_up {
                    next_moves.push(self.coordinate.line_n45_top()())
                }
                if can_go_left && can_go_down {
                    next_moves.push(self.coordinate.line_n45_bottom())
                }
            }
            PieceType::Knight => {
                let have_3_tails_up = y <= 7 - 3;
                let have_3_tails_down = y >= 3;
                let have_3_tails_right = x <= 7 - 3;
                let have_3_tails_left = x >= 3;
                if have_3_tails_up {
                    if can_go_left {
                        next_moves.push(Coordinate::new(x - 1, y + 3))
                    }
                    if can_go_right {
                        next_moves.push(Coordinate::new(x + 1, y + 3))
                    }
                }
                if have_3_tails_down {
                    if can_go_left {
                        next_moves.push(Coordinate::new(x - 1, y - 3))
                    }
                    if can_go_right {
                        next_moves.push(Coordinate::new(x + 1, y - 3))
                    }
                }
                if have_3_tails_left {
                    if can_go_up {
                        next_moves.push(Coordinate::new(x - 3, y + 1))
                    }
                    if can_go_down {
                        next_moves.push(Coordinate::new(x - 3, y - 1))
                    }
                }
                if have_3_tails_right {
                    if can_go_up {
                        next_moves.push(Coordinate::new(x + 3, y + 1))
                    }
                    if can_go_down {
                        next_moves.push(Coordinate::new(x + 3, y - 1))
                    }
                }
            }
            PieceType::Rook => {
                if can_go_up {
                    next_moves.push(self.coordinate.line_y_top())
                }
                if can_go_down {
                    next_moves.push(self.coordinate.line_y_bottom());
                }
                if can_go_left {
                    next_moves.push(self.coordinate.line_x_left())
                }
                if can_go_right {
                    next_moves.push(self.coordinate.line_x_right())
                }
            }
            PieceType::Pawn => {
                // todo prevent from glowing that the player can't
                if can_go_up {
                    next_moves.push(self.coordinate.crate_top());
                    if can_go_left {
                        next_moves.push(self.coordinate.create_top_left());
                    }
                    if can_go_right {
                        next_moves.push(self.coordinate.create_top_right());
                    }
                }
                if can_go_down {
                    next_moves.push(self.coordinate.create_bottom());
                    if can_go_left {
                        next_moves.push(self.coordinate.create_bottom_left());
                    }
                    if can_go_right {
                        next_moves.push(self.coordinate.create_bottom_right());
                    }
                }
            }
        }
        next_moves
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

pub struct ChessBoard<'player> {
    history: Vec<GameHistory>,
    current_state: GameHistory,
    pieces: Vec<ChessPiece<'player>>,
    tails: Vec<Tail>,
    current_player: &'player ChessPlayer,
}

impl<'player> ChessBoard<'player> {
    pub fn new(player_a: &'player ChessPlayer, player_b: &'player ChessPlayer) -> Self {
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

fn init_pieces(player: &'a ChessPlayer, top: bool) -> Vec<ChessPiece<'a>> {
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

    #[test]
    fn game_board_has_64_tails() {
        let player_1: ChessPlayer = ChessPlayer {
            id: "one".to_string(),
            color: Color::Black,
        };
        let player_2: ChessPlayer = ChessPlayer {
            id: "two".to_string(),
            color: Color::White,
        };

        let game = ChessBoard::new(&player_1, &player_2);

        assert_eq!(game.tails.len() as u64, 64 as u64);
        dbg!(&player_1, &player_2);
    }

    fn game_board_has_16_pieces() {}
}
