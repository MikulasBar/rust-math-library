use std::collections::HashMap;
use std::f64::consts::FRAC_PI_2;

use crate::{
    function::*,
    child::*,
    utils,
};

use super::basic::{MulFn, DivFn};

use crate::function::Function::*;
use crate::unary_new;


pub struct SinFn;

impl SinFn {
    unary_new!{Sin, child}

    pub fn eval(child: &Child, args: &HashMap<&str, f64>) -> Result<f64, EvalError> {
        child.eval(args)
            .map(f64::sin)
            .map(utils::round)
    }

    // fn substitute(&self, args: &HashMap<&str, Child>) -> Child {
    //     let child = self.child.substitute(args);
    //     Self::new(child).to_child()
    // }

    // fn get_type(&self) -> FnType {
    //     FnType::Unary(&self.child)
    // }

    pub fn derivative(child: &Child, var: &str) -> Child {
        let d_child = child.derivative(var);
        let cos = Cos(child.clone()).to_child();

        Mul(cos, d_child).to_child()
    }

    pub fn to_string(child: &Child) -> String {
        format!("sin{}", child.to_string())
    }
}



pub struct CosFn;

impl CosFn {
    unary_new!{Cos, child}

    pub fn eval(child: &Child, args: &HashMap<&str, f64>) -> Result<f64, EvalError> {
        child.eval(args)
            .map(f64::cos)
            .map(utils::round)
    }

    // fn substitute(&self, args: &HashMap<&str, Child>) -> Child {
    //     let child = self.child.substitute(args);
    //     Self::new(child).to_child()
    // }

    // fn get_type(&self) -> FnType {
    //     FnType::Unary(&self.child)
    // }

    pub fn derivative(child: &Child, var: &str) -> Child {
        let d_child = child.derivative(var);

        let coef = (-1.0).to_child();

        let child_copy = child.clone();
        let sin = Sin(child_copy).to_child();

        let d_coef = Mul(coef, d_child).to_child();

        Mul(d_coef, sin).to_child()
    }

    pub fn to_string(child: &Child) -> String {
        format!("cos{}", child.to_string())
    }
}


pub struct TanFn;

impl TanFn {
    unary_new!{Tan, child}

    pub fn eval(child: &Child, args: &HashMap<&str, f64>) -> Result<f64, EvalError> {
        let child = child.eval(args)?;

        if child == FRAC_PI_2 {
            return Err(EvalError::TanAtPiOverTwo)
        }
        Ok(utils::round(child.tan()))
    }

    // fn substitute(&self, args: &HashMap<&str, Child>) -> Child {
    //     let child = self.child.substitute(args);
    //     Self::new(child).to_child()
    // }

    // fn get_type(&self) -> FnType {
    //     FnType::Unary(&self.child)
    // }

    pub fn derivative(child: &Child, var: &str) -> Child {
        let d_child = child.derivative(var);

        let child_copy = child.clone();
        let den = Cos(child_copy).to_child();

        let ratio = Div(d_child, den).to_child();
        let pow = 2.0.to_child();

        Exp(ratio, pow).to_child()
    }

    pub fn to_string(child: &Child) -> String {
        format!("tan{}", child.to_string())
    }
}