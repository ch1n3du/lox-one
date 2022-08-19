// use super::Parser;
// use crate::{parser_errors::ParserError, scanner::Scanner, token_type::TokenType};
use super::*;

use crate::parser::Parser;
use crate::scanner::Scanner;

use crate::utils::{log_items, read_file};

fn assert_can_parse(title: &str, src: &str, verbose: bool) -> (Vec<Stmt>, Vec<ParserError>) {
    let tokens = Scanner::tokens_from_str(src, verbose);

    let mut parser = Parser::new(tokens);
    let (statements, errors) = parser.program();

    if errors.len() != 0 {
        log_items(format!("Errors parsing {}", title).as_str(), &errors)
    }

    // if verbose {
    //     log_items(
    //         format!("Statements from  parsing {}", title).as_str(),
    //         &statements,
    //     )
    // }

    (statements, errors)
}

fn assert_can_parse_file(path: &str, verbose: bool) -> (Vec<Stmt>, Vec<ParserError>) {
    let src = read_file(path);

    assert_can_parse(path, src.as_str(), verbose)
}

#[test]
fn can_parse_expr_statements() {
    assert_can_parse_file("examples/expr_stmt.lox", false);
}

#[test]
fn can_parse_print_statements() {
    assert_can_parse_file("examples/print_stmt.lox", false);
}

#[test]
fn can_parse_variable_declarations() {
    assert_can_parse_file("examples/variables.lox", false);
}

#[test]
fn can_parse_assignment_expressions() {
    assert_can_parse_file("examples/assignment.lox", false);
}

#[test]
fn can_parse_block_statements() {
    assert_can_parse_file("examples/variables.lox", false);
}

#[test]
fn can_parse_if_statements() {
    assert_can_parse_file("examples/if_stmt.lox", false);
}

#[test]
fn can_parse_if_else_statements() {
    assert_can_parse_file("examples/if_else_stmt.lox", false);
}

#[test]
fn can_parse_logical_and() {
    assert_can_parse_file("examples/logic_and.lox", false);
}

#[test]
fn can_parse_logical_or() {
    assert_can_parse_file("examples/logic_or.lox", false);
}

#[test]
fn can_parse_while_statements() {
    assert_can_parse_file("examples/while_stmt.lox", false);
}

#[test]
fn can_parse_for_statements() {
    assert_can_parse_file("examples/for_stmt.lox", false);
}

#[test]
fn can_parse_continue_statements() {
    assert_can_parse_file("examples/continue.lox", false);
}

#[test]
fn can_parse_call_expressions() {
    assert_can_parse_file("examples/call_stmt.lox", false);
}

#[test]
fn can_parse_fun_declaration() {
    assert_can_parse_file("examples/fun_decl.lox", false);
}
