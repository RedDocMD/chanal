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
    a: 180,
};

const PROMOTION_SHADOW: RaylibColour = RaylibColour {
    r: 40,
    g: 40,
    b: 40,
    a: 200,
};

const HIGHLIGHT_WHITE: RaylibColour = RaylibColour {
    r: 176,
    g: 176,
    b: 176,
    a: 255,
};

const HIGHTLIGHT_ORANGE: RaylibColour = RaylibColour {
    r: 214,
    g: 79,
    b: 0,
    a: 255,
};

const CHECK_RED: RaylibColour = RaylibColour {
    r: 239,
    g: 14,
    b: 48,
    a: 255,
};

fn main() {
    set_trace_log_level(TraceLogLevel::Error);
    set_exit_key(Key::Q);

    const DEFAULT_WIN_WIDTH: u32 = 496;
    const DEFAULT_WIN_HEIGHT: u32 = 496;
    const TITLE: &str = "Chanal";
    const FPS: u32 = 60;

    let mut win = Window::new(DEFAULT_WIN_WIDTH, DEFAULT_WIN_HEIGHT, TITLE);
    let _audio_dev = AudioDevice::new();

    let mut img_cache = ImageCache::new();
    let mut gs = GameState {
        game: Game::new(),
        mouse_state: MouseState::Normal,
        legal_moves: HashMap::new(),
        marked_square: None,
        to_unmark: false,
        pending_promotion: None,
    };

    let sounds = Sounds::new();

    win.set_state([ConfigFlag::WindowResizable]);
    win.set_target_fps(FPS);
    while !win.should_close() {
        let sizes = Sizes::new(&win);

        if gs.pending_promotion.is_some() {
            handle_promotion_mode(&mut gs, sizes);
        } else {
            handle_normal_mode(&mut gs, sizes, &sounds);
        }

        // Get the picked piece (if any)
        let picked_tex = if let MouseState::Picked(pp) = gs.mouse_state {
            let piece_img = img_cache.get_piece(pp.piece, pp.colour, sizes.piece_size);
            let piece_tex = Texture2D::from(piece_img);
            Some(piece_tex)
        } else {
            None
        };

        // Generate textures for board and pieces
        let board_img = img_cache.get_board(sizes.board_size);
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
                let piece_img = img_cache.get_piece(*piece, *col, sizes.piece_size);
                let piece_tex = Texture2D::from(piece_img);
                let xpos = sizes.boardx + file as u32 * sizes.piece_size;
                let ypos = sizes.boardy + rank as u32 * sizes.piece_size;
                piece_list.push((piece_tex, xpos, ypos, tint));
            }
        }

        let cap_img = img_cache.get_cap(sizes.piece_size);
        let cap_tex = Texture2D::from(cap_img);

        let shadow_tex = if gs.pending_promotion.is_some() {
            let shadow_img = img_cache.get_shadow(sizes.board_size);
            let shadow_tex = Texture2D::from(shadow_img);
            Some(shadow_tex)
        } else {
            None
        };

        let mut promotion_list = Vec::new();
        let white_tex = Texture2D::from(img_cache.get_white_sq(sizes.piece_size));
        let orange_tex = Texture2D::from(img_cache.get_orange_sq(sizes.piece_size));
        if let Some(ps) = &gs.pending_promotion {
            for (&(r, f), pp) in &ps.pieces {
                let xpos = sizes.boardx + f as u32 * sizes.piece_size;
                let ypos = sizes.boardy + r as u32 * sizes.piece_size;
                let piece_img = img_cache.get_piece(pp.piece, pp.colour, sizes.piece_size);
                let piece_tex = Texture2D::from(piece_img);
                let highlight_tex = if pp.highlighted {
                    &orange_tex
                } else {
                    &white_tex
                };
                promotion_list.push((xpos, ypos, piece_tex, highlight_tex));
            }
        }

        let check_tex = if gs.game.is_check() {
            let mut rtex = RenderTexture::new(sizes.piece_size, sizes.piece_size);
            rtex.do_draw(|| {
                let x = sizes.piece_size / 2;
                let y = sizes.piece_size / 2;
                let radius = (sizes.piece_size as f32 / 2.0) * 2.5;
                const EMPTY: RaylibColour = RaylibColour {
                    r: 239,
                    g: 14,
                    b: 48,
                    a: 0,
                };
                draw_circle_gradient(x, y, radius, CHECK_RED, EMPTY);
            });
            Some(rtex)
        } else {
            None
        };

        raylib::do_draw(|| {
            raylib::clear_background(WHITE);
            board_tex.draw(sizes.boardx, sizes.boardy, WHITE);

            if let Some((rank, file)) = gs.marked_square {
                let x = sizes.boardx + file as u32 * sizes.piece_size;
                let y = sizes.boardy + rank as u32 * sizes.piece_size;
                draw_rectangle(x, y, sizes.piece_size, sizes.piece_size, MARK_COLOUR);
            }
            if let Some(check_tex) = &check_tex {
                let (kr, kf) = gs.game.king_position();
                let x = sizes.boardx + kf as u32 * sizes.piece_size;
                let y = sizes.boardy + kr as u32 * sizes.piece_size;
                check_tex.draw(x, y, WHITE);
            }
            for (&(rank, file), mov) in &gs.legal_moves {
                if mov.has_capture() {
                    let x = sizes.boardx + file as u32 * sizes.piece_size;
                    let y = sizes.boardy + rank as u32 * sizes.piece_size;
                    cap_tex.draw(x, y, WHITE);
                } else {
                    let x = sizes.boardx + file as u32 * sizes.piece_size + sizes.piece_size / 2;
                    let y = sizes.boardy + rank as u32 * sizes.piece_size + sizes.piece_size / 2;
                    let radius = sizes.piece_size as f32 / 5.0;
                    draw_circle(x, y, radius, MARK_COLOUR);
                }
            }

            for (piece_tex, xpos, ypos, tint) in &piece_list {
                piece_tex.draw(*xpos, *ypos, *tint);
            }
            if let Some(picked_tex) = &picked_tex {
                let mx = sizes.mouse_pos.x as u32;
                let my = sizes.mouse_pos.y as u32;
                let x = (mx as i32 - sizes.piece_size as i32 / 2).max(0) as u32;
                let y = (my as i32 - sizes.piece_size as i32 / 2).max(0) as u32;
                picked_tex.draw(x, y, WHITE);
            }

            if let Some(shadow_tex) = &shadow_tex {
                shadow_tex.draw(sizes.boardx, sizes.boardy, WHITE);
            }

            for (x, y, tex, highlight) in &promotion_list {
                highlight.draw(*x, *y, WHITE);
                tex.draw(*x, *y, WHITE);
            }
        });
    }
}

fn handle_promotion_mode(gs: &mut GameState, sizes: Sizes) {
    let board_rect = Rectangle {
        x: sizes.boardx as _,
        y: sizes.boardy as _,
        width: sizes.board_size as _,
        height: sizes.board_size as _,
    };
    let is_mouse_on_board = check_collision_point_rect(sizes.mouse_pos, board_rect);
    set_mouse_cursor(MouseCursor::Default);
    if !is_mouse_on_board {
        return;
    }

    let mx = sizes.mouse_pos.x as u32;
    let my = sizes.mouse_pos.y as u32;
    let file = ((mx - sizes.boardx) / sizes.piece_size) as usize;
    let rank = ((my - sizes.boardy) / sizes.piece_size) as usize;

    let ps = gs.pending_promotion.as_mut().unwrap();
    for pp in ps.pieces.values_mut() {
        pp.highlighted = false;
    }
    if let Some(pp) = ps.pieces.get_mut(&(rank, file)) {
        pp.highlighted = true;
        set_mouse_cursor(MouseCursor::PointingHand);

        if is_mouse_button_pressed(MouseButton::Left) {
            ps.mov.set_promotion(pp.piece, pp.colour);
            gs.game.apply_move(ps.mov);
            gs.pending_promotion = None;
        }
    }
}

fn handle_normal_mode(gs: &mut GameState, sizes: Sizes, sounds: &Sounds) {
    // Set cursor
    let board_rect = Rectangle {
        x: sizes.boardx as _,
        y: sizes.boardy as _,
        width: sizes.board_size as _,
        height: sizes.board_size as _,
    };
    let is_mouse_on_board = check_collision_point_rect(sizes.mouse_pos, board_rect);
    if is_mouse_on_board {
        set_mouse_cursor(MouseCursor::PointingHand);
    } else {
        set_mouse_cursor(MouseCursor::Default);
    }

    // Check for piece clicking and dragging
    if is_mouse_on_board {
        if is_mouse_button_down(MouseButton::Left) {
            if let MouseState::Normal = gs.mouse_state {
                let mx = sizes.mouse_pos.x as u32;
                let my = sizes.mouse_pos.y as u32;
                let file = ((mx - sizes.boardx) / sizes.piece_size) as usize;
                let rank = ((my - sizes.boardy) / sizes.piece_size) as usize;
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
                    let mx = sizes.mouse_pos.x as u32;
                    let my = sizes.mouse_pos.y as u32;
                    let file = ((mx - sizes.boardx) / sizes.piece_size) as usize;
                    let rank = ((my - sizes.boardy) / sizes.piece_size) as usize;

                    gs.mouse_state = MouseState::Normal;
                    if let Some(&mov) = gs.legal_moves.get(&(rank, file)) {
                        if mov.may_promote() {
                            gs.pending_promotion = Some(PromotionState::new(mov));
                            let board = gs.game.board_mut();
                            board[pp.rank][pp.file] = Position::Empty;
                            board[rank][file] = Position::Empty;
                        } else {
                            gs.game.apply_move(mov);
                        }
                        gs.marked_square = None;
                        gs.legal_moves.clear();
                        gs.to_unmark = false;

                        if mov.has_check() {
                            sounds.check_sound.play();
                        } else if mov.has_capture() {
                            sounds.capture_sound.play();
                        } else {
                            sounds.move_sound.play();
                        }
                    } else {
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

        // Handle keyboard presses when piece is not picked
        if matches!(gs.mouse_state, MouseState::Normal) {
            if get_char_pressed() != '\0' {
                gs.legal_moves.clear();
                gs.marked_square = None;
                gs.to_unmark = false;
            }
            if is_key_released(Key::J) {
                gs.game.back();
            } else if is_key_released(Key::K) {
                let mov = gs.game.forward();
                if let Some(mov) = mov {
                    if mov.has_check() {
                        sounds.check_sound.play();
                    } else if mov.has_capture() {
                        sounds.capture_sound.play();
                    } else {
                        sounds.move_sound.play();
                    }
                }
            } else if is_key_released(Key::H) {
                gs.game.prev_variation();
            } else if is_key_released(Key::L) {
                gs.game.next_variation();
            }
        }
    }
}

#[derive(Copy, Clone)]
struct Sizes {
    width: u32,
    height: u32,
    boardx: u32,
    boardy: u32,
    board_size: u32,
    piece_size: u32,
    mouse_pos: Vector2,
}

impl Sizes {
    fn new(win: &Window) -> Self {
        let (width, height) = win.size();
        let (boardx, boardy) = (0, 0);
        let board_size = board_size(width, height);
        let piece_size = board_size / chess::BOARD_SIZE as u32;
        let mouse_pos = get_mouse_position();

        Self {
            width,
            height,
            boardx,
            boardy,
            board_size,
            piece_size,
            mouse_pos,
        }
    }
}

struct Sounds {
    move_sound: Sound,
    capture_sound: Sound,
    check_sound: Sound,
}

impl Sounds {
    fn new() -> Self {
        let move_wave = Wave::from_mem(WaveFormat::Ogg, assets::MOVE_OGG);
        let move_sound = Sound::from(&move_wave);
        let capture_wave = Wave::from_mem(WaveFormat::Ogg, assets::CAPTURE_OGG);
        let capture_sound = Sound::from(&capture_wave);
        let check_wave = Wave::from_mem(WaveFormat::Ogg, assets::CHECK_OGG);
        let check_sound = Sound::from(&check_wave);

        Self {
            move_sound,
            capture_sound,
            check_sound,
        }
    }
}

struct ImageCache {
    pieces: HashMap<(Piece, Colour), HashMap<u32, Image>>,
    boards: HashMap<u32, Image>,
    caps: HashMap<u32, Image>,
    shadows: HashMap<u32, Image>,
    white_sqs: HashMap<u32, Image>,
    orange_sqs: HashMap<u32, Image>,
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

        let mut cap_img = Image::gen_colour(piece_size, piece_size, MARK_COLOUR);
        let mut mask_img = Image::gen_colour(
            piece_size,
            piece_size,
            RaylibColour {
                r: MARK_COLOUR.a,
                g: MARK_COLOUR.a,
                b: MARK_COLOUR.a,
                a: MARK_COLOUR.a,
            },
        );
        mask_img.draw_circle(
            piece_size / 2,
            piece_size / 2,
            ((piece_size / 2) as f32 * 1.15) as u32,
            RaylibColour {
                r: 0,
                g: 0,
                b: 0,
                a: 0,
            },
        );
        cap_img.alpha_mask(&mask_img);
        let caps = HashMap::from([(piece_size, cap_img)]);

        let shadow_img = Image::gen_colour(bw, bw, PROMOTION_SHADOW);
        let shadows = HashMap::from([(bw, shadow_img)]);

        let white_sq_img = Image::gen_colour(piece_size, piece_size, HIGHLIGHT_WHITE);
        let white_sqs = HashMap::from([(piece_size, white_sq_img)]);

        let orange_sq_img = Image::gen_colour(piece_size, piece_size, HIGHTLIGHT_ORANGE);
        let orange_sqs = HashMap::from([(piece_size, orange_sq_img)]);

        Self {
            pieces,
            boards,
            board_size: bw,
            piece_size,
            caps,
            shadows,
            white_sqs,
            orange_sqs,
        }
    }

    fn get_board(&mut self, size: u32) -> &Image {
        get_image(&mut self.boards, size, self.board_size)
    }

    fn get_shadow(&mut self, size: u32) -> &Image {
        get_image(&mut self.shadows, size, self.board_size)
    }

    fn get_piece(&mut self, piece: Piece, colour: Colour, size: u32) -> &Image {
        let piece_cache = self.pieces.get_mut(&(piece, colour)).unwrap();
        get_image(piece_cache, size, self.piece_size)
    }

    fn get_cap(&mut self, size: u32) -> &Image {
        get_image(&mut self.caps, size, self.piece_size)
    }

    fn get_white_sq(&mut self, size: u32) -> &Image {
        get_image(&mut self.white_sqs, size, self.piece_size)
    }

    fn get_orange_sq(&mut self, size: u32) -> &Image {
        get_image(&mut self.orange_sqs, size, self.piece_size)
    }
}

fn get_image(imgs: &mut HashMap<u32, Image>, size: u32, def_size: u32) -> &Image {
    let mut new_img = imgs.get(&def_size).unwrap().clone();
    imgs.entry(size).or_insert_with(|| {
        new_img.resize(size, size);
        new_img
    })
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
    pending_promotion: Option<PromotionState>,
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

#[derive(Debug)]
struct PromotionState {
    mov: Move,
    pieces: HashMap<(usize, usize), PromotionPiece>,
}

#[derive(Debug)]
struct PromotionPiece {
    piece: Piece,
    colour: Colour,
    highlighted: bool,
}

impl PromotionState {
    fn new(mov: Move) -> Self {
        const PIECES: [Piece; 4] = [Piece::Queen, Piece::Knight, Piece::Rook, Piece::Bishop];
        let pieces = PIECES
            .into_iter()
            .enumerate()
            .map(|(i, p)| {
                let pos = (i, mov.to().1);
                let pp = PromotionPiece {
                    piece: p,
                    colour: mov.colour(),
                    highlighted: false,
                };
                (pos, pp)
            })
            .collect();
        Self { mov, pieces }
    }
}
