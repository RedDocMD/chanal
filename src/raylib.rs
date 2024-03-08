use std::borrow::Cow;
use std::ffi::{CStr, CString};

pub use sys::{Color, MouseButton, MouseCursor, Rectangle, TraceLogLevel, Vector2};

mod sys {
    use std::ffi::{c_char, c_float, c_int, c_uchar, c_uint, c_void};

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
        pub r: c_uchar,
        pub g: c_uchar,
        pub b: c_uchar,
        pub a: c_uchar,
    }

    impl Color {
        pub fn fade(self, factor: f32) -> Self {
            unsafe { Fade(self, factor) }
        }
    }

    #[repr(i32)]
    #[derive(Clone, Copy)]
    #[allow(dead_code)]
    pub enum MouseCursor {
        Default = 0,      // Default pointer shape
        Arrow = 1,        // Arrow shape
        Ibeam = 2,        // Text writing cursor shape
        Crosshair = 3,    // Cross shape
        PointingHand = 4, // Pointing hand cursor
        ResizeEw = 5,     // Horizontal resize/move arrow shape
        ResizeNs = 6,     // Vertical resize/move arrow shape
        ResizeNwse = 7,   // Top-left to bottom-right diagonal resize/move arrow shape
        ResizeNesw = 8,   // The top-right to bottom-left diagonal resize/move arrow shape
        ResizeAll = 9,    // The omnidirectional resize/move cursor shape
        NotAllowed = 10,  // The operation-not-allowed shape
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct Rectangle {
        pub x: c_float,
        pub y: c_float,
        pub width: c_float,
        pub height: c_float,
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct Vector2 {
        pub x: c_float,
        pub y: c_float,
    }

    #[repr(i32)]
    #[allow(dead_code)]
    pub enum MouseButton {
        Left = 0,    // Mouse button left
        Right = 1,   // Mouse button right
        Middle = 2,  // Mouse button middle (pressed wheel)
        Side = 3,    // Mouse button side (advanced mouse device)
        Extra = 4,   // Mouse button extra (advanced mouse device)
        Forward = 5, // Mouse button forward (advanced mouse device)
        Back = 6,    // Mouse button back (advanced mouse device)
    }

    #[repr(i32)]
    #[allow(dead_code)]
    pub enum TraceLogLevel {
        All = 0, // Display all logs
        Trace,   // Trace logging, intended for internal use only
        Debug, // Debug logging, used for internal debugging, it should be disabled on release builds
        Info,  // Info logging, used for program execution info
        Warning, // Warning logging, used on recoverable failures
        Error, // Error logging, used on unrecoverable failures
        Fatal, // Fatal logging, used to abort program: exit(EXIT_FAILURE)
        None,  // Disable logging
    }

    #[link(name = "raylib")]
    extern "C" {
        pub fn InitWindow(width: c_int, height: c_int, title: *const c_char);
        pub fn CloseWindow();
        pub fn WindowShouldClose() -> bool;
        pub fn SetTargetFPS(fps: c_int);
        pub fn GetRenderWidth() -> c_int;
        pub fn GetRenderHeight() -> c_int;

        pub fn LoadImageFromMemory(ext: *const c_char, data: *const c_uchar, size: c_int) -> Image;
        pub fn LoadImageSvg(file_name_or_str: *const c_char, width: c_int, height: c_int) -> Image;
        pub fn UnloadImage(image: Image);
        pub fn ImageResize(image: *mut Image, newWidth: c_int, newHeight: c_int);
        pub fn ImageCopy(image: Image) -> Image;

        pub fn LoadTextureFromImage(image: Image) -> Texture2D;
        pub fn UnloadTexture(texture: Texture2D);
        pub fn DrawTexture(texture: Texture2D, xpos: c_int, ypos: c_int, tint: Color);

        pub fn BeginDrawing();
        pub fn EndDrawing();
        pub fn ClearBackground(color: Color);

        pub fn SetMouseCursor(cursor: c_int);
        pub fn GetMousePosition() -> Vector2;
        pub fn IsMouseButtonDown(button: c_int) -> c_int;
        pub fn IsMouseButtonReleased(button: c_int) -> c_int;

        pub fn CheckCollisionPointRec(point: Vector2, rect: Rectangle) -> c_int;

        pub fn Fade(col: Color, alpha: c_float) -> Color;

        pub fn SetTraceLogLevel(log_level: c_int);
    }
}

pub const WHITE: Color = Color {
    r: 255,
    g: 255,
    b: 255,
    a: 255,
};

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

    pub fn from_svg_mem(data: &[u8], width: u32, height: u32) -> Self {
        let data = if data.last() == Some(&b'\0') {
            Cow::from(data)
        } else {
            let mut data = data.to_vec();
            data.push(b'\0');
            Cow::from(data)
        };
        let img = unsafe {
            sys::LoadImageSvg(
                CStr::from_bytes_with_nul(data.as_ref()).unwrap().as_ptr(),
                width as _,
                height as _,
            )
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

pub fn do_draw(draw_fn: impl FnOnce()) {
    unsafe { sys::BeginDrawing() };
    draw_fn();
    unsafe { sys::EndDrawing() };
}

pub fn clear_background(col: Color) {
    unsafe { sys::ClearBackground(col) };
}

pub fn set_mouse_cursor(cursor: MouseCursor) {
    unsafe { sys::SetMouseCursor(cursor as _) };
}

pub fn get_mouse_position() -> Vector2 {
    unsafe { sys::GetMousePosition() }
}

pub fn check_collision_point_rect(point: Vector2, rect: Rectangle) -> bool {
    unsafe { sys::CheckCollisionPointRec(point, rect) != 0 }
}

pub fn is_mouse_button_down(mb: MouseButton) -> bool {
    unsafe { sys::IsMouseButtonDown(mb as _) != 0 }
}

pub fn is_mouse_button_released(mb: MouseButton) -> bool {
    unsafe { sys::IsMouseButtonReleased(mb as _) != 0 }
}

pub fn set_trace_log_level(level: TraceLogLevel) {
    unsafe { sys::SetTraceLogLevel(level as _) };
}
