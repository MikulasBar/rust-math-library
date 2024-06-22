use std::collections::HashMap;
use std::f64::consts::E;

use crate::{
    function::*,
    child::*,
    context::Context,
};

use EvalError::*;
use Function::*;

use crate::binary_new;

pub struct ExpFn;

impl ExpFn {
    binary_new!{Exp, base, exponent}

    pub fn eval(base: &Child, exp: &Child, ctx: &Context) -> Result<f64, EvalError> {
        let exp = exp.eval(ctx)?;
        let base = base.eval(ctx)?;
        
        match (base, exp) {
            (0.0, 0.0) => Err(ZeroToZero),
            (0.0, e) if e < 0.0 => Err(ZeroBaseNonPositiveExponent),
            (b, e) if b < 0.0 && e.fract() != 0.0 => Err(NegativeBaseNonIntegerExponent),
            (_, _) => Ok(base.powf(exp)), 
        }
    }

    pub fn derivative(base: &Child, exp: &Child, var: &str) -> Child {
        let d_base = base.derivative(var);
        let d_exp = exp.derivative(var);

        let ln_base = Log(base.clone(), E.into()).into();
        let ratio = Div(exp.clone(), base.clone()).into();

        let left = Mul(d_exp, ln_base).into();
        let right = Mul(d_base, ratio).into();

        let self_copy = Exp(base.clone(), exp.clone()).into();
        let factor = Add(left, right).into();

        Mul(self_copy, factor).into()
    }

    pub fn to_string(base: &Child, exp: &Child) -> String {
        format!("{}^{}", base.to_string(), exp.to_string())
    }
}


pub struct LogFn;

impl LogFn {
    binary_new!{Log, base, argument}

    pub fn eval(base: &Child, arg: &Child, ctx: &Context) -> Result<f64, EvalError> {
        let base = base.eval(ctx)?;
        match base {
            b if b <= 0.0 => return Err(NonPositiveLogBase),
            b if b == 1.0 => return Err(LogBaseOne),
            _ => (),
        }

        let arg = arg.eval(ctx)?;
        match arg {
            a if a <= 0.0 => Err(NonPositiveLogArg),
            _ => Ok(arg.log(base))
        }
    }

    pub fn derivative(base: &Child, arg: &Child, variable: &str) -> Child {
        let d_base = base.derivative(variable);
        let d_arg = arg.derivative(variable);

        let base_ratio = Div(d_base, base.clone()).into();
        let arg_ratio = Div(d_arg, arg.clone()).into();

        let self_copy = Log(base.clone(), arg.clone()).into();

        let term = Mul(base_ratio, self_copy).into(); 

        Sub(arg_ratio, term).into()
    }

    pub fn to_string(base: &Child, arg: &Child) -> String {
        format!("log_{}({})", base.to_string(), arg.to_string())
    }
}