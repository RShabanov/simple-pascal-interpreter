use std::{iter::Peekable, str::Chars};

use crate::token::{
    delim::parse_delim, keyword::parse_keyword, literal::Literal, op::OpKind, Token,
};

fn is_word(ch: char) -> bool {
    ch.is_ascii_alphanumeric() || ch == '_'
}

#[derive(Debug)]
pub enum LexerErr {
    UndefinedChar,
}

#[derive(Debug)]
pub(crate) struct Lexer<'a> {
    current_char: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn next(&mut self) -> Result<Token, LexerErr> {
        while let Some(&next_char) = self.current_char.peek() {
            if next_char.is_whitespace() {
                self.skip_whitespace();
                continue;
            }

            if let Some(op) = self.op_kind(next_char) {
                self.current_char.next();

                return Ok(Token::Op(op));
            } else if let Some(delim) = parse_delim(next_char) {
                self.current_char.next();

                if delim.is_bin_kind() {
                    if next_char == '(' {
                        return Ok(Token::OpenDelim(delim));
                    }
                    return Ok(Token::CloseDelim(delim));
                }
                return Ok(Token::Delim(delim));
            } else if next_char.is_ascii_digit() {
                return Ok(Token::Literal(self.number()));
            } else if is_word(next_char) {
                // is literal or keyword, or ident
                let word = self.word();
                if let Some(keyword) = parse_keyword(&word) {
                    return Ok(Token::Keyword(keyword));
                } else {
                    // since we have only Literal::Integer and Literal::Float
                    // we can just return Token::Ident with no regard
                    return Ok(Token::Ident(word));
                }
            } else {
                return Err(LexerErr::UndefinedChar);
            }
        }
        Ok(Token::Eof)
    }

    pub fn set(&mut self, text: &'a str) {
        self.current_char = text.chars().peekable();
    }
}

impl Lexer<'_> {
    fn number(&mut self) -> Literal {
        let mut int_part = self.integer();

        match self.current_char.next_if_eq(&'.') {
            Some(dot) => {
                int_part.push(dot);
                Literal::Float(int_part + &self.integer())
            }
            None => Literal::Integer(int_part),
        }
    }

    fn integer(&mut self) -> String {
        let mut int = String::new();

        while let Some(ch) = self.current_char.next_if(|ch| ch.is_ascii_digit()) {
            int.push(ch);
        }
        int
    }

    fn skip_whitespace(&mut self) {
        while let Some(_) = self.current_char.next_if(|ch| ch.is_whitespace()) {}
    }

    fn op_kind(&mut self, ch: char) -> Option<OpKind> {
        use OpKind::*;

        match ch {
            '+' => Some(Plus),
            '-' => Some(Minus),
            '*' => Some(Star),
            '/' => Some(Slash),
            '%' => Some(Percent),
            '^' => Some(Caret),
            ':' => {
                self.current_char.next();
                match self.current_char.peek() {
                    Some(ch) => {
                        return if ch == &'=' {
                            Some(AssignEq)
                        } else { None }
                    },
                    None => return None,
                }
            }
            _ => None,
        }
    }

    fn word(&mut self) -> String {
        let mut word = String::new();

        while let Some(ch) = self.current_char.next_if(|ch| is_word(*ch)) {
            word.push(ch);
        }
        word
    }
}

impl<'a> From<&'a str> for Lexer<'a> {
    fn from(text: &'a str) -> Self {
        Self {
            current_char: text.chars().peekable(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::token::{delim::DelimKind, keyword::Keyword};

    use super::*;

    fn exprs_and_tokens() -> (Vec<&'static str>, Vec<Vec<Token>>) {
        (
            vec![
                "4 +3",
                "BEGIN\nEND.",
                "223 +      5.3",
                "2 + (2 - 4)*2.3",
                "2; 4 + 3;",
                "a :=3;",
                "",
            ],
            vec![
                vec![
                    Token::Literal(Literal::Integer(String::from("4"))),
                    Token::Op(OpKind::Plus),
                    Token::Literal(Literal::Integer(String::from("3"))),
                ],
                vec![
                    Token::Keyword(Keyword::Begin),
                    Token::Keyword(Keyword::End),
                    Token::Delim(DelimKind::Dot),
                ],
                vec![
                    Token::Literal(Literal::Integer(String::from("223"))),
                    Token::Op(OpKind::Plus),
                    Token::Literal(Literal::Float(String::from("5.3"))),
                ],
                vec![
                    Token::Literal(Literal::Integer(String::from("2"))),
                    Token::Op(OpKind::Plus),
                    Token::OpenDelim(DelimKind::Paren),
                    Token::Literal(Literal::Integer(String::from("2"))),
                    Token::Op(OpKind::Minus),
                    Token::Literal(Literal::Integer(String::from("4"))),
                    Token::CloseDelim(DelimKind::Paren),
                    Token::Op(OpKind::Star),
                    Token::Literal(Literal::Float(String::from("2.3"))),
                ],
                vec![
                    Token::Literal(Literal::Integer(String::from("2"))),
                    Token::Delim(DelimKind::Semicolon),
                    Token::Literal(Literal::Integer(String::from("4"))),
                    Token::Op(OpKind::Plus),
                    Token::Literal(Literal::Integer(String::from("3"))),
                    Token::Delim(DelimKind::Semicolon),
                ],
                vec![
                    Token::Ident(String::from("a")),
                    Token::Op(OpKind::AssignEq),
                    Token::Literal(Literal::Integer(String::from("3"))),
                    Token::Delim(DelimKind::Semicolon),
                ],
                vec![Token::Eof],
            ],
        )
    }

    #[test]
    fn from() {
        let lexer = Lexer::from("4 + 3");
        let lexer_str =
            "Lexer { current_char: Peekable { iter: Chars(['4', ' ', '+', ' ', '3']), peeked: None } }";

        assert_eq!(lexer_str, format!("{:?}", lexer));
    }

    #[test]
    fn set() {
        let mut lexer = Lexer::from("");
        let mut lexer_str = "Lexer { current_char: Peekable { iter: Chars([]), peeked: None } }";

        assert_eq!(lexer_str, format!("{:?}", lexer));

        lexer.set("4 + 3");
        lexer_str =
            "Lexer { current_char: Peekable { iter: Chars(['4', ' ', '+', ' ', '3']), peeked: None } }";

        assert_eq!(lexer_str, format!("{:?}", lexer));
    }

    #[test]
    fn tokenize_exprs() {
        let mut lexer = Lexer::from("");

        let (exprs, vec_tokens) = exprs_and_tokens();

        for i in 0..exprs.len() {
            lexer.set(exprs[i]);

            println!("{}", exprs[i]);

            for expected_token in &vec_tokens[i] {
                let token = lexer.next().unwrap();
                if token == Token::Eof {
                    break;
                }

                assert_eq!(*expected_token, token);
            }
        }
    }

    #[test]
    #[should_panic]
    fn fail_tokenization() {
        let mut lexer = Lexer::from("2 & 3");

        loop {
            lexer.next().unwrap();
        }
    }
}
