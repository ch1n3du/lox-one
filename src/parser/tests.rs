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

    // for stmt in &statements {
    //     println!("{stmt}")
    // }

    (statements, errors)
}

fn assert_can_parse_file(file_name: &str, verbose: bool) -> (Vec<Stmt>, Vec<ParserError>) {
    let path = format!("examples/{}.lox", file_name);
    let src = read_file(path.as_str());

    assert_can_parse(path.as_str(), src.as_str(), verbose)
}

#[test]
fn parses_expr_stmts() {
    assert_can_parse_file("expr_stmt", false);
}

#[test]
fn can_parse_print_stmt() {
    assert_can_parse_file("print_stmt", false);
}

#[test]
fn can_parse_variables() {
    assert_can_parse_file("variables", false);
}

//     ("assignment", false),
#[test]
fn can_parse_assignment() {
    assert_can_parse_file("assignment", false);
}

//     ("if_stmt", false),
#[test]
fn can_parse_if_stmt() {
    assert_can_parse_file("if_stmt", false);
}

//     ("if_else_stmt", false),
#[test]
fn can_logical_if_else_stmt() {
    assert_can_parse_file("if_else_stmt", false);
}

//     ("logic_and", false),
#[test]
fn can_parse_logical_and_stmt() {
    assert_can_parse_file("logic_and", false);
}

//     ("logic_or", false),
#[test]
fn can_parse_logical_or_stmt() {
    assert_can_parse_file("logic_or", false);
}

//     ("while_stmt", false),
#[test]
fn can_parse_while_stmt() {
    assert_can_parse_file("while_stmt", false);
}

//     ("for_stmt", false),
#[test]
fn can_parse_for_stmt() {
    assert_can_parse_file("for_stmt", false);
}

//     ("continue", false),
#[test]
fn can_parse_continue() {
    assert_can_parse_file("continue", false);
}

// //     ("call_stmt", false),
#[test]
fn can_parse_call_stmt() {
    assert_can_parse_file("call_stmt", false);
}

//     ("fun_decl", false),
#[test]
fn can_parse_fun_decl_stmt() {
    assert_can_parse_file("fun_decl", false);
}
