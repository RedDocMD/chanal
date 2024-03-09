use crate::chess::{Colour, Piece};

pub static WOOD4_JPG: &[u8] = include_bytes!("assets/wood4.jpg");

pub static MERIDA_WHITE_PAWN: &[u8] = include_bytes!("assets/merida/wP.svg");
pub static MERIDA_WHITE_ROOK: &[u8] = include_bytes!("assets/merida/wR.svg");
pub static MERIDA_WHITE_KNIGHT: &[u8] = include_bytes!("assets/merida/wN.svg");
pub static MERIDA_WHITE_BISHOP: &[u8] = include_bytes!("assets/merida/wB.svg");
pub static MERIDA_WHITE_QUEEN: &[u8] = include_bytes!("assets/merida/wQ.svg");
pub static MERIDA_WHITE_KING: &[u8] = include_bytes!("assets/merida/wK.svg");
pub static MERIDA_BLACK_PAWN: &[u8] = include_bytes!("assets/merida/bP.svg");
pub static MERIDA_BLACK_ROOK: &[u8] = include_bytes!("assets/merida/bR.svg");
pub static MERIDA_BLACK_KNIGHT: &[u8] = include_bytes!("assets/merida/bN.svg");
pub static MERIDA_BLACK_BISHOP: &[u8] = include_bytes!("assets/merida/bB.svg");
pub static MERIDA_BLACK_QUEEN: &[u8] = include_bytes!("assets/merida/bQ.svg");
pub static MERIDA_BLACK_KING: &[u8] = include_bytes!("assets/merida/bK.svg");

pub fn merida_piece_data(piece: Piece, colour: Colour) -> &'static [u8] {
    match (piece, colour) {
        (Piece::Pawn, Colour::White) => MERIDA_WHITE_PAWN,
        (Piece::Pawn, Colour::Black) => MERIDA_BLACK_PAWN,
        (Piece::Rook, Colour::White) => MERIDA_WHITE_ROOK,
        (Piece::Rook, Colour::Black) => MERIDA_BLACK_ROOK,
        (Piece::Knight, Colour::White) => MERIDA_WHITE_KNIGHT,
        (Piece::Knight, Colour::Black) => MERIDA_BLACK_KNIGHT,
        (Piece::Bishop, Colour::White) => MERIDA_WHITE_BISHOP,
        (Piece::Bishop, Colour::Black) => MERIDA_BLACK_BISHOP,
        (Piece::King, Colour::White) => MERIDA_WHITE_KING,
        (Piece::King, Colour::Black) => MERIDA_BLACK_KING,
        (Piece::Queen, Colour::White) => MERIDA_WHITE_QUEEN,
        (Piece::Queen, Colour::Black) => MERIDA_BLACK_QUEEN,
    }
}

pub static MOVE_OGG: &[u8] = include_bytes!("assets/sounds/Move.ogg");
pub static CAPTURE_OGG: &[u8] = include_bytes!("assets/sounds/Capture.ogg");
pub static CHECK_OGG: &[u8] = include_bytes!("assets/sounds/Check.ogg");
