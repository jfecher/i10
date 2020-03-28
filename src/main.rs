#[cfg(windows)]
extern crate winapi;

// #[macro_use]
// extern crate nom;

mod win32;
mod window_tree;

use window_tree::WindowTree;

fn main() {
    let windows = win32::get_all_tileable_windows();
    let mut tree = WindowTree::from(windows);

    let screen_rect = win32::get_usable_screen_rect();
    tree.tile_windows(screen_rect);
}