use std::result;
use std::{collections::HashMap};

use crate::functions::*;
use crate::utilities::parser_utils::*;
use crate::utilities::function_utils::Boxed;



pub struct FunctionStruct {
    pub definition: BoxedFunction,
}

impl FunctionStruct { 
    pub fn from_str(string: &str) -> Self {
        let definition = parse_to_func(string);
        Self {
            definition
        }
    }


    /// Warning - this function may not be fast, use `get_closure()` instead
    pub fn apply(&self, args: FunctionArgs) -> f32 {
        self.definition.apply(args)
    }

    pub fn get_closure(&self) -> impl Fn(FunctionArgs) -> f32 {
        todo!();
        |x| 0.0
    }
}

impl Default for FunctionStruct {
    fn default() -> Self {
        Self {
            definition: ConstF::new(0.0).boxed()
        }
    }
}


#[cfg(test)]
mod tests {


    use crate::functions::*;
    use super::*;

    #[test]
    fn test_func_struct() {
        let func = FunctionStruct {
            definition: PowF {
                power: 2.0,
                variable: "".to_string(),
                child: Some(AddF {
                    children: vec![
                        ExpF {
                            base: 2.0,
                            variable: "x".to_string(),
                            child: None,
                        }.boxed(),
                        MulF {
                            children: vec![
                                ConstF::new(3.0).boxed(),
                                LogF {
                                    base: 5.0,
                                    variable: "y".to_string(),
                                    child: None,
                                }.boxed(),
                            ],
                        }.boxed(),
                    ]
                }.boxed()),
            }.boxed(),
        };
        let mut args: FunctionArgs = FunctionArgs::new();
        args.insert("x".to_string(), 3.0);
        args.insert("y".to_string(), 25.0);
        let result = func.apply(args);

        assert_eq!(result, 196.0);
    }
}
