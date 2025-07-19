#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Text(String),
    Bold,
    Italic,
    Code,
    Pre,
    PreLanguage(String),
    Underline,
    Strikethrough,
    Spoiler,

    Link(String),
    Mention(String),
    MentionId(u64),
    Hashtag(String),
    Command(String),

    Emoji(String),
    CustomEmoji(String, u64),

    LineBreak,
    Escape(char),

    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,

    Star,
    Underscore,
    Backtick,
    Tilde,
    Pipe,
    At,
    Hash,
    Slash,

    Eof,
}

impl Token {
    pub fn is_delimiter(&self) -> bool {
        matches!(
            self,
            Token::Star | Token::Underscore | Token::Backtick | Token::Tilde | Token::Pipe
        )
    }

    pub fn is_structural(&self) -> bool {
        matches!(
            self,
            Token::LeftParen
                | Token::RightParen
                | Token::LeftBracket
                | Token::RightBracket
                | Token::LeftBrace
                | Token::RightBrace
        )
    }
}

pub struct Lexer {
    input: Vec<char>,
    position: usize,
}

impl Lexer {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.chars().collect(),
            position: 0,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        let mut tokens = Vec::new();

        while !self.is_at_end() {
            if let Some(token) = self.next_token() {
                tokens.push(token);
            }
        }

        tokens.push(Token::Eof);
        tokens
    }

    fn next_token(&mut self) -> Option<Token> {
        let ch = self.current_char()?;

        match ch {
            '*' => {
                self.advance();
                Some(Token::Star)
            }
            '_' => {
                self.advance();
                Some(Token::Underscore)
            }
            '`' => {
                self.advance();
                Some(Token::Backtick)
            }
            '~' => {
                self.advance();
                Some(Token::Tilde)
            }
            '|' => {
                self.advance();
                Some(Token::Pipe)
            }
            '@' => {
                self.advance();
                if let Some(mention) = self.read_mention() {
                    Some(mention)
                } else {
                    Some(Token::At)
                }
            }
            '#' => {
                self.advance();
                if let Some(hashtag) = self.read_hashtag() {
                    Some(hashtag)
                } else {
                    Some(Token::Hash)
                }
            }
            '/' => {
                self.advance();
                if let Some(command) = self.read_command() {
                    Some(command)
                } else {
                    Some(Token::Slash)
                }
            }
            '(' => {
                self.advance();
                Some(Token::LeftParen)
            }
            ')' => {
                self.advance();
                Some(Token::RightParen)
            }
            '[' => {
                self.advance();
                Some(Token::LeftBracket)
            }
            ']' => {
                self.advance();
                Some(Token::RightBracket)
            }
            '{' => {
                self.advance();
                Some(Token::LeftBrace)
            }
            '}' => {
                self.advance();
                Some(Token::RightBrace)
            }
            '\\' => {
                self.advance();
                if let Some(escaped) = self.current_char() {
                    self.advance();
                    Some(Token::Escape(escaped))
                } else {
                    Some(Token::Text("\\".to_string()))
                }
            }
            '\n' => {
                self.advance();
                Some(Token::LineBreak)
            }
            _ => Some(self.read_text()),
        }
    }

    fn read_text(&mut self) -> Token {
        let mut text = String::new();

        while let Some(ch) = self.current_char() {
            if matches!(
                ch,
                '*' | '_'
                    | '`'
                    | '~'
                    | '|'
                    | '@'
                    | '#'
                    | '/'
                    | '('
                    | ')'
                    | '['
                    | ']'
                    | '{'
                    | '}'
                    | '\\'
                    | '\n'
            ) {
                break;
            }
            text.push(ch);
            self.advance();
        }

        Token::Text(text)
    }

    fn read_mention(&mut self) -> Option<Token> {
        let start = self.position;
        let mut username = String::new();

        while let Some(ch) = self.current_char() {
            if ch.is_alphanumeric() || ch == '_' {
                username.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        if username.is_empty() {
            self.position = start;
            None
        } else {
            Some(Token::Mention(username))
        }
    }

    fn read_hashtag(&mut self) -> Option<Token> {
        let start = self.position;
        let mut tag = String::new();

        while let Some(ch) = self.current_char() {
            if ch.is_alphanumeric() || ch == '_' {
                tag.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        if tag.is_empty() {
            self.position = start;
            None
        } else {
            Some(Token::Hashtag(tag))
        }
    }

    fn read_command(&mut self) -> Option<Token> {
        let start = self.position;
        let mut command = String::new();

        while let Some(ch) = self.current_char() {
            if ch.is_alphanumeric() || ch == '_' {
                command.push(ch);
                self.advance();
            } else {
                break;
            }
        }

        if command.is_empty() {
            self.position = start;
            None
        } else {
            Some(Token::Command(command))
        }
    }

    fn current_char(&self) -> Option<char> {
        self.input.get(self.position).copied()
    }

    fn advance(&mut self) {
        self.position += 1;
    }

    fn is_at_end(&self) -> bool {
        self.position >= self.input.len()
    }
}
