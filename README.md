## Simple Pascal interpreter

Pascal grammar: https://bki.matecdn.ru/-/fd77ef3e-4c67-4e2b-903a-a14589b8abfa/pascal.xhtml
<hr>

#### Input:
```pascal
BEGIN
    y: = 2;
    BEGIN
        a := 3;
        a := a;
        b := 10 + a + 10 * y / 4;
        c := a - b
    END;
    x := 11;
END.
```

#### Code example:
```rust
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
```
Output:
```
Vars: Ok(
    [
        {
            "a": 3.0,
            "b": 18.0,
            "c": -15.0,
        },
        {
            "y": 2.0,
            "x": 11.0,
        },
    ],
)
```