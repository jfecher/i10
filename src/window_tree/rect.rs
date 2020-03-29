use std::convert::Into;

#[derive(PartialEq, Clone, Debug)]
pub struct Rect<T> {
    pub x: T,
    pub y: T,
    pub w: T,
    pub h: T,
}

impl<T> Rect<T> {
    pub fn new(x: T, y: T, w: T, h: T) -> Rect<T> {
        Rect { x, y, w, h }
    }

    pub fn at_origin(w: T, h: T) -> Rect<T> where T: From<i32> {
        Rect { x: 0.into(), y: 0.into(), w, h }
    }
}