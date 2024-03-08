use crate::raylib::{Image, ImgFormat, Texture2D, WHITE};

use self::raylib::Window;

mod assets;
mod raylib;

fn main() {
    const DEFAULT_WIN_WIDTH: u32 = 500;
    const DEFAULT_WIN_HEIGHT: u32 = 500;
    const TITLE: &str = "Chanal";
    const FPS: u32 = 60;

    let mut win = Window::new(DEFAULT_WIN_WIDTH, DEFAULT_WIN_HEIGHT, TITLE);
    let board_img = Image::from_mem(ImgFormat::Jpg, assets::WOOD4_JPG);
    let (w, h) = board_img.size();
    println!("Board is {} x {}", w, h);

    win.set_target_fps(FPS);
    while !win.should_close() {
        let (w, h) = win.size();
        let mut board_img = board_img.clone();
        board_img.resize(w, h);
        let board_tex = Texture2D::from(&board_img);
        raylib::do_draw(|| {
            raylib::clear_background(WHITE);
            board_tex.draw(0, 0, WHITE);
        });
    }
}
