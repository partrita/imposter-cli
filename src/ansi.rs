use crate::types::Theme;
use std::fmt::Write as _;
use std::io::{self, BufWriter, IsTerminal, Stdout, Write};
use std::thread;
use std::time::Duration;

pub const RESET: &str = "\x1b[0m";
pub const BOLD: &str = "\x1b[1m";
pub const DIM: &str = "\x1b[2m";
pub const RED: &str = "\x1b[31m";
pub const GREEN: &str = "\x1b[32m";
pub const YELLOW: &str = "\x1b[33m";
pub const GRAY: &str = "\x1b[90m";
/// Claude-ish warm accent, 24-bit.
pub const ACCENT: &str = "\x1b[38;2;215;119;87m";

// Light-theme 24-bit foreground palette (high contrast on light terminals).
const LIGHT_TEXT: &str = "\x1b[38;2;31;35;40m";
const LIGHT_BOLD: &str = "\x1b[1;38;2;15;23;42m";
const LIGHT_DIM: &str = "\x1b[38;2;87;96;106m";
const LIGHT_GRAY: &str = "\x1b[38;2;140;149;159m";
const LIGHT_RED: &str = "\x1b[38;2;164;14;38m";
const LIGHT_GREEN: &str = "\x1b[38;2;17;99;41m";
const LIGHT_YELLOW: &str = "\x1b[38;2;154;103;0m";
const LIGHT_ACCENT_CLAUDE: &str = "\x1b[38;2;180;76;43m";
const LIGHT_ACCENT_CODEX: &str = "\x1b[38;2;9;105;218m";
const LIGHT_ACCENT_OPENCODE: &str = "\x1b[38;2;37;99;235m";
const LIGHT_ACCENT_PI: &str = "\x1b[38;2;20;118;110m";
const LIGHT_ACCENT_AGY: &str = "\x1b[38;2;26;115;232m";

// Pi (`earendil-works/pi` dark.json & light.json) Box background + foreground tokens.
pub const PI_USER_BG: &str = "\x1b[48;2;52;53;65;38;2;236;236;241m";
pub const PI_TOOL_OK_BG: &str = "\x1b[48;2;40;50;40;38;2;212;220;212m";
pub const PI_TOOL_OK_TITLE: &str = "\x1b[48;2;40;50;40;1;38;2;138;190;183m";
pub const PI_TOOL_OK_DIM: &str = "\x1b[48;2;40;50;40;38;2;148;163;148m";
pub const PI_TOOL_OK_ADD: &str = "\x1b[48;2;40;50;40;38;2;134;239;172m";
pub const PI_TOOL_OK_DEL: &str = "\x1b[48;2;40;50;40;38;2;252;165;165m";
pub const PI_TOOL_PEND_BG: &str = "\x1b[48;2;40;40;50;38;2;212;212;224m";
pub const PI_TOOL_PEND_TITLE: &str = "\x1b[48;2;40;40;50;1;38;2;138;190;183m";
pub const PI_TOOL_PEND_DIM: &str = "\x1b[48;2;40;40;50;38;2;148;152;168m";
pub const PI_TOOL_ERR_BG: &str = "\x1b[48;2;60;40;40;38;2;252;165;165m";
pub const PI_TOOL_ERR_TITLE: &str = "\x1b[48;2;60;40;40;1;38;2;248;113;113m";

const LIGHT_PI_USER_BG: &str = "\x1b[48;2;229;231;235;38;2;15;23;42m";
const LIGHT_PI_TOOL_OK_BG: &str = "\x1b[48;2;222;238;222;38;2;20;40;20m";
const LIGHT_PI_TOOL_OK_TITLE: &str = "\x1b[48;2;222;238;222;1;38;2;17;99;41m";
const LIGHT_PI_TOOL_OK_DIM: &str = "\x1b[48;2;222;238;222;38;2;75;95;75m";
const LIGHT_PI_TOOL_OK_ADD: &str = "\x1b[48;2;222;238;222;38;2;17;99;41m";
const LIGHT_PI_TOOL_OK_DEL: &str = "\x1b[48;2;222;238;222;38;2;164;14;38m";
const LIGHT_PI_TOOL_PEND_BG: &str = "\x1b[48;2;232;234;242;38;2;30;41;59m";
const LIGHT_PI_TOOL_PEND_TITLE: &str = "\x1b[48;2;232;234;242;1;38;2;20;118;110m";
const LIGHT_PI_TOOL_PEND_DIM: &str = "\x1b[48;2;232;234;242;38;2;90;100;115m";
const LIGHT_PI_TOOL_ERR_BG: &str = "\x1b[48;2;250;226;226;38;2;153;27;27m";
const LIGHT_PI_TOOL_ERR_TITLE: &str = "\x1b[48;2;250;226;226;1;38;2;164;14;38m";

// OpenCode (`anomalyco/opencode`) `backgroundPanel` + `SplitBorder` tokens.
pub const OC_PANEL_BG: &str = "\x1b[48;2;40;44;52;38;2;226;232;240m";
pub const OC_PANEL_BOLD: &str = "\x1b[48;2;40;44;52;1;38;2;248;250;252m";
pub const OC_PANEL_DIM: &str = "\x1b[48;2;40;44;52;38;2;138;148;172m";
pub const OC_PANEL_BORDER: &str = "\x1b[48;2;40;44;52;38;2;86;94;122m";
pub const OC_PANEL_ACCENT: &str = "\x1b[48;2;40;44;52;1;38;2;86;156;255m";
pub const OC_PANEL_GREEN: &str = "\x1b[48;2;40;44;52;38;2;134;239;172m";
pub const OC_PANEL_RED: &str = "\x1b[48;2;40;44;52;38;2;252;165;165m";

const LIGHT_OC_PANEL_BG: &str = "\x1b[48;2;216;220;230;38;2;30;41;59m";
const LIGHT_OC_PANEL_BOLD: &str = "\x1b[48;2;216;220;230;1;38;2;15;23;42m";
const LIGHT_OC_PANEL_DIM: &str = "\x1b[48;2;216;220;230;38;2;90;105;125m";
const LIGHT_OC_PANEL_BORDER: &str = "\x1b[48;2;216;220;230;38;2;140;155;175m";
const LIGHT_OC_PANEL_ACCENT: &str = "\x1b[48;2;216;220;230;1;38;2;37;99;235m";
const LIGHT_OC_PANEL_GREEN: &str = "\x1b[48;2;216;220;230;38;2;17;99;41m";
const LIGHT_OC_PANEL_RED: &str = "\x1b[48;2;216;220;230;38;2;164;14;38m";

/// Windows consoles need opt-in before they interpret ANSI escapes.
/// Hand-rolled kernel32 calls keep the Windows build dependency-free; on every
/// other platform this compiles to nothing.
#[cfg(windows)]
pub fn enable_ansi() {
    use std::ffi::c_void;
    use std::os::windows::io::AsRawHandle;

    #[link(name = "kernel32")]
    extern "system" {
        fn GetConsoleMode(handle: *mut c_void, mode: *mut u32) -> i32;
        fn SetConsoleMode(handle: *mut c_void, mode: u32) -> i32;
    }

    const ENABLE_VIRTUAL_TERMINAL_PROCESSING: u32 = 0x0004;

    let handle = io::stdout().as_raw_handle();
    let mut mode = 0u32;
    // SAFETY: both calls take the console handle we just obtained from std and
    // a valid pointer to a u32. Failure is non-fatal; output is still readable.
    unsafe {
        if GetConsoleMode(handle, &mut mode) != 0 {
            let _ = SetConsoleMode(handle, mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING);
        }
    }
}

#[cfg(not(windows))]
#[inline]
pub fn enable_ansi() {}

#[cfg(unix)]
mod raw_input {
    use std::io::{self, IsTerminal};
    use std::sync::atomic::{AtomicBool, Ordering};

    #[repr(C)]
    #[derive(Clone, Copy)]
    struct Termios {
        c_iflag: u32,
        c_oflag: u32,
        c_cflag: u32,
        c_lflag: u32,
        c_line: u8,
        c_cc: [u8; 32],
        c_ispeed: u32,
        c_ospeed: u32,
    }

    const TCSANOW: i32 = 0;
    const ICANON: u32 = 0o000002;
    const ECHO: u32 = 0o000010;
    const VMIN: usize = 6;
    const VTIME: usize = 5;

    #[link(name = "c")]
    extern "C" {
        fn tcgetattr(fd: i32, termios_p: *mut Termios) -> i32;
        fn tcsetattr(fd: i32, optional_actions: i32, termios_p: *const Termios) -> i32;
        #[link_name = "read"]
        fn libc_read(fd: i32, buf: *mut std::ffi::c_void, count: usize) -> isize;
    }

    static HAS_ORIG: AtomicBool = AtomicBool::new(false);
    static mut ORIG_TERMIOS: Option<Termios> = None;

    pub struct RawModeGuard {
        active: bool,
    }

    impl RawModeGuard {
        pub fn new() -> Self {
            if !io::stdin().is_terminal() {
                return Self { active: false };
            }
            let mut orig = std::mem::MaybeUninit::<Termios>::uninit();
            if unsafe { tcgetattr(0, orig.as_mut_ptr()) } != 0 {
                return Self { active: false };
            }
            let orig = unsafe { orig.assume_init() };
            unsafe {
                ORIG_TERMIOS = Some(orig);
                HAS_ORIG.store(true, Ordering::SeqCst);
            }

            let mut raw = orig;
            raw.c_lflag &= !(ICANON | ECHO);
            raw.c_cc[VMIN] = 0;
            raw.c_cc[VTIME] = 0;
            let ok = unsafe { tcsetattr(0, TCSANOW, &raw) } == 0;
            Self { active: ok }
        }
    }

    impl Drop for RawModeGuard {
        fn drop(&mut self) {
            if self.active {
                restore_terminal();
            }
        }
    }

    pub fn restore_terminal() {
        if HAS_ORIG.swap(false, Ordering::SeqCst) {
            if let Some(orig) = unsafe { ORIG_TERMIOS } {
                unsafe {
                    tcsetattr(0, TCSANOW, &orig);
                }
            }
        }
    }

    pub fn check_esc() -> bool {
        let mut buf = [0u8; 32];
        let n = unsafe { libc_read(0, buf.as_mut_ptr() as *mut std::ffi::c_void, buf.len()) };
        if n > 0 {
            let slice = &buf[..n as usize];
            return slice.contains(&0x1b);
        }
        false
    }
}

#[cfg(windows)]
mod raw_input {
    use std::ffi::c_void;
    use std::io::{self, IsTerminal};
    use std::os::windows::io::AsRawHandle;
    use std::sync::atomic::{AtomicBool, Ordering};

    #[repr(C)]
    struct InputRecord {
        event_type: u16,
        _pad: u16,
        key_event: KeyEventRecord,
    }

    #[repr(C)]
    struct KeyEventRecord {
        key_down: i32,
        repeat_count: u16,
        virtual_key_code: u16,
        virtual_scan_code: u16,
        u_char: u16,
        control_key_state: u32,
    }

    const ENABLE_PROCESSED_INPUT: u32 = 0x0001;
    const ENABLE_LINE_INPUT: u32 = 0x0002;
    const ENABLE_ECHO_INPUT: u32 = 0x0004;
    const KEY_EVENT: u16 = 0x0001;
    const VK_ESCAPE: u16 = 0x1B;

    #[link(name = "kernel32")]
    extern "system" {
        fn GetConsoleMode(handle: *mut c_void, mode: *mut u32) -> i32;
        fn SetConsoleMode(handle: *mut c_void, mode: u32) -> i32;
        fn PeekConsoleInputW(
            handle: *mut c_void,
            buffer: *mut InputRecord,
            length: u32,
            events_read: *mut u32,
        ) -> i32;
        fn ReadConsoleInputW(
            handle: *mut c_void,
            buffer: *mut InputRecord,
            length: u32,
            events_read: *mut u32,
        ) -> i32;
    }

    static HAS_ORIG: AtomicBool = AtomicBool::new(false);
    static mut ORIG_MODE: u32 = 0;

    pub struct RawModeGuard {
        active: bool,
    }

    impl RawModeGuard {
        pub fn new() -> Self {
            if !io::stdin().is_terminal() {
                return Self { active: false };
            }
            let handle = io::stdin().as_raw_handle();
            let mut mode = 0u32;
            if unsafe { GetConsoleMode(handle, &mut mode) } == 0 {
                return Self { active: false };
            }
            unsafe {
                ORIG_MODE = mode;
                HAS_ORIG.store(true, Ordering::SeqCst);
            }
            let raw_mode = mode & !(ENABLE_LINE_INPUT | ENABLE_ECHO_INPUT);
            let ok = unsafe { SetConsoleMode(handle, raw_mode) } != 0;
            Self { active: ok }
        }
    }

    impl Drop for RawModeGuard {
        fn drop(&mut self) {
            if self.active {
                restore_terminal();
            }
        }
    }

    pub fn restore_terminal() {
        if HAS_ORIG.swap(false, Ordering::SeqCst) {
            let handle = io::stdin().as_raw_handle();
            unsafe {
                SetConsoleMode(handle, ORIG_MODE);
            }
        }
    }

    pub fn check_esc() -> bool {
        let handle = io::stdin().as_raw_handle();
        let mut records: [InputRecord; 16] = unsafe { std::mem::zeroed() };
        let mut read = 0u32;
        if unsafe { PeekConsoleInputW(handle, records.as_mut_ptr(), 16, &mut read) } != 0
            && read > 0
        {
            let mut saw_esc = false;
            for r in &records[..read as usize] {
                if r.event_type == KEY_EVENT
                    && r.key_event.key_down != 0
                    && (r.key_event.virtual_key_code == VK_ESCAPE || r.key_event.u_char == 0x1B)
                {
                    saw_esc = true;
                    break;
                }
            }
            let _ = unsafe { ReadConsoleInputW(handle, records.as_mut_ptr(), read, &mut read) };
            return saw_esc;
        }
        false
    }
}

pub use raw_input::{check_esc, restore_terminal, RawModeGuard};

const CLEAR_LINE: &str = "\r\x1b[2K";
const SPINNER_BRAILLE: [&str; 10] = [
    "\u{280b}", "\u{2819}", "\u{2839}", "\u{2838}", "\u{283c}", "\u{2834}", "\u{2826}", "\u{2827}",
    "\u{2807}", "\u{280f}",
];
const SPINNER_CLAUDE: [&str; 10] = SPINNER_BRAILLE;
const SPINNER_OPENCODE: [&str; 4] = ["\u{25a3}", "\u{25a4}", "\u{25a5}", "\u{25a6}"];
const SPINNER_AGY: [&str; 4] = ["\u{25cb}", "\u{25ce}", "\u{25cf}", "\u{25ce}"];

/// Which CLI's status line wording to use while "thinking".
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum SpinnerKind {
    Claude,
    Codex,
    Opencode,
    Pi,
    Antigravity,
}

#[cfg(unix)]
fn detect_terminal_width() -> Option<usize> {
    #[repr(C)]
    struct Winsize {
        ws_row: u16,
        ws_col: u16,
        ws_xpixel: u16,
        ws_ypixel: u16,
    }

    #[link(name = "c")]
    extern "C" {
        fn ioctl(fd: i32, request: LibcUlong, ...) -> i32;
    }

    #[cfg(target_os = "macos")]
    type LibcUlong = u64;
    #[cfg(all(unix, not(target_os = "macos")))]
    type LibcUlong = u64;

    #[cfg(target_os = "macos")]
    const TIOCGWINSZ: LibcUlong = 0x40087468;
    #[cfg(all(unix, not(target_os = "macos")))]
    const TIOCGWINSZ: LibcUlong = 0x5413;

    let mut ws = Winsize {
        ws_row: 0,
        ws_col: 0,
        ws_xpixel: 0,
        ws_ypixel: 0,
    };

    // Try stdout (1), then stdin (0), then stderr (2)
    for fd in [1, 0, 2] {
        if unsafe { ioctl(fd, TIOCGWINSZ, &mut ws) } == 0 && ws.ws_col > 0 {
            return Some(ws.ws_col as usize);
        }
    }
    None
}

#[cfg(windows)]
fn detect_terminal_width() -> Option<usize> {
    use std::ffi::c_void;
    use std::os::windows::io::AsRawHandle;

    #[repr(C)]
    struct Coord {
        x: i16,
        y: i16,
    }

    #[repr(C)]
    struct SmallRect {
        left: i16,
        top: i16,
        right: i16,
        bottom: i16,
    }

    #[repr(C)]
    struct ConsoleScreenBufferInfo {
        size: Coord,
        cursor_position: Coord,
        attributes: u16,
        window: SmallRect,
        maximum_window_size: Coord,
    }

    #[link(name = "kernel32")]
    extern "system" {
        fn GetConsoleScreenBufferInfo(
            handle: *mut c_void,
            info: *mut ConsoleScreenBufferInfo,
        ) -> i32;
    }

    let handle = io::stdout().as_raw_handle();
    let mut info = std::mem::MaybeUninit::<ConsoleScreenBufferInfo>::uninit();
    if unsafe { GetConsoleScreenBufferInfo(handle, info.as_mut_ptr()) } != 0 {
        let info = unsafe { info.assume_init() };
        let w = (info.window.right - info.window.left + 1).max(0) as usize;
        if w > 0 {
            return Some(w);
        }
    }
    None
}

#[cfg(not(any(unix, windows)))]
fn detect_terminal_width() -> Option<usize> {
    None
}

pub fn current_terminal_width() -> usize {
    if let Some(w) = detect_terminal_width() {
        return w.clamp(20, 200);
    }
    std::env::var("COLUMNS")
        .ok()
        .and_then(|v| v.trim().parse::<usize>().ok())
        .unwrap_or(80)
        .clamp(20, 200)
}

/// Buffered ANSI writer.
///
/// Design constraints, in order: small binary, low RSS, no terminal state left
/// behind. No alternate screen, no raw mode, no hidden cursor, no threads.
/// Every line is assembled in one reusable `String` and written with a single
/// `write_all`, and SGR codes are only emitted when the style actually changes.
pub struct Ui {
    out: BufWriter<Stdout>,
    line: String,
    color: bool,
    pub width: usize,
    pub animate: bool,
    pub theme: Theme,
}

impl Ui {
    pub fn new(color: bool, animate: bool, theme: Theme) -> Self {
        let width = current_terminal_width();
        Self {
            out: BufWriter::with_capacity(8 * 1024, io::stdout()),
            line: String::with_capacity(512),
            color,
            width,
            animate,
            theme,
        }
    }

    /// Refresh the cached terminal width to adapt to dynamic window resizes.
    #[inline]
    pub fn refresh_width(&mut self) -> usize {
        self.width = current_terminal_width();
        self.width
    }

    pub fn stdout_is_tty() -> bool {
        io::stdout().is_terminal()
    }

    /// Maps a dark-mode SGR token to its high-contrast light-mode equivalent
    /// when `self.theme == Theme::Light`.
    #[inline]
    pub fn resolve_code(&self, code: &'static str) -> &'static str {
        map_theme_code(self.theme, code)
    }

    pub fn parts(&mut self, parts: &[(&'static str, &str)]) {
        self.parts_inner(parts, true);
    }

    /// Renders parts inline without a trailing newline. Useful for progress bars.
    pub fn parts_inline(&mut self, parts: &[(&'static str, &str)]) {
        self.parts_inner(parts, false);
    }

    fn parts_inner(&mut self, parts: &[(&'static str, &str)], newline: bool) {
        let color = self.color;
        let theme = self.theme;
        let line = &mut self.line;
        line.clear();

        let mut current = "";
        for &(raw_code, text) in parts {
            let code = map_theme_code(theme, raw_code);
            if color && code != current {
                if !current.is_empty() {
                    line.push_str(RESET);
                }
                if !code.is_empty() {
                    line.push_str(code);
                }
                current = code;
            }
            line.push_str(text);
        }
        if color && !current.is_empty() {
            line.push_str(RESET);
        }
        if newline {
            line.push('\n');
        }

        if self.out.write_all(line.as_bytes()).is_err() {
            // Reader went away (e.g. `imposter | head`). Exit instead of
            // spinning forever against a closed pipe.
            std::process::exit(0);
        }
    }

    pub fn nl(&mut self) {
        if self.out.write_all(b"\n").is_err() {
            std::process::exit(0);
        }
    }

    /// Raw write with no trailing newline and no styling. Used by the prompt
    /// box, which has to type into the middle of a line.
    pub fn write_raw(&mut self, text: &str) {
        if self.out.write_all(text.as_bytes()).is_err() {
            std::process::exit(0);
        }
    }

    pub fn write_styled(&mut self, code: &'static str, text: &str) {
        let resolved = self.resolve_code(code);
        if self.color && !resolved.is_empty() {
            self.write_raw(resolved);
        }
        self.write_raw(text);
        if self.color && !resolved.is_empty() {
            self.write_raw(RESET);
        }
    }

    /// Typewriter effect: one flush per character. This is the only place that
    /// pays a syscall per character, and only when animation is on.
    pub fn type_text(&mut self, text: &str, per_char_ms: u64) {
        if !self.animate || per_char_ms == 0 {
            if self.color && self.theme == Theme::Light {
                self.write_styled("", text);
            } else {
                self.write_raw(text);
            }
            return;
        }
        if self.color && self.theme == Theme::Light {
            self.write_raw(LIGHT_TEXT);
        }
        for ch in text.chars() {
            let mut buf = [0u8; 4];
            self.write_raw(ch.encode_utf8(&mut buf));
            if self.out.flush().is_err() {
                std::process::exit(0);
            }
            if check_esc() {
                Self::exit_by_esc();
            }
            thread::sleep(Duration::from_millis(per_char_ms));
        }
        if self.color && self.theme == Theme::Light {
            self.write_raw(RESET);
        }
    }

    /// Typewriter effect inside a full-width background-tinted row (used by Pi's
    /// `UserMessageComponent` and OpenCode's `SplitBorder` user panel).
    pub fn type_on_bg(
        &mut self,
        bg_code: &'static str,
        lead: &str,
        text: &str,
        pad: &str,
        per_char_ms: u64,
    ) {
        self.type_on_bg_lead(bg_code, lead, bg_code, text, pad, per_char_ms);
    }

    pub fn type_on_bg_lead(
        &mut self,
        lead_code: &'static str,
        lead: &str,
        bg_code: &'static str,
        text: &str,
        pad: &str,
        per_char_ms: u64,
    ) {
        if !self.animate || per_char_ms == 0 {
            self.parts(&[(lead_code, lead), (bg_code, text), (bg_code, pad)]);
            return;
        }
        let resolved_lead = self.resolve_code(lead_code);
        let resolved_bg = self.resolve_code(bg_code);
        if self.color && !resolved_lead.is_empty() {
            self.write_raw(resolved_lead);
        }
        self.write_raw(lead);
        if self.color && resolved_bg != resolved_lead {
            if !resolved_lead.is_empty() {
                self.write_raw(RESET);
            }
            if !resolved_bg.is_empty() {
                self.write_raw(resolved_bg);
            }
        }
        for ch in text.chars() {
            let mut buf = [0u8; 4];
            self.write_raw(ch.encode_utf8(&mut buf));
            if self.out.flush().is_err() {
                std::process::exit(0);
            }
            if check_esc() {
                Self::exit_by_esc();
            }
            thread::sleep(Duration::from_millis(per_char_ms));
        }
        self.write_raw(pad);
        if self.color && (!resolved_bg.is_empty() || !resolved_lead.is_empty()) {
            self.write_raw(RESET);
        }
        self.nl();
    }

    pub fn blank(&mut self) {
        self.nl();
    }

    pub fn flush(&mut self) {
        if self.out.flush().is_err() {
            std::process::exit(0);
        }
    }

    /// Frame pacing in one place. Without a TTY (or with `--fast`) everything
    /// runs six times faster instead of sleeping for real seconds.
    pub fn pause(&self, ms: u64) {
        let ms = if self.animate { ms } else { ms / 6 };
        if ms == 0 {
            if check_esc() {
                Self::exit_by_esc();
            }
            return;
        }

        let step_ms = 25u64;
        let mut elapsed = 0u64;
        while elapsed < ms {
            if check_esc() {
                Self::exit_by_esc();
            }
            let chunk = (ms - elapsed).min(step_ms);
            thread::sleep(Duration::from_millis(chunk));
            elapsed += chunk;
        }
        if check_esc() {
            Self::exit_by_esc();
        }
    }

    pub fn exit_by_esc() -> ! {
        restore_terminal();
        let mut stdout = std::io::stdout();
        let _ = std::io::Write::write_all(&mut stdout, b"\x1b[?25h\x1b[0m\n");
        let _ = std::io::Write::flush(&mut stdout);
        std::process::exit(130);
    }

    /// One-line spinner that overwrites itself, reusing the line buffer for
    /// every frame: no allocation per tick.
    pub fn spinner(
        &mut self,
        accent_code: &'static str,
        verb: &str,
        secs: f32,
        tokens: u32,
        kind: SpinnerKind,
        model_name: &str,
    ) {
        let label = tokens_label(tokens);
        let agy_model = if model_name.is_empty() {
            "Gemini 3.1 Pro"
        } else {
            model_name
        };
        if !self.animate {
            match kind {
                SpinnerKind::Codex => self.parts(&[
                    (accent_code, "\u{2022}"),
                    (DIM, " "),
                    (DIM, verb),
                    (DIM, " (1s \u{2022} esc to interrupt)"),
                ]),
                SpinnerKind::Opencode => self.parts(&[
                    (DIM, "  "),
                    (accent_code, "\u{25a3}"),
                    (DIM, " "),
                    (DIM, verb),
                    (DIM, &format!("... ({label} tokens \u{00b7} esc interrupt)")),
                ]),
                SpinnerKind::Pi => self.parts(&[
                    (DIM, "  "),
                    (accent_code, "\u{280b}"),
                    (DIM, " "),
                    (DIM, verb),
                    (DIM, "... (escape to interrupt)"),
                ]),
                SpinnerKind::Antigravity => self.parts(&[
                    (accent_code, "\u{25cb}"),
                    (DIM, " "),
                    (DIM, verb),
                    (
                        DIM,
                        &format!(
                            "\u{2026} (esc to cancel \u{00b7} 1s \u{00b7} {agy_model} \u{00b7} {label} tokens)"
                        ),
                    ),
                ]),
                SpinnerKind::Claude => self.parts(&[
                    (accent_code, "\u{280b}"),
                    (DIM, " "),
                    (DIM, verb),
                    (
                        DIM,
                        &format!("\u{2026} (esc to interrupt \u{00b7} 1s \u{00b7} \u{2193} {label} tokens)"),
                    ),
                ]),
            }
            return;
        }

        let accent = if self.color {
            self.resolve_code(accent_code)
        } else {
            ""
        };
        let dim = if self.color {
            self.resolve_code(DIM)
        } else {
            ""
        };
        let body = if self.color && self.theme == Theme::Light {
            LIGHT_TEXT
        } else {
            ""
        };
        let reset = if self.color { RESET } else { "" };
        let step_ms = 80u64;
        let ticks = ((secs * 1000.0) as u64 / step_ms).max(4);
        let line = &mut self.line;

        for i in 0..ticks {
            let secs = i * step_ms / 1000;
            let idx = i as usize;
            line.clear();
            line.push_str(CLEAR_LINE);
            if matches!(kind, SpinnerKind::Opencode | SpinnerKind::Pi) {
                line.push_str("  ");
            }
            line.push_str(accent);
            let frame = match kind {
                SpinnerKind::Claude => SPINNER_CLAUDE[idx % SPINNER_CLAUDE.len()],
                SpinnerKind::Codex | SpinnerKind::Pi => {
                    SPINNER_BRAILLE[idx % SPINNER_BRAILLE.len()]
                }
                SpinnerKind::Opencode => SPINNER_OPENCODE[idx % SPINNER_OPENCODE.len()],
                SpinnerKind::Antigravity => SPINNER_AGY[idx % SPINNER_AGY.len()],
            };
            line.push_str(frame);
            line.push_str(reset);
            line.push(' ');
            line.push_str(body);
            line.push_str(verb);
            match kind {
                SpinnerKind::Claude => {
                    line.push('\u{2026}');
                    line.push_str(dim);
                    let _ = write!(
                        line,
                        " (esc to interrupt \u{00b7} {secs}s \u{00b7} \u{2193} {label} tokens)"
                    );
                }
                SpinnerKind::Codex => {
                    line.push_str(dim);
                    let _ = write!(line, " ({secs}s \u{2022} esc to interrupt)");
                }
                SpinnerKind::Opencode => {
                    line.push_str("...");
                    line.push_str(dim);
                    let _ = write!(line, " ({label} tokens \u{00b7} esc interrupt)");
                }
                SpinnerKind::Pi => {
                    line.push_str("...");
                    line.push_str(dim);
                    let _ = write!(line, " ({secs}s \u{00b7} escape to interrupt)");
                }
                SpinnerKind::Antigravity => {
                    line.push('\u{2026}');
                    line.push_str(dim);
                    let _ = write!(
                        line,
                        " (esc to cancel \u{00b7} {secs}s \u{00b7} {agy_model} \u{00b7} {label} tokens)"
                    );
                }
            }
            line.push_str(reset);

            if self.out.write_all(line.as_bytes()).is_err() || self.out.flush().is_err() {
                std::process::exit(0);
            }
            if check_esc() {
                let _ = self.out.write_all(CLEAR_LINE.as_bytes());
                let _ = self.out.flush();
                Self::exit_by_esc();
            }
            thread::sleep(Duration::from_millis(step_ms));
        }

        let _ = self.out.write_all(CLEAR_LINE.as_bytes());
        let _ = self.out.flush();
    }
}

#[inline]
fn map_theme_code(theme: Theme, code: &'static str) -> &'static str {
    if theme == Theme::Dark {
        return code;
    }
    match code {
        "" => LIGHT_TEXT,
        BOLD => LIGHT_BOLD,
        DIM => LIGHT_DIM,
        GRAY => LIGHT_GRAY,
        RED => LIGHT_RED,
        GREEN => LIGHT_GREEN,
        YELLOW => LIGHT_YELLOW,
        ACCENT => LIGHT_ACCENT_CLAUDE,
        "\x1b[38;2;114;161;255m" => LIGHT_ACCENT_CODEX,
        "\x1b[38;2;86;156;255m" => LIGHT_ACCENT_OPENCODE,
        "\x1b[38;2;138;190;183m" | "\x1b[38;2;216;110;199m" => LIGHT_ACCENT_PI,
        "\x1b[38;2;138;180;248m" | "\x1b[38;2;167;139;250m" => LIGHT_ACCENT_AGY,
        PI_USER_BG => LIGHT_PI_USER_BG,
        PI_TOOL_OK_BG => LIGHT_PI_TOOL_OK_BG,
        PI_TOOL_OK_TITLE => LIGHT_PI_TOOL_OK_TITLE,
        PI_TOOL_OK_DIM => LIGHT_PI_TOOL_OK_DIM,
        PI_TOOL_OK_ADD => LIGHT_PI_TOOL_OK_ADD,
        PI_TOOL_OK_DEL => LIGHT_PI_TOOL_OK_DEL,
        PI_TOOL_PEND_BG => LIGHT_PI_TOOL_PEND_BG,
        PI_TOOL_PEND_TITLE => LIGHT_PI_TOOL_PEND_TITLE,
        PI_TOOL_PEND_DIM => LIGHT_PI_TOOL_PEND_DIM,
        PI_TOOL_ERR_BG => LIGHT_PI_TOOL_ERR_BG,
        PI_TOOL_ERR_TITLE => LIGHT_PI_TOOL_ERR_TITLE,
        OC_PANEL_BG => LIGHT_OC_PANEL_BG,
        OC_PANEL_BOLD => LIGHT_OC_PANEL_BOLD,
        OC_PANEL_DIM => LIGHT_OC_PANEL_DIM,
        OC_PANEL_BORDER => LIGHT_OC_PANEL_BORDER,
        OC_PANEL_ACCENT => LIGHT_OC_PANEL_ACCENT,
        OC_PANEL_GREEN => LIGHT_OC_PANEL_GREEN,
        OC_PANEL_RED => LIGHT_OC_PANEL_RED,
        other => other,
    }
}

pub fn tokens_label(n: u32) -> String {
    if n >= 1000 {
        format!("{:.1}k", n as f32 / 1000.0)
    } else {
        n.to_string()
    }
}

/// Greedy word wrap on chars. The content is ASCII apart from a few
/// single-width glyphs, so a unicode width table would be pure overhead.
pub fn wrap(text: &str, width: usize) -> Vec<String> {
    let width = width.max(8);
    let mut out: Vec<String> = Vec::new();
    let mut cur = String::new();
    for word in text.split_whitespace() {
        let wlen = word.chars().count();
        if cur.is_empty() {
            cur.push_str(word);
        } else if cur.chars().count() + 1 + wlen <= width {
            cur.push(' ');
            cur.push_str(word);
        } else {
            out.push(std::mem::take(&mut cur));
            cur.push_str(word);
        }
    }
    if !cur.is_empty() {
        out.push(cur);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn wrap_respects_width() {
        for line in wrap("the quick brown fox jumps over the lazy dog", 12) {
            assert!(line.chars().count() <= 12);
        }
    }

    #[test]
    fn token_labels_are_short() {
        assert_eq!(tokens_label(999), "999");
        assert_eq!(tokens_label(12_400), "12.4k");
    }
}
