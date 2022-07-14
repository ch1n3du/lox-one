use lox_one::{interpreter::Interpreter, parser::Parser, scanner::Scanner};
fn main() {
    parse_str("2 * 3 * true == 24 / 4");
}

fn parse_str(source: &str) {
    let tokens = Scanner::new(source.as_bytes().to_vec())
        .scan_tokens()
        .to_owned();
    let parsed = Parser::new(tokens).expression();

    println!("\nInput: '{}'", source);

    match parsed {
        Err(e) => println!("\nError: {}", e),
        Ok(ast) => {
            let interpreter = Interpreter::new();
            println!("\nAst: {}", ast);

            match interpreter.evaluate(&ast) {
                Ok(res) => println!("\nResult: {}", res),
                Err(err) => println!("\nError: {}", err)
            }
        }
    }
}
