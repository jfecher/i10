#[cfg(windows)]
extern crate winapi;

// use std::io::Error;

// SetWindowPos(HWND window, HWND insertAfter, int x, int y, int sizeX, int sizeY, uint flags) -> bool success
use winapi::um::winuser::SetWindowPos;

use winapi::shared::windef::HWND;

// EnumWindows(WNDENUMPROC, LPARAM) -> bool
// type WNDENUMPROC = Option ( HWND -> LPARAM -> bool )
// type LPARAM = LONG_PTR = isize
use winapi::um::winuser::EnumWindows;

use winapi::um::winuser::*;

unsafe extern "system"
fn add_window(window: HWND, args: isize) -> i32 {
    let visible = IsWindowVisible(window) != 0;
    let title = get_window_name(window);

    if visible && !title.is_empty() {
        let windows: &mut Vec<HWND> = std::mem::transmute(args);
        windows.push(window);
    }
    1
}

fn get_window_name(window: HWND) -> String {
    let len = unsafe { GetWindowTextLengthA(window) + 1 };
    let mut buf = vec![0; len as usize];
    unsafe { GetWindowTextA(window, buf.as_mut_ptr(), len) };
    let mut buf: Vec<u8> = buf.iter().map(|&i| i as u8).collect();
    buf.pop();
    let s = String::from_utf8(buf).unwrap_or("".to_owned());
    String::from(s.trim())
}

fn main() {
    let mut windows: Vec<HWND> = Vec::new();
    unsafe {
        EnumWindows(Some(add_window), std::mem::transmute(&windows));

        let null = std::mem::transmute(0isize);
        let mut x = 0;
        for window in windows.iter_mut() {
            SetWindowPos(*window, null, x, 200, 500, 500, 0);
            println!("Tiling '{}'", get_window_name(*window));
            x += 100;
        }
        println!("\nTiled {} windows", windows.len());
    }
}