#![allow(dead_code)]

use crate::error::Error;
use crate::hir::*;
use anyhow::{anyhow, Error as AnyError, Result};
use nom::{
    character::complete::{line_ending, not_line_ending},
    combinator::complete,
    error::{convert_error, VerboseError},
    sequence::terminated,
    Err, IResult,
};

type ParseResult<I, O> = IResult<I, O, VerboseError<I>>;

/// Parse the input into a complete program, or print errors and report that
/// compilation failed.
pub fn parse(input: &str) -> Result<Program> {
    parse_with_errors(input)
        .map(|(_, output)| output)
        .map_err(|error| handle_error(input, error))
}

fn parse_with_errors(input: &str) -> ParseResult<&str, Program> {
    complete(line)(input).map(|(i, _)| (i, Program(Vec::new())))
}

fn line(input: &str) -> ParseResult<&str, &str> {
    terminated(not_line_ending, line_ending)(input)
}

fn ident(_input: &str) -> ParseResult<&str, Ident<'_>> {
    // idents can start with [a-zA-Z], but can then include [0-9] or -, _, +, >, <, *
    todo!()
}

fn bool(_input: &str) -> ParseResult<&str, Bool<'_>> {
    todo!()
}

fn ty(_input: &str) -> ParseResult<&str, Ty<'_>> {
    todo!()
}

fn int(_input: &str) -> ParseResult<&str, Int<'_>> {
    todo!()
}

fn float(_input: &str) -> ParseResult<&str, Float<'_>> {
    todo!()
}

fn ustr(_input: &str) -> ParseResult<&str, UStr<'_>> {
    todo!()
}

fn bstr(_input: &str) -> ParseResult<&str, BStr<'_>> {
    todo!()
}

fn char(_input: &str) -> ParseResult<&str, Char<'_>> {
    todo!()
}

fn symbol(_input: &str) -> ParseResult<&str, Symbol<'_>> {
    todo!()
}

fn array(_input: &str) -> ParseResult<&str, Array<'_>> {
    todo!()
}

fn tuple(_input: &str) -> ParseResult<&str, Tuple<'_>> {
    todo!()
}

fn map(_input: &str) -> ParseResult<&str, Map<'_>> {
    todo!()
}

fn handle_error(input: &str, error: Err<VerboseError<&str>>) -> AnyError {
    match error {
        // we call `complete` on the parser in `parse_with_errors` so this should never happen.
        Err::Incomplete(_) => unreachable!(),
        Err::Error(error) | Err::Failure(error) => println!("{}", convert_error(input, error)),
    }

    anyhow!(Error::ParseFailed)
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Keyword {
    // use
    Use,
    // fn
    Fn,
    // return
    Return,
    // and
    And,
    // or
    Or,
    // xor
    Xor,
    // not
    Not,
    // if
    If,
    // else
    Else,
    // else if
    ElseIf,
    // unless
    Unless,
    // else unless
    ElseUnless,
    // loop
    Loop,
    // while
    While,
    // until
    Until,
    // for
    For,
    // in
    In,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Operator {
    // =
    Assign,
    // +
    Add,
    // -
    Sub,
    // *
    Mul,
    // /
    Div,
    // %
    Rem,
    // ^ (e.g. x ^ 2 == x * x)
    Exp,
    // +=
    AddAssign,
    // -=
    SubAssign,
    // *=
    MulAssign,
    // /=
    DivAssign,
    // %=
    RemAssign,
    // ^=
    ExpAssign,
    // == (equality of value)
    Eq,
    // @ (get identity)
    Id,
    // >
    Greater,
    // <
    Less,
    // >=
    GreaterEq,
    // <=
    LessEq,
    // !=
    NotEq,
}
