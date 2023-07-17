use std::fmt::Display;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
pub enum PointerType {
    Mouse,
    Touch,
    Pen,
}

impl Display for PointerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Mouse => write!(f, "mouse"),
            Self::Touch => write!(f, "touch"),
            Self::Pen => write!(f, "pen"),
        }
    }
}

impl FromStr for PointerType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "mouse" => Ok(Self::Mouse),
            "touch" => Ok(Self::Touch),
            "pen" => Ok(Self::Pen),
            _ => Err(()),
        }
    }
}
