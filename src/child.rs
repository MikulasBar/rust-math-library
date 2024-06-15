use std::collections::HashMap;

use crate::fn_behaviour::{
    FnBehaviour,
    EvalError
};

use Child::*;



/// Type used for fields like `child` or `exponent` ... <br>
/// Can store function, variable or constant
pub enum Child {
    Fn(Box<dyn FnBehaviour>),
    Var(String),
    Const(f64)
}


impl Child {
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
}

impl Default for Child {
    fn default() -> Self {
        Const(0.0)
    }
}

impl Clone for Child {
    fn clone(&self) -> Self {
        match self {
            Fn(f) => Child::Fn(f.clone_box()),
            Var(v) => Child::Var(v.clone()),
            Const(c) => Child::Const(*c),
        }
    }
}

impl FnBehaviour for Child {
    fn clone_box(&self) -> Box<dyn FnBehaviour> {
        Box::new(self.clone())
    }

    fn evaluate(&self, args: &HashMap<&str, f64>) -> Result<f64, EvalError> {
        match self {
            Fn(f) => f.evaluate(args),
            Const(c) => Ok(*c),
            Var(v) => {
                match args.get(v.as_str()) {
                    Some(&value) => Ok(value),
                    _ => Err(EvalError::ParameterNotFoundError(v.clone()))
                }
            },
        }
    }

    fn substitute(&self, args: &HashMap<&str, Child>) -> Child {
        match self {
            Const(_) => self.clone(),
            Fn(f) => f.substitute(args),
            Var(v) => {
                if let Some(c) = args.get(&**v) {
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

    fn derivative(&self, variable: &str) -> Child {
        match self {
            Fn(f) => f.derivative(variable),
            Const(c) => 0.0.to_child(),
            Var(v) => {
                let mut value = 0.0;
                if **v == *variable {
                    value = 1.0;
                }
                value.to_child()
            }
        }
    }
}





/// Trait for converting a type to a Child <br>
/// Used instead of Into<Child> because Into cannot be used to convert FnBehaviour dynamic object
pub trait ToChild {
    fn to_child(self) -> Child;
}

impl<T: FnBehaviour + 'static> ToChild for T {
    fn to_child(self) -> Child {
        Child::Fn(Box::new(self))
    }
}

impl ToChild for f64 {
    fn to_child(self) -> Child {
        Child::Const(self)
    }
}

impl ToChild for &str {
    fn to_child(self) -> Child {
        Child::Var(self.to_string())
    }
}

impl ToChild for String {
    fn to_child(self) -> Child {
        Child::Var(self)
    }
}

impl<T: FnBehaviour + 'static> ToChild for Option<T> {
    fn to_child(self) -> Child {
        if let Some(res) = self {
            return res.to_child()
        }
        panic!("Cannot convert None to Child")
    }
}