mod error;
mod io;
mod parse;

use crate::error::Error;
use crate::io::read_file;
use crate::parse::parse;
use anyhow::Result;
use std::env::args_os;

fn main() -> Result<()> {
    let file_name = args_os().nth(1).ok_or(Error::NoFile)?;
    let contents = read_file(&file_name)?;
    let result = parse(&contents)?;
    println!("{:#?}", result);
    Ok(())
}
