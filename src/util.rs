#[derive(Debug)]
pub enum Side {
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Debug)]
pub struct Size {
    pub width: i32,
    pub height: i32,
}

#[derive(Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

pub enum Number {
    Absolute(i32),
    Percent(i32),
}

