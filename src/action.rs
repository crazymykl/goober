use std::fmt;

#[derive(Debug, Clone)]
pub enum Action {
    Up,
    Down,
    Left,
    Right,
    Swap,
    Spawn
}

impl fmt::Display for Action {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Action::Up    => write!(f, "↑"),
            Action::Down  => write!(f, "↓"),
            Action::Left  => write!(f, "←"),
            Action::Right => write!(f, "→"),
            Action::Swap  => write!(f, "↔"),
            Action::Spawn => write!(f, "+")
        }
    }
}
