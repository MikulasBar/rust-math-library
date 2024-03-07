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



pub struct FnStruct<'a> {
    pub definition: ChildFn<'a>,
}

impl<'a> FnStruct<'a> {
    pub fn new<T: ToChildFn<'a>>(def: T) -> Self {
        Self {
            definition: def.to_child()
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
    pub fn new<T>(children: Vec<T>) -> Self
    where 
        T: ToChildFn<'a>,
    {
        Self {
            children: children
                .into_iter()
                .map(|c| c.to_child())
                .collect(),
        }
    }

    pub fn add_child<T>(&mut self, new_child: T)
    where
        T: ToChildFn<'a>,
    {
        self.children.push(new_child.to_child());
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
            let child_result = child.apply(args)?;
            result += child_result;
        }
        Ok(result)
    }
}



pub struct MulFn<'a> {
    children: Vec<ChildFn<'a>>
}

impl<'a> MulFn<'a> {
    /// Initialise new function with no children
    pub fn new<T>(children: Vec<T>) -> Self
    where 
        T: ToChildFn<'a>,
    {
        Self {
            children: children
                .into_iter()
                .map(|c| c.to_child())
                .collect(),
        }
    }

    pub fn add_child<T>(&mut self, new_child: T)
    where
        T: ToChildFn<'a>,
    {
        self.children.push(new_child.to_child());
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
            let child_result = child.apply(args)?;
            result *= child_result;
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
        T: ToChildFn<'a>,
        U: ToChildFn<'a>,
    {
        Self {
            numerator: num.to_child(),
            denominator: denom.to_child(),
        }
    }
}

impl<'a> Function for DivFn<'a> {
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
pub struct CoefFn<'a> {
    pub coefficient: f32,
    pub child: ChildFn<'a>
}

impl<'a> CoefFn<'a> {
    pub fn new<C, F>(coeff: C, child: F) -> Self
    where
        C: Into<f32>,
        F: ToChildFn<'a>,
    {
        Self {
            coefficient: coeff.into(),
            child: child.to_child()
        }
    }
}

impl<'a> Function for CoefFn<'a> {
    fn apply(&self, args: &FnArgs) -> FnResult {
        let child_value = self.child.apply(args)?;

        Ok(self.coefficient * child_value)
    }
}

pub struct ExpFn<'a> {
    pub base: ChildFn<'a>,
    pub exponent: ChildFn<'a>
}

impl<'a> ExpFn<'a> {
    pub fn new<T, U>(base: T, exp: U) -> Self
    where
        T: ToChildFn<'a>,
        U: ToChildFn<'a>,
    {
        Self {
            base: base.to_child(),
            exponent: exp.to_child(),
        }
    }
}

impl<'a> Function for ExpFn<'a> {
    fn apply(&self, args: &FnArgs) -> FnResult {
        let base_value = self.base.apply(args)?;
        let exp_value = self.exponent.apply(args)?;

        if base_value < 0.0 && exp_value.fract() != 0.0 {
            return Err(NegativeBaseNonIntegerExponentError)
        }
        Ok(base_value.powf(exp_value))
    }
}



pub struct LogFn<'a> {
    base: ChildFn<'a>,
    argument: ChildFn<'a>
}

impl<'a> LogFn<'a> {
    pub fn new<T, U>(base: T, arg: U) -> Self
    where
        T: ToChildFn<'a>,
        U: ToChildFn<'a>,
    {
        Self {
            base: base.to_child(),
            argument: arg.to_child(),
        }
    }
}

impl<'a> Function for LogFn<'a> {
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

pub struct SinFn<'a> {
    child: ChildFn<'a>
}

impl<'a> SinFn<'a> {
    pub fn new<T>(child: T) -> Self
    where 
        T: ToChildFn<'a>,
    {
        Self {
            child: child.to_child()
        }
    }
}

impl<'a> Function for SinFn<'a> {
    fn apply(&self, args: &FnArgs) -> FnResult {
        let child_result = self.child.apply(args);
        child_result.map(f32::sin)
    }
}


pub struct CosFn<'a> {
    child: ChildFn<'a>
}

impl<'a> CosFn<'a> {
    pub fn new<T>(child: T) -> Self
    where 
        T: ToChildFn<'a>,
    {
        Self {
            child: child.to_child()
        }
    }
}

impl<'a> Function for CosFn<'a> {
    fn apply(&self, args: &FnArgs) -> FnResult {
        let child_result = self.child.apply(args);
        child_result.map(f32::cos)
    }
}


pub struct TanFn<'a> {
    child: ChildFn<'a>
}

impl<'a> TanFn<'a> {
    pub fn new<T>(child: T) -> Self
    where 
        T: ToChildFn<'a>,
    {
        Self {
            child: child.to_child()
        }
    }
}

impl<'a> Function for TanFn<'a> {
    fn apply(&self, args: &FnArgs) -> FnResult {
        let child_result = self.child.apply(args);

        let value = child_result?;
        if value == std::f32::consts::PI / 2.0 {
            return Err(TanAtPiOverTwoError)
        }
        Ok(value.tan())
    }
}

