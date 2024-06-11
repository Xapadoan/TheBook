use std::fmt::Display;

#[derive(Debug)]
pub enum HorizontalDirection {
    Left,
    Right,
}

impl Display for HorizontalDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HorizontalDirection::Left => write!(f, "left"),
            HorizontalDirection::Right => write!(f, "right"),
        }
    }
}
