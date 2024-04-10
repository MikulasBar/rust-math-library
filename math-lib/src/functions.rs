use std::{
    collections::HashMap, f64::consts::{E, FRAC_PI_2}, fmt::format
};
use core::fmt::Debug;

use crate::{
    antlr_parser::mathlexer::mathLexer,
    function_tree::FnTree,
    utilities::{
        ToChildFn,
        type_of,
    },
};
use ChildFn::*;
use EvalError::*;


pub type FnArgs<'a> = HashMap<&'a str, f64>;


pub enum FunctionType<'a> {
    None,
    Unary(&'a ChildFn),
    Binary(&'a ChildFn, &'a ChildFn),
    Variadic(&'a Vec<ChildFn>)
}

pub trait Function {
    /// This function needs to be implemented, so `ChildFn` can be cloned 
    fn clone_box(&self) -> Box<dyn Function>;
    fn evaluate(&self, args: &FnArgs) -> Result<f64, EvalError>;
    fn derivative(&self, variable: &str) -> ChildFn;

    fn get_type(&self) -> FunctionType {
        FunctionType::None
    }

    fn get_string_tree(&self) -> String {
        let name = type_of(&self);
        
        let body = match self.get_type() {
            FunctionType::Unary(c) => c.get_string_tree(),
            FunctionType::Binary(l, r) => format!{
                "{}, {}",
                l.get_string_tree(),
                r.get_string_tree(),
            },
            FunctionType::Variadic(v) => {
                let mut result = "".to_string();

                for c in v {
                    result += &c.get_string_tree();
                    result += ",";
                }
                result.truncate(result.len() - 1);

                result
            },
            FunctionType::None => panic!(
                "Functions of type None cannot uses implicit definition of the this function, \n
                implement this function / change function type in get_type function"
            )
        };

        format!("{} {{ {} }}", name, body)
    }

    fn depends_on(&self, variable: &str) -> bool {
        match self.get_type() {
            FunctionType::Unary(c) => c.depends_on(variable),
            FunctionType::Binary(l, r) => {
                let left = l.depends_on(variable);
                let right = r.depends_on(variable);
                left || right
            },
            FunctionType::Variadic(v) =>  {
                v.iter()
                    .any(|c| c.depends_on(variable))
            }
            FunctionType::None => panic!(
                "Functions of type None cannot uses implicit definition of the this function, \n
                implement this function / change function type in get_type function"
            )
        }
    }
}


#[derive(Debug)]
pub enum EvalError {
    DivisionByZeroError,
    NegativeEvenRootError,
    NonPositiveLogArgError,
    NonPositiveLogBaseError,
    LogBaseOneError,
    NegativeBaseNonIntegerExponentError,
    TanAtPiOverTwoError,

    ParameterNotFoundError(Box<str>),
}

impl PartialEq for EvalError {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}


impl Clone for ChildFn {
    fn clone(&self) -> Self {
        match self {
            ChildFn::Fn(f) => ChildFn::Fn(f.clone_box()),
            ChildFn::Var(v) => ChildFn::Var(v.clone()),
            ChildFn::Const(c) => ChildFn::Const(*c),
        }
    }
}


/// Type used for fields like `child` or `exponent` ...
pub enum ChildFn {
    Fn(Box<dyn Function>),
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
    fn clone_box(&self) -> Box<dyn Function> {
        Box::new(self.clone())
    }

    fn evaluate(&self, args: &FnArgs) -> Result<f64, EvalError> {
        match self {
            Fn(f) => f.evaluate(args),
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

    fn get_string_tree(&self) -> String {
        match self {
            Const(c) => c.to_string(),
            Var(v) => v.to_string(),
            Fn(f) => f.get_string_tree(),
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
            Const(c) => 0.0.to_child_fn(),
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


#[derive(Clone)]
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
    fn clone_box(&self) -> Box<dyn Function> {
        Box::new(self.clone())
    }

    fn evaluate(&self, args: &FnArgs) -> Result<f64, EvalError> {
        let left = self.left.evaluate(args)?;
        let right = self.right.evaluate(args)?;

        Ok(left + right)
    }

    fn get_type(&self) -> FunctionType {
        FunctionType::Binary(&self.left, &self.right)
    }

    fn derivative(&self, variable: &str) -> ChildFn {
        let left = self.left.derivative(variable);
        let right = self.right.derivative(variable);

        AddFn::new(left, right).to_child_fn()
    }
}


#[derive(Clone)]
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
    fn clone_box(&self) -> Box<dyn Function> {
        Box::new(self.clone())
    }

    fn evaluate(&self, args: &FnArgs) -> Result<f64, EvalError> {
        let left = self.left.evaluate(args)?;
        let right = self.right.evaluate(args)?;

        Ok(left - right)
    }

    fn get_type(&self) -> FunctionType {
        FunctionType::Binary(&self.left, &self.right)
    }

    fn derivative(&self, variable: &str) -> ChildFn {
        let left = self.left.derivative(variable);
        let right = self.right.derivative(variable);

        SubFn::new(left, right).to_child_fn()
    }
}


#[derive(Clone)]
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
    fn clone_box(&self) -> Box<dyn Function> {
        Box::new(self.clone())
    }

    fn evaluate(&self, args: &FnArgs) -> Result<f64, EvalError> {
        let left = self.left.evaluate(args)?;
        let right = self.right.evaluate(args)?;

        Ok(left * right)
    }

    fn get_type(&self) -> FunctionType {
        FunctionType::Binary(&self.left, &self.right)
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


#[derive(Clone)]
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
    fn clone_box(&self) -> Box<dyn Function> {
        Box::new(self.clone())
    }

    fn evaluate(&self, args: &FnArgs) -> Result<f64, EvalError> {
        let num_value = self.numerator.evaluate(args)?;
        let den_value = self.denominator.evaluate(args)?;

        if den_value == 0.0 {
            return Err(DivisionByZeroError)
        }
        Ok(num_value / den_value)
    }

    fn get_type(&self) -> FunctionType {
        FunctionType::Binary(&self.numerator, &self.denominator)
    }

    fn derivative(&self, variable: &str) -> ChildFn {
        let d_num = self.numerator.derivative(variable);
        let d_denom = self.denominator.derivative(variable);

        let num_first = MulFn::new(
            self.denominator.clone(),
            d_num
        );

        let num_second = MulFn::new(
            self.numerator.clone(),
            d_denom
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
#[derive(Clone)]
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
    fn clone_box(&self) -> Box<dyn Function> {
        Box::new(self.clone())
    }

    fn evaluate(&self, args: &FnArgs) -> Result<f64, EvalError> {
        let child_value = self.child.evaluate(args)?;

        Ok(self.coefficient * child_value)
    }

    fn depends_on(&self, variable: &str) -> bool {
        self.child.depends_on(variable)
    }

    fn derivative(&self, variable: &str) -> ChildFn {
        if self.child.depends_on(variable) {
            return CoefFn::new(
                self.coefficient,
                self.child.derivative(variable)
            ).to_child_fn()
        }
        0.0.to_child_fn()
    }
}


#[derive(Clone)]
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
    fn clone_box(&self) -> Box<dyn Function> {
        Box::new(self.clone())
    }

    fn evaluate(&self, args: &FnArgs) -> Result<f64, EvalError> {
        let base_value = self.base.evaluate(args)?;
        let exp_value = self.exponent.evaluate(args)?;

        if base_value < 0.0 && exp_value.fract() != 0.0 {
            return Err(NegativeBaseNonIntegerExponentError)
        }
        if base_value < 0.0 && exp_value.fract() > 0.0 {
            return Err(NegativeEvenRootError)
        }
        Ok(base_value.powf(exp_value))
    }

    fn get_type(&self) -> FunctionType {
        FunctionType::Binary(&self.base, &self.exponent)
    }

    fn derivative(&self, variable: &str) -> ChildFn {
        let d_exp = self.exponent.derivative(variable);
        let d_base = self.base.derivative(variable);

        let left_term = MulFn::new(
            d_exp,
            LogFn::new(E, self.base.clone())
        );

        let right_term = DivFn::new(
            MulFn::new(self.exponent.clone(), d_base), 
            self.base.clone()
        );
        
        let factor = AddFn::new(left_term, right_term);

        MulFn::new(self.clone(), factor).to_child_fn()
    }
}


#[derive(Clone)]
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
    fn clone_box(&self) -> Box<dyn Function> {
        Box::new(self.clone())
    }

    fn evaluate(&self, args: &FnArgs) -> Result<f64, EvalError> {
        let base_value = self.base.evaluate(args)?;
        let arg_value = self.argument.evaluate(args)?;

        match (base_value, arg_value) {
            (_, a) if a <= 0.0 => Err(NonPositiveLogArgError),
            (b, _) if b <= 0.0 => Err(NonPositiveLogBaseError),
            (b, _) if b == 1.0 => Err(LogBaseOneError),
            _ => Ok(arg_value.log(base_value))
        }
    }

    fn get_type(&self) -> FunctionType {
        FunctionType::Binary(&self.base, &self.argument)
    }

    fn derivative(&self, variable: &str) -> ChildFn {
        let d_base = self.base.derivative(variable);
        let d_arg = self.argument.derivative(variable);

        let left = DivFn::new(d_arg, self.argument.clone());
        let right = MulFn::new(
            self.clone(),
            DivFn::new(d_base, self.base.clone())
        );

        let factor = SubFn::new(left, right); 

        DivFn::new(
            factor,
            LogFn::new(E, self.base.clone())
        ).to_child_fn()
    }
}


#[derive(Clone)]
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
    fn clone_box(&self) -> Box<dyn Function> {
        Box::new(self.clone())
    }

    fn evaluate(&self, args: &FnArgs) -> Result<f64, EvalError> {
        self.child
            .evaluate(args)
            .map(f64::sin)
    }

    fn get_type(&self) -> FunctionType {
        FunctionType::Unary(&self.child)
    }

    fn derivative(&self, variable: &str) -> ChildFn {
        MulFn::new(
            CosFn::new(self.child.clone()),
            self.child.derivative(variable)
        ).to_child_fn()
    }
}


#[derive(Clone)]
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
    fn clone_box(&self) -> Box<dyn Function> {
        Box::new(self.clone())
    }

    fn evaluate(&self, args: &FnArgs) -> Result<f64, EvalError> {
        self.child
            .evaluate(args)
            .map(f64::cos)
    }

    fn get_type(&self) -> FunctionType {
        FunctionType::Unary(&self.child)
    }

    fn derivative(&self, variable: &str) -> ChildFn {
        MulFn::new(
            CoefFn::new(-1, SinFn::new(self.child.clone())),
            self.child.derivative(variable)
        ).to_child_fn()
    }
}


#[derive(Clone)]
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
    fn clone_box(&self) -> Box<dyn Function> {
        Box::new(self.clone())
    }

    fn evaluate(&self, args: &FnArgs) -> Result<f64, EvalError> {
        let child_value = self.child.evaluate(args)?;

        if child_value == FRAC_PI_2 {
            return Err(TanAtPiOverTwoError)
        }
        Ok(child_value.tan())
    }

    fn get_type(&self) -> FunctionType {
        FunctionType::Unary(&self.child)
    }

    fn derivative(&self, variable: &str) -> ChildFn {
        let denom = ExpFn::new(CosFn::new(self.child.clone()), 2.0);

        DivFn::new(
            self.child.derivative(variable),
            denom
        ).to_child_fn()
    }
}



// Seq stands for sequence,
// it means the function has arbitrary number of children 

#[derive(Clone)]
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
    fn clone_box(&self) -> Box<dyn Function> {
        Box::new(self.clone())
    }

    fn evaluate(&self, args: &FnArgs) -> Result<f64, EvalError> {
        let mut result: f64 = 0.0;

        for child in &self.children {
            let child_result = child.evaluate(args)?;
            result += child_result;
        }
        Ok(result)
    }

    fn get_type(&self) -> FunctionType {
        FunctionType::Variadic(&self.children)
    }

    fn derivative(&self, variable: &str) -> ChildFn {
        let children: Vec<_> = self.children.clone()
            .into_iter()
            .map(|c| c.derivative(variable))
            .collect();

        SeqAddFn::new(children).to_child_fn()
    }
}


#[derive(Clone)]
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
    fn clone_box(&self) -> Box<dyn Function> {
        Box::new(self.clone())
    }

    fn evaluate(&self, args: &FnArgs) -> Result<f64, EvalError> {
        let mut result: f64 = 1.0;

        for child in &self.children {
            let child_result = child.evaluate(args)?;
            result *= child_result;
        }
        Ok(result)
    }

    fn get_type(&self) -> FunctionType {
        FunctionType::Variadic(&self.children)
    }

    fn derivative(&self, variable: &str) -> ChildFn {
        let terms: Vec<_> = self.children.clone()
            .into_iter()
            .map(|c|
                DivFn::new(
                    c.derivative(variable),
                    c
                )
            ).collect();

        MulFn::new(
            self.clone(),
            SeqAddFn::new(terms)
        ).to_child_fn()
    }
}


