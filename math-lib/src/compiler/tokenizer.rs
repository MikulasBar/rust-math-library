
enum Separator {
    OpenParen,
    CloseParen,
}

enum FnToken {
    Identifier(String),
    Operator(char),
    Separator(Separator)
}




