use winapi::shared::windef::*;
use winapi::um::winuser::*;
use winapi::um::errhandlingapi::*;
use winapi::um::winbase::{
    FormatMessageA,
    FORMAT_MESSAGE_FROM_SYSTEM,
    FORMAT_MESSAGE_IGNORE_INSERTS,
    FORMAT_MESSAGE_FROM_HMODULE
};
use winapi::um::libloaderapi::{LoadLibraryA, FreeLibrary};
use winapi::shared::minwindef::{DWORD, HINSTANCE};

use std::mem::{transmute, zeroed};

use crate::window_tree::rect::Rect;

// win32 module is for re-exports of winapi functions/types with an easier to use
// or i10-specific wrapper around them

pub mod window_info;

pub fn get_error_string(error_code: DWORD) -> Option<String> {
    let capacity = 512;
    let mut buf = vec![0; capacity];

    unsafe {
        // Try to get the message from the system errors.
        let mut count = FormatMessageA(FORMAT_MESSAGE_FROM_SYSTEM | FORMAT_MESSAGE_IGNORE_INSERTS,
                zeroed(), error_code, 0, buf.as_mut_ptr(), capacity as u32, zeroed());

        if 0 == count {
            // The error code did not exist in the system errors.
            // Try Ntdsbmsg.dll for the error code.
            let inst = LoadLibraryA(std::mem::transmute("Ntdsbmsg.dll".as_ptr()));
            if inst == zeroed() {
                return None
            }

            // Try getting message text from ntdsbmsg.
            count = FormatMessageA(FORMAT_MESSAGE_FROM_HMODULE | FORMAT_MESSAGE_IGNORE_INSERTS,
                    transmute(inst), error_code, 0, buf.as_mut_ptr(), capacity as u32, zeroed());

            FreeLibrary(inst);
        }

        let buf: Vec<u8> = buf.iter()
            .take(count as usize)
            .map(|&i| i as u8)
            .collect();

        Some(String::from_utf8(buf).unwrap_or_default())
    }
}

pub fn get_window_name(window: HWND) -> String {
    let len = unsafe { GetWindowTextLengthA(window) + 1 };
    let mut buf = vec![0; len as usize];
    unsafe { GetWindowTextA(window, buf.as_mut_ptr(), len) };
    let mut buf: Vec<u8> = buf.iter().map(|&i| i as u8).collect();
    buf.pop();
    String::from_utf8(buf).unwrap_or_default()
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
        (t.dwStyle & WS_SIZEBOX) == WS_SIZEBOX
    }
}

pub fn is_tileable(window: HWND) -> bool {
    let visible = unsafe { IsWindowVisible(window) != 0 };
    visible && !get_window_name(window).is_empty() && is_foreground_window(window)
}

pub fn tile_window(window: HWND, rect: &Rect<i32>) {
    unsafe {
        ShowWindow(window, 1); // un-maximize+un-minimize the window if needed
        if 0 == SetWindowPos(window, HWND_TOP, rect.x, rect.y, rect.w, rect.h, SWP_SHOWWINDOW) {
            let error_code = GetLastError();
            get_error_string(error_code).map(|s| println!("error: {:?}", s.to_ascii_lowercase()));
        }
    }
}

pub fn get_all_tileable_windows() -> Vec<HWND> {
    let windows: Vec<HWND> = Vec::new();

    // Add the window to the windows Vec above if it is tileable
    unsafe extern "system"
    fn add_window(window: HWND, args: isize) -> i32 {
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