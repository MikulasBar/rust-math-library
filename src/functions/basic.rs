use std::collections::HashMap;

use crate::{
    child::*,
    fn_behaviour::*,
};

use super::ExpFn;


///# Functionality
/// Struct that represents function that is sum of only two expressions
/// 
///# Example
/// ```
/// let function = AddFn::new("x", "y");
/// 
/// let args = hashmap!{
///     "x" => 2.0,
///     "y" => 3.0
/// };
/// 
/// let result = function.evaluate(&args).unwrap();
/// let expected = 5.0;
/// 
/// assert_eq!(result, expected);
/// ```
#[derive(Clone)]
pub struct AddFn {
    left: Child,
    right: Child
}

impl AddFn {
    pub fn new<T, U>(left: T, right: U) -> Self
    where
        T: ToChild,
        U: ToChild
    {
        Self {
            left: left.to_child(),
            right: right.to_child()
        }
    }
}

impl FnBehaviour for AddFn {
    fn clone_box(&self) -> Box<dyn FnBehaviour> {
        Box::new(self.clone())
    }

    fn evaluate(&self, args: &HashMap<&str, f64>) -> Result<f64, EvalError> {
        let left = self.left.evaluate(args)?;
        let right = self.right.evaluate(args)?;

        Ok(left + right)
    }

    fn substitute(&self, args: &HashMap<&str, Child>) -> Child {
        let left = self.left.substitute(args);
        let right = self.right.substitute(args);

        Self::new(left, right).to_child()
    }

    fn get_type(&self) -> FnType {
        FnType::Binary(&self.left, &self.right)
    }

    fn derivative(&self, variable: &str) -> Child {
        let left = self.left.derivative(variable);
        let right = self.right.derivative(variable);

        AddFn::new(left, right).to_child()
    }
}


///# Functionality
/// Struct that represents function that is difference of only two expressions
/// 
/// # Example
/// ```
/// let function = SubFn::new("x", "y");
/// 
/// let args = hashmap!{
///     "x" => 2.0,
///     "y" => 3.0
/// };
/// 
/// let result = function.evaluate(&args).unwrap();
/// let expected = -1.0;
/// 
/// assert_eq!(result, expected);
/// ```
#[derive(Clone)]
pub struct SubFn {
    left: Child,
    right: Child
}

impl SubFn {
    pub fn new<T, U>(left: T, right: U) -> Self
    where
        T: ToChild,
        U: ToChild
    {
        Self {
            left: left.to_child(),
            right: right.to_child()
        }
    }
}

impl FnBehaviour for SubFn {
    fn clone_box(&self) -> Box<dyn FnBehaviour> {
        Box::new(self.clone())
    }

    fn evaluate(&self, args: &HashMap<&str, f64>) -> Result<f64, EvalError> {
        let left = self.left.evaluate(args)?;
        let right = self.right.evaluate(args)?;

        Ok(left - right)
    }

    fn substitute(&self, args: &HashMap<&str, Child>) -> Child {
        let left = self.left.substitute(args);
        let right = self.right.substitute(args);

        Self::new(left, right).to_child()
    }

    fn get_type(&self) -> FnType {
        FnType::Binary(&self.left, &self.right)
    }

    fn derivative(&self, variable: &str) -> Child {
        let left = self.left.derivative(variable);
        let right = self.right.derivative(variable);

        SubFn::new(left, right).to_child()
    }
}


///# Functionality
/// Struct that represents function that is product of only two expressions
/// 
/// # Example
/// ```
/// let function = MulFn::new("x", "y");
/// 
/// let args = hashmap!{
///     "x" => 2.0,
///     "y" => 3.0
/// };
/// 
/// let result = function.evaluate(&args).unwrap();
/// let expected = 6.0;
/// 
/// assert_eq!(result, expected);
/// ```
#[derive(Clone)]
pub struct MulFn {
    left: Child,
    right: Child
}

impl MulFn {
    pub fn new<T, U>(left: T, right: U) -> Self
    where
        T: ToChild,
        U: ToChild
    {
        Self {
            left: left.to_child(),
            right: right.to_child()
        }
    }
}

impl FnBehaviour for MulFn {
    fn clone_box(&self) -> Box<dyn FnBehaviour> {
        Box::new(self.clone())
    }

    fn evaluate(&self, args: &HashMap<&str, f64>) -> Result<f64, EvalError> {
        let left = self.left.evaluate(args)?;
        let right = self.right.evaluate(args)?;

        Ok(left * right)
    }

    fn substitute(&self, args: &HashMap<&str, Child>) -> Child {
        let left = self.left.substitute(args);
        let right = self.right.substitute(args);

        Self::new(left, right).to_child()
    }

    fn get_type(&self) -> FnType {
        FnType::Binary(&self.left, &self.right)
    }

    fn derivative(&self, variable: &str) -> Child {
        let d_left = self.left.derivative(variable);
        let d_right = self.right.derivative(variable);

        let result_left = MulFn::new(
            d_left,
            self.right.clone()
        );

        let result_right = MulFn::new(
            d_right,
            self.left.clone()
        );

        AddFn::new(result_left, result_right).to_child()
    }
}


///# Functionality
/// Struct that represents function that is division of only two expressions
/// 
/// # Example
/// ```
/// let function = DivFn::new("x", "y");
/// 
/// let args = hashmap!{
///     "x" => 2.0,
///     "y" => 3.0
/// };
/// 
/// let result = function.evaluate(&args).unwrap();
/// let expected = 2.0 / 3.0;
/// 
/// assert_eq!(result, expected);
/// ```
#[derive(Clone)]
pub struct DivFn {
    numerator: Child,
    denominator: Child
}

impl DivFn {
    pub fn new<T, U>(num: T, denom: U) -> Self
    where 
        T: ToChild,
        U: ToChild,
    {
        Self {
            numerator: num.to_child(),
            denominator: denom.to_child(),
        }
    }
}

impl FnBehaviour for DivFn {
    fn clone_box(&self) -> Box<dyn FnBehaviour> {
        Box::new(self.clone())
    }

    fn evaluate(&self, args: &HashMap<&str, f64>) -> Result<f64, EvalError> {
        let num_value = self.numerator.evaluate(args)?;
        let den_value = self.denominator.evaluate(args)?;

        if den_value == 0.0 {
            return Err(EvalError::DivisionByZeroError)
        }
        Ok(num_value / den_value)
    }

    fn substitute(&self, args: &HashMap<&str, Child>) -> Child {
        let num = self.numerator.substitute(args);
        let denom = self.denominator.substitute(args);

        Self::new(num, denom).to_child()
    }

    fn get_type(&self) -> FnType {
        FnType::Binary(&self.numerator, &self.denominator)
    }

    fn derivative(&self, variable: &str) -> Child {
        let d_num = self.numerator.derivative(variable);
        let d_denom = self.denominator.derivative(variable);

        let num_first = MulFn::new(
            self.denominator.clone(),
            d_num
        );

        let num_second = MulFn::new(
            self.numerator.clone(),
            d_denom
        );


        let result_num = SubFn::new(
            num_first,
            num_second
        );

        let result_denom = ExpFn::new(
            self.denominator.clone(),
            2.0
        );

        DivFn::new(result_num, result_denom).to_child()
    }
}


///# Functionality
/// Struct that represents function that has coefficient in front of it
/// 
/// # Example
/// ```
/// let function = CoefFn::new(2.0, "x");
/// 
/// let args = hashmap!{
///     "x" => 3.0
/// };
/// 
/// let result = function.evaluate(&args).unwrap();
/// let expected = 6.0;
/// 
/// assert_eq!(result, expected);
/// ```
#[derive(Clone)]
pub struct CoefFn {
    coefficient: f64,
    child: Child
}

impl CoefFn {
    pub fn new<C, F>(coeff: C, child: F) -> Self
    where
        C: Into<f64>,
        F: ToChild,
    {
        Self {
            coefficient: coeff.into(),
            child: child.to_child()
        }
    }
}

impl FnBehaviour for CoefFn {
    fn clone_box(&self) -> Box<dyn FnBehaviour> {
        Box::new(self.clone())
    }

    fn evaluate(&self, args: &HashMap<&str, f64>) -> Result<f64, EvalError> {
        let child_value = self.child.evaluate(args)?;

        Ok(self.coefficient * child_value)
    }

    fn substitute(&self, args: &HashMap<&str, Child>) -> Child {
        let coef = self.coefficient.clone();
        let child = self.child.substitute(args);

        Self::new(coef, child).to_child()
    }

    fn depends_on(&self, variable: &str) -> bool {
        self.child.depends_on(variable)
    }

    fn derivative(&self, variable: &str) -> Child {
        if self.child.depends_on(variable) {
            return CoefFn::new(
                self.coefficient,
                self.child.derivative(variable)
            ).to_child()
        }
        0.0.to_child()
    }
}
