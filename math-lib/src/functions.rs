use std::{
    collections::HashMap,
};
use crate::{
    impl_sequence_func,
};
use ChildFn::*;
use FnError::*;


pub type FnResult = Result<f32, FnError>;


pub type FnArgs = HashMap<String, f32>;
pub trait Function {
    fn apply(&self, args: &FnArgs) -> FnResult;
    //fn derivative(&self) -> Self;
}


#[derive(Debug)]
pub enum FnError {
    DivisionByZeroError,
    NegativeEvenRootError,
    NonPositiveLogArgError,
    NonPositiveLogBaseError,
    LogBaseOneError,
    NegativeBaseNonIntegerExponentError,

    ParameterNotFoundError,
}

impl PartialEq for FnError {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}


/// Type used for fields like `child` or `exponent` ... 
pub enum ChildFn {
    Fn(Box<dyn Function>),
    Var(String),
    Const(f32)
}

impl Function for ChildFn {
    fn apply(&self, args: &FnArgs) -> FnResult {
        match self {
            Fn(f) => f.apply(args),
            Var(ref s) => {
                match args.get(s) {
                    Some(&v) => Ok(v),
                    _ => Err(ParameterNotFoundError)
                }
            },
            Const(c) => Ok(*c)
        }
    }
}


impl Into<ChildFn> for String {
    fn into(self) -> ChildFn {
        Var(self)
    }
}

impl Into<ChildFn> for &str {
    fn into(self) -> ChildFn {
        Var(self.to_string())
    }
}

impl Into<ChildFn> for f32 {
    fn into(self) -> ChildFn {
        Const(self)
    }
}



pub struct AddFn {           
    pub children: Vec<ChildFn>
}

impl_sequence_func!(AddFn);

impl Function for AddFn {
    fn apply(&self, args: &FnArgs) -> FnResult {
        let mut result: f32 = 0.0;

        for child in &self.children {
            let child_result = child.apply(args);
            match child_result {
                Ok(v) => result += v,
                e => return e,
            }
        }
        Ok(result)
    }
}



pub struct MulFn {
    pub children: Vec<ChildFn>
}

impl_sequence_func!(MulFn);

impl Function for MulFn {
    fn apply(&self, args: &FnArgs) -> FnResult {
        let mut result: f32 = 1.0;

        for child in &self.children {
            let child_result = child.apply(args);
            match child_result {
                Ok(v) => result *= v,
                e => return e,
            }
        }
        Ok(result)
    }
}


pub struct DivFn {
    pub numerator: ChildFn,
    pub denominator: ChildFn
}

impl DivFn {
    pub fn new<T: Into<ChildFn>>(num: T, denom: T) -> Self {
        Self {
            numerator: num.into(),
            denominator: denom.into(),
        }
    }
}

impl Function for DivFn {
    fn apply(&self, args: &FnArgs) -> FnResult {
        let num_result = self.numerator.apply(args);
        let den_result = self.denominator.apply(args);

        // apply_on_result2(num_result, den_result, |n, d| {
        //     if d == 0.0 {
        //         Err(())
        //     } else {
        //         Ok(n / d)
        //     }
        // })

        match (num_result, den_result) {
            (Ok(n), Ok(d)) => {
                if d == 0.0 {
                    Err(DivisionByZeroError)
                } else {
                    Ok(n / d)
                }
            },
            (Err(e), _) | (_, Err(e)) => Err(e),
        }
    }
}



/// This function is used for adding coefficient without using `MulFn` <br>
pub struct CoefFn {
    pub coefficient: f32,
    pub child: ChildFn
}

impl CoefFn {
    pub fn new(coefficient: f32, child: impl Into<ChildFn>) -> Self {
        Self {
            coefficient,
            child: child.into()
        }
    }
}

impl Function for CoefFn {
    fn apply(&self, args: &FnArgs) -> FnResult {
        let child_result = self.child.apply(args);

        match child_result {
            Ok(v) => Ok(self.coefficient * v),
            e => e,
        }
    }
}

pub struct ExpFn {
    pub base: ChildFn,
    pub exponent: ChildFn
}

impl ExpFn {
    pub fn new<T, U>(base: T, exp: U) -> Self
    where
        T: Into<ChildFn>,
        U: Into<ChildFn>,
    {
        Self {
            base: base.into(),
            exponent: exp.into(),
        }
    }
}

impl Function for ExpFn {
    fn apply(&self, args: &FnArgs) -> FnResult {
        let base_result = self.base.apply(args);
        let exp_result = self.exponent.apply(args);

        match (base_result, exp_result) {
            (Ok(b), Ok(n)) => {
                if b < 0.0 && n.fract() != 0.0 {
                    Err(NegativeBaseNonIntegerExponentError)
                } else {
                    Ok(b.powf(n))
                }
            },
            (Err(e), _) | (_, Err(e)) => Err(e),
        }
    }
}



pub struct LogFn {
    pub base: ChildFn,
    pub argument: ChildFn
}

impl LogFn {
    pub fn new<T, U>(base: T, arg: U) -> Self
    where
        T: Into<ChildFn>,
        U: Into<ChildFn>,
    {
        Self {
            base: base.into(),
            argument: arg.into(),
        }
    }
}

impl Function for LogFn {
    fn apply(&self, args: &FnArgs) -> FnResult {
        let base_result = self.base.apply(args);
        let arg_result = self.argument.apply(args);

        match (base_result, arg_result) {
            (Ok(b), Ok(a)) => {
                if a <= 0.0 {
                    Err(NonPositiveLogArgError)
                } else if b <= 0.0 {
                    Err(NonPositiveLogBaseError)
                } else {
                    Ok(a.log(b))
                }
            },
            (Err(e), _) | (_, Err(e)) => Err(e),
        }
    }
}


#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::utilities::function_utils::{
        ToChildFn, ToFnArgs
    };


    #[test]
    fn test_AddFn() {
        let fx = CoefFn::new(2.0, "x").to_child();
        let fy = CoefFn::new(3.0, "y").to_child();

        let add_func = AddFn::new(vec![fx, fy]);

        let args: FnArgs = vec![
            ("x", 4.0),
            ("y", 5.0),
        ].to_fn_args();

        assert_eq!(add_func.apply(&args), Ok(23.0));
    }

    #[test]
    fn test_MulFn() {
        let fx = CoefFn::new(2.0, "x").to_child();
        let fy = CoefFn::new(3.0, "y").to_child();

        let mul_func = MulFn::new(vec![fx, fy]);

        let args: FnArgs = vec![
            ("x", 4.0),
            ("y", 5.0),
        ].to_fn_args();

        assert_eq!(mul_func.apply(&args), Ok(120.0));
    }

    #[test]
    fn test_DivFn() {
        let div_func = DivFn::new("x", "y");

        let args: FnArgs = vec![
            ("x", 48.0),
            ("y", 4.0),
        ].to_fn_args();

        assert_eq!(div_func.apply(&args), Ok(12.0));
    }

    #[test]
    fn test_CoefFn() {
        let coef_func = CoefFn::new(5.0, "x");

        let args: FnArgs = vec![
            ("x", 6.0),
        ].to_fn_args();

        assert_eq!(coef_func.apply(&args), Ok(30.0));
    }

    #[test]
    fn test_ExpFn() {
        let exp_func = ExpFn::new("x", "y");

        let args: FnArgs = vec![
            ("x", 5.0),
            ("y", 3.0),
        ].to_fn_args();

        assert_eq!(exp_func.apply(&args), Ok(125.0));
    }

    #[test]
    fn test_LogFn() {
        let log_func = LogFn::new("x", "y");

        let args: FnArgs = vec![
            ("x", 2.0),
            ("y", 16.0),
        ].to_fn_args();

        assert_eq!(log_func.apply(&args), Ok(4.0));
    }
}