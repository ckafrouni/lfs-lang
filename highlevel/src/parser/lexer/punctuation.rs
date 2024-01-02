#[derive(Debug, PartialEq)]
pub enum Punctuation {
    Plus,       // +
    Minus,      // -
    Star,       // *
    Slash,      // /
    Percent,    // %
    Caret,      // ^
    Not,        // !
    And,        // &
    Or,         // |
    AndAnd,     // &&
    OrOr,       // ||
    Shl,        // <<
    Shr,        // >>
    PlusEq,     // +=
    MinusEq,    // -=
    StarEq,     // *=
    SlashEq,    // /=
    PercentEq,  // %=
    CaretEq,    // ^=
    AndEq,      // &=
    OrEq,       // |=
    ShlEq,      // <<=
    ShrEq,      // >>=
    Eq,         // =
    EqEq,       // ==
    Ne,         // !=
    Gt,         // >
    Lt,         // <
    Ge,         // >=
    Le,         // <=
    At,         // @
    Underscore, // _
    Dot,        // .
    DotDot,     // ..
    DotDotDot,  // ...
    DotDotEq,   // ..=
    Comma,      // ,
    Semi,       // ;
    Colon,      // :
    PathSep,    // ::
    RArrow,     // ->
    FatArrow,   // =>
    Pound,      // #
    Dollar,     // $
    Question,   // ?
    Tilde,      // ~

    Unknown(String),
}

/// Convert a string to a punctuation
impl From<&str> for Punctuation {
    fn from(s: &str) -> Self {
        match s {
            "..." => Punctuation::DotDotDot,
            "..=" => Punctuation::DotDotEq,
            "->" => Punctuation::RArrow,
            "=>" => Punctuation::FatArrow,
            ">=" => Punctuation::Ge,
            "<=" => Punctuation::Le,
            "+=" => Punctuation::PlusEq,
            "-=" => Punctuation::MinusEq,
            "*=" => Punctuation::StarEq,
            "/=" => Punctuation::SlashEq,
            "%=" => Punctuation::PercentEq,
            "^=" => Punctuation::CaretEq,
            "&=" => Punctuation::AndEq,
            "|=" => Punctuation::OrEq,
            "<<=" => Punctuation::ShlEq,
            ">>=" => Punctuation::ShrEq,
            "&&" => Punctuation::AndAnd,
            "||" => Punctuation::OrOr,
            "<<" => Punctuation::Shl,
            ">>" => Punctuation::Shr,
            ">=" => Punctuation::Ge,
            "<=" => Punctuation::Le,
            _ if s.len() == 1 => match s.chars().next().unwrap() {
                '+' => Punctuation::Plus,
                '-' => Punctuation::Minus,
                '*' => Punctuation::Star,
                '/' => Punctuation::Slash,
                '%' => Punctuation::Percent,
                '^' => Punctuation::Caret,
                '!' => Punctuation::Not,
                '&' => Punctuation::And,
                '|' => Punctuation::Or,
                '=' => Punctuation::Eq,
                '>' => Punctuation::Gt,
                '<' => Punctuation::Lt,
                '@' => Punctuation::At,
                '_' => Punctuation::Underscore,
                '.' => Punctuation::Dot,
                ',' => Punctuation::Comma,
                ';' => Punctuation::Semi,
                ':' => Punctuation::Colon,
                '#' => Punctuation::Pound,
                '$' => Punctuation::Dollar,
                '?' => Punctuation::Question,
                '~' => Punctuation::Tilde,
                _ => Punctuation::Unknown(s.to_string()),
            },
            _ => Punctuation::Unknown(s.to_string()),
        }
    }
}