#![allow(dead_code)]

use std::collections::HashMap;
use crate::child::Child;
use crate::function::{Function, EvalError};

use Function::*;

use crate::binary_new;
use crate::_context::Context;


pub struct AddFn;

impl AddFn {
    binary_new!{Add, left, right}

    pub fn eval(left: &Child, right: &Child, ctx: &Context) -> Result<f64, EvalError> {
        let left = left.eval(ctx)?;
        let right = right.eval(ctx)?;

        Ok(left + right)
    }

    pub fn derivative(left: &Child, right: &Child, var: &str) -> Child {
        let left = left.derivative(var);
        let right = right.derivative(var);
        Add(left, right).into()
    }

    pub fn to_string(left: &Child, right: &Child) -> String {
        format!("({} + {})", left.to_string(), right.to_string())
    }
}


pub struct SubFn;

impl SubFn {
    binary_new!{Sub, left, right}

    pub fn eval(left: &Child, right: &Child, ctx: &Context) -> Result<f64, EvalError> {
        let left = left.eval(ctx)?;
        let right = right.eval(ctx)?;

        Ok(left - right)
    }

    pub fn derivative(left: &Child, right: &Child, var: &str) -> Child {
        let d_left = left.derivative(var);
        let d_right = right.derivative(var);
        Sub(d_left, d_right).into()
    }

    pub fn to_string(left: &Child, right: &Child) -> String {
        format!("({} - {})", left.to_string(), right.to_string())
    }
}


pub struct MulFn;

impl MulFn {
    binary_new!{Mul, left, right}

    pub fn eval(left: &Child, right: &Child, ctx: &Context) -> Result<f64, EvalError> {
        let left = left.eval(ctx)?;
        let right = right.eval(ctx)?;
        Ok(left * right)
    }

    pub fn derivative(left: &Child, right: &Child, var: &str) -> Child {
        let d_left = left.derivative(var);
        let d_right = right.derivative(var);

        let first_term = Mul(d_left, right.clone()).into();
        let second_term = Mul(left.clone(), d_right).into();

        Add(first_term, second_term).into()
    }

    pub fn to_string(left: &Child, right: &Child) -> String {
        format!("({} * {})", left.to_string(), right.to_string())
    }
}


pub struct DivFn;

impl DivFn {
    binary_new!{Div, num, den}

    pub fn eval(num: &Child, den: &Child, ctx: &Context) -> Result<f64, EvalError> {
        let num = num.eval(ctx)?;
        let den = den.eval(ctx)?;

        if den == 0.0 {
            return Err(EvalError::DivisionByZero)
        }
        Ok(num / den)
    }

    pub fn derivative(num: &Child, den: &Child, var: &str) -> Child {
        let d_num = num.derivative(var);
        let d_den = den.derivative(var);

        let left = Mul(d_num, den.clone()).into();
        let right = Mul(num.clone(), d_den).into();

        let res_num = Sub(left, right).into();
        let res_den = Exp(den.clone(), 2.0.into()).into();

        Div(res_num, res_den).into()
    }

    pub fn to_string(num: &Child, den: &Child) -> String {
        format!("({} / {})", num.to_string(), den.to_string())
    }
}

