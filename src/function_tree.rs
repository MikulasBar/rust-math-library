use derive_more::Display;
use maplit::hashmap;
use std::{
    collections::HashMap,
    f64::consts::{E, PI},
};
use antlr_rust::{
    common_token_stream::CommonTokenStream,
    tree::{ParseTree, ParseTreeVisitorCompat},
    InputStream
};

use crate::{
    functions::*,
    utilities::*,
    visitor::Visitor,
    antlr_parser::{
        mathlexer::*,
        mathparser::*,
        mathvisitor::*,
    }
};


#[derive(Debug, Display)]
pub enum ParsingError {
    Default,
    UnrecognizedFunctionNameError,
    AntlrError
}

/// this exists only because the antlr visitor pattern trait only accepts
/// Return types only that have `Default` implementation <br>
/// it is converted to normal `Result<ChildFn, ParsingError>` in `ParserFn` struct
// #[derive(Debug)]
pub enum ParsingResult {
    Ok(ChildFn),
    Err(ParsingError),
}


impl ParsingResult {
    pub fn is_err(&self) -> bool {
        match *self {
            Self::Err(_) => true,
            _ => false
        }
    }

    pub fn unwrap(self) -> ChildFn {
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

impl Into<Result<ChildFn, ParsingError>> for ParsingResult {
    fn into(self) -> Result<ChildFn, ParsingError> {
        match self {
            ParsingResult::Ok(v) => Ok(v),
            ParsingResult::Err(e) => Err(e)
        }
    }
}



pub trait ParsingRules {
    fn get_function(&self, name: &str, args: Vec<ChildFn>) -> Option<ChildFn>;
    fn get_constant(&self, name: &str) -> Option<f64>;
}

#[derive(Clone)]
pub struct DefaultParsingRules;

impl ParsingRules for DefaultParsingRules {
    fn get_function(&self, name: &str, mut args: Vec<ChildFn>) -> Option<ChildFn> {
        Some (
            match name {
                "sin" => args.pop().map(SinFn::new).to_child_fn(),
                "cos" => args.pop().map(CosFn::new).to_child_fn(),
                "tan" => args.pop().map(TanFn::new).to_child_fn(),
                "log" => {
                    let arg = args.pop()?;
                    LogFn::new(10.0, arg).to_child_fn()
                },
                "ln" => {
                    let arg = args.pop()?;
                    LogFn::new(E, arg).to_child_fn()
                },
                _ => return None
            }
        )
    }

    fn get_constant(&self, name: &str) -> Option<f64> {
        Some(
            match name {
                "pi" => PI,
                "e" => E,
                _ => return None
            }
        )
    }
}


pub struct FnParser {
    visitor: Visitor,
}

impl FnParser {
    pub fn new() -> Self {
        FnParser {
            visitor: Visitor::new(DefaultParsingRules)
        }
    }

    pub fn change_rules<T>(&mut self, rules: T)
    where
        T: ParsingRules + 'static
    {
        self.visitor = Visitor::new(rules);
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


#[derive(Clone)]
pub struct FnTree {
    definition: ChildFn,
    string_tree: String,
}

impl FnTree {
    pub fn new<T>(definition: T) -> Self 
    where
        T: ToChildFn
    {
        let mut fn_tree = Self {
            definition: definition.to_child_fn(),
            string_tree: "".to_string()
        };
        fn_tree.string_tree = fn_tree.definition.get_string_tree();
        fn_tree
    }
}

impl Function for FnTree {
    fn clone_box(&self) -> Box<dyn Function> {
        Box::new(self.clone())
    }

    fn evaluate(&self, args: &HashMap<&str, f64>) -> Result<f64, EvalError> {
        self.definition.evaluate(args)
    }

    fn substitute(&self, args: &HashMap<&str, ChildFn>) -> ChildFn {
        self.definition.substitute(args)
    }

    fn get_string_tree(&self) -> String {
        self.string_tree.clone()
    }

    fn derivative(&self, variable: &str) -> ChildFn {
        self.definition.derivative(variable)
    }
}


// 2^(3 - 1) * (1 - cos(pi/x)) + log_5(y + ln(e))
#[test]
fn test_parser() {
    let mut parser = FnParser::new();
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