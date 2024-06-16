use std::collections::HashMap;

use crate::fn_behaviour::{
    Function,
    EvalError
};

use Child::*;



/// Type used for fields like `child` or `exponent` ... <br>
/// Can store function, variable or constant
#[derive(Debug, PartialEq, Clone)]
pub enum Child<'a> {
    Fn(Function<'a>),
    Var(&'a str),
    Const(f64)
}


impl<'a> Child<'a> {
    ///# Functionality
    /// checks if `self` is function
    pub fn is_function(&self) -> bool {
        matches!(self, Fn(_))
    }

    ///# Functionality
    /// checks if `self` is variable
    pub fn is_var(&self) -> bool {
        matches!(self, Var(_))
    }

    ///# Functionality
    /// checks if `self` is constant
    pub fn is_const(&self) -> bool {
        matches!(self, Const(_))
    }

    pub fn to_string(&self) -> String {
        match self {
            Const(c) => c.to_string(),
            Var(v) => v.to_string(),
            Fn(f) => f.to_string(),
        }
    }
}

impl<'a> Default for Child<'a> {
    fn default() -> Self {
        Const(0.0)
    }
}

impl<'a> Child<'a> {
    pub fn eval(&self, args: &HashMap<&str, f64>) -> Result<f64, EvalError> {
        match self {
            Fn(f) => f.eval(args),
            Const(c) => Ok(*c),
            Var(v) => {
                args.get(v).copied()
                    .ok_or(EvalError::ParameterNotFoundError(v.to_string()))
            },
        }
    }

    fn substitute(&self, args: &HashMap<&str, Child>) -> Child {
        match self {
            Const(_) => self.clone(),
            Fn(f) => f.substitute(args),
            Var(v) => {
                if let Some(c) = args.get(v) {
                    return c.clone().to_child()
                }
                self.clone()
            },
        }
    }

    fn get_string_tree(&self) -> String {
        match self {
            Const(c) => c.to_string(),
            Var(v) => v.to_string(),
            Fn(f) => f.get_string_tree(),
        }
    }

    fn depends_on(&self, variable: &str) -> bool {
        match self {
            Fn(f) => f.depends_on(variable),
            Var(v) => **v == *variable,
            Const(_) => false,
        }
    }

    pub fn derivative(&self, variable: &str) -> Child {
        match self {
            Fn(f) => f.derivative(variable),
            Const(c) => (0.0).to_child(),
            Var(v) => {
                match **v == *variable {
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
    fn to_child<'a>(self) -> Child<'a>;
}

impl<'a> ToChild for Function<'a> {
    fn to_child<'b>(self) -> Child<'b> {
        Child::Fn(self)
    }
}

impl ToChild for f64 {
    fn to_child<'a>(self) -> Child<'a> {
        Child::Const(self)
    }
}

impl ToChild for &str {
    fn to_child<'a>(self) -> Child<'a> {
        Child::Var(self)
    }
}

impl ToChild for String {
    fn to_child<'a>(self) -> Child<'a> {
        Child::Var(self.as_str())
    }
}
