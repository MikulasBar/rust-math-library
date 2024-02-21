use std::collections::HashMap;
use maplit::hashmap;
use Token::*;
use once_cell::sync::Lazy;

#[derive(Debug, PartialEq, Clone)]
enum Separator {
    LParen,
    RParen,
    //Comma,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    //Kw(String),
    Op(Operator),
    Sep(Separator),
    Lit(f32),
    Id(String),
}

static OP_TO_ENUM: Lazy<HashMap<char, Token>> = Lazy::new(|| hashmap! {
    '+' => Op(Operator::Add),
    '-' => Op(Operator::Sub),
});




pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let chars = input.chars();

    let mut buffer = String::new();
    for c in chars {
        match c {
            ' ' => (),
            op @ ('+' | '-') => {
                if !buffer.is_empty() {
                    tokens.push(Lit(buffer.parse().unwrap()));
                    buffer.clear();
                }

                tokens.push(OP_TO_ENUM[&op].clone())
            },
            '0'..='9' => buffer.push(c),
            _ => unreachable!(),
        }
    }

    if !buffer.is_empty() {
        tokens.push(Lit(buffer.parse().unwrap()));
    }
    tokens
}