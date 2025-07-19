use crate::ast::*;
use crate::error::{Error, Result};
use crate::token::{Lexer, Token};

pub trait Parse: Sized {
    fn parse(input: ParseStream) -> Result<Self>;
}

pub struct ParseStream<'a> {
    tokens: &'a [Token],
    cursor: usize,
}

impl<'a> ParseStream<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self { tokens, cursor: 0 }
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.cursor)
    }

    pub fn peek_ahead(&self, n: usize) -> Option<&Token> {
        self.tokens.get(self.cursor + n)
    }

    pub fn advance(&mut self) -> Option<Token> {
        let token = self.tokens.get(self.cursor).cloned();
        if token.is_some() {
            self.cursor += 1;
        }
        token
    }

    pub fn consume(&mut self, expected: &Token) -> Result<()> {
        match self.advance() {
            Some(token) if &token == expected => Ok(()),
            Some(token) => Err(Error::Parse(format!(
                "Expected {:?}, found {:?}",
                expected, token
            ))),
            None => Err(Error::UnexpectedEof),
        }
    }

    pub fn is_at_end(&self) -> bool {
        matches!(self.peek(), None | Some(Token::Eof))
    }

    pub fn parse<T: Parse>(&mut self) -> Result<T> {
        T::parse(ParseStream {
            tokens: self.tokens,
            cursor: self.cursor,
        })
    }
}

pub fn parse(input: &str) -> Result<Vec<Element>> {
    let mut lexer = Lexer::new(input);
    let tokens = lexer.tokenize();
    let mut stream = ParseStream::new(&tokens);

    let mut elements = Vec::new();

    while !stream.is_at_end() {
        elements.push(parse_element(&mut stream)?);
    }

    Ok(elements)
}

fn parse_element(stream: &mut ParseStream) -> Result<Element> {
    let token = stream.peek().cloned();
    match token {
        Some(Token::Star) => parse_bold_or_italic(stream),
        Some(Token::Underscore) => parse_italic_or_underline(stream),
        Some(Token::Backtick) => parse_code_or_pre(stream),
        Some(Token::Tilde) => parse_strikethrough_or_spoiler(stream),
        Some(Token::LeftBracket) => parse_link(stream),
        Some(Token::Mention(username)) => {
            stream.advance();
            Ok(Element::Mention { username })
        }
        Some(Token::Hashtag(tag)) => {
            stream.advance();
            Ok(Element::Hashtag(tag))
        }
        Some(Token::Command(cmd)) => {
            stream.advance();
            Ok(Element::Command {
                name: cmd,
                args: Vec::new(),
            })
        }
        Some(Token::Text(text)) => {
            stream.advance();
            Ok(Element::Text(text))
        }
        Some(Token::Escape(ch)) => {
            stream.advance();
            Ok(Element::Text(ch.to_string()))
        }
        Some(Token::LineBreak) => {
            stream.advance();
            Ok(Element::Text("\n".to_string()))
        }
        _ => {
            stream.advance();
            Ok(Element::Text("".to_string()))
        }
    }
}

fn parse_bold_or_italic(stream: &mut ParseStream) -> Result<Element> {
    stream.consume(&Token::Star)?;

    if matches!(stream.peek(), Some(Token::Star)) {
        stream.advance();
        let content = parse_until_double_star(stream)?;
        Ok(Element::Bold(content))
    } else {
        let content = parse_until_single_star(stream)?;
        Ok(Element::Italic(content))
    }
}

fn parse_italic_or_underline(stream: &mut ParseStream) -> Result<Element> {
    stream.consume(&Token::Underscore)?;

    if matches!(stream.peek(), Some(Token::Underscore)) {
        stream.advance();
        let content = parse_until_double_underscore(stream)?;
        Ok(Element::Underline(content))
    } else {
        let content = parse_until_single_underscore(stream)?;
        Ok(Element::Italic(content))
    }
}

fn parse_code_or_pre(stream: &mut ParseStream) -> Result<Element> {
    stream.consume(&Token::Backtick)?;

    if matches!(stream.peek(), Some(Token::Backtick)) {
        stream.advance();
        if matches!(stream.peek(), Some(Token::Backtick)) {
            stream.advance();
            parse_pre_block(stream)
        } else {
            Ok(Element::Code("".to_string()))
        }
    } else {
        let mut code = String::new();
        while let Some(token) = stream.peek() {
            match token {
                Token::Backtick => {
                    stream.advance();
                    return Ok(Element::Code(code));
                }
                Token::Text(text) => {
                    code.push_str(text);
                    stream.advance();
                }
                _ => break,
            }
        }
        Err(Error::Parse("Unclosed code block".to_string()))
    }
}

fn parse_pre_block(stream: &mut ParseStream) -> Result<Element> {
    let language = match stream.peek() {
        Some(Token::Text(lang)) => {
            let language = Some(lang.clone());
            stream.advance();
            language
        }
        _ => None,
    };

    if language.is_some() {
        if let Some(Token::LineBreak) = stream.peek() {
            stream.advance();
        }
    }

    let mut code = String::new();
    let mut backtick_count = 0;

    while let Some(token) = stream.peek() {
        match token {
            Token::Backtick => {
                backtick_count += 1;
                stream.advance();
                if backtick_count == 3 {
                    return Ok(Element::Pre(PreBlock { code, language }));
                } else {
                    code.push('`');
                }
            }
            Token::Text(text) => {
                backtick_count = 0;
                code.push_str(text);
                stream.advance();
            }
            Token::LineBreak => {
                backtick_count = 0;
                code.push('\n');
                stream.advance();
            }
            _ => {
                backtick_count = 0;
                stream.advance();
            }
        }
    }

    Err(Error::Parse("Unclosed pre block".to_string()))
}

fn parse_strikethrough_or_spoiler(stream: &mut ParseStream) -> Result<Element> {
    stream.consume(&Token::Tilde)?;

    if matches!(stream.peek(), Some(Token::Tilde)) {
        stream.advance();
        let content = parse_until_double_tilde(stream)?;
        Ok(Element::Strikethrough(content))
    } else {
        let content = parse_until_single_tilde(stream)?;
        Ok(Element::Spoiler(content))
    }
}

fn parse_link(stream: &mut ParseStream) -> Result<Element> {
    stream.consume(&Token::LeftBracket)?;

    let text = parse_until_right_bracket(stream)?;
    stream.consume(&Token::RightBracket)?;
    stream.consume(&Token::LeftParen)?;

    let mut url = String::new();
    while let Some(token) = stream.peek() {
        match token {
            Token::RightParen => {
                stream.advance();
                return Ok(Element::Link { text, url });
            }
            Token::Text(text) => {
                url.push_str(text);
                stream.advance();
            }
            Token::Slash => {
                url.push('/');
                stream.advance();
            }
            Token::At => {
                url.push('@');
                stream.advance();
            }
            Token::Hash => {
                url.push('#');
                stream.advance();
            }
            Token::Underscore => {
                url.push('_');
                stream.advance();
            }
            Token::Star => {
                url.push('*');
                stream.advance();
            }
            Token::Tilde => {
                url.push('~');
                stream.advance();
            }
            Token::Pipe => {
                url.push('|');
                stream.advance();
            }
            Token::LeftBracket => {
                url.push('[');
                stream.advance();
            }
            Token::RightBracket => {
                url.push(']');
                stream.advance();
            }
            Token::LeftBrace => {
                url.push('{');
                stream.advance();
            }
            Token::RightBrace => {
                url.push('}');
                stream.advance();
            }
            Token::Command(cmd) => {
                url.push('/');
                url.push_str(cmd);
                stream.advance();
            }
            Token::Mention(name) => {
                url.push('@');
                url.push_str(name);
                stream.advance();
            }
            Token::Hashtag(tag) => {
                url.push('#');
                url.push_str(tag);
                stream.advance();
            }
            _ => break,
        }
    }

    Err(Error::Parse("Unclosed link".to_string()))
}

fn parse_until_double_star(stream: &mut ParseStream) -> Result<Vec<Element>> {
    let mut elements = Vec::new();

    while let Some(token) = stream.peek() {
        if matches!(token, Token::Star) {
            if matches!(stream.peek_ahead(1), Some(Token::Star)) {
                stream.advance();
                stream.advance();
                return Ok(elements);
            }
        }
        elements.push(parse_element(stream)?);
    }

    Err(Error::Parse("Unclosed bold".to_string()))
}

fn parse_until_single_star(stream: &mut ParseStream) -> Result<Vec<Element>> {
    let mut elements = Vec::new();

    while let Some(token) = stream.peek() {
        if matches!(token, Token::Star) {
            stream.advance();
            return Ok(elements);
        }
        elements.push(parse_element(stream)?);
    }

    Err(Error::Parse("Unclosed italic".to_string()))
}

fn parse_until_double_underscore(stream: &mut ParseStream) -> Result<Vec<Element>> {
    let mut elements = Vec::new();

    while let Some(token) = stream.peek() {
        if matches!(token, Token::Underscore) {
            if matches!(stream.peek_ahead(1), Some(Token::Underscore)) {
                stream.advance();
                stream.advance();
                return Ok(elements);
            }
        }
        elements.push(parse_element(stream)?);
    }

    Err(Error::Parse("Unclosed underline".to_string()))
}

fn parse_until_single_underscore(stream: &mut ParseStream) -> Result<Vec<Element>> {
    let mut elements = Vec::new();

    while let Some(token) = stream.peek() {
        if matches!(token, Token::Underscore) {
            stream.advance();
            return Ok(elements);
        }
        elements.push(parse_element(stream)?);
    }

    Err(Error::Parse("Unclosed italic".to_string()))
}

fn parse_until_double_tilde(stream: &mut ParseStream) -> Result<Vec<Element>> {
    let mut elements = Vec::new();

    while let Some(token) = stream.peek() {
        if matches!(token, Token::Tilde) {
            if matches!(stream.peek_ahead(1), Some(Token::Tilde)) {
                stream.advance();
                stream.advance();
                return Ok(elements);
            }
        }
        elements.push(parse_element(stream)?);
    }

    Err(Error::Parse("Unclosed strikethrough".to_string()))
}

fn parse_until_single_tilde(stream: &mut ParseStream) -> Result<Vec<Element>> {
    let mut elements = Vec::new();

    while let Some(token) = stream.peek() {
        if matches!(token, Token::Tilde) {
            stream.advance();
            return Ok(elements);
        }
        elements.push(parse_element(stream)?);
    }

    Err(Error::Parse("Unclosed spoiler".to_string()))
}

fn parse_until_right_bracket(stream: &mut ParseStream) -> Result<Vec<Element>> {
    let mut elements = Vec::new();

    while let Some(token) = stream.peek() {
        if matches!(token, Token::RightBracket) {
            return Ok(elements);
        }
        elements.push(parse_element(stream)?);
    }

    Err(Error::Parse("Unclosed bracket".to_string()))
}
