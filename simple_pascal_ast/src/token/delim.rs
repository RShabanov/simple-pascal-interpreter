#[derive(Clone)]
pub enum DelimKind {
    Paren,      // ()
    
    // these kinds don't have close ones
    Dot,
    Semicolon
}