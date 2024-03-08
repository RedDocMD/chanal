use std::str::FromStr;

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
pub struct Board(pub [[Position; BOARD_SIZE]; BOARD_SIZE]);

#[derive(Debug)]
pub struct Game {
    fen: Fen,
}

impl Game {
    pub fn new() -> Self {
        const INIT_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let fen = INIT_FEN.parse::<Fen>().unwrap();
        Self { fen }
    }

    pub fn board(&self) -> &Board {
        &self.fen.board
    }
}

#[derive(Debug)]
struct Fen {
    board: Board,
    to_move: Colour,
    white_king_castle: bool,
    white_queen_castle: bool,
    black_king_castle: bool,
    black_queen_castle: bool,
    en_passant: Option<(usize, usize)>,
    halfmove_clock: u32,
    move_cnt: u32,
}

#[derive(Debug, thiserror::Error)]
enum FenParseError {
    #[error("Insufficient parts, expected 6 but got {0}")]
    InsufficientParts(usize),

    #[error("Invalid colour to move: {0}")]
    InvalidToMove(String),

    #[error("Invalid castle character: {0}")]
    InvalidCastleCharacter(char),

    #[error("Invalid square len: {0}")]
    InvalidSquareLen(usize),

    #[error("Invalid file: {0}")]
    InvalidFile(char),

    #[error("Invalid rank: {0}")]
    InvalidRank(char),

    #[error("Number parse failed: {0}")]
    InvalidNumber(String),

    #[error("Insufficient ranks in position, expected 8 got {0}")]
    InsufficientRanks(usize),

    #[error("Invalid piece: {0}")]
    InvalidPiece(char),
}

impl FromStr for Fen {
    type Err = FenParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(' ').collect();
        if parts.len() != 6 {
            return Err(FenParseError::InsufficientParts(parts.len()));
        }

        let board = parse_position(parts[0])?;

        let to_move = if parts[1] == "w" {
            Colour::White
        } else if parts[1] == "b" {
            Colour::Black
        } else {
            return Err(FenParseError::InvalidToMove(parts[1].to_string()));
        };

        let mut white_king_castle = false;
        let mut white_queen_castle = false;
        let mut black_king_castle = false;
        let mut black_queen_castle = false;
        if parts[2] != "-" {
            for b in parts[2].bytes() {
                if b == b'K' {
                    white_king_castle = true;
                } else if b == b'Q' {
                    white_queen_castle = true;
                } else if b == b'k' {
                    black_king_castle = true;
                } else if b == b'q' {
                    black_queen_castle = true;
                } else {
                    return Err(FenParseError::InvalidCastleCharacter(b as char));
                }
            }
        }

        let en_passant = if parts[3] == "-" {
            None
        } else {
            let sq = parse_square(parts[3])?;
            Some(sq)
        };

        let halfmove_clock = parts[4]
            .parse::<u32>()
            .map_err(|e| FenParseError::InvalidNumber(e.to_string()))?;

        let move_cnt = parts[4]
            .parse::<u32>()
            .map_err(|e| FenParseError::InvalidNumber(e.to_string()))?;

        Ok(Fen {
            board,
            to_move,
            white_king_castle,
            white_queen_castle,
            black_king_castle,
            black_queen_castle,
            en_passant,
            halfmove_clock,
            move_cnt,
        })
    }
}

fn parse_square(s: &str) -> Result<(usize, usize), FenParseError> {
    if s.len() != 2 {
        return Err(FenParseError::InvalidSquareLen(s.len()));
    }
    let sb = s.as_bytes();
    let file = sb[0];
    let rank = sb[1];
    if !(b'a'..=b'h').contains(&file) {
        return Err(FenParseError::InvalidFile(file as char));
    }
    if !(b'1'..=b'8').contains(&rank) {
        return Err(FenParseError::InvalidRank(rank as char));
    }
    Ok(((8 - (rank - b'0')) as usize, (file - b'a') as usize))
}

fn parse_position(position: &str) -> Result<Board, FenParseError> {
    let mut board = [[Position::Empty; BOARD_SIZE]; BOARD_SIZE];
    let ranks: Vec<_> = position.split('/').collect();
    if ranks.len() != 8 {
        return Err(FenParseError::InsufficientRanks(ranks.len()));
    }
    for (rank, rank_str) in ranks.into_iter().enumerate() {
        let mut file = 0;
        for b in rank_str.bytes() {
            match b {
                b'p' => {
                    board[rank][file] = Position::Occupied(Piece::Pawn, Colour::Black);
                    file += 1;
                }
                b'P' => {
                    board[rank][file] = Position::Occupied(Piece::Pawn, Colour::White);
                    file += 1;
                }
                b'r' => {
                    board[rank][file] = Position::Occupied(Piece::Rook, Colour::Black);
                    file += 1;
                }
                b'R' => {
                    board[rank][file] = Position::Occupied(Piece::Rook, Colour::White);
                    file += 1;
                }
                b'n' => {
                    board[rank][file] = Position::Occupied(Piece::Knight, Colour::Black);
                    file += 1;
                }
                b'N' => {
                    board[rank][file] = Position::Occupied(Piece::Knight, Colour::White);
                    file += 1;
                }
                b'b' => {
                    board[rank][file] = Position::Occupied(Piece::Bishop, Colour::Black);
                    file += 1;
                }
                b'B' => {
                    board[rank][file] = Position::Occupied(Piece::Bishop, Colour::White);
                    file += 1;
                }
                b'k' => {
                    board[rank][file] = Position::Occupied(Piece::King, Colour::Black);
                    file += 1;
                }
                b'K' => {
                    board[rank][file] = Position::Occupied(Piece::King, Colour::White);
                    file += 1;
                }
                b'q' => {
                    board[rank][file] = Position::Occupied(Piece::Queen, Colour::Black);
                    file += 1;
                }
                b'Q' => {
                    board[rank][file] = Position::Occupied(Piece::Queen, Colour::White);
                    file += 1;
                }
                b'1'..=b'8' => file += (b - b'0') as usize,
                _ => return Err(FenParseError::InvalidPiece(b as char)),
            }
        }
    }
    Ok(Board(board))
}
