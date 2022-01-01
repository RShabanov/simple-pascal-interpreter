pub enum Fixity {
    Left,
    Right,
    None,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OpKind {
    Plus,    // +
    Minus,   // -
    Star,    // *
    Slash,   // /
    Caret,   // ^
    Percent, // %

    AssignEq, // :=
}

impl OpKind {
    pub fn fixity(&self) -> Fixity {
        use OpKind::*;

        match self {
            AssignEq => Fixity::Right,
            Plus | Minus | Star | Slash | Caret | Percent => Fixity::Left,
            _ => Fixity::None,
        }
    }
}
