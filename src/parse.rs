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
    complete(line)(input).map(|(i, _)| (i, Program::empty()))
}

fn line(input: &str) -> ParseResult<&str, &str> {
    terminated(not_line_ending, line_ending)(input)
}

fn ident(_input: &str) -> ParseResult<&str, Program> {
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
