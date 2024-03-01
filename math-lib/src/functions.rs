use std::{
    collections::HashMap,
};

use ChildFn::*;
use FnError::*;


pub type FnResult = Result<f32, FnError>;


pub type FnArgs<'a> = HashMap<&'a str, f32>;
pub trait Function {
    fn apply(&self, args: &FnArgs) -> FnResult;
    //fn derivative(&self) -> Self;
}


#[derive(Debug)]
pub enum FnError {
    DivisionByZeroError,
    NegativeEvenRootError,
    NonPositiveLogArgError,
    NonPositiveLogBaseError,
    LogBaseOneError,
    NegativeBaseNonIntegerExponentError,

    ParameterNotFoundError,
}

impl PartialEq for FnError {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}


/// Type used for fields like `child` or `exponent` ... 
pub enum ChildFn<'a> {
    Fn(Box<dyn Function>),
    Var(&'a str),
    Const(f32)
}

impl<'a> Function for ChildFn<'a> {
    fn apply(&self, args: &FnArgs) -> FnResult {
        match self {
            Fn(f) => f.apply(args),
            Const(c) => Ok(*c),
            Var(ref s) => {
                match args.get(s) {
                    Some(&v) => Ok(v),
                    _ => Err(ParameterNotFoundError)
                }
            },
        }
    }
}


// impl<'a> Into<ChildFn<'a>> for String {
//     fn into(self) -> ChildFn<'a> {
//         Var(self.as_str())
//     }
// }

impl<'a> Into<ChildFn<'a>> for &'a str {
    fn into(self) -> ChildFn<'a> {
        Var(self)
    }
}

impl<'a> Into<ChildFn<'a>> for f32 {
    fn into(self) -> ChildFn<'a> {
        Const(self)
    }
}


pub struct FnStruct<'a> {
    pub definition: ChildFn<'a>,
}

impl<'a> FnStruct<'a> {
    pub fn new<T: Into<ChildFn<'a>>>(def: T) -> Self {
        Self {
            definition: def.into()
        }
    }

    pub fn apply(&self, args: &FnArgs) -> FnResult {
        self.definition.apply(args)
    } 
}


pub struct AddFn<'a> {           
    pub children: Vec<ChildFn<'a>>
}

impl<'a> AddFn<'a> {
    /// Initialise new function with no children
    pub fn new(children: Vec<ChildFn<'a>>) -> Self {
        Self {
            children,
        }
    }

    pub fn add_child(&mut self, child: ChildFn<'a>) {
        self.children.push(child);
    }
}

impl<'a> Default for AddFn<'a> {
    fn default() -> Self {
        Self {
            children: Vec::new()
        }
    }
}

impl<'a> Function for AddFn<'a> {
    fn apply(&self, args: &FnArgs) -> FnResult {
        let mut result: f32 = 0.0;

        for child in &self.children {
            let child_result = child.apply(args);
            match child_result {
                Ok(v) => result += v,
                e => return e,
            }
        }
        Ok(result)
    }
}



pub struct MulFn<'a> {
    pub children: Vec<ChildFn<'a>>
}

impl<'a> MulFn<'a> {
    /// Initialise new function with no children
    pub fn new(children: Vec<ChildFn<'a>>) -> Self {
        Self {
            children,
        }
    }

    pub fn add_child(&mut self, child: ChildFn<'a>) {
        self.children.push(child);
    }
}

impl<'a> Default for MulFn<'a> {
    fn default() -> Self {
        Self {
            children: Vec::new()
        }
    }
}

impl<'a> Function for MulFn<'a> {
    fn apply(&self, args: &FnArgs) -> FnResult {
        let mut result: f32 = 1.0;

        for child in &self.children {
            let child_result = child.apply(args);
            match child_result {
                Ok(v) => result *= v,
                e => return e,
            }
        }
        Ok(result)
    }
}


pub struct DivFn<'a> {
    pub numerator: ChildFn<'a>,
    pub denominator: ChildFn<'a>
}

impl<'a> DivFn<'a> {
    pub fn new<T, U>(num: T, denom: U) -> Self
    where 
        T: Into<ChildFn<'a>>,
        U: Into<ChildFn<'a>>,
    {
        Self {
            numerator: num.into(),
            denominator: denom.into(),
        }
    }
}

impl<'a> Function for DivFn<'a> {
    fn apply(&self, args: &FnArgs) -> FnResult {
        let num_result = self.numerator.apply(args);
        let den_result = self.denominator.apply(args);

        match (num_result, den_result) {
            (Ok(n), Ok(d)) => {
                if d == 0.0 {
                    return Err(DivisionByZeroError)
                }
                Ok(n / d)
            },
            (Err(e), _) | (_, Err(e)) => Err(e),
        }
    }
}



/// This function is used for adding coefficient without using `MulFn` <br>
pub struct CoefFn<'a> {
    pub coefficient: f32,
    pub child: ChildFn<'a>
}

impl<'a> CoefFn<'a> {
    pub fn new<C, F>(coeff: C, child: F) -> Self
    where
        C: Into<f32>,
        F: Into<ChildFn<'a>>,
    {
        Self {
            coefficient: coeff.into(),
            child: child.into()
        }
    }
}

impl<'a> Function for CoefFn<'a> {
    fn apply(&self, args: &FnArgs) -> FnResult {
        let child_result = self.child.apply(args);

        match child_result {
            Ok(v) => Ok(self.coefficient * v),
            e => e,
        }
    }
}

pub struct ExpFn<'a> {
    pub base: ChildFn<'a>,
    pub exponent: ChildFn<'a>
}

impl<'a> ExpFn<'a> {
    pub fn new<T, U>(base: T, exp: U) -> Self
    where
        T: Into<ChildFn<'a>>,
        U: Into<ChildFn<'a>>,
    {
        Self {
            base: base.into(),
            exponent: exp.into(),
        }
    }
}

impl<'a> Function for ExpFn<'a> {
    fn apply(&self, args: &FnArgs) -> FnResult {
        let base_result = self.base.apply(args);
        let exp_result = self.exponent.apply(args);

        match (base_result, exp_result) {
            (Ok(b), Ok(n)) => {
                if b < 0.0 && n.fract() != 0.0 {
                    return Err(NegativeBaseNonIntegerExponentError)
                }
                Ok(b.powf(n))
            },
            (Err(e), _) | (_, Err(e)) => Err(e),
        }
    }
}



pub struct LogFn<'a> {
    pub base: ChildFn<'a>,
    pub argument: ChildFn<'a>
}

impl<'a> LogFn<'a> {
    pub fn new<T, U>(base: T, arg: U) -> Self
    where
        T: Into<ChildFn<'a>>,
        U: Into<ChildFn<'a>>,
    {
        Self {
            base: base.into(),
            argument: arg.into(),
        }
    }
}

impl<'a> Function for LogFn<'a> {
    fn apply(&self, args: &FnArgs) -> FnResult {
        let base_result = self.base.apply(args);
        let arg_result = self.argument.apply(args);

        match (base_result, arg_result) {
            (Ok(b), Ok(a)) => {
                if a <= 0.0 {
                    return Err(NonPositiveLogArgError)
                } else if b <= 0.0 {
                    return Err(NonPositiveLogBaseError)
                } else if b == 1.0 {
                    return Err(LogBaseOneError)
                }
                Ok(a.log(b))
            },
            (Err(e), _) | (_, Err(e)) => Err(e),
        }
    }
}

