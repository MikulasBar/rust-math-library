
use std::{
    f64::consts::{E, PI},
};
use antlr_rust::{
    common_token_stream::CommonTokenStream,
    tree::{ParseTree, ParseTreeVisitorCompat},
    InputStream
};
use crate::visitor::MathVisitor;
use crate::functions::*;
use crate::utilities::*;
use crate::antlr_parser::{
    mathlexer::*,
    mathparser::*,
    mathvisitor::*,
};

pub enum ParsingError {
    PlaceHolder,
    UnrecognizedFunctionNameError,
    AntlrError
}

/// this exists only because the antlr visitor pattern trait only accepts
/// Return types only that have `Default` implementation <br>
/// it is converted to normal `Result<ChildFn, ParsingError>` in `ParserFn` struct 
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
            _ => panic!()
        }
    }
}

impl Default for ParsingResult {
    fn default() -> Self {
        Self::Err(ParsingError::PlaceHolder)
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
    visitor: MathVisitor,
}

impl FnParser {
    pub fn new() -> Self {
        FnParser {
            visitor: MathVisitor::new(
                Box::new(DefaultParsingRules)
            )
        }
    }

    pub fn change_rules(&mut self, rules: Box<dyn ParsingRules>) {
        self.visitor = MathVisitor::new(rules);
    }

    pub fn parse(&mut self, input: &str) -> Result<FnTree, ParsingError> {
        let lexer = mathLexer::new(InputStream::new(input.into()));
        let token_source = CommonTokenStream::new(lexer);
        let mut parser = mathParser::new(token_source);

        let root = parser.prog();
        if root.is_err() {
            return Err(ParsingError::AntlrError)
        }

        let root = root.unwrap();
        let parsing_result = self.visitor.visit(&*root);

        match parsing_result {
            ParsingResult::Ok(v) => Ok(FnTree::new(v)),
            ParsingResult::Err(e) => Err(e),
        }
    }
}



pub struct FnTree {
    definition: ChildFn,
}

impl FnTree {
    pub fn new<T>(definition: T) -> Self
    where
        T: ToChildFn
    {
        Self {
            definition: definition.to_child_fn()
        }
    }
}

impl Function for FnTree {
    fn apply(&self, args: &FnArgs) -> Result<f64, FnApplyError> {
        self.definition.apply(args)
    } 
}

impl Default for FnTree {
    fn default() -> Self {
        Self {
            definition: "x".to_child_fn()
        }
    }
}