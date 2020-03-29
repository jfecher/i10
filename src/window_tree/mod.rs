use winapi::shared::windef::HWND;
use self::WindowTreeKind::*;
use self::rect::Rect;
use crate::win32::get_window_name;

pub mod rect;

pub struct WindowTree {
    memory: Vec<Node>,
}

struct Node {
    parent: Option<usize>,
    this: usize,
    kind: WindowTreeKind,
}

impl Node {
    fn new(parent: Option<usize>, this: usize, kind: WindowTreeKind) -> Node {
        Node {
            parent,
            this,
            kind,
        }
    }
}

enum WindowTreeKind {
    HSplit(Vec<usize>),
    VSplit(Vec<usize>),
    Window(HWND),
}

impl From<Vec<HWND>> for WindowTree {
    fn from(vec: Vec<HWND>) -> Self {
        let children = (0 .. vec.len()).map(|i| i + 1).collect();

        let mut memory = Vec::new();
        memory.push(Node::new(None, 0, HSplit(children)));

        for window in vec.iter() {
            let node = Node::new(Some(0), memory.len(), Window(*window));
            memory.push(node);
        }

        WindowTree { memory }
    }
}

impl WindowTree {
    pub fn tile_windows(&self, screen_rect: Rect<i32>) {
        if !self.memory.is_empty() {
            self.tile_window(screen_rect, 0);
        }else{
            println!("No windows to tile");
        }
    }

    fn tile_window(&self, screen_rect: Rect<i32>, window: usize) {
        let window = &self.memory[window];
        match &window.kind {
            HSplit(vec) => {
                let count = std::cmp::max(vec.len(), 1) as i32;
                let width = (screen_rect.w - screen_rect.x) / count;
                let mut i = 0;
                for window in vec.iter().copied() {
                    let rect = Rect::new(width * i, screen_rect.y, width, screen_rect.h);
                    self.tile_window(rect, window);
                    i += 1;
                }
            },
            VSplit(vec) => {
                let count = std::cmp::max(vec.len(), 1) as i32;
                let height = (screen_rect.h - screen_rect.y) / count;
                let mut i = 0;
                for window in vec.iter().copied() {
                    let rect = Rect::new(screen_rect.x, height * i, screen_rect.w, height);
                    self.tile_window(rect, window);
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