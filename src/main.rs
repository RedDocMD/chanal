use self::raylib::Window;

mod raylib;

fn main() {
    const DEFAULT_WIN_WIDTH: u32 = 500;
    const DEFAULT_WIN_HEIGHT: u32 = 500;
    const TITLE: &str = "Chanal";
    const FPS: u32 = 60;

    let mut win = Window::new(DEFAULT_WIN_WIDTH, DEFAULT_WIN_HEIGHT, TITLE);

    win.set_target_fps(FPS);
    while !win.should_close() {}
}
