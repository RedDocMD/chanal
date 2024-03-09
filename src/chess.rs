use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
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

impl Colour {
    const fn opposite(self) -> Self {
        match self {
            Colour::White => Colour::Black,
            Colour::Black => Colour::White,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Position {
    Empty,
    Occupied(Piece, Colour),
    Picked(Piece, Colour),
}

pub const BOARD_SIZE: usize = 8;

#[derive(Debug, Clone, Copy)]
pub struct Board(pub [[Position; BOARD_SIZE]; BOARD_SIZE]);

impl Deref for Board {
    type Target = [[Position; BOARD_SIZE]; BOARD_SIZE];

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Board {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Default)]
struct KingCheckCnt {
    white: usize,
    black: usize,
}

impl KingCheckCnt {
    fn check_cnt(&self, colour: Colour) -> usize {
        match colour {
            Colour::White => self.white,
            Colour::Black => self.black,
        }
    }
}

impl Board {
    fn apply_move(&self, mov: Move) -> Board {
        let (fr, ff) = mov.from;
        let (tr, tf) = mov.to;
        let mut board = *self;
        board[fr][ff] = Position::Empty;
        if let Some(cap) = mov.capture {
            let (cr, cf) = cap.pos;
            board[cr][cf] = Position::Empty;
        }
        board[tr][tf] = Position::Occupied(mov.piece, mov.colour);
        if mov.piece == Piece::King {
            if mov.colour == Colour::White {
                if mov.from == (7, 4) && mov.to == (7, 6) {
                    board[7][7] = Position::Empty;
                    board[7][5] = Position::Occupied(Piece::Rook, Colour::White);
                } else if mov.from == (7, 4) && mov.to == (7, 2) {
                    board[7][0] = Position::Empty;
                    board[7][3] = Position::Occupied(Piece::Rook, Colour::White);
                }
            } else if mov.from == (0, 4) && mov.to == (0, 6) {
                board[0][7] = Position::Empty;
                board[0][5] = Position::Occupied(Piece::Rook, Colour::Black);
            } else if mov.from == (0, 4) && mov.to == (0, 2) {
                board[0][0] = Position::Empty;
                board[0][3] = Position::Occupied(Piece::Rook, Colour::Black);
            }
        }
        board
    }

    fn move_verify_checks(&self, mov: &mut Move) -> bool {
        let nb = self.apply_move(*mov);
        let kic = nb.king_check_cnt();
        if kic.check_cnt(mov.colour) != 0 {
            false
        } else {
            mov.check_cnt = kic.check_cnt(mov.colour.opposite());
            true
        }
    }

    fn king_check_cnt(&self) -> KingCheckCnt {
        let mut kic = KingCheckCnt::default();
        for rank in 0..BOARD_SIZE {
            for file in 0..BOARD_SIZE {
                let pos = self[rank][file];
                if let Position::Occupied(Piece::King, Colour::White) = pos {
                    kic.white = self.king_check_cnt_colour(rank, file, Colour::White);
                } else if let Position::Occupied(Piece::King, Colour::Black) = pos {
                    kic.black = self.king_check_cnt_colour(rank, file, Colour::Black);
                }
            }
        }
        kic
    }

    fn king_check_cnt_colour(&self, rank: usize, file: usize, colour: Colour) -> usize {
        let mut check_cnt = 0;
        // Find knight checks
        for (nr, nf) in knight_distance_positions(rank, file) {
            let pos = self[nr][nf];
            if matches!(pos, Position::Occupied(Piece::Knight, col) if col == colour.opposite()) {
                check_cnt += 1;
            }
        }
        // Find file checks (from rook and queen)
        for nr in (0..rank).rev() {
            let pos = self[nr][file];
            match pos {
                Position::Empty | Position::Picked(_, _) => continue,
                Position::Occupied(np, nc) => {
                    if nc == colour.opposite() && (np == Piece::Rook || np == Piece::Queen) {
                        check_cnt += 1;
                    }
                    break;
                }
            }
        }
        for nr in rank + 1..BOARD_SIZE {
            let pos = self[nr][file];
            match pos {
                Position::Empty | Position::Picked(_, _) => continue,
                Position::Occupied(np, nc) => {
                    if nc == colour.opposite() && (np == Piece::Rook || np == Piece::Queen) {
                        check_cnt += 1;
                    }
                    break;
                }
            }
        }
        // Find rank checks (from rook and queen)
        for nf in (0..file).rev() {
            let pos = self[rank][nf];
            match pos {
                Position::Empty | Position::Picked(_, _) => continue,
                Position::Occupied(np, nc) => {
                    if nc == colour.opposite() && (np == Piece::Rook || np == Piece::Queen) {
                        check_cnt += 1;
                    }
                    break;
                }
            }
        }
        for nf in file + 1..BOARD_SIZE {
            let pos = self[rank][nf];
            match pos {
                Position::Empty | Position::Picked(_, _) => continue,
                Position::Occupied(np, nc) => {
                    if nc == colour.opposite() && (np == Piece::Rook || np == Piece::Queen) {
                        check_cnt += 1;
                    }
                    break;
                }
            }
        }
        // Find diagonal checks (from queen and bishop)
        for diff in 1..=rank.min(file) {
            let nr = rank - diff;
            let nf = file - diff;
            let pos = self[nr][nf];
            match pos {
                Position::Empty | Position::Picked(_, _) => continue,
                Position::Occupied(np, nc) => {
                    if nc == colour.opposite() && (np == Piece::Bishop || np == Piece::Queen) {
                        check_cnt += 1;
                    }
                    break;
                }
            }
        }
        for diff in 1..(BOARD_SIZE - rank).min(BOARD_SIZE - file) {
            let nr = rank + diff;
            let nf = file + diff;
            let pos = self[nr][nf];
            match pos {
                Position::Empty | Position::Picked(_, _) => continue,
                Position::Occupied(np, nc) => {
                    if nc == colour.opposite() && (np == Piece::Bishop || np == Piece::Queen) {
                        check_cnt += 1;
                    }
                    break;
                }
            }
        }
        for diff in 1..rank.min(BOARD_SIZE - file) {
            let nr = rank - diff;
            let nf = file + diff;
            let pos = self[nr][nf];
            match pos {
                Position::Empty | Position::Picked(_, _) => continue,
                Position::Occupied(np, nc) => {
                    if nc == colour.opposite() && (np == Piece::Bishop || np == Piece::Queen) {
                        check_cnt += 1;
                    }
                    break;
                }
            }
        }
        for diff in 1..(BOARD_SIZE - rank).min(file) {
            let nr = rank + diff;
            let nf = file - diff;
            let pos = self[nr][nf];
            match pos {
                Position::Empty | Position::Picked(_, _) => continue,
                Position::Occupied(np, nc) => {
                    if nc == colour.opposite() && (np == Piece::Bishop || np == Piece::Queen) {
                        check_cnt += 1;
                    }
                    break;
                }
            }
        }
        // Find pawn checks
        for (nr, nf) in attacking_pawn_positions(rank, file, colour) {
            let pos = self[nr][nf];
            if matches!(pos, Position::Occupied(Piece::Pawn, col) if col == colour.opposite()) {
                check_cnt += 1;
            }
        }
        check_cnt
    }
}

fn diff_positions(
    rank: usize,
    file: usize,
    rank_diff: &[isize],
    file_diff: &[isize],
) -> Vec<(usize, usize)> {
    rank_diff
        .iter()
        .zip(file_diff)
        .filter_map(|(rd, fd)| {
            let new_rank = rank as isize + *rd;
            let new_file = file as isize + *fd;
            if new_rank < 0
                || new_rank >= BOARD_SIZE as isize
                || new_file < 0
                || new_file >= BOARD_SIZE as isize
            {
                None
            } else {
                Some((new_rank as usize, new_file as usize))
            }
        })
        .collect()
}

fn attacking_pawn_positions(rank: usize, file: usize, colour: Colour) -> Vec<(usize, usize)> {
    let (rank_diff, file_diff) = if colour == Colour::Black {
        ([1, 1], [-1, 1])
    } else {
        ([-1, -1], [-1, 1])
    };
    diff_positions(rank, file, &rank_diff, &file_diff)
}

fn knight_distance_positions(rank: usize, file: usize) -> Vec<(usize, usize)> {
    const RANK_DIFF: [isize; 8] = [-2, -2, 2, 2, -1, -1, 1, 1];
    const FILE_DIFF: [isize; 8] = [-1, 1, -1, 1, -2, 2, -2, 2];
    diff_positions(rank, file, &RANK_DIFF, &FILE_DIFF)
}

fn king_distance_positions(rank: usize, file: usize) -> Vec<(usize, usize)> {
    const RANK_DIFF: [isize; 8] = [-1, 0, 1, -1, 1, -1, 0, 1];
    const FILE_DIFF: [isize; 8] = [-1, -1, -1, 0, 0, 1, 1, 1];
    diff_positions(rank, file, &RANK_DIFF, &FILE_DIFF)
}

#[derive(Debug)]
pub struct Game {
    fen: Fen,
    moves: Vec<Move>,
}

impl Game {
    pub fn new() -> Self {
        const INIT_FEN: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
        let fen = INIT_FEN.parse::<Fen>().unwrap();
        Self {
            fen,
            moves: Vec::new(),
        }
    }

    pub fn board(&self) -> &Board {
        &self.fen.board
    }

    pub fn board_mut(&mut self) -> &mut Board {
        &mut self.fen.board
    }

    pub fn to_move(&self) -> Colour {
        self.fen.to_move
    }

    pub fn legal_moves(&self, rank: usize, file: usize) -> HashMap<(usize, usize), Move> {
        self.fen.legal_moves(rank, file)
    }

    pub fn apply_move(&mut self, mov: Move) {
        self.fen.apply_move(mov);
        self.moves.push(mov);
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

#[derive(Debug, Clone, Copy)]
pub struct Move {
    piece: Piece,
    colour: Colour,
    from: (usize, usize),
    to: (usize, usize),
    capture: Option<CapturedPiece>,
    check_cnt: usize,
}

#[derive(Debug, Clone, Copy)]
struct CapturedPiece {
    piece: Piece,
    colour: Colour,
    pos: (usize, usize),
}

impl Fen {
    fn apply_move(&mut self, mov: Move) {
        if mov.colour == Colour::Black {
            self.move_cnt += 1;
        }
        self.board = self.board.apply_move(mov);
        self.to_move = self.to_move.opposite();

        // Update castling
        if mov.colour == Colour::White {
            if mov.piece == Piece::King {
                self.white_king_castle = false;
                self.white_queen_castle = false;
            }
            if mov.piece == Piece::Rook && mov.from == (7, 7) {
                self.white_king_castle = false;
            }
            if mov.piece == Piece::Rook && mov.from == (7, 0) {
                self.white_queen_castle = false;
            }
        } else {
            if mov.piece == Piece::King {
                self.black_king_castle = false;
                self.black_queen_castle = false;
            }
            if mov.piece == Piece::Rook && mov.from == (0, 7) {
                self.black_king_castle = false;
            }
            if mov.piece == Piece::Rook && mov.from == (0, 0) {
                self.black_queen_castle = false;
            }
        }

        // Update en-passant
        if mov.piece == Piece::Pawn {
            if mov.colour == Colour::White && mov.to.0 == 4 && mov.from.0 == 6 {
                self.en_passant = Some((5, mov.to.1));
            } else if mov.colour == Colour::Black && mov.to.1 == 3 && mov.from.0 == 1 {
                self.en_passant = Some((2, mov.to.1));
            } else {
                self.en_passant = None;
            }
        } else {
            self.en_passant = None;
        }

        if mov.piece != Piece::Pawn && mov.capture.is_none() {
            self.halfmove_clock += 1;
        } else {
            self.halfmove_clock = 0;
        }
    }

    fn make_move(
        &self,
        piece: Piece,
        colour: Colour,
        rank: usize,
        file: usize,
        new_rank: usize,
        new_file: usize,
    ) -> Option<Move> {
        let pos = self.board[new_rank][new_file];
        let mut mov = match pos {
            Position::Empty => Move {
                piece,
                colour,
                from: (rank, file),
                to: (new_rank, new_file),
                capture: None,
                check_cnt: 0,
            },
            Position::Occupied(np, nc) => {
                if nc == colour {
                    return None;
                } else {
                    Move {
                        piece,
                        colour,
                        from: (rank, file),
                        to: (new_rank, new_file),
                        capture: Some(CapturedPiece {
                            piece: np,
                            colour: nc,
                            pos: (new_rank, new_file),
                        }),
                        check_cnt: 0,
                    }
                }
            }
            Position::Picked(_, _) => return None,
        };
        if self.board.move_verify_checks(&mut mov) {
            Some(mov)
        } else {
            None
        }
    }

    pub fn legal_moves(&self, rank: usize, file: usize) -> HashMap<(usize, usize), Move> {
        let (Position::Occupied(piece, colour) | Position::Picked(piece, colour)) =
            self.board[rank][file]
        else {
            return HashMap::new();
        };

        let positions = match piece {
            Piece::Knight => knight_distance_positions(rank, file),
            Piece::King => {
                let mut positions = king_distance_positions(rank, file);
                if colour == Colour::White {
                    if self.white_king_castle
                        && matches!(self.board[7][5], Position::Empty)
                        && matches!(self.board[7][6], Position::Empty)
                    {
                        positions.push((7, 6));
                    }
                    if self.white_queen_castle
                        && matches!(self.board[7][1], Position::Empty)
                        && matches!(self.board[7][2], Position::Empty)
                        && matches!(self.board[7][3], Position::Empty)
                    {
                        positions.push((7, 2));
                    }
                } else {
                    if self.black_king_castle
                        && matches!(self.board[0][5], Position::Empty)
                        && matches!(self.board[0][6], Position::Empty)
                    {
                        positions.push((0, 6));
                    }
                    if self.black_queen_castle
                        && matches!(self.board[0][1], Position::Empty)
                        && matches!(self.board[0][2], Position::Empty)
                        && matches!(self.board[0][3], Position::Empty)
                    {
                        positions.push((0, 2));
                    }
                }
                positions
            }
            Piece::Rook => self.rook_move_positions(rank, file, colour),
            Piece::Bishop => self.bishop_move_positions(rank, file, colour),
            Piece::Queen => {
                let mut pos = self.rook_move_positions(rank, file, colour);
                pos.extend(self.bishop_move_positions(rank, file, colour));
                pos
            }
            Piece::Pawn => {
                let mut positions = Vec::new();
                if colour == Colour::White {
                    if rank > 0 && matches!(self.board[rank - 1][file], Position::Empty) {
                        positions.push((rank - 1, file));
                    }
                    if rank == 6
                        && matches!(self.board[rank - 1][file], Position::Empty)
                        && matches!(self.board[rank - 2][file], Position::Empty)
                    {
                        positions.push((rank - 2, file));
                    }
                    if rank > 0
                        && file > 0
                        && matches!(self.board[rank - 1][file - 1], Position::Occupied(_, nc) if nc == colour.opposite())
                    {
                        positions.push((rank - 1, file - 1));
                    }
                    if rank > 0
                        && file < BOARD_SIZE - 1
                        && matches!(self.board[rank - 1][file + 1], Position::Occupied(_, nc) if nc == colour.opposite())
                    {
                        positions.push((rank - 1, file + 1));
                    }
                } else {
                    if rank < BOARD_SIZE - 1
                        && matches!(self.board[rank + 1][file], Position::Empty)
                    {
                        positions.push((rank + 1, file));
                    }
                    if rank == 1
                        && matches!(self.board[rank + 1][file], Position::Empty)
                        && matches!(self.board[rank + 2][file], Position::Empty)
                    {
                        positions.push((rank + 2, file));
                    }
                    if rank < BOARD_SIZE - 1
                        && file > 0
                        && matches!(self.board[rank + 1][file - 1], Position::Occupied(_, nc) if nc == colour.opposite())
                    {
                        positions.push((rank + 1, file - 1));
                    }
                    if rank < BOARD_SIZE - 1
                        && file < BOARD_SIZE - 1
                        && matches!(self.board[rank + 1][file + 1], Position::Occupied(_, nc) if nc == colour.opposite())
                    {
                        positions.push((rank + 1, file + 1));
                    }
                }
                let mut moves = positions
                    .into_iter()
                    .filter_map(|(nr, nf)| {
                        self.make_move(piece, colour, rank, file, nr, nf)
                            .map(|m| ((nr, nf), m))
                    })
                    .collect::<HashMap<_, _>>();
                if let Some((epr, epf)) = self.en_passant {
                    if epr == 2
                        && colour == Colour::White
                        && rank == 3
                        && (file + 1 == epf || file == epf + 1)
                    {
                        let mut mov = Move {
                            piece,
                            colour,
                            from: (rank, file),
                            to: (epr, epf),
                            capture: Some(CapturedPiece {
                                piece: Piece::Pawn,
                                colour: Colour::Black,
                                pos: (3, epf),
                            }),
                            check_cnt: 0,
                        };
                        if self.board.move_verify_checks(&mut mov) {
                            moves.insert((epr, epf), mov);
                        }
                    } else if epr == 5
                        && colour == Colour::Black
                        && rank == 4
                        && (file + 1 == epf || file == epf + 1)
                    {
                        let mut mov = Move {
                            piece,
                            colour,
                            from: (rank, file),
                            to: (epr, epf),
                            capture: Some(CapturedPiece {
                                piece: Piece::Pawn,
                                colour: Colour::Black,
                                pos: (4, epf),
                            }),
                            check_cnt: 0,
                        };
                        if self.board.move_verify_checks(&mut mov) {
                            moves.insert((epr, epf), mov);
                        }
                    }
                }
                return moves;
            }
        };

        positions
            .into_iter()
            .filter_map(|(nr, nf)| {
                self.make_move(piece, colour, rank, file, nr, nf)
                    .map(|m| ((nr, nf), m))
            })
            .collect()
    }

    fn rook_move_positions(&self, rank: usize, file: usize, colour: Colour) -> Vec<(usize, usize)> {
        let mut positions = Vec::new();
        for nr in (0..rank).rev() {
            let pos = self.board[nr][file];
            match pos {
                Position::Empty => positions.push((nr, file)),
                Position::Occupied(_, nc) => {
                    if nc == colour.opposite() {
                        positions.push((nr, file));
                    }
                    break;
                }
                Position::Picked(_, _) => continue,
            }
        }
        for nr in rank + 1..BOARD_SIZE {
            let pos = self.board[nr][file];
            match pos {
                Position::Empty => positions.push((nr, file)),
                Position::Occupied(_, nc) => {
                    if nc == colour.opposite() {
                        positions.push((nr, file));
                    }
                    break;
                }
                Position::Picked(_, _) => continue,
            }
        }
        for nf in (0..file).rev() {
            let pos = self.board[rank][nf];
            match pos {
                Position::Empty => positions.push((rank, nf)),
                Position::Occupied(_, nc) => {
                    if nc == colour.opposite() {
                        positions.push((rank, nf));
                    }
                    break;
                }
                Position::Picked(_, _) => continue,
            }
        }
        for nf in file + 1..BOARD_SIZE {
            let pos = self.board[rank][nf];
            match pos {
                Position::Empty => positions.push((rank, nf)),
                Position::Occupied(_, nc) => {
                    if nc == colour.opposite() {
                        positions.push((rank, nf));
                    }
                    break;
                }
                Position::Picked(_, _) => continue,
            }
        }
        positions
    }

    fn bishop_move_positions(
        &self,
        rank: usize,
        file: usize,
        colour: Colour,
    ) -> Vec<(usize, usize)> {
        let mut positions = Vec::new();
        for diff in 1..=rank.min(file) {
            let nr = rank - diff;
            let nf = file - diff;
            let pos = self.board[nr][nf];
            match pos {
                Position::Empty => positions.push((nr, nf)),
                Position::Occupied(_, nc) => {
                    if nc == colour.opposite() {
                        positions.push((nr, nf));
                    }
                    break;
                }
                Position::Picked(_, _) => continue,
            }
        }
        for diff in 1..(BOARD_SIZE - rank).min(BOARD_SIZE - file) {
            let nr = rank + diff;
            let nf = file + diff;
            let pos = self.board[nr][nf];
            match pos {
                Position::Empty => positions.push((nr, nf)),
                Position::Occupied(_, nc) => {
                    if nc == colour.opposite() {
                        positions.push((nr, nf));
                    }
                    break;
                }
                Position::Picked(_, _) => continue,
            }
        }
        for diff in 1..rank.min(BOARD_SIZE - file) {
            let nr = rank - diff;
            let nf = file + diff;
            let pos = self.board[nr][nf];
            match pos {
                Position::Empty => positions.push((nr, nf)),
                Position::Occupied(_, nc) => {
                    if nc == colour.opposite() {
                        positions.push((nr, nf));
                    }
                    break;
                }
                Position::Picked(_, _) => continue,
            }
        }
        for diff in 1..(BOARD_SIZE - rank).min(file) {
            let nr = rank + diff;
            let nf = file - diff;
            let pos = self.board[nr][nf];
            match pos {
                Position::Empty => positions.push((nr, nf)),
                Position::Occupied(_, nc) => {
                    if nc == colour.opposite() {
                        positions.push((nr, nf));
                    }
                    break;
                }
                Position::Picked(_, _) => continue,
            }
        }
        positions
    }
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
