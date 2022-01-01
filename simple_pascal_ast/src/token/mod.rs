pub mod delim;
pub mod keyword;
pub mod literal;
pub mod op;

use delim::DelimKind;
use keyword::Keyword;
use literal::Literal;
use op::OpKind;

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

    Eof,
}
