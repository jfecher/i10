#[cfg(windows)]
extern crate winapi;

use std::io::Error;

// SetWindowPos(HWND window, HWND insertAfter, int x, int y, int sizeX, int sizeY, uint flags) -> bool success
use winapi::um::winuser::SetWindowPos;

// EnumWindows(WNDENUMPROC, LPARAM) -> bool
// type WNDENUMPROC = Option ( HWND -> LPARAM -> bool )
// type LPARAM = LONG_PTR = isize
use winapi::um::winuser::EnumWindows;

#[cfg(windows)]
fn print_message(msg: &str) -> Result<i32, Error> {
    use std::ffi::OsStr;
    use std::iter::once;
    use std::os::windows::ffi::OsStrExt;
    use std::ptr::null_mut;
    use winapi::um::winuser::{MB_OK, MessageBoxW};
    let wide: Vec<u16> = OsStr::new(msg).encode_wide().chain(once(0)).collect();
    let ret = unsafe {
        MessageBoxW(null_mut(), wide.as_ptr(), wide.as_ptr(), MB_OK)
    };
    if ret == 0 { Err(Error::last_os_error()) }
    else { Ok(ret) }
}
#[cfg(not(windows))]
fn print_message(msg: &str) -> Result<(), Error> {
    println!("{}", msg);
    Ok(())
}
fn main() {
    print_message("Hello, world!").unwrap();
}
