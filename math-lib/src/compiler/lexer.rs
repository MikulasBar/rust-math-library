use std::collections::HashMap;
use maplit::hashmap;
use Token::*;
use once_cell::sync::Lazy;

#[derive(Debug, PartialEq, Clone)]
pub enum Separator {
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

    /// Operator like `+`,`-`, `*`, ...
    Op(Operator),

    /// Separator like `(`, `)`
    Sep(Separator),

    /// Literal like `1.5`, `-28`, ...
    Lit(f32),

    /// Identifier like `x`, `my_name`, ...
    Id(String),
}

static OP_TO_ENUM: Lazy<HashMap<char, Token>> = Lazy::new(|| hashmap! {
    '+' => Op(Operator::Add),
    '-' => Op(Operator::Sub),
    '*' => Op(Operator::Mul),
    '/' => Op(Operator::Div),
    '^' => Op(Operator::Pow),
});


static SEP_TO_ENUM: Lazy<HashMap<char, Token>> = Lazy::new(|| hashmap! {
    '(' => Sep(Separator::LParen),
    ')' => Sep(Separator::RParen),
});



pub fn tokenize(input: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let chars = input.chars();

    let mut num_buffer = String::new();
    for c in chars {
        match c {
            ' ' => (),
            '0'..='9' => num_buffer.push(c),
            '(' | ')' => {
                if !num_buffer.is_empty() {
                    tokens.push(Lit(num_buffer.parse().unwrap()));
                    num_buffer.clear();
                }

                tokens.push(SEP_TO_ENUM[&c].clone());
            },
            '+' | '-' => {
                if !num_buffer.is_empty() {
                    tokens.push(Lit(num_buffer.parse().unwrap()));
                    num_buffer.clear();
                }

                tokens.push(OP_TO_ENUM[&c].clone())
            },
            _ => unreachable!(),
        }
    }

    if !num_buffer.is_empty() {
        tokens.push(Lit(num_buffer.parse().unwrap()));
    }
    tokens
}