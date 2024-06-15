use std::collections::HashMap;
use std::f64::consts::E;

use crate::{
    fn_behaviour::*,
    child::*,
};

use super::{AddFn, SubFn, MulFn, DivFn};

use EvalError::*;


///# Functionality
/// Struct that represents function that is exponentiation of two expressions
/// 
/// # Example
/// ```
/// let function = ExpFn::new("x", "y");
/// 
/// let args = hashmap!{
///     "x" => 2.0,
///     "y" => 3.0
/// };
/// 
/// let result = function.evaluate(&args).unwrap();
/// let expected = 8.0;
/// 
/// assert_eq!(result, expected);
#[derive(Clone)]
pub struct ExpFn {
    base: Child,
    exponent: Child
}

impl ExpFn {
    pub fn new<T, U>(base: T, exp: U) -> Self
    where
        T: ToChild,
        U: ToChild,
    {
        Self {
            base: base.to_child(),
            exponent: exp.to_child(),
        }
    }
}

impl FnBehaviour for ExpFn {
    fn clone_box(&self) -> Box<dyn FnBehaviour> {
        Box::new(self.clone())
    }

    fn evaluate(&self, args: &HashMap<&str, f64>) -> Result<f64, EvalError> {
        let base_value = self.base.evaluate(args)?;
        let exp_value = self.exponent.evaluate(args)?;

        if base_value < 0.0 && exp_value.fract() != 0.0 {
            return Err(NegativeBaseNonIntegerExponentError)
        }
        if base_value < 0.0 && exp_value.fract() > 0.0 {
            return Err(NegativeEvenRootError)
        }
        Ok(base_value.powf(exp_value))
    }

    fn substitute(&self, args: &HashMap<&str, Child>) -> Child {
        let base = self.base.substitute(args);
        let exp = self.exponent.substitute(args);

        Self::new(base, exp).to_child()
    }

    fn get_type(&self) -> FnType {
        FnType::Binary(&self.base, &self.exponent)
    }

    fn derivative(&self, variable: &str) -> Child {
        let d_exp = self.exponent.derivative(variable);
        let d_base = self.base.derivative(variable);

        let left_term = MulFn::new(
            d_exp,
            LogFn::new(E, self.base.clone())
        );

        let right_term = DivFn::new(
            MulFn::new(self.exponent.clone(), d_base), 
            self.base.clone()
        );
        
        let factor = AddFn::new(left_term, right_term);

        MulFn::new(self.clone(), factor).to_child()
    }
}


///# Functionality
/// Struct that represents function that is logarithm of expression with base
/// 
/// # Example
/// ```
/// let function = LogFn::new("x", "y");
/// 
/// let args = hashmap!{
///     "x" => 2.0
///     "y" => 8.0
/// };
/// 
/// let result = function.evaluate(&args).unwrap();
/// let expected = 3.0;
/// 
/// assert_eq!(result, expected);
/// ```
#[derive(Clone)]
pub struct LogFn {
    base: Child,
    argument: Child
}

impl LogFn {
    pub fn new<T, U>(base: T, arg: U) -> Self
    where
        T: ToChild,
        U: ToChild,
    {
        Self {
            base: base.to_child(),
            argument: arg.to_child(),
        }
    }
}

impl FnBehaviour for LogFn {
    fn clone_box(&self) -> Box<dyn FnBehaviour> {
        Box::new(self.clone())
    }

    fn evaluate(&self, args: &HashMap<&str, f64>) -> Result<f64, EvalError> {
        let base_value = self.base.evaluate(args)?;
        let arg_value = self.argument.evaluate(args)?;

        match (base_value, arg_value) {
            (_, a) if a <= 0.0 => Err(NonPositiveLogArgError),
            (b, _) if b <= 0.0 => Err(NonPositiveLogBaseError),
            (b, _) if b == 1.0 => Err(LogBaseOneError),
            _ => Ok(arg_value.log(base_value))
        }
    }

    fn substitute(&self, args: &HashMap<&str, Child>) -> Child {
        let base = self.base.substitute(args);
        let arg = self.argument.substitute(args);

        Self::new(base, arg).to_child()
    }

    fn get_type(&self) -> FnType {
        FnType::Binary(&self.base, &self.argument)
    }

    fn derivative(&self, variable: &str) -> Child {
        let d_base = self.base.derivative(variable);
        let d_arg = self.argument.derivative(variable);

        let left = DivFn::new(d_arg, self.argument.clone());
        let right = MulFn::new(
            self.clone(),
            DivFn::new(d_base, self.base.clone())
        );

        let factor = SubFn::new(left, right); 

        DivFn::new(
            factor,
            LogFn::new(E, self.base.clone())
        ).to_child()
    }
}





// Seq stands for sequence,
// it means the function has arbitrary number of children 

///# Functionality
/// Struct that represents function that is sum of arbitrary number of expressions
/// 
/// # Example
/// ```
/// let function = SeqAddFn::new(vec!["x", "y", "z"]);
/// 
/// let args = hashmap!{
///     "x" => 1.0,
///     "y" => 2.0,
///     "z" => 3.0
/// };
/// 
/// let result = function.evaluate(&args).unwrap();
/// let expected = 6.0;
/// 
/// assert_eq!(result, expected);
/// ```
#[derive(Clone)]
pub struct SeqAddFn {           
    children: Vec<Child>
}

impl SeqAddFn {
    /// Initialise new function with no children
    pub fn new<T>(children: Vec<T>) -> Self
    where 
        T: ToChild,
    { 
        Self {
            children: children
                .into_iter()
                .map(|c| c.to_child())
                .collect(),
        }
    }
}

impl FnBehaviour for SeqAddFn {
    fn clone_box(&self) -> Box<dyn FnBehaviour> {
        Box::new(self.clone())
    }

    fn evaluate(&self, args: &HashMap<&str, f64>) -> Result<f64, EvalError> {
        let mut result: f64 = 0.0;

        for child in &self.children {
            let child_result = child.evaluate(args)?;
            result += child_result;
        }
        Ok(result)
    }

    fn substitute(&self, args: &HashMap<&str, Child>) -> Child {
        let children: Vec<Child> = self.children.clone()
            .into_iter()
            .map(|c| c.substitute(args))
            .collect();

        Self::new(children).to_child()
    }

    fn get_type(&self) -> FnType {
        FnType::Variadic(&self.children)
    }

    fn derivative(&self, variable: &str) -> Child {
        let children: Vec<_> = self.children.clone()
            .into_iter()
            .map(|c| c.derivative(variable))
            .collect();

        SeqAddFn::new(children).to_child()
    }
}


///# Functionality
/// Struct that represents function that is product of arbitrary number of expressions
/// 
/// # Example
/// ```
/// let function = SeqMulFn::new(vec!["x", "y", "z"]);
/// 
/// let args = hashmap!{
///     "x" => 1.0,
///     "y" => 2.0,
///     "z" => 3.0
/// };
/// 
/// let result = function.evaluate(&args).unwrap();
/// let expected = 6.0;
/// 
/// assert_eq!(result, expected);
/// ```
#[derive(Clone)]
pub struct SeqMulFn {
    children: Vec<Child>
}

impl SeqMulFn {
    /// Initialise new function with no children
    pub fn new<T>(children: Vec<T>) -> Self
    where 
        T: ToChild,
    {
        Self {
            children: children
                .into_iter()
                .map(|c| c.to_child())
                .collect(),
        }
    }
}

impl FnBehaviour for SeqMulFn {
    fn clone_box(&self) -> Box<dyn FnBehaviour> {
        Box::new(self.clone())
    }

    fn evaluate(&self, args: &HashMap<&str, f64>) -> Result<f64, EvalError> {
        let mut result: f64 = 1.0;

        for child in &self.children {
            let child_result = child.evaluate(args)?;
            result *= child_result;
        }
        Ok(result)
    }

    fn substitute(&self, args: &HashMap<&str, Child>) -> Child {
        let children: Vec<Child> = self.children.clone()
            .into_iter()
            .map(|c| c.substitute(args))
            .collect();

        Self::new(children).to_child()
    }

    fn get_type(&self) -> FnType {
        FnType::Variadic(&self.children)
    }

    fn derivative(&self, variable: &str) -> Child {
        let terms: Vec<_> = self.children.clone()
            .into_iter()
            .map(|c|
                DivFn::new(
                    c.derivative(variable),
                    c
                )
            ).collect();

        MulFn::new(
            self.clone(),
            SeqAddFn::new(terms)
        ).to_child()
    }
}


