#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("parsing failed")]
    ParseFailed,

    #[error("no input file")]
    NoFile,
}
