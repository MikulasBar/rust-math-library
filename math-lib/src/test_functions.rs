#![allow(non_snake_case, unused)]

use crate::{fn_args, functions::*};
use crate::utilities::ToChildFn;
use std::f64::{
    consts::{FRAC_PI_2, FRAC_PI_4},
    EPSILON,
};



#[test]
fn test_AddFn() {
    let func = AddFn::new("x", "y");

    let args = fn_args!{
        "x" => 4.0,
        "y" => 5.0,
    };

    assert_eq!(func.evaluate(&args), Ok(9.0));
}

#[test]
fn test_MulFn() {
    let func = MulFn::new("x", "y");

    let args = fn_args!{
        "x" => 4.0,
        "y" => 5.0,
    };

    assert_eq!(func.evaluate(&args), Ok(20.0));
}

#[test]
fn test_DivFn() {
    let func = DivFn::new("x", "y");

    let args = fn_args!{
        "x" => 48.0,
        "y" => 4.0,
    };

    assert_eq!(func.evaluate(&args), Ok(12.0));
}

#[test]
fn test_CoefFn() {
    let func = CoefFn::new(5.0, "x");

    let args = fn_args!{
        "x" => 6.0,
    };

    assert_eq!(func.evaluate(&args), Ok(30.0));
}

#[test]
fn test_ExpFn() {
    let func = ExpFn::new("x", "y");

    let args = fn_args!{
        "x" => 5.0,
        "y" => 3.0,
    };

    assert_eq!(func.evaluate(&args), Ok(125.0));
}

#[test]
fn test_LogFn() {
    let func = LogFn::new("x", "y");

    let args = fn_args!{
        "x" => 2.0,
        "y" => 16.0,
    };

    assert_eq!(func.evaluate(&args), Ok(4.0));
}

#[test]
fn test_SinFn() {
    let func = SinFn::new("x");

    let args = fn_args!{
        "x" => FRAC_PI_2,
    };
    let value = func.evaluate(&args).unwrap() - 1f64;

    assert!(value.abs() <= EPSILON);
}

#[test]
fn test_CosFn() {
    let func = CosFn::new("x");

    let args = fn_args!{
        "x" => FRAC_PI_2,
    };
    let value = func.evaluate(&args).unwrap();

    assert!(value <= EPSILON);
}

#[test]
fn test_TanFn() {
    let func = TanFn::new("x");

    let args = fn_args!{
        "x" => FRAC_PI_4,
    };
    let value = func.evaluate(&args).unwrap() - 1f64;

    assert!(value.abs() <= EPSILON);
}
