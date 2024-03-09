use std::collections::HashMap;

use itertools::iproduct;

use crate::{chess::*, raylib::*};

mod assets;
mod chess;
mod raylib;

const MARK_COLOUR: RaylibColour = RaylibColour {
    r: 31,
    g: 102,
    b: 77,
    a: 130,
};

fn main() {
    set_trace_log_level(TraceLogLevel::Error);

    const DEFAULT_WIN_WIDTH: u32 = 500;
    const DEFAULT_WIN_HEIGHT: u32 = 500;
    const TITLE: &str = "Chanal";
    const FPS: u32 = 60;

    let mut win = Window::new(DEFAULT_WIN_WIDTH, DEFAULT_WIN_HEIGHT, TITLE);
    let mut img_cache = ImageCache::new();
    let mut gs = GameState {
        game: Game::new(),
        mouse_state: MouseState::Normal,
        legal_moves: HashMap::new(),
        marked_square: None,
        to_unmark: false,
    };

    win.set_target_fps(FPS);
    while !win.should_close() {
        let (width, height) = win.size();
        let (boardx, boardy) = (0, 0);
        let board_size = board_size(width, height);
        let piece_size = board_size / chess::BOARD_SIZE as u32;

        // Set cursor
        let mouse_pos = get_mouse_position();
        let board_rect = Rectangle {
            x: boardx as _,
            y: boardy as _,
            width: board_size as _,
            height: board_size as _,
        };
        let is_mouse_on_board = check_collision_point_rect(mouse_pos, board_rect);
        if is_mouse_on_board {
            set_mouse_cursor(MouseCursor::PointingHand);
        } else {
            set_mouse_cursor(MouseCursor::Default);
        }

        // Check for piece clicking and dragging
        if is_mouse_on_board {
            if is_mouse_button_down(MouseButton::Left) {
                if let MouseState::Normal = gs.mouse_state {
                    let mx = mouse_pos.x as u32;
                    let my = mouse_pos.y as u32;
                    let file = ((mx - boardx) / piece_size) as usize;
                    let rank = ((my - boardy) / piece_size) as usize;
                    let pos = gs.game.board()[rank][file];
                    if let Position::Occupied(piece, colour) = pos {
                        if colour == gs.game.to_move() {
                            let pp = PickedPiece {
                                piece,
                                colour,
                                rank,
                                file,
                            };
                            gs.mouse_state = MouseState::Picked(pp);
                            gs.game.board_mut()[rank][file] = Position::Picked(piece, colour);
                            gs.legal_moves = gs.game.legal_moves(rank, file);
                            if let Some((or, of)) = gs.marked_square {
                                gs.to_unmark = or == rank && of == file;
                            } else {
                                gs.to_unmark = false;
                            }
                            gs.marked_square = Some((rank, file));
                        } else {
                            gs.mouse_state = MouseState::Clicked;
                        }
                    } else {
                        gs.mouse_state = MouseState::Clicked;
                    }
                }
            } else if is_mouse_button_released(MouseButton::Left) {
                match gs.mouse_state {
                    MouseState::Normal => {}
                    MouseState::Clicked => {
                        gs.mouse_state = MouseState::Normal;
                    }
                    MouseState::Picked(pp) => {
                        let mx = mouse_pos.x as u32;
                        let my = mouse_pos.y as u32;
                        let file = ((mx - boardx) / piece_size) as usize;
                        let rank = ((my - boardy) / piece_size) as usize;

                        if file == pp.file && rank == pp.rank {
                            gs.mouse_state = MouseState::Normal;
                            gs.game.board_mut()[pp.rank][pp.file] =
                                Position::Occupied(pp.piece, pp.colour);
                        }

                        if gs.to_unmark {
                            gs.marked_square = None;
                            gs.legal_moves.clear();
                        }
                    }
                }
            }
        }

        // Get the picked piece (if any)
        let picked_tex = if let MouseState::Picked(pp) = gs.mouse_state {
            let piece_img = img_cache.get_piece(pp.piece, pp.colour, piece_size);
            let piece_tex = Texture2D::from(piece_img);
            Some(piece_tex)
        } else {
            None
        };

        // Generate textures for board and pieces
        let board_img = img_cache.get_board(board_size);
        let board_tex = Texture2D::from(board_img);

        let mut piece_list = Vec::new();
        let board = gs.game.board();
        for (rank, rp) in board.iter().enumerate() {
            for (file, pos) in rp.iter().enumerate() {
                let (Position::Occupied(piece, col) | Position::Picked(piece, col)) = pos else {
                    continue;
                };
                let tint = if matches!(pos, Position::Occupied(_, _)) {
                    WHITE
                } else {
                    WHITE.fade(0.5)
                };
                let piece_img = img_cache.get_piece(*piece, *col, piece_size);
                let piece_tex = Texture2D::from(piece_img);
                let xpos = boardx + file as u32 * piece_size;
                let ypos = boardy + rank as u32 * piece_size;
                piece_list.push((piece_tex, xpos, ypos, tint));
            }
        }

        raylib::do_draw(|| {
            raylib::clear_background(WHITE);
            board_tex.draw(boardx, boardy, WHITE);
            for (piece_tex, xpos, ypos, tint) in &piece_list {
                piece_tex.draw(*xpos, *ypos, *tint);
            }
            if let Some(picked_tex) = &picked_tex {
                let mx = mouse_pos.x as u32;
                let my = mouse_pos.y as u32;
                let x = (mx as i32 - piece_size as i32 / 2).max(0) as u32;
                let y = (my as i32 - piece_size as i32 / 2).max(0) as u32;
                picked_tex.draw(x, y, WHITE);
            }
            if let Some((rank, file)) = gs.marked_square {
                let x = boardx + file as u32 * piece_size;
                let y = boardy + rank as u32 * piece_size;
                draw_rectangle(x, y, piece_size, piece_size, MARK_COLOUR);
            }
            for &(rank, file) in gs.legal_moves.keys() {
                let x = boardx + file as u32 * piece_size + piece_size / 2;
                let y = boardy + rank as u32 * piece_size + piece_size / 2;
                let radius = piece_size as f32 / 5.0;
                draw_circle(x, y, radius, MARK_COLOUR);
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

fn board_size(width: u32, height: u32) -> u32 {
    let min_dim = width.min(height);
    let res = min_dim % chess::BOARD_SIZE as u32;
    min_dim - res
}

#[derive(Debug)]
struct GameState {
    game: Game,
    mouse_state: MouseState,
    legal_moves: HashMap<(usize, usize), Move>,
    marked_square: Option<(usize, usize)>,
    to_unmark: bool,
}

#[derive(Debug)]
enum MouseState {
    Normal,
    Clicked,
    Picked(PickedPiece),
}

#[derive(Debug, Clone, Copy)]
struct PickedPiece {
    piece: Piece,
    colour: Colour,
    rank: usize,
    file: usize,
}
