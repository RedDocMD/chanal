use std::ffi::CString;

mod sys {
    use std::ffi::{c_char, c_int};

    #[link(name = "raylib")]
    extern "C" {
        pub fn InitWindow(width: c_int, height: c_int, title: *const c_char);
        pub fn CloseWindow();
        pub fn WindowShouldClose() -> bool;

        pub fn SetTargetFPS(fps: c_int);
    }
}

pub struct Window;

impl Window {
    pub fn new(width: u32, height: u32, title: &str) -> Self {
        let title = CString::new(title).unwrap();
        unsafe { sys::InitWindow(width as _, height as _, title.as_ptr()) };
        Self
    }

    pub fn should_close(&self) -> bool {
        unsafe { sys::WindowShouldClose() }
    }

    pub fn set_target_fps(&mut self, fps: u32) {
        unsafe { sys::SetTargetFPS(fps as _) };
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe { sys::CloseWindow() };
    }
}
