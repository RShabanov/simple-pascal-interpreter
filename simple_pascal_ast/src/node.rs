use std::collections::LinkedList;

use crate::token::{literal::Literal, op::OpKind, keyword::Keyword, delim::DelimKind};

#[derive(Debug, PartialEq, Eq)]
pub enum Node {
    UnaryOp(UnaryOp),
    BinOp(BinOp),
    Literal(Literal),
    Ident(String),
    Keyword(Keyword),
    Delim(DelimKind),
    Compound(CompoundNode),
    None
}

pub type NodeList = LinkedList<Node>;

#[derive(Debug, PartialEq, Eq, Default)]
pub struct CompoundNode {
    pub children: NodeList
}

#[derive(Debug, PartialEq, Eq)]
pub struct UnaryOp {
    pub op: OpKind,
    pub node: Box<Node>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct BinOp {
    pub lhs: Box<Node>,
    pub op: OpKind,
    pub rhs: Box<Node>,
}

impl BinOp {
    pub fn new(lhs: Node, op: OpKind, rhs: Node) -> Node {
        Node::BinOp(Self {
            lhs: Box::new(lhs),
            op,
            rhs: Box::new(rhs),
        })
    }
}

impl UnaryOp {
    pub fn new(op: OpKind, node: Node) -> Node {
        Node::UnaryOp(Self {
            op,
            node: Box::new(node),
        })
    }
}

impl CompoundNode {
    pub fn new() -> Node {
        Node::Compound(Default::default())
    }

    pub fn from_list(children: NodeList) -> Node {
        Node::Compound(Self { children })
    }
}

impl Node {
    pub fn is_delim(&self) -> bool {
        match self {
            &Self::Delim(_) => true,
            _ => false
        }
    }

    pub fn is_keyword(&self) -> bool {
        match self {
            &Self::Keyword(_) => true,
            _ => false
        }
    }
}