use derive_more::Display;
use maplit::hashmap;
use std::f64::consts::PI;

use antlr_rust::{
    common_token_stream::CommonTokenStream,
    InputStream,
    tree::ParseTreeVisitorCompat,
};

#[rustfmt::skip]
use crate::{
    antlr_parser::{
        mathlexer::*,
        mathparser::*,
        mathvisitor::*
    },
    visitor::*,
    tree::FnTree,
    child::*,
    fn_behaviour::*,
};



pub struct Parser {
    visitor: Visitor,
}

impl Parser {
    pub fn new() -> Self {
        Parser {
            visitor: Visitor::new()
        }
    }

    pub fn parse(&mut self, input: &str) -> Result<FnTree, ParsingError> {
        let lexer = mathLexer::new(InputStream::new(input.into()));
        let token_source = CommonTokenStream::new(lexer);
        let mut parser = mathParser::new(token_source);

        let root = parser.root();
        if root.is_err() {
            return Err(ParsingError::AntlrError)
        }

        let root = root.unwrap();
        let parsing_result = self.visitor.visit(&*root);

    
        let function = match parsing_result {
            ParsingResult::Ok(v) => FnTree::new(v),
            ParsingResult::Err(e) => return Err(e),
        };

        Ok(function)
    }
}



#[derive(Debug, Display)]
pub enum ParsingError {
    Default,
    UnrecognizedFunctionNameError,
    AntlrError
}

/// this exists only because the antlr visitor pattern trait only accepts
/// Return types only that have `Default` implementation <br>
/// it is converted to normal `Result<Child, ParsingError>` in `ParserFn` struct
// #[derive(Debug)]
pub enum ParsingResult {
    Ok(Child),
    Err(ParsingError),
}


impl ParsingResult {
    pub fn is_err(&self) -> bool {
        match *self {
            Self::Err(_) => true,
            _ => false
        }
    }

    pub fn unwrap(self) -> Child {
        match self {
            Self::Ok(v) => v,
            _ => panic!("Err cannot be unwraped")
        }
    }
}

impl Default for ParsingResult {
    fn default() -> Self {
        Self::Err(ParsingError::Default)
    }
}

impl Into<Result<Child, ParsingError>> for ParsingResult {
    fn into(self) -> Result<Child, ParsingError> {
        match self {
            ParsingResult::Ok(v) => Ok(v),
            ParsingResult::Err(e) => Err(e)
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    // 2^(3 - 1) * (1 - cos(pi/x)) + log_5(y + ln(e))
    #[test]
    fn test_parser() {
        let mut parser = Parser::new();
        let fn_result = parser.parse("2^(3 - 1) * (1 - cos(pi/x)) + log_5(y + ln(e))");

        let func = fn_result.unwrap();
        let dfunc = func.derivative("x");

        let args = hashmap!{
            "x" => 2.0,
            "y" => 4.0,
        };

        assert_eq!(func.evaluate(&args), Ok(5.0));
        assert_eq!(dfunc.evaluate(&args), Ok(-PI));
    }
}
