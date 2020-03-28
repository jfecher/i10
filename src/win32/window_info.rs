use winapi::shared::windef::HWND;
use winapi::um::winuser::{WINDOWINFO, GetWindowInfo};
use crate::win32::get_window_name;

fn empty() -> WINDOWINFO {
    let mut t = unsafe { std::mem::zeroed::<WINDOWINFO>() };
    t.cbSize = std::mem::size_of::<WINDOWINFO>() as u32;
    t
}

pub fn get_window_info(window: HWND) -> WINDOWINFO {
    let mut info = empty();
    unsafe { GetWindowInfo(window, &mut info) };
    info
}

pub fn print_window_style(window: HWND) {
    let info = get_window_info(window);
    println!("'{}': Style: {:x},\tExStyle: {:x}", get_window_name(window), info.dwStyle, info.dwExStyle);
}