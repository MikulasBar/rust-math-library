#![allow(dead_code)]

use antlr_rust::{
    common_token_stream::CommonTokenStream,
    InputStream,
    tree::ParseTreeVisitorCompat,
};

#[rustfmt::skip]
use crate::{
    antlr::{
        mathlexer::*,
        mathparser::*,
    },
    visitor::*,
    child::*,
    parsing_result::*,
};



pub struct Parser {
    visitor: Visitor,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            visitor: Visitor::new()
        }
    }

    // facade for parsing the function from text format
    pub fn parse(&mut self, input: &str) -> Result<Child, ParsingError> {
        let lexer = mathLexer::new(InputStream::new(input.into()));
        let token_source = CommonTokenStream::new(lexer);
        let mut parser = mathParser::new(token_source);

        let root = parser.root();
        if root.is_err() {
            return Err(ParsingError::Antlr)
        }

        let root = root.unwrap();
        let result = self.visitor.visit(&*root);

        result.into()
    }
}


#[cfg(test)]
mod test {
    use maplit::hashmap;
    use std::f64::consts::PI;
    
    use super::Parser;

    // // 2^(3 - 1) * (1 - cos(pi/x)) + log_5(y + ln(e))
    // // not yet implemented
    // #[should_panic]
    // #[test]
    // fn parser() {
    //     let mut parser = Parser::new();
    //     let fn_result = parser.parse("2^(3 - 1) * (1 - cos(pi/x)) + log_5(y + ln(e))");

    //     let func = fn_result.unwrap();
    //     let dfunc = func.derivative("x");

    //     let args = hashmap!{
    //         "x" => 2.0,
    //         "y" => 4.0,
    //     };

    //     assert_eq!(func.eval(&args), Ok(5.0));
    //     assert_eq!(dfunc.eval(&args), Ok(-PI));
    // }
}
