use super::lexer::Token;

pub enum ParserError {
    TokenError(Token),
}

pub struct Error {
    pub error: ParserError,
    pub line: u32,
    pub column: u32,
}