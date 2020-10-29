use anyhow::Result;
use std::env::args_os;
use std::fs::read_to_string;
use tinhir::{parse, Error};

fn main() -> Result<()> {
    let file_name = args_os().nth(1).ok_or(Error::NoFile)?;
    let contents = read_to_string(&file_name)?;
    let result = parse(&contents)?;
    println!("{:#?}", result);
    Ok(())
}
