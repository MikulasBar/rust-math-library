use std::fmt::Binary;
use std::{collections::HashMap};
use std::f64::consts::{E, FRAC_PI_2};
use core::fmt::Debug;
use crate::function_tree::FnTree;
use crate::{antlr_parser::mathlexer::mathLexer, utilities::ToChildFn};
use ChildFn::*;
use ApplyError::*;


pub type FnArgs<'a> = HashMap<&'a str, f64>;


enum FunctionType<'a> {
    None,
    Unary(&'a ChildFn),
    Binary(&'a ChildFn, &'a ChildFn),
    Other(&'a Vec<ChildFn>)
}

pub trait Function {
    const TYPE: FunctionType<'static>;

    fn apply(&self, args: &FnArgs) -> Result<f64, ApplyError>;
    fn depends_on(&self, variable: &str) -> bool {
        match Self::TYPE {
            FunctionType::Unary(c) => c.depends_on(variable),
            FunctionType::Binary(l, r) => {
                let left = l.depends_on(variable);
                let right = r.depends_on(variable);
                left || right
            },
            FunctionType::Other(v) =>  {
                v.iter()
                    .any(|c| c.depends_on(variable))
            }
            FunctionType::None => panic!("Functions of type `None` cannot uses implicit definition of the depends_on function")
        }
    }
    fn derivative(&self, variable: &str) -> ChildFn;
}


#[derive(Debug)]
pub enum ApplyError {
    DivisionByZeroError,
    NegativeEvenRootError,
    NonPositiveLogArgError,
    NonPositiveLogBaseError,
    LogBaseOneError,
    NegativeBaseNonIntegerExponentError,
    TanAtPiOverTwoError,

    ParameterNotFoundError(Box<str>),
}

impl PartialEq for ApplyError {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}


/// Type used for fields like `child` or `exponent` ...
#[derive(Debug, Clone)]
pub enum ChildFn {
    Fn(Box<FnTree>),
    Var(Box<str>),
    Const(f64)
}

impl ChildFn {  
    pub fn is_function(&self) -> bool {
        match *self {
            Fn(_) => true,
            _ => false
        }
    }

    pub fn is_var(&self) -> bool {
        match *self {
            Var(_) => true,
            _ => false
        }
    }

    pub fn is_const(&self) -> bool {
        match *self {
            Const(_) => true,
            _ => false
        }
    }
}

impl Function for ChildFn {
    const TYPE: FunctionType<'static> = FunctionType::None;

    fn apply(&self, args: &FnArgs) -> Result<f64, ApplyError> {
        match self {
            Fn(f) => f.apply(args),
            Const(c) => Ok(*c),
            Var(v) => {
                match args.get(v.as_ref()) {
                    Some(&value) => Ok(value),
                    _ => {
                        Err(ParameterNotFoundError(v.clone()))
                    }
                }
            },
        }
    }

    fn depends_on(&self, variable: &str) -> bool {
        match self {
            Fn(f) => f.depends_on(variable),
            Var(v) => **v == *variable,
            Const(_) => false,
        }
    }

    fn derivative(&self, variable: &str) -> ChildFn {
        match self {
            Fn(f) => f.derivative(variable),
            Const(c) => c.to_child_fn(),
            Var(v) => {
                let mut value = 0.0;
                if **v == *variable {
                    value = 1.0;
                }
                value.to_child_fn()
            }
        }
    }
}

impl Default for ChildFn {
    fn default() -> Self {
        Const(0.0)
    }
}


#[derive(Debug, Clone)]
pub struct AddFn {
    left: ChildFn,
    right: ChildFn
}

impl AddFn {
    pub fn new<T, U>(left: T, right: U) -> Self
    where
        T: ToChildFn,
        U: ToChildFn
    {
        Self {
            left: left.to_child_fn(),
            right: right.to_child_fn()
        }
    }
}

impl Function for AddFn {
    const TYPE: FunctionType<'static> = Binary();
    fn apply(&self, args: &FnArgs) -> Result<f64, ApplyError> {
        let left = self.left.apply(args)?;
        let right = self.right.apply(args)?;

        Ok(left + right)
    }

    fn depends_on(&self, variable: &str) -> bool {
        let left = self.left.depends_on(variable);
        let right = self.right.depends_on(variable);

        left || right
    }

    fn derivative(&self, variable: &str) -> ChildFn {
        let left = self.left.derivative(variable);
        let right = self.right.derivative(variable);

        AddFn::new(left, right).to_child_fn()
    }
}


#[derive(Debug, Clone)]
pub struct SubFn {
    left: ChildFn,
    right: ChildFn
}

impl SubFn {
    pub fn new<T, U>(left: T, right: U) -> Self
    where
        T: ToChildFn,
        U: ToChildFn
    {
        Self {
            left: left.to_child_fn(),
            right: right.to_child_fn()
        }
    }
}

impl Function for SubFn {
    fn apply(&self, args: &FnArgs) -> Result<f64, ApplyError> {
        let left = self.left.apply(args)?;
        let right = self.right.apply(args)?;

        Ok(left - right)
    }

    fn depends_on(&self, variable: &str) -> bool {
        let left = self.left.depends_on(variable);
        let right = self.right.depends_on(variable);

        left || right
    }

    fn derivative(&self, variable: &str) -> ChildFn {
        let left = self.left.derivative(variable);
        let right = self.right.derivative(variable);

        SubFn::new(left, right).to_child_fn()
    }
}


#[derive(Debug, Clone)]
pub struct MulFn {
    left: ChildFn,
    right: ChildFn
}

impl MulFn {
    pub fn new<T, U>(left: T, right: U) -> Self
    where
        T: ToChildFn,
        U: ToChildFn
    {
        Self {
            left: left.to_child_fn(),
            right: right.to_child_fn()
        }
    }
}

impl Function for MulFn {
    fn apply(&self, args: &FnArgs) -> Result<f64, ApplyError> {
        let left = self.left.apply(args)?;
        let right = self.right.apply(args)?;

        Ok(left * right)
    }

    fn depends_on(&self, variable: &str) -> bool {
        let left = self.left.depends_on(variable);
        let right = self.right.depends_on(variable);

        left || right
    }

    fn derivative(&self, variable: &str) -> ChildFn {
        let d_left = self.left.derivative(variable);
        let d_right = self.right.derivative(variable);

        let result_left = MulFn::new(
            d_left,
            self.right.clone()
        );

        let result_right = MulFn::new(
            d_right,
            self.left.clone()
        );

        AddFn::new(result_left, result_right).to_child_fn()
    }
}


#[derive(Debug, Clone)]
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
            numerator: num.to_child_fn(),
            denominator: denom.to_child_fn(),
        }
    }
}

impl Function for DivFn {
    fn apply(&self, args: &FnArgs) -> Result<f64, ApplyError> {
        let num_value = self.numerator.apply(args)?;
        let den_value = self.denominator.apply(args)?;

        if den_value == 0.0 {
            return Err(DivisionByZeroError)
        }
        Ok(num_value / den_value)
    }

    fn depends_on(&self, variable: &str) -> bool {
        let num = self.numerator.depends_on(variable);
        let denom = self.denominator.depends_on(variable);

        num || denom
    }

    fn derivative(&self, variable: &str) -> ChildFn {
        let d_num = self.numerator.derivative(variable);
        let d_denom = self.denominator.derivative(variable);

        let num_first = MulFn::new(
            self.numerator.clone(),
            d_denom
        );

        let num_second = MulFn::new(
            self.numerator.clone(),
            d_num
        );


        let result_num = SubFn::new(
            num_first,
            num_second
        );

        let result_denom = ExpFn::new(
            self.denominator.clone(),
            2.0
        );

        DivFn::new(result_num, result_denom).to_child_fn()
    }
}


/// This function is used for adding coefficient without using `SeqMulFn` <br>
#[derive(Debug, Clone)]
pub struct CoefFn {
    coefficient: f64,
    child: ChildFn
}

impl CoefFn {
    pub fn new<C, F>(coeff: C, child: F) -> Self
    where
        C: Into<f64>,
        F: ToChildFn,
    {
        Self {
            coefficient: coeff.into(),
            child: child.to_child_fn()
        }
    }
}

impl Function for CoefFn {
    fn apply(&self, args: &FnArgs) -> Result<f64, ApplyError> {
        let child_value = self.child.apply(args)?;

        Ok(self.coefficient * child_value)
    }

    fn depends_on(&self, variable: &str) -> bool {
        self.child.depends_on(variable)
    }

    fn derivative(&self, variable: &str) -> ChildFn {
        CoefFn::new(
            self.coefficient,
            self.child.derivative(variable)
        ).to_child_fn()
    }
}


#[derive(Debug, Clone)]
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
            base: base.to_child_fn(),
            exponent: exp.to_child_fn(),
        }
    }
}

impl Function for ExpFn {
    fn apply(&self, args: &FnArgs) -> Result<f64, ApplyError> {
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

    fn depends_on(&self, variable: &str) -> bool {
        let base = self.base.depends_on(variable);
        let exp = self.exponent.depends_on(variable);

        base || exp
    }

    fn derivative(&self, variable: &str) -> ChildFn {
        let d_exp = self.exponent.derivative(variable);
        let d_base = self.base.derivative(variable);

        let left_factor = MulFn::new(
            d_exp,
            LogFn::new(E, self.base.clone())
        );

        let right_factor = DivFn::new(
            MulFn::new(self.exponent.clone(), d_base), 
            self.base.clone()
        );
        
        let factor = AddFn::new(left_factor, right_factor);

        MulFn::new(self.clone(), factor).to_child_fn()
    }
}


#[derive(Debug, Clone)]
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
            base: base.to_child_fn(),
            argument: arg.to_child_fn(),
        }
    }
}

impl Function for LogFn {
    fn apply(&self, args: &FnArgs) -> Result<f64, ApplyError> {
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

    fn depends_on(&self, variable: &str) -> bool {
        let base = self.base.depends_on(variable);
        let arg = self.argument.depends_on(variable);

        base || arg
    }

    fn derivative(&self, variable: &str) -> ChildFn {
        let d_base = self.argument.derivative(variable);
        let d_arg = self.base.derivative(variable);

        let left_term = DivFn::new(
            self.argument.clone(),
            d_arg
        );

        let right_term = DivFn::new(
            MulFn::new(d_base, self.clone()),
            self.base.clone()
        );

        let factor = SubFn::new(left_term, right_term);

        DivFn::new(
            factor,
            LogFn::new(E, self.base.clone())
        ).to_child_fn()
    }
}


#[derive(Debug, Clone)]
pub struct SinFn {
    child: ChildFn
}

impl SinFn {
    pub fn new<T>(child: T) -> Self
    where 
        T: ToChildFn,
    {
        Self {
            child: child.to_child_fn()
        }
    }
}

impl Function for SinFn {
    fn apply(&self, args: &FnArgs) -> Result<f64, ApplyError> {
        self.child
            .apply(args)
            .map(f64::sin)
    }

    fn depends_on(&self, variable: &str) -> bool {
        self.child.depends_on(variable)
    }

    fn derivative(&self, variable: &str) -> ChildFn {
        CosFn::new(self.child.clone()).to_child_fn()
    }
}


#[derive(Debug, Clone)]
pub struct CosFn {
    child: ChildFn
}

impl CosFn {
    pub fn new<T>(child: T) -> Self
    where 
        T: ToChildFn,
    {
        Self {
            child: child.to_child_fn()
        }
    }
}

impl Function for CosFn {
    fn apply(&self, args: &FnArgs) -> Result<f64, ApplyError> {
        self.child
            .apply(args)
            .map(f64::cos)
    }

    fn derivative(&self, variable: &str) -> ChildFn {
        CoefFn::new(-1, SinFn::new(self.child)).to_child_fn()
    }
}


#[derive(Debug, Clone)]
pub struct TanFn {
    child: ChildFn
}

impl TanFn {
    pub fn new<T>(child: T) -> Self
    where 
        T: ToChildFn,
    {
        Self {
            child: child.to_child_fn()
        }
    }
}

impl Function for TanFn {
    fn apply(&self, args: &FnArgs) -> Result<f64, ApplyError> {
        let child_value = self.child.apply(args)?;

        if child_value == FRAC_PI_2 {
            return Err(TanAtPiOverTwoError)
        }
        Ok(child_value.tan())
    }

    fn derivative(&self, variable: &str) -> ChildFn {
        let denom = ExpFn::new(CosFn::new(self.child), 2.0);
        DivFn::new(1.0, denom).to_child_fn()
    }
}




#[derive(Debug, Clone)]
pub struct SeqAddFn {           
    children: Vec<ChildFn>
}

impl SeqAddFn {
    /// Initialise new function with no children
    pub fn new<T>(children: Vec<T>) -> Self
    where 
        T: ToChildFn,
    { 
        Self {
            children: children
                .into_iter()
                .map(|c| c.to_child_fn())
                .collect(),
        }
    }
}

impl Function for SeqAddFn {
    fn apply(&self, args: &FnArgs) -> Result<f64, ApplyError> {
        let mut result: f64 = 0.0;

        for child in &self.children {
            let child_result = child.apply(args)?;
            result += child_result;
        }
        Ok(result)
    }

    fn derivative(&self, variable: &str) -> ChildFn {
        todo!()
    }
}


#[derive(Debug, Clone)]
pub struct SeqMulFn {
    children: Vec<ChildFn>
}

impl SeqMulFn {
    /// Initialise new function with no children
    pub fn new<T>(children: Vec<T>) -> Self
    where 
        T: ToChildFn,
    {
        Self {
            children: children
                .into_iter()
                .map(|c| c.to_child_fn())
                .collect(),
        }
    }
}

impl Function for SeqMulFn {
    fn apply(&self, args: &FnArgs) -> Result<f64, ApplyError> {
        let mut result: f64 = 1.0;

        for child in &self.children {
            let child_result = child.apply(args)?;
            result *= child_result;
        }
        Ok(result)
    }

    fn derivative(&self, variable: &str) -> ChildFn {
        todo!()
    }
}


