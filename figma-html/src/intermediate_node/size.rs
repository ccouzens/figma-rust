use std::{fmt, ops::Add};

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub enum Size {
    Pixels(f64),
    Other(String),
}

impl fmt::Display for Size {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Size::Pixels(p) => write!(f, "{p}px"),
            Size::Other(o) => write!(f, "{o}"),
        }
    }
}

impl Add for &Size {
    type Output = Size;

    fn add(self, other: Self) -> Size {
        match (self, other) {
            (Size::Pixels(a), Size::Pixels(b)) => Size::Pixels(a + b),
            _ => Size::Other(format!("calc({self} + {other})")),
        }
    }
}
