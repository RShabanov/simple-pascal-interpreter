use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Clone)]
pub enum Keyword {
    Begin,
    End,
}

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, Keyword> =
        [("BEGIN", Keyword::Begin), ("END", Keyword::End),]
            .iter()
            .cloned()
            .collect();
}

pub fn parse_keyword(keyword: &str) -> Option<Keyword> {
    Some(KEYWORDS.get(keyword)?.clone())
}
