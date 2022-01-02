use crate::{
    token::{
        Token,
        op::OpKind, 
        delim::DelimKind,
        keyword::Keyword,
    }, 
    lexer::Lexer, 
    node::*
};

#[derive(Debug, PartialEq, Eq)]
pub enum ParserErr {
    InvalidToken(String),
    TokenMismatch(String),
    MissingToken(String),
    InvalidExpr,
    Undefined,
} 

#[derive(Debug)]
pub struct Parser<'a> {
    current_token: Token,
    lexer: Lexer<'a>
}

impl<'a> Parser<'a> {
    pub fn parse(&mut self, text: &'a str) -> Result<Node, ParserErr> {
        self.lexer.set(text);

        let ast = self.program()?;

        if self.current_token != Token::Eof {
            return Err(ParserErr::InvalidExpr);
        }

        Ok(ast)
    }
}

impl Parser<'_> {
    pub fn new() -> Self {
        Default::default()
    }

    fn program(&mut self) -> Result<Node, ParserErr> {
        let node_list = self.complex_statement()?;

        self.next_token()?;
        if let Token::Delim(delim) = self.current_token.clone() {
            if delim != DelimKind::Dot {
                return Err(ParserErr::MissingToken(
                    format!("Program must end up with a dot, got {:?}", delim)
                ));
            } else {
                self.next_token()?;
            }
        }

        Ok(node_list)
    }

    fn complex_statement(&mut self) -> Result<Node, ParserErr> {
        self.next_token()?;
        
        if let Token::Keyword(keyword) = self.current_token.clone() {
            if keyword != Keyword::Begin {
                return Err(ParserErr::MissingToken(
                    format!("Expected keyword `BEGIN`, got {:?}", keyword)
                ));
            }
        }

        let node_list = self.statement_list()?;

        if let Token::Keyword(keyword) = self.current_token.clone() {
            if keyword != Keyword::End {
                return Err(ParserErr::MissingToken(
                    format!("Expected keyword `END`, got {:?}", keyword)
                ));
            }
        }

        Ok(node_list)
    }

    fn statement_list(&mut self) -> Result<Node, ParserErr> {
        let mut nodes = NodeList::from([self.statement()?]);

        while let Token::Delim(delim) = self.current_token.clone() {
            if delim != DelimKind::Semicolon {
                break;
            }
            nodes.push_back(self.statement()?);
        }

        Ok(Node::Compound(
            CompoundNode {
                children: nodes
            }
        ))
    }

    fn statement(&mut self) -> Result<Node, ParserErr> {
        self.next_token()?;

        match self.current_token.clone() {
            Token::Keyword(keyword) => {
                if keyword == Keyword::Begin {
                    self.complex_statement()
                } else {
                    // since there are only 2 keywords: BEGIN and END
                    Ok(Node::None)
                }
            },
            Token::Ident(ident) => {
                self.next_token()?;

                let op = if let Token::Op(op) = self.current_token.clone() {
                    self.next_token()?;
                    op
                } else {
                    return Err(ParserErr::InvalidExpr);
                };

                Ok(BinOp::new(
                    Node::Ident(ident),
                    op,
                    self.expr()?
                ))
            },
            _ => {
                self.expr()
            }
        }
    }

    fn next_token(&mut self) -> Result<(), ParserErr> {
        match self.lexer.next() {
            Ok(token) => Ok(self.current_token = token),
            Err(_) => Err(ParserErr::Undefined),
        }
    }

    fn expr(&mut self) -> Result<Node, ParserErr> {
        let mut res = self.term()?;

        if !(res.is_delim() || res.is_keyword()) {
            while self.is_expr_token() {
                if let Token::Op(op) = self.current_token.clone() {
                    self.next_token()?;

                    let node = self.term()?;

                    match node {
                        Node::Keyword(keyword) => return Err(ParserErr::TokenMismatch(
                            format!("Binary operator doesn't support keywords, got {:?}", keyword)
                        )),
                        Node::Delim(_) | Node::None => return Err(ParserErr::InvalidExpr),
                        _ => res = BinOp::new(res, op, node)
                    }
                }
            }
        }
        Ok(res)
    }

    fn term(&mut self) -> Result<Node, ParserErr> {
        let mut res = self.factor()?;

        if !(res.is_delim() || res.is_keyword()) {
            while self.is_term_token() {
                if let Token::Op(op) = self.current_token.clone() {
                    self.next_token()?;

                    let node = self.factor()?;

                    match node {
                        Node::Keyword(keyword) => return Err(ParserErr::TokenMismatch(
                            format!("Binary operator doesn't support keywords, got {:?}", keyword)
                        )),
                        Node::Delim(_) | Node::None => return Err(ParserErr::InvalidExpr),
                        _ => res = BinOp::new(res, op, node)
                    }
                }
            }
        }
        Ok(res)
    }

    fn factor(&mut self) -> Result<Node, ParserErr> {
        let token = self.current_token.clone();
        self.next_token()?;

        match token {
            Token::Literal(lit) => Ok(Node::Literal(lit)),
            Token::Ident(ident) => Ok(Node::Ident(ident)),
            Token::OpenDelim(open_delim) => self.bin_delim_factor(open_delim),
            Token::Op(op) => self.unary_op_factor(op),
            Token::Delim(delim) => {
                match delim {
                    DelimKind::Dot | DelimKind::Semicolon => Ok(Node::Delim(delim)),
                    _ => Err(ParserErr::MissingToken(
                        format!("Missing delimiter for {:?}", delim)
                    ))
                }
            },
            Token::Eof => Ok(Node::None),
            Token::Keyword(keyword) => Ok(Node::Keyword(keyword)),
            _ => Err(ParserErr::Undefined)
        }
    }

    fn bin_delim_factor(&mut self, open_delim: DelimKind) -> Result<Node, ParserErr> {
        let res = self.expr()?;

        match self.current_token.clone() {
            Token::CloseDelim(close_delim) => {
                self.next_token()?;
                if open_delim == close_delim {
                    Ok(res)
                } else {
                    Err(ParserErr::TokenMismatch(
                        format!("Expected close delimiter for {:?}, got {:?}", open_delim, close_delim)
                    ))
                }
            },
            _ => Err(ParserErr::TokenMismatch(
                format!("Expected close delimiter for {:?}", open_delim)
            ))
        }
    }

    fn unary_op_factor(&mut self, op: OpKind) -> Result<Node, ParserErr> {
        match op {
            OpKind::Plus | OpKind::Minus => {
                Ok(UnaryOp::new(op, self.factor()?))
            },
            _ => Err(ParserErr::TokenMismatch(
                format!("Unary operator supports only `+` and `-`, got {:?}", op)
            ))
        }
    }

    fn is_expr_token(&self) -> bool {
        match &self.current_token {
            Token::Op(ref op) => {
                match op {
                    &OpKind::Plus | &OpKind::Minus => true,
                    _ => false
                }
            },
            _ => false
        }
    }

    fn is_term_token(&self) -> bool {
        match &self.current_token {
            Token::Op(ref op) => {
                match op {
                    &OpKind::Slash | &OpKind::Star | &OpKind::Percent | &OpKind::Caret => true,
                    _ => false
                }
            },
            _ => false
        }
    }
}

impl Default for Parser<'_> {
    fn default() -> Self {
        Self {
            current_token: Token::Eof,
            lexer: Lexer::from("\0"),
        }
    }
}
