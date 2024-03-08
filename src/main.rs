use std::collections::HashMap;

use itertools::iproduct;

use crate::{
    chess::{Colour, Game, Piece, Position},
    raylib::{Image, ImgFormat, Texture2D, Window, WHITE},
};

mod assets;
mod chess;
mod raylib;

fn board_size(width: u32, height: u32) -> u32 {
    let min_dim = width.min(height);
    let res = min_dim % chess::BOARD_SIZE as u32;
    min_dim - res
}

fn main() {
    const DEFAULT_WIN_WIDTH: u32 = 500;
    const DEFAULT_WIN_HEIGHT: u32 = 500;
    const TITLE: &str = "Chanal";
    const FPS: u32 = 60;

    let mut win = Window::new(DEFAULT_WIN_WIDTH, DEFAULT_WIN_HEIGHT, TITLE);
    let mut img_cache = ImageCache::new();
    let mut game = Game::new();

    win.set_target_fps(FPS);
    while !win.should_close() {
        let (width, height) = win.size();
        let board_size = board_size(width, height);
        let piece_size = board_size / chess::BOARD_SIZE as u32;

        let board_img = img_cache.get_board(board_size);
        let board_tex = Texture2D::from(board_img);

        let mut piece_list = Vec::new();
        let board = game.board();
        for (rank, rp) in board.0.iter().enumerate() {
            for (file, pos) in rp.iter().enumerate() {
                if let &Position::Occupied(piece, col) = pos {
                    let piece_img = img_cache.get_piece(piece, col, piece_size);
                    let piece_tex = Texture2D::from(piece_img);
                    let xpos = file as u32 * piece_size;
                    let ypos = rank as u32 * piece_size;
                    piece_list.push((piece_tex, xpos, ypos));
                }
            }
        }

        raylib::do_draw(|| {
            raylib::clear_background(WHITE);
            board_tex.draw(0, 0, WHITE);
            for (piece_tex, xpos, ypos) in &piece_list {
                piece_tex.draw(*xpos, *ypos, WHITE);
            }
        });
    }
}

struct ImageCache {
    pieces: HashMap<(Piece, Colour), HashMap<u32, Image>>,
    boards: HashMap<u32, Image>,
    board_size: u32,
    piece_size: u32,
}

impl ImageCache {
    fn new() -> Self {
        let board_img = Image::from_mem(ImgFormat::Jpg, assets::WOOD4_JPG);
        let (bw, bh) = board_img.size();
        assert!(bw == bh);
        let boards = HashMap::from([(bw, board_img)]);

        let piece_size = bw / chess::BOARD_SIZE as u32;
        let pieces = iproduct!(
            enum_iterator::all::<Colour>(),
            enum_iterator::all::<Piece>()
        )
        .map(|(colour, piece)| {
            let img_data = assets::merida_piece_data(piece, colour);
            let img = Image::from_svg_mem(img_data, piece_size, piece_size);
            let map = HashMap::from([(piece_size, img)]);
            ((piece, colour), map)
        })
        .collect();
        Self {
            pieces,
            boards,
            board_size: bw,
            piece_size,
        }
    }

    fn get_board(&mut self, size: u32) -> &Image {
        let mut new_img = self.boards.get(&self.board_size).unwrap().clone();
        self.boards.entry(size).or_insert_with(|| {
            new_img.resize(size, size);
            new_img
        })
    }

    fn get_piece(&mut self, piece: Piece, colour: Colour, size: u32) -> &Image {
        let piece_cache = self.pieces.get_mut(&(piece, colour)).unwrap();
        let mut new_img = piece_cache.get(&self.piece_size).unwrap().clone();
        piece_cache.entry(size).or_insert_with(|| {
            new_img.resize(size, size);
            new_img
        })
    }
}
