use simple_pascal_ast::parser::Parser;
use simple_pascal_interpreter::Interpreter;

fn main() {
    // example
    let mut parser = Parser::new();
    
    let ast = parser.parse(
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
    ).unwrap();
    
    let mut interpreter = Interpreter::new();
    println!("Vars: {:#?}", interpreter.interpret(ast));
}
