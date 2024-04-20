use std::f64::{consts::PI, EPSILON};
use std::collections::HashMap;
use maplit::hashmap;

use crate::functions::{SeqMulFn, TanFn};
use crate::{
    antlr_parser::mathlexer::mathLexer,
    function_tree::{FnParser, ParsingRules},
    functions::{ChildFn, Function, EvalError, FunctionType}, utilities::ToChildFn
};


// note that the parser have to be mutable
// parser contains visitor that have to be changed when traversing AST
// default parsing rules for parser is DefaultParsingRules (see in function_tree)  


// example with default parsing rules
#[test]
fn simple_example() {
    let mut parser = FnParser::new();

    // parsing text format
    let func = parser.parse("sin(pi * x) + cos(pi / y)").unwrap();

    // derivative of the function with respect to x
    // df/dx = pi * cos(pi * x)
    let d_func = func.derivative("x");

    // hashmap as arguments for function
    let args = hashmap! {
        "x" => 2.0,
        "y" => 2.0,
    };

    // results of evaluated functions 
    let result = func.evaluate(&args).unwrap();
    let d_result = d_func.evaluate(&args).unwrap();

    // expected values
    let expected = 0.0;
    let d_expected = PI;

    // function evaluated with args
    // goniometric functions are not precise - use EPSILON
    assert!((result - expected).abs() <= EPSILON);
    
    // derivative test
    assert!((d_result - d_expected).abs() <= EPSILON);
}






// custom function type - sec(x) or 1/cos(x)
// derive clone, so you can use implement clone_box function later
#[derive(Clone)]
struct SecFn {
    // always use ChildFn as type for inner functions
    child: ChildFn
}

impl SecFn {
    pub fn new<T>(child: T) -> Self
        // use ToChildFn trait for convertable type
        // types will be converted to ChildFn enum such that 
        // &str -> variable
        // f64 -> constant
        // type that implements Function trait -> function (only if type has static lifetime)
        where T: ToChildFn
    {
        Self {
            // use to_child_fn() to convert types to ChildFn
            child: child.to_child_fn()
        }
    }
}

// implement Function trait, so the type can be used as function
impl Function for SecFn {
    // this function returns just cloned version of self
    fn clone_box(&self) -> Box<dyn Function> {
        Box::new(self.clone())
    }

    // evaluate function with arguments
    fn evaluate(&self, args: &HashMap<&str, f64>) -> Result<f64, EvalError> {
        // get value of child (inner) function
        let child_value = self.child.evaluate(args)?;

        let denominator = child_value.cos();

        // error handling
        if denominator == 0.0 {
            return Err(EvalError::DivisionByZeroError)
        }

        Ok(1.0 / denominator)
    }

    fn substitute(&self, args: &HashMap<&str, ChildFn>) -> ChildFn {
        let child = self.child.substitute(args);
        Self::new(child).to_child_fn()
    }

    // this function can be handy if your function is not special
    // automatically implements other functions for this type (get_string_tree, depends_on)
    // if you dont implement this function you cant use implicit definitions of function mentioned above

    // implement this function if your type is struct and has one child or more
    // you have to pass reference to all children of your type
    // if your type is enum or its somehow special dont implement this function at all
    fn get_type(&self) -> FunctionType {
        // you can choose between
        // unary (1 child)
        // binary (2 children)
        // variadic (any number of children - in vector)
        // none (default option)
        FunctionType::Unary(&self.child)
    }

    // returns derivative of self
    // dont mutate self
    // instead use clone to make new object 
    fn derivative(&self, variable: &str) -> ChildFn {
        // derivative of sec(x) is tan(x) * sec(x) * x'

        // derivative of child
        let d_child = self.child.derivative(variable);
        let tan = TanFn::new(self.child.clone());
        let sec = self.clone();

        // Seq stands for sequence - is capable of having more than 2 children
        SeqMulFn::new(
            vec![
                tan.to_child_fn(),
                sec.to_child_fn(),
                d_child
            ]
        ).to_child_fn()
    }
}



// because we want to include our function in parsing
// we have to make custom parsing rules

// custom parsing rules
struct SecParsingRules;

// implement ParsingRules trait for our custom rules 
impl ParsingRules for SecParsingRules {
    // functions take string name and give specific value for it
    // this operation can fail
    // these function cannot be made into HashMap - trait would not be object safe
    // functions like sin or constants like pi are not automatically implemented
    // so if you are using your own rules you have to implement all functions your self


    fn get_constant(&self, name: &str) -> Option<f64> {
        match name {
            "pi" => Some(PI),
            _ => return None
        } 
    }

    // this operation can fail because of the length of arguments
    fn get_function(&self, name: &str, mut args: Vec<ChildFn>) -> Option<ChildFn> {
        if args.len() == 1 && name == "sec" {
            return Some (
                args.pop().map(SecFn::new).to_child_fn()
            ) 
        }
        return None
    }
}



// example with custom rules
#[test]
fn complex_example() {
    let mut parser = FnParser::new();

    // change rules to your custom rules
    parser.change_rules(SecParsingRules);

    let func = parser.parse("sec(pi * x)").unwrap();

    let args = hashmap! {
        "x" => 1.0
    };

    let result = func.evaluate(&args).unwrap();

    let expected = -1.0;

    assert!((result - expected).abs() <= EPSILON)
}