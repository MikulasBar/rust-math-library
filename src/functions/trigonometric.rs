use std::f64::consts::FRAC_PI_2;

use crate::_context::Context;
use crate::{
    function::*,
    child::*,
};


use crate::function::Function::*;
use crate::unary_new;


pub struct SinFn;

impl SinFn {
    unary_new!{Sin, child}

    pub fn eval(child: &Child, ctx: &Context) -> Result<f64, EvalError> {
        child.eval(ctx)
            .map(f64::sin)
            .map(Function::round)
    }

    pub fn derivative(child: &Child, var: &str) -> Child {
        let d_child = child.derivative(var);
        let cos = Cos(child.clone()).into();

        Mul(cos, d_child).into()
    }

    pub fn to_string(child: &Child) -> String {
        format!("sin{}", child.to_string())
    }
}



pub struct CosFn;

impl CosFn {
    unary_new!{Cos, child}

    pub fn eval(child: &Child, ctx: &Context) -> Result<f64, EvalError> {
        child.eval(ctx)
            .map(f64::cos)
            .map(Function::round)
    }

    pub fn derivative(child: &Child, var: &str) -> Child {
        let d_child = child.derivative(var);

        let coef = (-1.0).into();

        let child_copy = child.clone();
        let sin = Sin(child_copy).into();

        let d_coef = Mul(coef, d_child).into();

        Mul(d_coef, sin).into()
    }

    pub fn to_string(child: &Child) -> String {
        format!("cos{}", child.to_string())
    }
}


pub struct TanFn;

impl TanFn {
    unary_new!{Tan, child}

    pub fn eval(child: &Child, ctx: &Context) -> Result<f64, EvalError> {
        let child = child.eval(ctx)?;

        if child == FRAC_PI_2 {
            return Err(EvalError::TanAtPiOverTwo)
        }
        Ok(Function::round(child.tan()))
    }

    // fn substitute(&self, args: &HashMap<&str, Child>) -> Child {
    //     let child = self.child.substitute(args);
    //     Self::new(child).into()
    // }

    // fn get_type(&self) -> FnType {
    //     FnType::Unary(&self.child)
    // }

    pub fn derivative(child: &Child, var: &str) -> Child {
        let d_child = child.derivative(var);

        let child_copy = child.clone();
        let den = Cos(child_copy).into();

        let ratio = Div(d_child, den).into();
        let pow = 2.0.into();

        Exp(ratio, pow).into()
    }

    pub fn to_string(child: &Child) -> String {
        format!("tan{}", child.to_string())
    }
}