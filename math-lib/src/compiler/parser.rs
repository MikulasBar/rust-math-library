
use std::{process::Child, vec};

use crate::{
    functions::*, utilities::ToChildFn,
};
use super::lexer::*;
use Token::*;

fn parse(tokens: Vec<Token>) -> ChildFn {
    let mut f = AddFn::new(vec![]);
    let mut sign: f32 = 1.0;
    let mut buffer: Vec<Token> = Vec::new();
    let mut in_paren = false;

    for t in tokens.iter() {
        if !in_paren {
            // out of parentheses
            
            match t {
                Sep(Separator::LParen) => {
                    in_paren = true;
                },
                Lit(v) => {
                    f.children.push(ChildFn::Const(*v * sign));
                    sign = 1.0;
                },
                Op(Operator::Sub) => sign *= -1.0,
                _ => (),
            }
        } else {
            // inside parentheses
            
            match t {
                Sep(Separator::RParen) => {
                    /*if sign == -1.0 {
                        f.children.push(
                            CoefFn::new(-1.0, parse(buffer.clone())).to_child()
                        );
                    } else {
                        f.children.push(
                            parse(buffer.clone())
                        );
                    }*/

                    f.children.push(
                        CoefFn::new(sign, parse(buffer.clone())).to_child()
                    );


                    buffer.clear();
                    in_paren = false;
                    sign = 1.0;
                }
                Op(_) | Lit(_) | Sep(Separator::LParen) => {
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
        let input = "1-(-1)";
        let tokens = tokenize(input);

        //println!("{:?}", tokens);

        let f = parse(tokens);
        assert_eq!(f.apply(&FnArgs::new()), Ok(0.0));
    }
}


// !!! wierd result: 1--(-(--1)), expected: 0, got: 1
// solve nested parentheses
// solve negative sign
// solve multiple parentheses



