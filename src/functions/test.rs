
use maplit::hashmap;
use std::f64::consts::{E, FRAC_PI_2, PI};

use crate::context::Context;

use super::{
    basic::*,
    trigonometric::*,
    advanced::*,
};

#[test]
fn add() {
    let func = AddFn::new("x", "y");
    let dfunc = func.derivative("x");

    let ctx: Context = hashmap!{
        "x" => 8.0,
        "y" => 6.0
    }.into();

    assert_eq!(func.eval(&ctx), Ok(14.0));
    assert_eq!(dfunc.eval(&ctx), Ok(1.0));
}

#[test]
fn sub() {
    let func = SubFn::new("x", "y");
    let dfunc = func.derivative("x");

    let ctx: Context = hashmap!{
        "x" => 8.0,
        "y" => 6.0
    }.into();

    assert_eq!(func.eval(&ctx), Ok(2.0));
    assert_eq!(dfunc.eval(&ctx), Ok(1.0));
}

#[test]
fn mul() {
    let func = MulFn::new("x", "y");
    let dfunc = func.derivative("x");

    let ctx: Context = hashmap!{
        "x" => 2.0,
        "y" => 3.0
    }.into();

    assert_eq!(func.eval(&ctx), Ok(6.0));
    assert_eq!(dfunc.eval(&ctx), Ok(3.0));
}

#[test]
fn div() {
    let func = DivFn::new("x", "y");
    let dfunc = func.derivative("x");

    let ctx: Context = hashmap!{
        "x" => 50.0,
        "y" => 10.0
    }.into();

    assert_eq!(func.eval(&ctx), Ok(5.0));
    assert_eq!(dfunc.eval(&ctx), Ok(0.1));
}

#[test]
fn exp() {
    let func = ExpFn::new("x", "y");
    let dfunc = func.derivative("x");

    let ctx: Context = hashmap!{
        "x" => 2.0,
        "y" => 5.0
    }.into();

    assert_eq!(func.eval(&ctx), Ok(32.0));
    assert_eq!(dfunc.eval(&ctx), Ok(80.0));
}

#[test]
fn log() {
    let func = LogFn::new("x", "y");
    let dfunc = func.derivative("x");

    let ctx: Context = hashmap!{
        "x" => E,
        "y" => E.powf(2.0)
    }.into();

    assert_eq!(func.eval(&ctx), Ok(2.0));
    assert_eq!(dfunc.eval(&ctx), Ok(-2.0/E));
}

#[test]
fn sin() {
    let func = SinFn::new("x");
    let dfunc = func.derivative("x");

    let ctx: Context = hashmap!{
        "x" => FRAC_PI_2,
    }.into();

    assert_eq!(func.eval(&ctx), Ok(1.0));
    assert_eq!(dfunc.eval(&ctx), Ok(0.0));
}

#[test]
fn cos() {
    let func = CosFn::new("x");
    let dfunc = func.derivative("x");

    let ctx: Context = hashmap!{
        "x" => FRAC_PI_2,
    }.into();

    assert_eq!(func.eval(&ctx), Ok(0.0));
    assert_eq!(dfunc.eval(&ctx), Ok(-1.0));
}

#[test]
fn tan() {
    let func = TanFn::new("x");
    let dfunc = func.derivative("x");

    let ctx: Context = hashmap!{
        "x" => PI
    }.into();

    assert_eq!(func.eval(&ctx), Ok(0.0));
    assert_eq!(dfunc.eval(&ctx), Ok(1.0));
}