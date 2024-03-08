use enum_iterator::Sequence;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Sequence)]
pub enum Piece {
    Pawn,
    Rook,
    Knight,
    Bishop,
    King,
    Queen,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Sequence)]
pub enum Colour {
    White,
    Black,
}

#[derive(Debug, Clone, Copy)]
pub enum Position {
    Empty,
    Occupied(Piece, Colour),
}

pub const BOARD_SIZE: usize = 8;

#[derive(Debug)]
pub struct Board([[Position; BOARD_SIZE]; BOARD_SIZE]);

#[derive(Debug)]
pub struct Game {
    board: Board,
}

impl Game {
    pub fn new() -> Self {
        let board = [[Position::Empty; BOARD_SIZE]; BOARD_SIZE];
        Self {
            board: Board(board),
        }
    }
}
