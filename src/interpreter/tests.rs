    use super::*;
    use crate::parser::Parser;
    use crate::scanner::Scanner;

    use crate::utils::{log_items, read_file};

    fn assert_execution_of(title: &str, src: &str, verbose: bool) -> Interpreter {
        let tokens = Scanner::tokens_from_str(src, verbose);

        let mut parser = Parser::new(tokens);
        let (statements, errors) = parser.program();

        if errors.len() != 0 {
            log_items(title, &errors)
        }

        let mut interpreter = Interpreter::new();

        if verbose {
            println!("Interpreter:\n{:?}", interpreter);
        }

        interpreter.interpret(&statements).unwrap();

        interpreter
    }

    fn assert_execution_of_file(title: &str, path: &str, verbose: bool) -> Interpreter {
        let src = read_file(path);
        assert_execution_of(title, src.as_str(), verbose)
    }

    #[test]
    fn executes_expr_statements() {
        assert_execution_of_file(
            "Errors executing Expression statements",
            "examples/expr_stmt.lox",
            false,
        );
    }

    #[test]
    fn executes_print_statements() {
        assert_execution_of_file(
            "Errors executing Print statements",
            "examples/print_stmt.lox",
            false,
        );
    }

    #[test]
    fn executes_variables() {
        assert_execution_of_file(
            "Errors executing Variable declarations",
            "examples/variables.lox",
            false,
        );
    }

    #[test]
    fn executes_assignment_expressions() {
        assert_execution_of_file(
            "Errors executing Variable declarations",
            "examples/assignment.lox",
            false,
        );
    }

    #[test]
    fn executes_block_statements() {
        assert_execution_of_file(
            "Errors executing Block statements",
            "examples/block_stmt.lox",
            false,
        );
    }

    #[test]
    fn executes_if_statements() {
        assert_execution_of_file(
            "Errors executing If statements",
            "examples/if_stmt.lox",
            false,
        );
    }

    #[test]
    fn executes_if_else_statements() {
        assert_execution_of_file(
            "Errors executing If/Else statements",
            "examples/if_else_stmt.lox",
            false,
        );
    }

    #[test]
    fn executes_logical_or() {
        assert_execution_of_file(
            "Errors executing Logical Or",
            "examples/logic_or.lox",
            false,
        );
    }

    #[test]
    fn executes_logical_and() {
        assert_execution_of_file(
            "Errors executing Logical And",
            "examples/logic_and.lox",
            false,
        );
    }

    #[test]
    fn executes_while_statements() {
        assert_execution_of_file(
            "Errors executing While statements",
            "examples/while_stmt.lox",
            false,
        );
    }

    #[test]
    fn executes_for_statements() {
        assert_execution_of_file(
            "Errors executing While statements",
            "examples/for_stmt.lox",
            false,
        );
    }
    #[test]
    fn executes_call_statements() {
        assert_execution_of_file(
            "Errors executing While statements",
            "examples/call_stmt.lox",
            false,
        );
    }
