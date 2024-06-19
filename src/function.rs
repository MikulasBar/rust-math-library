use std::collections::HashMap;
use std::ops::RangeInclusive;
use maplit::hashmap;

use crate::child::*;
use std::f64::{EPSILON, consts::FRAC_PI_2};

use crate::functions::basic::*;
use crate::functions::trigonometric::*;
use crate::functions::advanced::*;



/// Enum with all errors that can occur during evaluating function <br>
/// contains only errors for elementary functions
#[derive(Debug, PartialEq)]
pub enum EvalError {
    DivisionByZero,

    /// Error that occurs when you try to take logarithm of non positive number
    NonPositiveLogArg,

    /// Error that occurs when you try to take logarithm with non positive base
    NonPositiveLogBase,

    LogBaseOne,

    /// Error that occurs when you try to take root of negative number with non integer exponent
    NegativeBaseNonIntegerExponent,

    /// Error that occurs when you try to exponentiate 0 to non positive number
    ZeroBaseNonPositiveExponent,

    TanAtPiOverTwo,

    /// Error that occurs when you try to evaluate parameter that doesn't exist
    ParameterNotFound(String),

    ZeroToZero,
}


// 'Function' enum with all possible functions
// this type is only for storing the data
// all methods except utility functions are implemented in separate files

// for every enum variant there is type (for example for 'Add' we have 'AddFn') that doesn't store data
// but has all methods that can be used on Add variant
// that includes new function that doesn't create instance of 'AddFn' but 'Function::Add' variant


#[derive(Debug, PartialEq, Clone)]
pub enum Function {
    Add(Child, Child),
    Sub(Child, Child),
    Mul(Child, Child),

    // first is numerator, second is denominator
    Div(Child, Child),

    // first is base, second is exponents
    Exp(Child, Child),
    // first is base, second is argument
    Log(Child, Child),

    Sin(Child),
    Cos(Child),
    Tan(Child),

    // Named {
    //     name: String,
    //     body: Child,
    // },
}


// utility functions
impl Function {
    /// Check if a number is zero <br>
    /// Uses `f64::EPSILON` as the threshold
    pub fn is_zero(num: f64) -> bool {
        num.abs() < EPSILON
    }

    /// Use to result from goniometric or other non-precise functions <br>
    /// Rounds only if the fractional part is less than [`f64::EPSILON`]
    pub fn round(num: f64) -> f64 {
        const SCALE_FACTOR: f64 = 1e15;

        let scaled_fract = num.fract() * SCALE_FACTOR;

        // this is part on the number that is less than EPSILON
        // we move all digits to the left by 15 places
        // and takes the fractional part
        // this will ensure that we extract only the small part of the number
        // then we divide it by SCALE_FACTOR to get the real value
        let small_part = scaled_fract.fract() / SCALE_FACTOR;

        // we round the number by removing the small part
        num - small_part
    }
}

impl Function {
    pub fn eval(&self, args: &HashMap<&str, f64>) -> Result<f64, EvalError> {
        use Function::*;

        match self {
            Add(a, b) =>    AddFn::eval(a, b, args),
            Sub(a, b) =>    SubFn::eval(a, b, args),
            Mul(a, b) =>    MulFn::eval(a, b, args),
            Div(a, b) =>    DivFn::eval(a, b, args),
            Exp(a, b) =>    ExpFn::eval(a, b, args),
            Log(a, b) =>    LogFn::eval(a, b, args),
            Sin(c) =>       SinFn::eval(c, args),
            Cos(c) =>       CosFn::eval(c, args),
            Tan(c) =>       TanFn::eval(c, args),
        }
    }

    pub fn derivative(&self, var: &str) -> Child {
        use Function::*;

        match self {
            Add(a, b) =>    AddFn::derivative(a, b, var),
            Sub(a, b) =>    SubFn::derivative(a, b, var),
            Mul(a, b) =>    MulFn::derivative(a, b, var),
            Div(a, b) =>    DivFn::derivative(a, b, var),
            Exp(a, b) =>    ExpFn::derivative(a, b, var),
            Log(a, b) =>    LogFn::derivative(a, b, var),
            Sin(c) =>       SinFn::derivative(c, var),
            Cos(c) =>       CosFn::derivative(c, var),
            Tan(c) =>       TanFn::derivative(c, var),
        }
    }

    pub fn to_string(&self) -> String {
        use Function::*;

        match self {
            Add(a, b) =>    AddFn::to_string(a, b),
            Sub(a, b) =>    SubFn::to_string(a, b),
            Mul(a, b) =>    MulFn::to_string(a, b),
            Div(a, b) =>    DivFn::to_string(a, b),
            Exp(a, b) =>    ExpFn::to_string(a, b),
            Log(a, b) =>    LogFn::to_string(a, b),
            Sin(c) =>       SinFn::to_string(c),
            Cos(c) =>       CosFn::to_string(c),
            Tan(c) =>       TanFn::to_string(c),
        }
    }
}