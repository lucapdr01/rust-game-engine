pub enum Key {
    Left,
    Right,
    Up,
    Down,
    Space
}

pub enum Event {
    KeyDown(Key),
    Draw,
}