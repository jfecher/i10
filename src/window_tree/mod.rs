use winapi::shared::windef::HWND;
use self::WindowTree::*;
use self::rect::Rect;
use crate::win32::get_window_name;

pub mod rect;

pub enum WindowTree {
    HSplit(Vec<WindowTree>),
    VSplit(Vec<WindowTree>),
    Window(HWND),
}

impl From<Vec<HWND>> for WindowTree {
    fn from(vec: Vec<HWND>) -> Self {
        let trees = vec.iter().map(|win| Window(*win)).collect();
        HSplit(trees)
    }
}

impl WindowTree {
    pub fn tile_windows(&mut self, screen_rect: Rect<i32>) {
        match self {
            HSplit(vec) => {
                let width = (screen_rect.w - screen_rect.x) / vec.len() as i32;
                let mut i = 0;
                for tree in vec.iter_mut() {
                    let rect = Rect::new(width * i, screen_rect.y, width, screen_rect.h);
                    tree.tile_windows(rect);
                    i += 1;
                }
            },
            VSplit(vec) => {
                let height = (screen_rect.h - screen_rect.y) / vec.len() as i32;
                let mut i = 0;
                for tree in vec.iter_mut() {
                    let rect = Rect::new(screen_rect.x, height * i, screen_rect.w, height);
                    tree.tile_windows(rect);
                    i += 1;
                }
            },
            Window(win) => {
                println!("Tiling {}", get_window_name(*win));
                crate::win32::tile_window(*win, &screen_rect);
            },
        }
    }
}