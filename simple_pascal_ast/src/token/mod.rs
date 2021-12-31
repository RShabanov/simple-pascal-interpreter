pub mod keyword;
pub mod delim;
pub mod op;
pub mod literal;

use keyword::Keyword;
use delim::DelimKind;
use op::OpKind;
use literal::Literal;

#[derive(Clone)]
pub enum Token {
    Ident(String),
    Keyword(Keyword),
    
    // for delimiters like `{}`, `()`, etc.
    OpenDelim(DelimKind),
    CloseDelim(DelimKind),

    // for delimiters like `.`, `;`, `,`, etc.
    Delim(DelimKind),
    
    Op(OpKind),
    Literal(Literal),
    
    Eof
}