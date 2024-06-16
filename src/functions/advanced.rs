use std::{collections::HashMap, ops::Div};
use std::f64::consts::E;

use crate::{
    function::*,
    child::*,
};

use EvalError::*;
use Function::*;

use crate::binary_new;

pub struct ExpFn;

impl ExpFn {
    binary_new!{Exp, base, exponent}

    pub fn eval(base: &Child, exp: &Child, args: &HashMap<&str, f64>) -> Result<f64, EvalError> {
        let exp = exp.eval(args)?;
        let base = base.eval(args)?;
        
        match (base, exp) {
            (0.0, 0.0) => Err(ZeroToZero),
            (0.0, e) if e < 0.0 => Err(ZeroBaseNonPositiveExponent),
            (b, e) if b < 0.0 && e.fract() != 0.0 => Err(NegativeBaseNonIntegerExponent),
            (_, _) => Ok(base.powf(exp)), 
        }
    }

    // fn substitute(&self, args: &HashMap<&str, Child>) -> Child {
    //     let base = self.base.substitute(args);
    //     let exp = self.exponent.substitute(args);

    //     Self::new(base, exp).to_child()
    // }

    // fn get_type(&self) -> FnType {
    //     FnType::Binary(&self.base, &self.exponent)
    // }

    pub fn derivative(base: &Child, exp: &Child, var: &str) -> Child {
        let d_base = base.derivative(var);
        let d_exp = exp.derivative(var);

        let ln_base = Log(base.clone(), E.to_child()).to_child();
        let ratio = Div(exp.clone(), base.clone()).to_child();

        let left = Mul(d_exp, ln_base).to_child();
        let right = Mul(d_base, ratio).to_child();

        let self_copy = Exp(base.clone(), exp.clone()).to_child();
        let factor = Add(left, right).to_child();

        Mul(self_copy, factor).to_child()
    }

    pub fn to_string(base: &Child, exp: &Child) -> String {
        format!("{}^{}", base.to_string(), exp.to_string())
    }
}


pub struct LogFn;

impl LogFn {
    binary_new!{Log, base, argument}

    pub fn eval(base: &Child, arg: &Child, args: &HashMap<&str, f64>) -> Result<f64, EvalError> {
        let base = base.eval(args)?;
        match base {
            b if b <= 0.0 => return Err(NonPositiveLogBase),
            b if b == 1.0 => return Err(LogBaseOne),
            _ => (),
        }

        let arg = arg.eval(args)?;
        match arg {
            a if a <= 0.0 => Err(NonPositiveLogArg),
            _ => Ok(arg.log(base))
        }
    }

    // fn substitute(&self, args: &HashMap<&str, Child>) -> Child {
    //     let base = self.base.substitute(args);
    //     let arg = self.argument.substitute(args);

    //     Self::new(base, arg).to_child()
    // }

    // fn get_type(&self) -> FnType {
    //     FnType::Binary(&self.base, &self.argument)
    // }

    pub fn derivative(base: &Child, arg: &Child, variable: &str) -> Child {
        let d_base = base.derivative(variable);
        let d_arg = arg.derivative(variable);

        let base_ratio = Div(d_base, base.clone()).to_child();
        let arg_ratio = Div(d_arg, arg.clone()).to_child();

        let self_copy = Log(base.clone(), arg.clone()).to_child();

        let term = Mul(base_ratio, self_copy).to_child(); 

        Sub(arg_ratio, term).to_child()
    }

    pub fn to_string(base: &Child, arg: &Child) -> String {
        format!("log_{}({})", base.to_string(), arg.to_string())
    }
}






// #[derive(Clone)]
// pub struct SeqAddFn {           
//     children: Vec<Child>
// }

// impl SeqAddFn {
//     /// Initialise new function with no children
//     pub fn new<T>(children: Vec<T>) -> Self
//     where 
//         T: ToChild,
//     { 
//         Self {
//             children: children
//                 .into_iter()
//                 .map(|c| c.to_child())
//                 .collect(),
//         }
//     }
// }

// impl FnBehaviour for SeqAddFn {
//     fn clone_box(&self) -> Box<dyn FnBehaviour> {
//         Box::new(self.clone())
//     }

//     fn evaluate(&self, args: &HashMap<&str, f64>) -> Result<f64, EvalError> {
//         let mut result: f64 = 0.0;

//         for child in &self.children {
//             let child_result = child.evaluate(args)?;
//             result += child_result;
//         }
//         Ok(result)
//     }

//     fn substitute(&self, args: &HashMap<&str, Child>) -> Child {
//         let children: Vec<Child> = self.children.clone()
//             .into_iter()
//             .map(|c| c.substitute(args))
//             .collect();

//         Self::new(children).to_child()
//     }

//     fn get_type(&self) -> FnType {
//         FnType::Variadic(&self.children)
//     }

//     fn derivative(&self, variable: &str) -> Child {
//         let children: Vec<_> = self.children.clone()
//             .into_iter()
//             .map(|c| c.derivative(variable))
//             .collect();

//         SeqAddFn::new(children).to_child()
//     }
// }



// pub struct SeqMulFn {
//     children: Vec<Child>
// }

// impl SeqMulFn {
//     /// Initialise new function with no children
//     pub fn new<T>(children: Vec<T>) -> Self
//     where 
//         T: ToChild,
//     {
//         Self {
//             children: children
//                 .into_iter()
//                 .map(|c| c.to_child())
//                 .collect(),
//         }
//     }
// }

// impl FnBehaviour for SeqMulFn {
//     fn clone_box(&self) -> Box<dyn FnBehaviour> {
//         Box::new(self.clone())
//     }

//     fn evaluate(&self, args: &HashMap<&str, f64>) -> Result<f64, EvalError> {
//         let mut result: f64 = 1.0;

//         for child in &self.children {
//             let child_result = child.evaluate(args)?;
//             result *= child_result;
//         }
//         Ok(result)
//     }

//     fn substitute(&self, args: &HashMap<&str, Child>) -> Child {
//         let children: Vec<Child> = self.children.clone()
//             .into_iter()
//             .map(|c| c.substitute(args))
//             .collect();

//         Self::new(children).to_child()
//     }

//     fn get_type(&self) -> FnType {
//         FnType::Variadic(&self.children)
//     }

//     fn derivative(&self, variable: &str) -> Child {
//         let terms: Vec<_> = self.children.clone()
//             .into_iter()
//             .map(|c|
//                 DivFn::new(
//                     c.derivative(variable),
//                     c
//                 )
//             ).collect();

//         MulFn::new(
//             self.clone(),
//             SeqAddFn::new(terms)
//         ).to_child()
//     }
// }


