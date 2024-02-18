use std::result;
use std::{collections::HashMap};

use crate::functions::*;
use crate::utilities::parser_utils::*;
use crate::utilities::function_utils::ToChildFn;



pub struct FunctionStruct {
    pub definition: ChildFn,
}

impl FunctionStruct { 
    pub fn from_str(string: &str) -> Self {
        let definition = parse_to_func(string);
        Self {
            definition
        }
    }


    /// Warning - this function may not be fast, use `get_closure()` instead
    pub fn apply(&self, args: FnArgs) -> FnResult {
        self.definition.apply(args)
    }

    pub fn get_closure(&self) -> impl Fn(FnArgs) -> f32 {
        todo!();
        |x| 0.0
    }
}

impl Default for FunctionStruct {
    fn default() -> Self {
        Self {
            definition: ChildFn::Const(0.0)
        }
    }
}


/// Describes which operation <br>
/// is on the surface level of the string that is being parsed
enum ParseState {
    Addition(Vec<String>),
    Subtraction(String, String),
    Multiplication(Vec<String>),
    Division(String, String),
    Power(String, String),
    NamedFunction(String, FnArgs),
    Constant(f32),
    None,
}


/// Get the operation that is on the surface level of the string
fn split_surface_by_op(string: &str) -> ParseState {
    ParseState::None
}


// make the DivFn and CoefF functions
// make the substraction somehow better



pub fn parse_to_func(string: &str) -> ChildFn {
    let surface_level_op = split_surface_by_op(string);
    return AddFn::new(vec![]).to_child();
    // match surface_level_op {
    //     ParseState::Addition(v) => {
    //         return AddFn::new(
    //             v.into_iter()
    //                 .map(|x| parse_to_func(&x))
    //                 .collect()
    //         ).boxed();
    //     },
    //     ParseState::Multiplication(v) => {
    //         return MulFn::new(
    //             v.into_iter()
    //                 .map(|x| parse_to_func(&x))
    //                 .collect()
    //         ).boxed();
    //     },
    //     ParseState::Subtraction(a, b) => {},
    //     ParseState::Division(a, b) => {},
    //     ParseState::Power(a, b) => {},
    //     ParseState::NamedFunction(name, args) => {},
    //     ParseState::Constant(c) => return ConstF::new(c).boxed(),
    //     _ => {},
    // }
    // unreachable!();
}


#[cfg(test)]
mod tests {


    use crate::functions::*;
    use super::*;

    #[should_panic]
    #[test]
    fn test_func_struct() {
        let func = FunctionStruct {
            definition: CoefFn::new(2.0, "x".to_string()).to_child()
        };

        let mut args: FnArgs = FnArgs::new();
        args.insert("x".to_string(), 3.0);
        args.insert("y".to_string(), 25.0);
        let result = func.apply(args);

        assert_eq!(result, Ok(196.0));
    }
}
