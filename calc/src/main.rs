/// This is the main command-line application for arithmetic expression evaluator
use std::io;

mod parsemath;
use parsemath::ast;
use parsemath::parser::{ParseError, Parser};

fn evaluate(expr: String) -> Result<f64, ParseError> {
    let expr = expr.split_whitespace().collect::<String>();
    let mut math_parser = Parser::new(&expr)?;
    let ast = math_parser.parse()?;
    println!("The generated AST is {:?}", ast);

    Ok(ast::eval(ast)?)
}

fn main() {
    println!("Arithmetic expression evaluator.");
    println!("- Allowed numbers: positive, negative and decimals.");
    println!("- Supported operations: Add, Subtract, Multiply, Divide, PowerOf(^).\n");

    loop {
        println!("Enter your arithmetic expression below:");

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match evaluate(input) {
                    Ok(val) => println!("The computed number is {}\n", val),
                    Err(_) => println!("Invalid expression\n"),
                };
            }
            Err(e) => println!("error: {}", e),
        }
    }
}
