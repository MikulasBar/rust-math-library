use std::collections::HashMap;
use crate::child::{Child, ToChild};
use crate::fn_behaviour::{Function, EvalError};

use Function::*;

pub struct AddFn;

impl AddFn {
    pub fn eval(left: &Child, right: &Child, args: &HashMap<&str, f64>) -> Result<f64, EvalError> {
        let left = left.eval(args)?;
        let right = right.eval(args)?;
        Ok(left + right)
    }

    pub fn derivative<'a>(left: &'a Child, right: &'a Child, var: &str) -> Child<'a> {
        let left = left.derivative(var);
        let right = right.derivative(var);
        Add(&left, &right).to_child()
    }

    pub fn to_string(left: &Child, right: &Child) -> String {
        format!("({} + {})", left.to_string(), right.to_string())
    }
}


pub struct SubFn;

impl SubFn {
    pub fn eval(left: &Child, right: &Child, args: &HashMap<&str, f64>) -> Result<f64, EvalError> {
        let left = left.eval(args)?;
        let right = right.eval(args)?;
        Ok(left - right)
    }

    pub fn derivative<'a>(left: &'a Child, right: &'a Child, var: &str) -> Child<'a> {
        let d_left = left.derivative(var);
        let d_right = right.derivative(var);
        Sub(&d_left, &d_right).to_child()
    }

    pub fn to_string(left: &Child, right: &Child) -> String {
        format!("({} - {})", left.to_string(), right.to_string())
    }
}


pub struct MulFn;

impl MulFn {
    pub fn eval(left: &Child, right: &Child, args: &HashMap<&str, f64>) -> Result<f64, EvalError> {
        let left = left.eval(args)?;
        let right = right.eval(args)?;
        Ok(left * right)
    }

    pub fn derivative<'a>(left: &'a Child, right: &'a Child, var: &str) -> Child<'a> {
        let d_left = left.derivative(var);
        let d_right = right.derivative(var);

        let first_term = Mul(&d_left, right).to_child();
        let second_term = Mul(left, &d_right).to_child();

        Add(&first_term, &second_term).to_child()
    }

    pub fn to_string(left: &Child, right: &Child) -> String {
        format!("({} * {})", left.to_string(), right.to_string())
    }
}


pub struct DivFn;

impl DivFn {
    pub fn eval(num: &Child, den: &Child, args: &HashMap<&str, f64>) -> Result<f64, EvalError> {
        let num = num.eval(args)?;
        let den = den.eval(args)?;

        if den == 0.0 {
            return Err(EvalError::DivisionByZero)
        }
        Ok(num / den)
    }

    pub fn derivative<'a>(num: &'a Child, den: &'a Child, var: &str) -> Child<'a> {
        let d_num = num.derivative(var);
        let d_den = den.derivative(var);

        let left = Mul(&d_num, den).to_child();
        let right = Mul(num, &d_den).to_child();

        let res_num = Sub(&left, &right).to_child();
        let res_den = Exp(den, &2.0.to_child()).to_child();

        Div(&res_num, &res_den).to_child()
    }

    pub fn to_string(num: &Child, den: &Child) -> String {
        format!("({} / {})", num.to_string(), den.to_string())
    }
}

