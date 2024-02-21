
enum Separator {
    OpenParen,
    CloseParen,
}


enum FnToken {
    Identifier(String),
    Operator(char),
    Keyword(String),
    Separator(Separator)
}




