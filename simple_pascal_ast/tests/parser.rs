use simple_pascal_ast::{
    node::*,
    token::{op::OpKind, literal::Literal},
    parser::*
};

fn exprs_and_trees() -> (Vec<&'static str>, Vec<Node>) {
    (vec![
        r"BEGIN
        END.",
        r"BEGIN
        ;-2;
        END.",
        r"BEGIN
            x:= 2 + 3 * (2 + 3);
            y:= 2 / 2 - 2 + 3 * ((1 + 1) + (1 + 1));
        END.",
        r"BEGIN
            y := 2;
            BEGIN
                a := 3;
                a := a;
                b := 10 + a + 10 * y / 4;
                c := a - b
            END;
            x := 11;
        END."
    ],
    vec![
        CompoundNode::from_list(
            NodeList::from([Node::None])
        ),
        CompoundNode::from_list(
            NodeList::from([
                UnaryOp::new(
                    OpKind::Minus,
                    Node::Literal(Literal::Integer(String::from("2")))
                ),
                Node::None
            ])
        ),
        CompoundNode::from_list(
            NodeList::from([
                BinOp::new(
                    Node::Ident(String::from("x")),
                    OpKind::AssignEq,
                    BinOp::new(
                        Node::Literal(Literal::Integer(String::from("2"))),
                        OpKind::Plus,
                        BinOp::new(
                            Node::Literal(Literal::Integer(String::from("3"))),
                            OpKind::Star,
                            BinOp::new(
                                Node::Literal(Literal::Integer(String::from("2"))),
                                OpKind::Plus,
                                Node::Literal(Literal::Integer(String::from("3"))),
                            )
                        )
                    )
                ),
                BinOp::new(
                    Node::Ident(String::from("y")),
                    OpKind::AssignEq,
                    BinOp::new(
                        BinOp::new(
                            BinOp::new(
                                Node::Literal(Literal::Integer(String::from("2"))),
                                OpKind::Slash,
                                Node::Literal(Literal::Integer(String::from("2"))),
                            ),
                            OpKind::Minus,
                            Node::Literal(Literal::Integer(String::from("2"))),
                        ),
                        OpKind::Plus,
                        BinOp::new(
                            Node::Literal(Literal::Integer(String::from("3"))),
                            OpKind::Star,
                            BinOp::new(
                                BinOp::new(
                                    Node::Literal(Literal::Integer(String::from("1"))),
                                    OpKind::Plus,
                                    Node::Literal(Literal::Integer(String::from("1"))),
                                ),
                                OpKind::Plus,
                                BinOp::new(
                                    Node::Literal(Literal::Integer(String::from("1"))),
                                    OpKind::Plus,
                                    Node::Literal(Literal::Integer(String::from("1"))),
                                )
                            )
                        )
                    )
                ),
                Node::None,
            ])
        ),
        CompoundNode::from_list(
            NodeList::from([
                BinOp::new(
                    Node::Ident(String::from("y")),
                    OpKind::AssignEq,
                    Node::Literal(Literal::Integer(String::from("2")))
                ),
                CompoundNode::from_list(
                    NodeList::from([
                        BinOp::new(
                            Node::Ident(String::from("a")),
                            OpKind::AssignEq,
                            Node::Literal(Literal::Integer(String::from("3")))
                        ),
                        BinOp::new(
                            Node::Ident(String::from("a")),
                            OpKind::AssignEq,
                            Node::Ident(String::from("a")),
                        ),
                        BinOp::new(
                            Node::Ident(String::from("b")),
                            OpKind::AssignEq,
                            BinOp::new(
                                BinOp::new(
                                    Node::Literal(Literal::Integer(String::from("10"))),
                                    OpKind::Plus,
                                    Node::Ident(String::from("a")),
                                ),
                                OpKind::Plus,
                                BinOp::new(
                                    BinOp::new(
                                        Node::Literal(Literal::Integer(String::from("10"))),
                                        OpKind::Star,
                                        Node::Ident(String::from("y")),
                                    ),
                                    OpKind::Slash,
                                    Node::Literal(Literal::Integer(String::from("4"))),
                                )
                            )
                        ),
                        BinOp::new(
                            Node::Ident(String::from("c")),
                            OpKind::AssignEq,
                            BinOp::new(
                                Node::Ident(String::from("a")),
                                OpKind::Minus,
                                Node::Ident(String::from("b")),
                            )
                        )
                    ])
                ),
                BinOp::new(
                    Node::Ident(String::from("x")),
                    OpKind::AssignEq,
                    Node::Literal(Literal::Integer(String::from("11")))
                ),
                Node::None
            ])
        )
    ])
}

#[test]
fn parse() {
    let mut parser = Parser::new();

    let (exprs, trees) = exprs_and_trees();

    for (i, expr) in exprs.iter().enumerate() {
        assert_eq!(trees[i], parser.parse(expr).unwrap());
    }
}

#[test]
#[should_panic]
fn invalid_unary_op() {
    let mut parser = Parser::new();
    parser.parse(r"
    BEGIN
    ;+;
    END.
    ").unwrap();
}

#[test]
#[should_panic]
fn miss_delimiter() {
    let mut parser = Parser::new();
    parser.parse(r"
    BEGIN
    END
    ").unwrap();
}

#[test]
#[should_panic]
fn miss_delimiter_end() {
    let mut parser = Parser::new();
    parser.parse(r"
    BEGIN
        BEGIN
        END
        x := 1;
    END.").unwrap();
}

#[test]
#[should_panic]
fn invalid_assignment() {
    let mut parser = Parser::new();
    parser.parse(r"
    BEGIN
        2 := 2;
    END.").unwrap();
}