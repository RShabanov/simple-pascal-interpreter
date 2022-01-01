#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DelimKind {
    Paren, // ()

    // these kinds don't have close ones
    Dot,
    Semicolon,
}

pub fn parse_delim(delim: char) -> Option<DelimKind> {
    use DelimKind::*;

    match delim {
        '(' | ')' => Some(Paren),
        '.' => Some(Dot),
        ';' => Some(Semicolon),
        _ => None,
    }
}

impl DelimKind {
    pub fn is_bin_kind(&self) -> bool {
        use DelimKind::*;

        match self {
            &Paren => true,
            _ => false,
        }
    }
}
