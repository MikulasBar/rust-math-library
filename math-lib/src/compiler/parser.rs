
use std::{process::Child, vec};

use crate::{
    functions::*, utilities::ToChildFn,
};
use super::lexer::*;
use Token::*;

fn parse(tokens: Vec<Token>) -> ChildFn {
    let mut f = AddFn::new(vec![]);
    let mut buffer: Vec<Token> = Vec::new();
    let mut sign: f32 = 1.0;
    let mut depth = 0;

    for t in tokens.iter() {
        if depth == 0 {
            // out of parentheses
            
            match t {
                Sep(Separator::LParen) => {
                    depth += 1;
                },
                Lit(v) => {
                    f.add_child(ChildFn::Const(*v * sign));
                    sign = 1.0;
                },
                Op(Operator::Sub) => sign *= -1.0,
                _ => (),
            }
        } else {
            // inside parentheses
            
            match t {
                Sep(Separator::LParen) => {
                    depth += 1;
                    buffer.push(t.clone());
                },
                Sep(Separator::RParen) => {
                    if depth == 1 {
                        f.add_child(
                            CoefFn::new(sign, parse(buffer.clone())).to_child()
                        );
                        buffer.clear();
                        sign = 1.0;
                    } else {
                        buffer.push(t.clone());
                    }

                    depth -= 1;
                }
                Op(_) | Lit(_) => {
                    buffer.push(t.clone());
                },
                _ => (),
            }
        }
    }

    f.to_child()
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "1--(-(--1))";
        let tokens = tokenize(input);

        println!("{:?}", tokens);

        let f = parse(tokens);
        assert_eq!(f.apply(&FnArgs::new()), Ok(0.0));
    }
}


