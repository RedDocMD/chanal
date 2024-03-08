use std::collections::HashMap;

use crate::chess::{Colour, Piece};
use crate::raylib::{Image, ImgFormat, Texture2D, WHITE};

use self::raylib::Window;

mod assets;
mod chess;
mod raylib;

fn main() {
    const DEFAULT_WIN_WIDTH: u32 = 500;
    const DEFAULT_WIN_HEIGHT: u32 = 500;
    const TITLE: &str = "Chanal";
    const FPS: u32 = 60;

    let mut win = Window::new(DEFAULT_WIN_WIDTH, DEFAULT_WIN_HEIGHT, TITLE);
    let board_img = Image::from_mem(ImgFormat::Jpg, assets::WOOD4_JPG);
    let (board_width, board_height) = board_img.size();
    println!("Board is {} x {}", board_width, board_height);

    let piece_width = board_width / chess::BOARD_SIZE as u32;
    let piece_height = board_height / chess::BOARD_SIZE as u32;
    let mut piece_imgs = HashMap::new();
    let mut add_piece_img = |piece, colour| {
        let img_data = assets::merida_piece_data(piece, colour);
        let img = Image::from_svg_mem(img_data, piece_width, piece_height);
        piece_imgs.insert((piece, colour), img);
    };
    add_piece_img(Piece::Pawn, Colour::White);
    add_piece_img(Piece::Rook, Colour::White);
    add_piece_img(Piece::Knight, Colour::White);
    add_piece_img(Piece::Bishop, Colour::White);
    add_piece_img(Piece::King, Colour::White);
    add_piece_img(Piece::Queen, Colour::White);
    add_piece_img(Piece::Pawn, Colour::Black);
    add_piece_img(Piece::Rook, Colour::Black);
    add_piece_img(Piece::Knight, Colour::Black);
    add_piece_img(Piece::Bishop, Colour::Black);
    add_piece_img(Piece::King, Colour::Black);
    add_piece_img(Piece::Queen, Colour::Black);

    win.set_target_fps(FPS);
    while !win.should_close() {
        let (board_width, board_height) = win.size();
        let piece_width = board_width / chess::BOARD_SIZE as u32;
        let piece_height = board_height / chess::BOARD_SIZE as u32;
        let mut board_img = board_img.clone();
        board_img.resize(board_width, board_height);
        let board_tex = Texture2D::from(&board_img);
        let mut pawn_img = piece_imgs
            .get(&(Piece::Pawn, Colour::White))
            .unwrap()
            .clone();
        pawn_img.resize(piece_width, piece_height);
        let pawn_tex = Texture2D::from(&pawn_img);
        raylib::do_draw(|| {
            raylib::clear_background(WHITE);
            board_tex.draw(0, 0, WHITE);
            pawn_tex.draw(0, 0, WHITE);
        });
    }
}
