#![allow(non_snake_case, unused)]

use maplit::hashmap;
use std::{
    f64::{
        consts::{E, FRAC_PI_2, PI},
        EPSILON
    },
    collections::HashMap,
};

use crate::functions::*;

#[test]
fn test_AddFn() {
    let func = AddFn::new("x", "y");
    let dfunc = func.derivative("x");

    let args = hashmap!{
        "x" => 8.0,
        "y" => 6.0
    };

    assert_eq!(func.evaluate(&args), Ok(14.0));
    assert_eq!(dfunc.evaluate(&args), Ok(1.0));
}

#[test]
fn test_SubFn() {
    let func = SubFn::new("x", "y");
    let dfunc = func.derivative("x");

    let args = hashmap!{
        "x" => 8.0,
        "y" => 6.0
    };

    assert_eq!(func.evaluate(&args), Ok(2.0));
    assert_eq!(dfunc.evaluate(&args), Ok(1.0));
}

#[test]
fn test_MulFn() {
    let func = MulFn::new("x", "y");
    let dfunc = func.derivative("x");

    let args = hashmap!{
        "x" => 2.0,
        "y" => 3.0
    };

    assert_eq!(func.evaluate(&args), Ok(6.0));
    assert_eq!(dfunc.evaluate(&args), Ok(3.0));
}

#[test]
fn test_DivFn() {
    let func = DivFn::new("x", "y");
    let dfunc = func.derivative("x");

    let args = hashmap!{
        "x" => 50.0,
        "y" => 10.0
    };

    assert_eq!(func.evaluate(&args), Ok(5.0));
    assert_eq!(dfunc.evaluate(&args), Ok(0.1));
}

#[test]
fn test_CoefFn() {
    let func = CoefFn::new(5.0, "x");
    let dfunc = func.derivative("x");

    let args = hashmap!{
        "x" => 6.0
    };

    assert_eq!(func.evaluate(&args), Ok(30.0));
    assert_eq!(dfunc.evaluate(&args), Ok(5.0));
}

#[test]
fn test_ExpFn() {
    let func = ExpFn::new("x", "y");
    let dfunc = func.derivative("x");

    let args = hashmap!{
        "x" => 2.0,
        "y" => 5.0
    };

    assert_eq!(func.evaluate(&args), Ok(32.0));
    assert_eq!(dfunc.evaluate(&args), Ok(80.0));
}

#[test]
fn test_LogFn() {
    let func = LogFn::new("x", "y");
    let dfunc = func.derivative("x");

    let args = hashmap!{
        "x" => E,
        "y" => E.powf(2.0)
    };

    assert_eq!(func.evaluate(&args), Ok(2.0));
    assert_eq!(dfunc.evaluate(&args), Ok(-2.0/E));
}

#[test]
fn test_SinFn() {
    let func = SinFn::new("x");
    let dfunc = func.derivative("x");

    let args = hashmap!{
        "x" => FRAC_PI_2,
    };

    let result = func.evaluate(&args).unwrap();
    let d_result = dfunc.evaluate(&args).unwrap();

    assert!((result - 1.0).abs() <= EPSILON);
    assert!(d_result.abs() <= EPSILON);
}

#[test]
fn test_CosFn() {
    let func = CosFn::new("x");
    let dfunc = func.derivative("x");

    let args = hashmap!{
        "x" => FRAC_PI_2,
    };

    let result = func.evaluate(&args).unwrap();
    let d_result = dfunc.evaluate(&args).unwrap();

    assert!(result.abs() <= EPSILON);
    assert!((d_result + 1.0).abs() <= EPSILON);
}

#[test]
fn test_TanFn() {
    let func = TanFn::new("x");
    let dfunc = func.derivative("x");

    let args = hashmap!{
        "x" => PI
    };

    let result = func.evaluate(&args).unwrap();
    let d_result = dfunc.evaluate(&args).unwrap();

    assert!(result.abs() <= EPSILON);
    assert!((d_result - 1.0).abs() <= EPSILON);
}

#[test]
fn test_SeqAddFn() {
    let func = SeqAddFn::new(vec!["x", "y", "z", "x"]);
    let dfunc = func.derivative("x");

    let args = hashmap!{
        "x" => 1.0,
        "y" => 5.0,
        "z" => 3.0
    };

    assert_eq!(func.evaluate(&args), Ok(10.0));
    assert_eq!(dfunc.evaluate(&args), Ok(2.0));
}

#[test]
fn test_SeqMulFn() {
    let func = SeqMulFn::new(vec!["x", "y", "z", "x"]);
    let dfunc = func.derivative("x");

    let args = hashmap!{
        "x" => 2.0,
        "y" => 5.0,
        "z" => 3.0
    };

    assert_eq!(func.evaluate(&args), Ok(60.0));
    assert_eq!(dfunc.evaluate(&args), Ok(60.0));
}