use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Parse error: {0}")]
    Parse(String),

    #[error("Invalid token at position {position}: {message}")]
    InvalidToken { position: usize, message: String },

    #[error("Unexpected end of input")]
    UnexpectedEof,

    #[error("Formatter not found: {0}")]
    FormatterNotFound(String),

    #[error("Invalid formatter value: {0}")]
    InvalidFormatterValue(String),

    #[error("Generation error: {0}")]
    Generation(String),

    #[error("Invalid table structure: {0}")]
    InvalidTable(String),

    #[error("Regex error: {0}")]
    Regex(#[from] regex::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
