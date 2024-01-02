use nom::{IResult, branch::alt};

pub mod keyword;
pub mod punctuation;

#[derive(Debug, PartialEq)]
pub enum Token {
    Keyword(keyword::Keyword), 
    Punctuation(punctuation::Punctuation),

    Unknown(String),
}

pub struct Lexer {
    pub tokens: Vec<Token>,
    pub string: String,
    pub line: u32,
    pub column: u32,
}

impl Lexer {
    pub fn new() -> Self {
        Self {
            tokens: Vec::new(),
            string: String::new(),
            line: 0,
            column: 0,
        }
    }

    pub fn next_token(&mut self) -> Option<Token> {
        if self.tokens.len() > 0 {
            return Some(self.tokens.remove(0));
        }

        if self.string.len() == 0 {
            return None;
        }

        let mut token = None;

        if let Ok((rest, t)) = keyword::parse_keyword(&self.string) {
            token = Some(t);
            self.string = rest.to_string();
        }

        if let Ok((rest, t)) = punctuation::parse_punctuation(&self.string) {
            token = Some(t);
            self.string = rest.to_string();
        }

        if let Some(t) = token {
            return Some(t);
        }

        let mut chars = self.string.chars();
        let c = chars.next().unwrap();
        self.string = chars.as_str().to_string();

        match c {
            '\n' => {
                self.line += 1;
                self.column = 0;
            },
            _ => {
                self.column += 1;
            }
        }

        Some(Token::Unknown(c.to_string()))
    }
}