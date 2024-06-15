use std::collections::HashMap;
use std::f64::consts::FRAC_PI_2;

use crate::{
    fn_behaviour::*,
    child::*,
    utils,
};

use super::{MulFn, DivFn, CoefFn, ExpFn};




///# Functionality
/// Struct that represents function that is sine of expression
/// 
/// # Example
/// ```
/// let function = SinFn::new("x");
/// 
/// let args = hashmap!{
///     "x" => 0.0
/// };
/// 
/// let result = function.evaluate(&args).unwrap();
/// let expected = 0.0;
/// 
/// assert_eq!(result, expected);
/// ```
#[derive(Clone)]
pub struct SinFn {
    child: Child
}

impl SinFn {
    pub fn new<T>(child: T) -> Self
    where 
        T: ToChild,
    {
        Self {
            child: child.to_child()
        }
    }
}

impl FnBehaviour for SinFn {
    fn clone_box(&self) -> Box<dyn FnBehaviour> {
        Box::new(self.clone())
    }

    fn evaluate(&self, args: &HashMap<&str, f64>) -> Result<f64, EvalError> {
        self.child
            .evaluate(args)
            .map(f64::sin)
            .map(utils::round)
    }

    fn substitute(&self, args: &HashMap<&str, Child>) -> Child {
        let child = self.child.substitute(args);
        Self::new(child).to_child()
    }

    fn get_type(&self) -> FnType {
        FnType::Unary(&self.child)
    }

    fn derivative(&self, variable: &str) -> Child {
        MulFn::new(
            CosFn::new(self.child.clone()),
            self.child.derivative(variable)
        ).to_child()
    }
}


///# Functionality
/// Struct that represents function that is cosine of expression
/// 
/// # Example
/// ```
/// let function = CosFn::new("x");
/// 
/// let args = hashmap!{
///     "x" => 0.0
/// };
/// 
/// let result = function.evaluate(&args).unwrap();
/// let expected = 1.0;
/// 
/// assert_eq!(result, expected);
/// ```
#[derive(Clone)]
pub struct CosFn {
    child: Child
}

impl CosFn {
    pub fn new<T>(child: T) -> Self
    where 
        T: ToChild,
    {
        Self {
            child: child.to_child()
        }
    }
}

impl FnBehaviour for CosFn {
    fn clone_box(&self) -> Box<dyn FnBehaviour> {
        Box::new(self.clone())
    }

    fn evaluate(&self, args: &HashMap<&str, f64>) -> Result<f64, EvalError> {
        self.child
            .evaluate(args)
            .map(f64::cos)
            .map(utils::round)
    }

    fn substitute(&self, args: &HashMap<&str, Child>) -> Child {
        let child = self.child.substitute(args);
        Self::new(child).to_child()
    }

    fn get_type(&self) -> FnType {
        FnType::Unary(&self.child)
    }

    fn derivative(&self, variable: &str) -> Child {
        MulFn::new(
            CoefFn::new(-1, SinFn::new(self.child.clone())),
            self.child.derivative(variable)
        ).to_child()
    }
}


///# Functionality
/// Struct that represents function that is tangent of expression
/// 
/// # Example
/// ```
/// let function = TanFn::new("x");
/// 
/// let args = hashmap!{
///     "x" => 0.0
/// };
/// 
/// let result = function.evaluate(&args).unwrap();
/// let expected = 0.0;
/// 
/// assert_eq!(result, expected);
/// ```
#[derive(Clone)]
pub struct TanFn {
    child: Child
}

impl TanFn {
    pub fn new<T>(child: T) -> Self
    where 
        T: ToChild,
    {
        Self {
            child: child.to_child()
        }
    }
}

impl FnBehaviour for TanFn {
    fn clone_box(&self) -> Box<dyn FnBehaviour> {
        Box::new(self.clone())
    }

    fn evaluate(&self, args: &HashMap<&str, f64>) -> Result<f64, EvalError> {
        let child_value = self.child.evaluate(args)?;

        if child_value == FRAC_PI_2 {
            return Err(EvalError::TanAtPiOverTwoError)
        }
        Ok(utils::round(child_value.tan()))
    }

    fn substitute(&self, args: &HashMap<&str, Child>) -> Child {
        let child = self.child.substitute(args);
        Self::new(child).to_child()
    }

    fn get_type(&self) -> FnType {
        FnType::Unary(&self.child)
    }

    fn derivative(&self, variable: &str) -> Child {
        let denom = ExpFn::new(CosFn::new(self.child.clone()), 2.0);

        DivFn::new(
            self.child.derivative(variable),
            denom
        ).to_child()
    }
}