pub mod ast;
pub mod conditional;
pub mod error;
pub mod formatter;
pub mod generator;
pub mod parser;
pub mod token;

pub use ast::*;
pub use error::{Error, Result};
pub use formatter::CustomFormatter;
pub use generator::{Generate, Generator, ParseMode};
pub use parser::{parse, Parse, ParseStream};
pub use token::Token;

pub use tg_message_macro::{el, msg};

#[cfg(test)]
mod tests;
