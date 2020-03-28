use winapi::shared::windef::*;
use winapi::um::winuser::*;

use std::mem::zeroed;

use crate::window_tree::rect::Rect;

// win32 module is for re-exports of winapi functions/types with an easier to use
// or i10-specific wrapper around them

pub mod window_info;

mod setwindowpos_flags {
    pub const SHOW_WINDOW: u32 = 0x0040;
}

pub fn get_window_name(window: HWND) -> String {
    let len = unsafe { GetWindowTextLengthA(window) + 1 };
    let mut buf = vec![0; len as usize];
    unsafe { GetWindowTextA(window, buf.as_mut_ptr(), len) };
    let mut buf: Vec<u8> = buf.iter().map(|&i| i as u8).collect();
    buf.pop();
    String::from_utf8(buf).unwrap_or("".to_owned())
}

pub fn get_usable_screen_rect() -> Rect<i32> {
    unsafe {
        // Get a handle to the desktop window
        let desktop = GetDesktopWindow();
        let mut rect = zeroed();
        // Get the size of screen to the variable desktop
        GetWindowRect(desktop, &mut rect);
        // The top left corner will have coordinates (0,0)
        // and the bottom right corner will have coordinates
        // (horizontal, vertical)
        Rect::at_origin(rect.right, rect.bottom)
    }
}

pub fn is_foreground_window(window: HWND) -> bool {
    unsafe {
        let mut t = std::mem::zeroed::<WINDOWINFO>();
        t.cbSize = std::mem::size_of::<WINDOWINFO>() as u32;
        GetWindowInfo(window, &mut t);
        // TODO: Find a better way to weed out hidden/background windows marked as visible
        (t.dwStyle & WS_BORDER) != 0
    }
}

pub fn is_tileable(window: HWND) -> bool {
    let visible = unsafe { IsWindowVisible(window) != 0 };
    visible && !get_window_name(window).is_empty() && is_foreground_window(window)
}

pub fn tile_window(window: HWND, rect: &Rect<i32>) {
    unsafe {
        ShowWindow(window, 1); // un-maximize+un-minimize the window if needed
        SetWindowPos(window, HWND_TOP, rect.x, rect.y, rect.w, rect.h, SWP_SHOWWINDOW);
    }
}

pub fn get_all_tileable_windows() -> Vec<HWND> {
    let windows: Vec<HWND> = Vec::new();

    // Add the window to the windows Vec above if it is tileable
    unsafe extern "system"
    fn add_window(window: HWND, args: isize) -> i32 {
        window_info::print_window_style(window);
        if is_tileable(window) {
            let windows: &mut Vec<HWND> = std::mem::transmute(args);
            windows.push(window);
        }
        1
    }

    unsafe {
        EnumWindows(Some(add_window), std::mem::transmute(&windows));
    }
    windows
}