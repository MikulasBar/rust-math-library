use std::collections::HashMap;

use crate::function::{
    Function,
    EvalError
};

use Child::*;



/// Type used for fields like `child` or `exponent` ... <br>
/// Can store function, variable or constant
#[derive(Debug, PartialEq, Clone)]
pub enum Child {
    Fn(Box<Function>),
    Var(String),
    Const(f64)
}


impl Child {
    // pub fn is_function(&self) -> bool {
    //     matches!(self, Fn(_))
    // }

    // pub fn is_var(&self) -> bool {
    //     matches!(self, Var(_))
    // }

    // pub fn is_const(&self) -> bool {
    //     matches!(self, Const(_))
    // }

    pub fn to_string(&self) -> String {
        match self {
            Const(c) => c.to_string(),
            Var(v) => v.to_string(),
            Fn(f) => f.to_string(),
        }
    }
}

impl<'a> Default for Child {
    fn default() -> Self {
        Const(0.0)
    }
}

impl Child {
    pub fn eval(&self, args: &HashMap<&str, f64>) -> Result<f64, EvalError> {
        match self {
            Fn(f) => f.eval(args),
            Const(c) => Ok(*c),
            Var(v) => {
                args.get(v.as_str()).copied()
                    .ok_or(EvalError::ParameterNotFound(v.clone()))
            },
        }
    }

    // fn substitute(&self, args: &HashMap<&str, Child>) -> Child {
    //     match self {
    //         Const(_) => self.clone(),
    //         Fn(f) => f.substitute(args),
    //         Var(v) => {
    //             if let Some(c) = args.get(v) {
    //                 return c.clone().to_child()
    //             }
    //             self.clone()
    //         },
    //     }
    // }

    // fn depends_on(&self, variable: &str) -> bool {
    //     match self {
    //         Fn(f) => f.depends_on(variable),
    //         Var(v) => **v == *variable,
    //         Const(_) => false,
    //     }
    // }

    pub fn derivative(&self, variable: &str) -> Child {
        match self {
            Fn(f) => f.derivative(variable),
            Const(_) => (0.0).to_child(),
            Var(v) => {
                match *v == *variable {
                    true => 1.0,
                    false => 0.0,
                }.to_child()
            }
        }
    }
}





/// Trait for converting a type to a Child <br>
/// Used instead of Into<Child> because Into cannot be used to convert FnBehaviour dynamic object
pub trait ToChild {
    fn to_child(self) -> Child;
}

impl ToChild for Function {
    #[inline]
    fn to_child(self) -> Child {
        Child::Fn(Box::new(self))
    }
}

impl ToChild for f64 {
    #[inline]
    fn to_child(self) -> Child {
        Child::Const(self)
    }
}

impl ToChild for &str {
    #[inline]
    fn to_child(self) -> Child {
        Child::Var(self.to_string())
    }
}

impl ToChild for String {
    #[inline]
    fn to_child(self) -> Child {
        Child::Var(self)
    }
}
