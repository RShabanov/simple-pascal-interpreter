use std::collections::{LinkedList, HashMap};

use simple_pascal_ast::parser::Parser;
use simple_pascal_interpreter::Interpreter;

fn exprs_and_vars() -> (Vec<&'static str>, Vec<LinkedList<HashMap<String, f64>>>) {
    (
        vec![
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
            LinkedList::from([
                HashMap::default()
            ]),
            LinkedList::from([
                HashMap::default()
            ]),
            LinkedList::from([
                HashMap::from([
                    (String::from("x"), 17.0),
                    (String::from("y"), 11.0)
                ])
            ]),
            LinkedList::from([
                HashMap::from([
                    (String::from("a"), 3.0),
                    (String::from("b"), 18.0),
                    (String::from("c"), -15.0)
                ]),
                HashMap::from([
                    (String::from("x"), 11.0),
                    (String::from("y"), 2.0)
                ])
            ])
        ]
    )
}

#[test]
fn interpret() {
    let mut parser = Parser::new();
    let mut interpreter = Interpreter::new();

    let (exprs, vars) = exprs_and_vars();

    for (i, expr) in exprs.iter().enumerate() {
        assert_eq!(
            interpreter.interpret(
                parser.parse(expr).unwrap()
            ).unwrap(),
            vars[i]
        )
    }
}