use std::borrow::Cow;
use std::ffi::{CStr, CString};

pub use sys::{
    ConfigFlag, Key, MouseButton, MouseCursor, RaylibColour, Rectangle, TraceLogLevel, Vector2,
};

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
    pub struct RaylibColour {
        pub r: c_uchar,
        pub g: c_uchar,
        pub b: c_uchar,
        pub a: c_uchar,
    }

    impl RaylibColour {
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
    #[derive(Clone, Copy, Debug)]
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

    #[repr(u32)]
    #[allow(dead_code)]
    pub enum ConfigFlag {
        VsyncHint = 0x00000040,              // Set to try enabling V-Sync on GPU
        FullscreenMode = 0x00000002,         // Set to run program in fullscreen
        WindowResizable = 0x00000004,        // Set to allow resizable window
        WindowUndecorated = 0x00000008,      // Set to disable window decoration (frame and buttons)
        WindowHidden = 0x00000080,           // Set to hide window
        WindowMinimized = 0x00000200,        // Set to minimize window (iconify)
        WindowMaximized = 0x00000400,        // Set to maximize window (expanded to monitor)
        WindowUnfocused = 0x00000800,        // Set to window non focused
        WindowTopmost = 0x00001000,          // Set to window always on top
        WindowAlwaysRun = 0x00000100,        // Set to allow windows running while minimized
        WindowTransparent = 0x00000010,      // Set to allow transparent framebuffer
        WindowHighdpi = 0x00002000,          // Set to support HighDPI
        WindowMousePassthrough = 0x00004000, // Set to support mouse passthrough, only supported when FLAG_WINDOW_UNDECORATED
        BorderlessWindowedMode = 0x00008000, // Set to run program in borderless windowed mode
        Msaa4xHint = 0x00000020,             // Set to try enabling MSAA 4X
        InterlacedHint = 0x00010000, // Set to try enabling interlaced video format (for V3D)
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct Wave {
        frame_count: c_uint,
        sample_rate: c_uint,
        sample_size: c_uint,
        channels: c_uint,
        data: *mut c_void,
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    struct AudioStream {
        buffer: *mut c_void,
        processor: *mut c_void,
        sample_rate: c_uint,
        sample_size: c_uint,
        channels: c_uint,
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct Sound {
        audio_stream: AudioStream,
        frame_count: c_uint,
    }

    #[repr(C)]
    #[derive(Clone, Copy)]
    pub struct RenderTexture {
        id: c_uint,
        pub texture: Texture2D,
        depth: Texture2D,
    }

    #[repr(i32)]
    #[derive(Clone, Copy)]
    #[allow(dead_code)]
    pub enum Key {
        Null = 0, // Key: NULL, used for no key pressed
        // Alphanumeric keys
        Apostrophe = 39,   // Key: '
        Comma = 44,        // Key: ,
        Minus = 45,        // Key: -
        Period = 46,       // Key: .
        Slash = 47,        // Key: /
        Zero = 48,         // Key: 0
        One = 49,          // Key: 1
        Two = 50,          // Key: 2
        Three = 51,        // Key: 3
        Four = 52,         // Key: 4
        Five = 53,         // Key: 5
        Six = 54,          // Key: 6
        Seven = 55,        // Key: 7
        Eight = 56,        // Key: 8
        Nine = 57,         // Key: 9
        Semicolon = 59,    // Key: ;
        Equal = 61,        // Key: =
        A = 65,            // Key: A | a
        B = 66,            // Key: B | b
        C = 67,            // Key: C | c
        D = 68,            // Key: D | d
        E = 69,            // Key: E | e
        F = 70,            // Key: F | f
        G = 71,            // Key: G | g
        H = 72,            // Key: H | h
        I = 73,            // Key: I | i
        J = 74,            // Key: J | j
        K = 75,            // Key: K | k
        L = 76,            // Key: L | l
        M = 77,            // Key: M | m
        N = 78,            // Key: N | n
        O = 79,            // Key: O | o
        P = 80,            // Key: P | p
        Q = 81,            // Key: Q | q
        R = 82,            // Key: R | r
        S = 83,            // Key: S | s
        T = 84,            // Key: T | t
        U = 85,            // Key: U | u
        V = 86,            // Key: V | v
        W = 87,            // Key: W | w
        X = 88,            // Key: X | x
        Y = 89,            // Key: Y | y
        Z = 90,            // Key: Z | z
        LeftBracket = 91,  // Key: [
        Backslash = 92,    // Key: '\'
        RightBracket = 93, // Key: ]
        Grave = 96,        // Key: `
        // Function keys
        Space = 32,         // Key: Space
        Escape = 256,       // Key: Esc
        Enter = 257,        // Key: Enter
        Tab = 258,          // Key: Tab
        Backspace = 259,    // Key: Backspace
        Insert = 260,       // Key: Ins
        Delete = 261,       // Key: Del
        Right = 262,        // Key: Cursor right
        Left = 263,         // Key: Cursor left
        Down = 264,         // Key: Cursor down
        Up = 265,           // Key: Cursor up
        PageUp = 266,       // Key: Page up
        PageDown = 267,     // Key: Page down
        Home = 268,         // Key: Home
        End = 269,          // Key: End
        CapsLock = 280,     // Key: Caps lock
        ScrollLock = 281,   // Key: Scroll down
        NumLock = 282,      // Key: Num lock
        PrintScreen = 283,  // Key: Print screen
        Pause = 284,        // Key: Pause
        F1 = 290,           // Key: F1
        F2 = 291,           // Key: F2
        F3 = 292,           // Key: F3
        F4 = 293,           // Key: F4
        F5 = 294,           // Key: F5
        F6 = 295,           // Key: F6
        F7 = 296,           // Key: F7
        F8 = 297,           // Key: F8
        F9 = 298,           // Key: F9
        F10 = 299,          // Key: F10
        F11 = 300,          // Key: F11
        F12 = 301,          // Key: F12
        LeftShift = 340,    // Key: Shift left
        LeftControl = 341,  // Key: Control left
        LeftAlt = 342,      // Key: Alt left
        LeftSuper = 343,    // Key: Super left
        RightShift = 344,   // Key: Shift right
        RightControl = 345, // Key: Control right
        RightAlt = 346,     // Key: Alt right
        RightSuper = 347,   // Key: Super right
        KbMenu = 348,       // Key: KB menu
        // Keypad keys
        Kp0 = 320,        // Key: Keypad 0
        Kp1 = 321,        // Key: Keypad 1
        Kp2 = 322,        // Key: Keypad 2
        Kp3 = 323,        // Key: Keypad 3
        Kp4 = 324,        // Key: Keypad 4
        Kp5 = 325,        // Key: Keypad 5
        Kp6 = 326,        // Key: Keypad 6
        Kp7 = 327,        // Key: Keypad 7
        Kp8 = 328,        // Key: Keypad 8
        Kp9 = 329,        // Key: Keypad 9
        KpDecimal = 330,  // Key: Keypad .
        KpDivide = 331,   // Key: Keypad /
        KpMultiply = 332, // Key: Keypad *
        KpSubtract = 333, // Key: Keypad -
        KpAdd = 334,      // Key: Keypad +
        KpEnter = 335,    // Key: Keypad Enter
        KpEqual = 336,    // Key: Keypad =
        // Android key buttons
        Back = 4,        // Key: Android back button
        Menu = 5,        // Key: Android menu button
        VolumeUp = 24,   // Key: Android volume up button
        VolumeDown = 25, // Key: Android volume down button
    }

    #[link(name = "raylib")]
    extern "C" {
        pub fn InitWindow(width: c_int, height: c_int, title: *const c_char);
        pub fn CloseWindow();
        pub fn WindowShouldClose() -> bool;
        pub fn SetTargetFPS(fps: c_int);
        pub fn GetRenderWidth() -> c_int;
        pub fn GetRenderHeight() -> c_int;
        pub fn SetWindowState(flags: c_uint);
        pub fn SetWindowSize(width: c_int, height: c_int);
        pub fn IsWindowResized() -> c_int;

        pub fn LoadImageFromMemory(ext: *const c_char, data: *const c_uchar, size: c_int) -> Image;
        pub fn LoadImageSvg(file_name_or_str: *const c_char, width: c_int, height: c_int) -> Image;
        pub fn UnloadImage(image: Image);
        pub fn ImageResize(image: *mut Image, newWidth: c_int, newHeight: c_int);
        pub fn ImageCopy(image: Image) -> Image;
        pub fn GenImageColor(width: c_int, height: c_int, color: RaylibColour) -> Image;
        pub fn ImageAlphaMask(image: *mut Image, mask: Image);
        pub fn ImageDrawCircle(
            image: *mut Image,
            x: c_int,
            y: c_int,
            radius: c_int,
            color: RaylibColour,
        );

        pub fn LoadTextureFromImage(image: Image) -> Texture2D;
        pub fn UnloadTexture(texture: Texture2D);
        pub fn DrawTexture(texture: Texture2D, xpos: c_int, ypos: c_int, tint: RaylibColour);
        pub fn LoadRenderTexture(width: c_int, height: c_int) -> RenderTexture;
        pub fn UnloadRenderTexture(render_texture: RenderTexture);

        pub fn BeginDrawing();
        pub fn EndDrawing();
        pub fn ClearBackground(color: RaylibColour);
        pub fn BeginTextureMode(target: RenderTexture);
        pub fn EndTextureMode();

        pub fn SetMouseCursor(cursor: c_int);
        pub fn GetMousePosition() -> Vector2;
        pub fn IsMouseButtonDown(button: c_int) -> c_int;
        pub fn IsMouseButtonReleased(button: c_int) -> c_int;
        pub fn IsMouseButtonPressed(button: c_int) -> c_int;
        pub fn SetExitKey(key: c_int);
        pub fn IsKeyReleased(key: c_int) -> c_int;
        pub fn GetCharPressed() -> c_int;

        pub fn CheckCollisionPointRec(point: Vector2, rect: Rectangle) -> c_int;

        pub fn Fade(col: RaylibColour, alpha: c_float) -> RaylibColour;

        pub fn SetTraceLogLevel(log_level: c_int);

        pub fn DrawRectangle(
            pos_x: c_int,
            pos_y: c_int,
            width: c_int,
            height: c_int,
            colour: RaylibColour,
        );
        pub fn DrawCircle(center_x: c_int, center_y: c_int, radius: c_float, colour: RaylibColour);
        pub fn DrawCircleGradient(
            x: c_int,
            y: c_int,
            radius: c_float,
            colour1: RaylibColour,
            colour2: RaylibColour,
        );

        pub fn InitAudioDevice();
        pub fn CloseAudioDevice();
        pub fn LoadWaveFromMemory(
            file_type: *const c_char,
            file_data: *const c_uchar,
            data_size: c_int,
        ) -> Wave;
        pub fn LoadSoundFromWave(wave: Wave) -> Sound;
        pub fn UnloadWave(wave: Wave);
        pub fn UnloadSound(sound: Sound);
        pub fn PlaySound(sound: Sound);
    }
}

pub const WHITE: RaylibColour = RaylibColour {
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

    pub fn set_state(&mut self, flags: impl IntoIterator<Item = ConfigFlag>) {
        let mut flags_val = 0u32;
        for flag in flags.into_iter() {
            flags_val |= flag as u32;
        }
        unsafe { sys::SetWindowState(flags_val) };
    }

    pub fn set_size(&mut self, width: u32, height: u32) {
        unsafe { sys::SetWindowSize(width as _, height as _) };
    }

    pub fn is_resized(&self) -> bool {
        unsafe { sys::IsWindowResized() != 0 }
    }
}

impl Drop for Window {
    fn drop(&mut self) {
        unsafe { sys::CloseWindow() };
    }
}

pub struct AudioDevice;

impl AudioDevice {
    pub fn new() -> Self {
        unsafe { sys::InitAudioDevice() };
        Self
    }
}

impl Drop for AudioDevice {
    fn drop(&mut self) {
        unsafe { sys::CloseAudioDevice() };
    }
}

#[derive(Clone, Copy)]
pub enum WaveFormat {
    Ogg,
}

impl WaveFormat {
    pub fn to_cstr(self) -> &'static CStr {
        match self {
            WaveFormat::Ogg => CStr::from_bytes_with_nul(b".ogg\0").unwrap(),
        }
    }
}

pub struct Wave {
    wave: sys::Wave,
}

impl Wave {
    pub fn from_mem(format: WaveFormat, data: &[u8]) -> Self {
        let wave = unsafe {
            sys::LoadWaveFromMemory(format.to_cstr().as_ptr(), data.as_ptr(), data.len() as _)
        };
        Self { wave }
    }
}

impl Drop for Wave {
    fn drop(&mut self) {
        unsafe { sys::UnloadWave(self.wave) };
    }
}

pub struct Sound {
    sound: sys::Sound,
}

impl Sound {
    pub fn play(&self) {
        unsafe { sys::PlaySound(self.sound) };
    }
}

impl From<&Wave> for Sound {
    fn from(wave: &Wave) -> Self {
        let sound = unsafe { sys::LoadSoundFromWave(wave.wave) };
        Self { sound }
    }
}

impl Drop for Sound {
    fn drop(&mut self) {
        unsafe { sys::UnloadSound(self.sound) };
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

    pub fn gen_colour(width: u32, height: u32, colour: RaylibColour) -> Self {
        let img = unsafe { sys::GenImageColor(width as _, height as _, colour) };
        Self { img }
    }

    pub fn size(&self) -> (u32, u32) {
        (self.img.width as _, self.img.height as _)
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        unsafe { sys::ImageResize(&mut self.img as *mut _, width as _, height as _) };
    }

    pub fn alpha_mask(&mut self, mask: &Image) {
        unsafe { sys::ImageAlphaMask(&mut self.img as *mut _, mask.img) };
    }

    pub fn draw_circle(&mut self, x: u32, y: u32, radius: u32, colour: RaylibColour) {
        unsafe {
            sys::ImageDrawCircle(&mut self.img as *mut _, x as _, y as _, radius as _, colour)
        };
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
    pub fn draw(&self, x: u32, y: u32, tint: RaylibColour) {
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

pub struct RenderTexture {
    rtex: sys::RenderTexture,
}

impl RenderTexture {
    pub fn new(width: u32, height: u32) -> Self {
        let rtex = unsafe { sys::LoadRenderTexture(width as _, height as _) };
        Self { rtex }
    }

    pub fn draw(&self, x: u32, y: u32, tint: RaylibColour) {
        unsafe { sys::DrawTexture(self.rtex.texture, x as _, y as _, tint) };
    }

    pub fn do_draw(&mut self, draw_fn: impl FnOnce()) {
        unsafe { sys::BeginTextureMode(self.rtex) };
        draw_fn();
        unsafe { sys::EndTextureMode() };
    }
}

impl Drop for RenderTexture {
    fn drop(&mut self) {
        unsafe { sys::UnloadRenderTexture(self.rtex) };
    }
}

pub fn do_draw(draw_fn: impl FnOnce()) {
    unsafe { sys::BeginDrawing() };
    draw_fn();
    unsafe { sys::EndDrawing() };
}

pub fn clear_background(col: RaylibColour) {
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

pub fn is_mouse_button_pressed(mb: MouseButton) -> bool {
    unsafe { sys::IsMouseButtonPressed(mb as _) != 0 }
}

pub fn is_mouse_button_released(mb: MouseButton) -> bool {
    unsafe { sys::IsMouseButtonReleased(mb as _) != 0 }
}

pub fn is_key_released(key: Key) -> bool {
    unsafe { sys::IsKeyReleased(key as _) != 0 }
}

pub fn get_char_pressed() -> char {
    char::from_u32(unsafe { sys::GetCharPressed() } as u32).unwrap()
}

pub fn set_exit_key(key: Key) {
    unsafe { sys::SetExitKey(key as _) };
}

pub fn set_trace_log_level(level: TraceLogLevel) {
    unsafe { sys::SetTraceLogLevel(level as _) };
}

pub fn draw_rectangle(x: u32, y: u32, width: u32, height: u32, colour: RaylibColour) {
    unsafe { sys::DrawRectangle(x as _, y as _, width as _, height as _, colour) };
}

pub fn draw_circle(x: u32, y: u32, radius: f32, colour: RaylibColour) {
    unsafe { sys::DrawCircle(x as _, y as _, radius as _, colour) };
}

pub fn draw_circle_gradient(
    x: u32,
    y: u32,
    radius: f32,
    colour1: RaylibColour,
    colour2: RaylibColour,
) {
    unsafe { sys::DrawCircleGradient(x as _, y as _, radius as _, colour1, colour2) };
}
