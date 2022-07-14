use lox_one::{interpreter::Interpreter, parser::Parser, scanner::Scanner};
fn main() {
    parse_str("print \"Hello world\";");
}

fn parse_str(source: &str) {
    let tokens = Scanner::new(source.as_bytes().to_vec())
        .scan_tokens()
        .to_owned();
    let parsed = Parser::new(tokens).parse();

    println!("\nInput: '{}'", source);

    match parsed {
        Err(e) => println!("\nError: {}", e),
        Ok(statements) => {
            Interpreter::new().interpret(statements);
        }
    }
}
