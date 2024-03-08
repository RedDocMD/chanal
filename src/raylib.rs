use std::ffi::{CStr, CString};

pub use sys::{Color, WHITE};

mod sys {
    use std::ffi::{c_char, c_int, c_uchar, c_uint, c_void};

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct Image {
        data: *mut c_void,
        pub width: c_int,
        pub height: c_int,
        mipmaps: c_int,
        format: c_int,
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct Texture2D {
        id: c_uint,
        width: c_int,
        height: c_int,
        mipmaps: c_int,
        format: c_int,
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct Color {
        r: c_uchar,
        g: c_uchar,
        b: c_uchar,
        a: c_uchar,
    }

    pub const WHITE: Color = Color {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };

    #[link(name = "raylib")]
    extern "C" {
        pub fn InitWindow(width: c_int, height: c_int, title: *const c_char);
        pub fn CloseWindow();
        pub fn WindowShouldClose() -> bool;
        pub fn SetTargetFPS(fps: c_int);
        pub fn GetRenderWidth() -> c_int;
        pub fn GetRenderHeight() -> c_int;

        pub fn LoadImageFromMemory(ext: *const c_char, data: *const c_uchar, size: c_int) -> Image;
        pub fn UnloadImage(image: Image);
        pub fn ImageResize(image: *mut Image, newWidth: c_int, newHeight: c_int);
        pub fn ImageCopy(image: Image) -> Image;

        pub fn LoadTextureFromImage(image: Image) -> Texture2D;
        pub fn UnloadTexture(texture: Texture2D);
        pub fn DrawTexture(texture: Texture2D, xpos: c_int, ypos: c_int, tint: Color);

        pub fn BeginDrawing();
        pub fn EndDrawing();
        pub fn ClearBackground(color: Color);
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

    pub fn size(&self) -> (u32, u32) {
        let width = unsafe { sys::GetRenderWidth() };
        let height = unsafe { sys::GetRenderHeight() };
        (width as _, height as _)
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe { sys::CloseWindow() };
    }
}

#[derive(Clone, Copy)]
pub enum ImgFormat {
    Jpg,
}

impl ImgFormat {
    pub fn to_cstr(self) -> &'static CStr {
        match self {
            ImgFormat::Jpg => CStr::from_bytes_with_nul(b".jpg\0").unwrap(),
        }
    }
}

pub struct Image {
    img: sys::Image,
}

impl Image {
    pub fn from_mem(format: ImgFormat, data: &[u8]) -> Self {
        let img = unsafe {
            sys::LoadImageFromMemory(format.to_cstr().as_ptr(), data.as_ptr(), data.len() as _)
        };
        Self { img }
    }

    pub fn size(&self) -> (u32, u32) {
        (self.img.width as _, self.img.height as _)
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        unsafe { sys::ImageResize(&mut self.img as *mut sys::Image, width as _, height as _) };
    }
}

impl Clone for Image {
    fn clone(&self) -> Self {
        let img = unsafe { sys::ImageCopy(self.img) };
        Self { img }
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        unsafe { sys::UnloadImage(self.img) };
    }
}

pub struct Texture2D {
    tex: sys::Texture2D,
}

impl Texture2D {
    pub fn draw(&self, x: u32, y: u32, tint: Color) {
        unsafe { sys::DrawTexture(self.tex, x as _, y as _, tint) };
    }
}

impl From<&Image> for Texture2D {
    fn from(img: &Image) -> Self {
        let tex = unsafe { sys::LoadTextureFromImage(img.img) };
        Self { tex }
    }
}

impl Drop for Texture2D {
    fn drop(&mut self) {
        unsafe { sys::UnloadTexture(self.tex) };
    }
}

pub fn do_draw(draw_fn: impl Fn()) {
    unsafe { sys::BeginDrawing() };
    draw_fn();
    unsafe { sys::EndDrawing() };
}

pub fn clear_background(col: Color) {
    unsafe { sys::ClearBackground(col) };
}
