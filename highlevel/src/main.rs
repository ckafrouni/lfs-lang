use highlevel::parser::lexer::{keyword, punctuation};


fn main() {
    let keywords = [
        "fnn let",
        "let",
        "type",
        "unsafe",
        "use",
        "where",
        "while",
    ];

    let punctuations = [
        "+",
        "-",
        "*",
        "/",
        "%",
        "^",
        "!",
        "=",
        "==",
        "!=",
        "+=",
        ">="
    ];

    println!("\nKeywords:");
    for keyword in keywords.iter() {
        println!("{}: {:?}", keyword, keyword::parse_keyword(keyword));
    }

    println!("\nPunctuations:");
    for punctuation in punctuations.iter() {
        println!("{}: {:?}", punctuation, punctuation::parse_punctuation(punctuation));
    }

    let tokens = [
        "let",
        "type",
        "unsafe",
        "use",
        "where",
        "while",
        "+",
        "-",
        "*",
        "/",
        "%",
        "^",
        "!",
        "=",
        "==",
        "!=",
        "+=",
        ">=",
    ];

    println!("\nTokens:");
    for token in tokens.iter() {
        println!("{}: {:?}", token, highlevel::parser::lexer::parse_token(token));
    }


}