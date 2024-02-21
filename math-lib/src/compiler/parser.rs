
use crate::{
    functions::*,
    utilities::ToChildFn
};
use super::lexer::*;
use Token::*;

fn parse(tokens: Vec<Token>) -> Box<dyn Function> {
    let mut f = AddFn::new(vec![]);
    let mut sign = 1.0;

    for t in tokens.iter() {
        match t {
            Lit(v) => {
                f.children.push(ChildFn::Const(*v * sign));
                sign = 1.0;
            },
            Op(Operator::Sub) => sign *= -1.0,
            _ => (),
        }
    }

    Box::new(f)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "1--+29-+18";
        let tokens = tokenize(input);
        let f = parse(tokens);
        assert_eq!(f.apply(&FnArgs::new()), Ok(12.0));
    }
}



