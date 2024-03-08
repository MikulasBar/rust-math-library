use std::{
    collections::HashMap,
};
use crate::utilities::ToChildFn;
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
    TanAtPiOverTwoError,

    ParameterNotFoundError,
}

impl PartialEq for FnError {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}


/// Type used for fields like `child` or `exponent` ... 
pub enum ChildFn {
    Fn(Box<dyn Function>),
    Var(Box<str>),
    Const(f32)
}

impl Function for ChildFn {
    fn apply(&self, args: &FnArgs) -> FnResult {
        match self {
            Fn(f) => f.apply(args),
            Const(c) => Ok(*c),
            Var(s) => {
                match args.get(s.as_ref()) {
                    Some(&v) => Ok(v),
                    _ => Err(ParameterNotFoundError)
                }
            },
        }
    }
}



pub struct FnStruct {
    definition: ChildFn,
}

impl FnStruct {
    pub fn new<T>(def: T) -> Self 
    where
        T: ToChildFn,
    {
        Self {
            definition: def.to_child()
        }
    }

    pub fn apply(&self, args: &FnArgs) -> FnResult {
        self.definition.apply(args)
    } 
}


pub struct AddFn {           
    children: Vec<ChildFn>
}

impl AddFn {
    /// Initialise new function with no children
    pub fn new<T>(children: Vec<T>) -> Self
    where 
        T: ToChildFn,
    { 
        Self {
            children: children
                .into_iter()
                .map(|c| c.to_child())
                .collect(),
        }
    }
}

impl Default for AddFn {
    fn default() -> Self {
        Self {
            children: Vec::new()
        }
    }
}

impl Function for AddFn {
    fn apply(&self, args: &FnArgs) -> FnResult {
        let mut result: f32 = 0.0;

        for child in &self.children {
            let child_result = child.apply(args)?;
            result += child_result;
        }
        Ok(result)
    }
}



pub struct MulFn {
    children: Vec<ChildFn>
}

impl MulFn {
    /// Initialise new function with no children
    pub fn new<T>(children: Vec<T>) -> Self
    where 
        T: ToChildFn,
    {
        Self {
            children: children
                .into_iter()
                .map(|c| c.to_child())
                .collect(),
        }
    }
}

impl Default for MulFn {
    fn default() -> Self {
        Self {
            children: Vec::new()
        }
    }
}

impl Function for MulFn {
    fn apply(&self, args: &FnArgs) -> FnResult {
        let mut result: f32 = 1.0;

        for child in &self.children {
            let child_result = child.apply(args)?;
            result *= child_result;
        }
        Ok(result)
    }
}


pub struct DivFn {
    numerator: ChildFn,
    denominator: ChildFn
}

impl DivFn {
    pub fn new<T, U>(num: T, denom: U) -> Self
    where 
        T: ToChildFn,
        U: ToChildFn,
    {
        Self {
            numerator: num.to_child(),
            denominator: denom.to_child(),
        }
    }
}

impl Function for DivFn {
    fn apply(&self, args: &FnArgs) -> FnResult {
        let num_value = self.numerator.apply(args)?;
        let den_value = self.denominator.apply(args)?;

        if den_value == 0.0 {
            return Err(DivisionByZeroError)
        }
        Ok(num_value / den_value)
    }
}



/// This function is used for adding coefficient without using `MulFn` <br>
pub struct CoefFn {
    coefficient: f32,
    child: ChildFn
}

impl CoefFn {
    pub fn new<C, F>(coeff: C, child: F) -> Self
    where
        C: Into<f32>,
        F: ToChildFn,
    {
        Self {
            coefficient: coeff.into(),
            child: child.to_child()
        }
    }
}

impl Function for CoefFn {
    fn apply(&self, args: &FnArgs) -> FnResult {
        let child_value = self.child.apply(args)?;

        Ok(self.coefficient * child_value)
    }
}

pub struct ExpFn {
    base: ChildFn,
    exponent: ChildFn
}

impl ExpFn {
    pub fn new<T, U>(base: T, exp: U) -> Self
    where
        T: ToChildFn,
        U: ToChildFn,
    {
        Self {
            base: base.to_child(),
            exponent: exp.to_child(),
        }
    }
}

impl Function for ExpFn {
    fn apply(&self, args: &FnArgs) -> FnResult {
        let base_value = self.base.apply(args)?;
        let exp_value = self.exponent.apply(args)?;

        if base_value < 0.0 && exp_value.fract() != 0.0 {
            return Err(NegativeBaseNonIntegerExponentError)
        }
        if base_value < 0.0 && exp_value.fract() > 0.0 {
            return Err(NegativeEvenRootError)
        }
        Ok(base_value.powf(exp_value))
    }
}



pub struct LogFn {
    base: ChildFn,
    argument: ChildFn
}

impl LogFn {
    pub fn new<T, U>(base: T, arg: U) -> Self
    where
        T: ToChildFn,
        U: ToChildFn,
    {
        Self {
            base: base.to_child(),
            argument: arg.to_child(),
        }
    }
}

impl Function for LogFn {
    fn apply(&self, args: &FnArgs) -> FnResult {
        let base_value = self.base.apply(args)?;
        let arg_value = self.argument.apply(args)?;

        if arg_value <= 0.0 {
            return Err(NonPositiveLogArgError)
        } else if base_value <= 0.0 {
            return Err(NonPositiveLogBaseError)
        } else if base_value == 1.0 {
            return Err(LogBaseOneError)
        }
        Ok(arg_value.log(base_value))
    }
}


// Goniometry functions

pub struct SinFn {
    child: ChildFn
}

impl SinFn {
    pub fn new<T>(child: T) -> Self
    where 
        T: ToChildFn,
    {
        Self {
            child: child.to_child()
        }
    }
}

impl Function for SinFn {
    fn apply(&self, args: &FnArgs) -> FnResult {
        self.child
            .apply(args)
            .map(f32::sin)
    }
}


pub struct CosFn {
    child: ChildFn
}

impl CosFn {
    pub fn new<T>(child: T) -> Self
    where 
        T: ToChildFn,
    {
        Self {
            child: child.to_child()
        }
    }
}

impl Function for CosFn {
    fn apply(&self, args: &FnArgs) -> FnResult {
        self.child
            .apply(args)
            .map(f32::cos)
    }
}


pub struct TanFn {
    child: ChildFn
}

impl TanFn {
    pub fn new<T>(child: T) -> Self
    where 
        T: ToChildFn,
    {
        Self {
            child: child.to_child()
        }
    }
}

impl Function for TanFn {
    fn apply(&self, args: &FnArgs) -> FnResult {
        let child_value = self.child.apply(args)?;

        if child_value == std::f32::consts::FRAC_PI_2 {
            return Err(TanAtPiOverTwoError)
        }
        Ok(child_value.tan())
    }
}

