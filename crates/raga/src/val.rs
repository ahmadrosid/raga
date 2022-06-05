use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Clone, PartialEq)]
pub enum Val {
    Number(i32),
    Unit,
}

impl fmt::Display for Val {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(val) => write!(f, "{}", val),
            Self::Unit => write!(f, "Unit")
        }
    }
}