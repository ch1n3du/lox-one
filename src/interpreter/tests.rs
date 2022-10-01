use super::*;
use crate::parser::Parser;
use crate::scanner::Scanner;

use crate::utils::{log_items, read_file};

fn assert_execution_of(title: &str, src: &str, verbose: bool) -> Interpreter {
    let tokens = Scanner::tokens_from_str(src, verbose);

    let statements = Parser::parse_str(src).unwrap_or_else(|e| {
        println!("{e}");
        panic!()
    });

    let mut interpreter = Interpreter::new();

    if verbose {
        println!("Interpreter:\n{:?}", interpreter);
    }

    interpreter.interpret(&statements, false, false).unwrap();

    interpreter
}

fn assert_execution_of_file(path: &str, verbose: bool) -> Interpreter {
    let path = path.to_string();
    let src = read_file(path.as_str());
    let title = format!("Errors executing {}", &path);
    assert_execution_of(title.as_str(), src.as_str(), verbose)
}

// #[test]
// fn executes_expr_statements() {
//     assert_execution_of_file("examples/expr_stmt.lox", false);
// }

// #[test]
// fn executes_print_statements() {
//     assert_execution_of_file("examples/print_stmt.lox", false);
// }

// #[test]
// fn executes_variables() {
//     assert_execution_of_file("examples/variables.lox", false);
// }

// #[test]
// fn executes_assignment_expressions() {
//     assert_execution_of_file("examples/assignment.lox", false);
// }

// #[test]
// fn executes_block_statements() {
//     assert_execution_of_file("examples/block_stmt.lox", false);
// }

// #[test]
// fn executes_if_statements() {
//     assert_execution_of_file("examples/if_stmt.lox", false);
// }

// #[test]
// fn executes_if_else_statements() {
//     assert_execution_of_file("examples/if_else_stmt.lox", false);
// }

// #[test]
// fn executes_logical_or() {
//     assert_execution_of_file("examples/logic_or.lox", false);
// }

// #[test]
// fn executes_logical_and() {
//     assert_execution_of_file("examples/logic_and.lox", false);
// }

#[test]
fn executes_while_statements() {
    assert_execution_of_file("examples/while_stmt.lox", false);
}

#[test]
fn executes_for_statements() {
    assert_execution_of_file("examples/for_stmt.lox", false);
}

// #[test]
// fn executes_continue_statements() {
//     assert_execution_of_file("examples/continue.lox", false);
// }

// #[test]
// fn executes_call_statements() {
//     assert_execution_of_file("examples/call_stmt.lox", false);
// }

// #[test]
// fn executes_fun_declaration() {
//     assert_execution_of_file("examples/fun_decl.lox", false);
// }
