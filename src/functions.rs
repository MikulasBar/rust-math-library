use core::fmt::Debug;
use maplit::hashmap;
use std::{
    collections::HashMap,
    f64::consts::{E, FRAC_PI_2},
    ops::RangeInclusive,
};

use crate::utilities::{self, ToChildFn, type_of};
use ChildFn::*;
use EvalError::*;

/// Error you get if you use implicitly implemented function that only works if you implement `get_type` function in `Function` trait
const FUNCTION_TYPE_ERROR: &str = "Function of this FunctionType cannot use implicit definition of the this function,
    implement this function / change function type in get_type function";

///# Functionality
/// Implement this trait for your struct if you want to treat it like mathematical function
/// 
/// All functions that manipulates with objects should be implemented such that they will produce new object <br>
/// Derive `Clone` trait for your struct <br>
/// 
/// Override implicit definition of `get_type` if you can <br>
/// If you do that some functions don't have to be implemented
pub trait Function {

    ///# Functionality
    /// Clones `self` and wraps it in `Box`
    /// 
    /// Derive `Clone` trait for your struct, so you can implement this function <br> 
    /// This function needs to be implemented, so `ChildFn` can be cloned
    fn clone_box(&self) -> Box<dyn Function>;

    ///# Functionality 
    /// Evaluate function with arguments <br>
    /// Arguments are HashMap<&str, f64>
    /// 
    ///# Example
    /// ```
    /// let function = AddFn::new("x", "y");
    /// 
    /// // the args will substitute 2.0 for "x"
    /// // and 3.0 for "y"
    /// let args = hashmap!{
    ///     "x" => 2.0,
    ///     "y" => 3.0  
    /// };
    /// 
    /// let result = function.evaluate(&args).unwrap();
    /// let expected = 5.0;
    /// 
    /// assert_eq!(result, expected);
    /// ```
    fn evaluate(&self, args: &HashMap<&str, f64>) -> Result<f64, EvalError>;

    ///# Functionality
    /// Takes derivative of function with respect to a variable <br>
    /// All other variables will be treated like constants
    /// 
    /// The function shouldn't return self <br>
    /// Instead use .clone() to make new object
    /// 
    ///# Example
    /// ```
    /// let function = AddFn::new("x", "y");
    /// 
    /// let derivative = function.derivative("x");
    /// 
    /// // derivative doesn't require any arguments
    /// // since the derivative of x + y is 1 + 0
    /// let args = hashmap!();
    /// 
    /// assert_eq!(1.0, derivative.evaluate(&args));
    /// ```
    fn derivative(&self, variable: &str) -> ChildFn;

    ///# Functionality
    /// Takes nth derivative of function with respect to a `variable` <br>
    fn nth_derivative(&self, variable: &str, n: u32) -> ChildFn {
        let mut result = self.derivative(variable);

        for _ in 0..n-1 {
            result = result.derivative(variable);
        }
        result.to_child_fn()
    }

    ///# Functionality
    /// Replace all matched variables with possibly non constant ChildFn
    /// 
    /// The function shouldn't return self <br>
    /// Instead use .clone() to make new object
    /// 
    ///# Example
    /// ```
    /// let
    /// ```
    fn substitute(&self, args: &HashMap<&str, ChildFn>) -> ChildFn;

    ///# Functionality
    /// If you implement this function your self you don't have to implement some functions<br>
    /// Functions that get_type will work on: `depends_on`, `get_string_tree`
    /// 
    /// In `get_type` function you have to specify type of your function <br>
    /// The type depends on how many 'children' your struct can have
    /// 
    /// If you use implicit definition of function that requires get_type function to be implemented <br>
    /// Or if this function returns `None` type your program will panic
    /// 
    ///# Example
    /// ```
    /// impl Function for AddFn {
    ///     fn get_type(&self) -> FunctionType {
    ///         // AddFn has binary type because it has two children: left + right side
    ///         // You have to pass in the references to children, so other functions can use it
    ///         FunctionType::Binary(&self.left, &self.right)
    ///     }
    ///     // ...
    /// }
    /// ```
    fn get_type(&self) -> FunctionType {
        FunctionType::None
    }
    
    ///# Functionality
    /// Simply returns string representation of `self` object
    /// 
    /// This function don't format the output (output has only 1 line)
    fn get_string_tree(&self) -> String {
        use FunctionType::*;

        let name = type_of(&self);
        
        let body = match self.get_type() {
            Unary(c) => c.get_string_tree(),
            Binary(l, r) => format!{
                "{}, {}",
                l.get_string_tree(),
                r.get_string_tree(),
            },
            Variadic(v) => {
                let mut result = "".to_string();

                for c in v {
                    result += &c.get_string_tree();
                    result += ",";
                }
                result.truncate(result.len() - 1);

                result
            },
            None => panic!("{}", FUNCTION_TYPE_ERROR)
        };

        format!("{} {{ {} }}", name, body)
    }

    ///# Functionality
    /// Checks if `self` object is dependent on variable
    /// 
    ///# Example
    /// ```
    /// let function = AddFn::new("x", "y");
    /// 
    /// let depends_on_x = function.depends_on("x");
    /// let depends_on_z = function.depends_on("z");
    /// 
    /// assert!(depends_on_x);
    /// 
    /// // this assertion will fail
    /// assert!(depends_on_z);
    /// ```
    fn depends_on(&self, variable: &str) -> bool {
        use FunctionType::*;

        match self.get_type() {
            Unary(c) => c.depends_on(variable),
            Binary(l, r) => {
                let left = l.depends_on(variable);
                let right = r.depends_on(variable);
                left || right
            },
            Variadic(v) =>  {
                v.iter()
                    .any(|c| c.depends_on(variable))
            }
            None => panic!("{}", FUNCTION_TYPE_ERROR)
        }
    }

    ///# Functionality
    /// Sums function over range of values <br>
    /// evaluated function is treated as a function of `variable`
    fn sum(&self, variable: &str, range: RangeInclusive<i64>) -> Result<f64, EvalError> {
        let mut result = 0.0;

        for i in range {
            let args = hashmap! {
                variable => i as f64
            };
            result += self.evaluate(&args)?;
        }
        Ok(result)
    }
}




///# Functionality
/// Helps recognizing how many children struct has <br>
/// Stores references to all the children
pub enum FunctionType<'a> {

    /// Default type of struct <br>
    /// Struct with this type can't use implicit definitions of some functions in `Function` trait 
    None,

    /// Has only one child
    Unary(&'a ChildFn),

    /// Has two children <br>
    /// often usecase is binary operators: `+ - * /`
    Binary(&'a ChildFn, &'a ChildFn),

    /// You can use it if you have 1 or more children <br>
    /// Recommended only for structs that have 3 or more children <br>
    /// 
    /// Works even if number of children can be changed
    Variadic(&'a Vec<ChildFn>)
}

///# Functionality
/// Enum with all errors that can occur during evaluating function <br>
/// contains only errors for elementary functions
#[derive(Debug)]
pub enum EvalError {
    /// Error that occurs when you divide by zero
    DivisionByZeroError,

    /// Error that occurs when you try to take square root of negative number
    NegativeEvenRootError,

    /// Error that occurs when you try to take logarithm of non positive number
    NonPositiveLogArgError,

    /// Error that occurs when you try to take logarithm with non positive base
    NonPositiveLogBaseError,

    /// Error that occurs when you try to take logarithm with base 1
    LogBaseOneError,

    /// Error that occurs when you try to take root of negative number with non integer exponent
    NegativeBaseNonIntegerExponentError,

    /// Error that occurs when you try to take tangent of pi/2 or its multiples
    TanAtPiOverTwoError,

    /// Error that occurs when you try to access parameter that doesn't exist
    ParameterNotFoundError(String),
}

impl PartialEq for EvalError {
    fn eq(&self, other: &Self) -> bool {
        self == other
    }
}


/// Type used for fields like `child` or `exponent` ... <br>
/// Can store function, variable or constant
pub enum ChildFn {
    Fn(Box<dyn Function>),
    Var(String),
    Const(f64)
}


impl ChildFn {
    ///# Functionality
    /// checks if `self` is function
    pub fn is_function(&self) -> bool {
        matches!(self, Fn(_))
    }

    ///# Functionality
    /// checks if `self` is variable
    pub fn is_var(&self) -> bool {
        matches!(self, Var(_))
    }

    ///# Functionality
    /// checks if `self` is constant
    pub fn is_const(&self) -> bool {
        matches!(self, Const(_))
    }
}

impl Default for ChildFn {
    fn default() -> Self {
        Const(0.0)
    }
}

impl Clone for ChildFn {
    fn clone(&self) -> Self {
        match self {
            Fn(f) => ChildFn::Fn(f.clone_box()),
            Var(v) => ChildFn::Var(v.clone()),
            Const(c) => ChildFn::Const(*c),
        }
    }
}

impl Function for ChildFn {
    fn clone_box(&self) -> Box<dyn Function> {
        Box::new(self.clone())
    }

    fn evaluate(&self, args: &HashMap<&str, f64>) -> Result<f64, EvalError> {
        match self {
            Fn(f) => f.evaluate(args),
            Const(c) => Ok(*c),
            Var(v) => {
                match args.get(v.as_str()) {
                    Some(&value) => Ok(value),
                    _ => Err(ParameterNotFoundError(v.clone()))
                }
            },
        }
    }

    fn substitute(&self, args: &HashMap<&str, ChildFn>) -> ChildFn {
        match self {
            Const(_) => self.clone(),
            Fn(f) => f.substitute(args),
            Var(v) => {
                if let Some(c) = args.get(&**v) {
                    return c.clone().to_child_fn()
                }
                self.clone()
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


///# Functionality
/// Struct that represents function that is sum of only two expressions
/// 
///# Example
/// ```
/// let function = AddFn::new("x", "y");
/// 
/// let args = hashmap!{
///     "x" => 2.0,
///     "y" => 3.0
/// };
/// 
/// let result = function.evaluate(&args).unwrap();
/// let expected = 5.0;
/// 
/// assert_eq!(result, expected);
/// ```
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

    fn evaluate(&self, args: &HashMap<&str, f64>) -> Result<f64, EvalError> {
        let left = self.left.evaluate(args)?;
        let right = self.right.evaluate(args)?;

        Ok(left + right)
    }

    fn substitute(&self, args: &HashMap<&str, ChildFn>) -> ChildFn {
        let left = self.left.substitute(args);
        let right = self.right.substitute(args);

        Self::new(left, right).to_child_fn()
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


///# Functionality
/// Struct that represents function that is difference of only two expressions
/// 
/// # Example
/// ```
/// let function = SubFn::new("x", "y");
/// 
/// let args = hashmap!{
///     "x" => 2.0,
///     "y" => 3.0
/// };
/// 
/// let result = function.evaluate(&args).unwrap();
/// let expected = -1.0;
/// 
/// assert_eq!(result, expected);
/// ```
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

    fn evaluate(&self, args: &HashMap<&str, f64>) -> Result<f64, EvalError> {
        let left = self.left.evaluate(args)?;
        let right = self.right.evaluate(args)?;

        Ok(left - right)
    }

    fn substitute(&self, args: &HashMap<&str, ChildFn>) -> ChildFn {
        let left = self.left.substitute(args);
        let right = self.right.substitute(args);

        Self::new(left, right).to_child_fn()
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


///# Functionality
/// Struct that represents function that is product of only two expressions
/// 
/// # Example
/// ```
/// let function = MulFn::new("x", "y");
/// 
/// let args = hashmap!{
///     "x" => 2.0,
///     "y" => 3.0
/// };
/// 
/// let result = function.evaluate(&args).unwrap();
/// let expected = 6.0;
/// 
/// assert_eq!(result, expected);
/// ```
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

    fn evaluate(&self, args: &HashMap<&str, f64>) -> Result<f64, EvalError> {
        let left = self.left.evaluate(args)?;
        let right = self.right.evaluate(args)?;

        Ok(left * right)
    }

    fn substitute(&self, args: &HashMap<&str, ChildFn>) -> ChildFn {
        let left = self.left.substitute(args);
        let right = self.right.substitute(args);

        Self::new(left, right).to_child_fn()
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


///# Functionality
/// Struct that represents function that is division of only two expressions
/// 
/// # Example
/// ```
/// let function = DivFn::new("x", "y");
/// 
/// let args = hashmap!{
///     "x" => 2.0,
///     "y" => 3.0
/// };
/// 
/// let result = function.evaluate(&args).unwrap();
/// let expected = 2.0 / 3.0;
/// 
/// assert_eq!(result, expected);
/// ```
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

    fn evaluate(&self, args: &HashMap<&str, f64>) -> Result<f64, EvalError> {
        let num_value = self.numerator.evaluate(args)?;
        let den_value = self.denominator.evaluate(args)?;

        if den_value == 0.0 {
            return Err(DivisionByZeroError)
        }
        Ok(num_value / den_value)
    }

    fn substitute(&self, args: &HashMap<&str, ChildFn>) -> ChildFn {
        let num = self.numerator.substitute(args);
        let denom = self.denominator.substitute(args);

        Self::new(num, denom).to_child_fn()
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


///# Functionality
/// Struct that represents function that has coefficient in front of it
/// 
/// # Example
/// ```
/// let function = CoefFn::new(2.0, "x");
/// 
/// let args = hashmap!{
///     "x" => 3.0
/// };
/// 
/// let result = function.evaluate(&args).unwrap();
/// let expected = 6.0;
/// 
/// assert_eq!(result, expected);
/// ```
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

    fn evaluate(&self, args: &HashMap<&str, f64>) -> Result<f64, EvalError> {
        let child_value = self.child.evaluate(args)?;

        Ok(self.coefficient * child_value)
    }

    fn substitute(&self, args: &HashMap<&str, ChildFn>) -> ChildFn {
        let coef = self.coefficient.clone();
        let child = self.child.substitute(args);

        Self::new(coef, child).to_child_fn()
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


///# Functionality
/// Struct that represents function that is exponentiation of two expressions
/// 
/// # Example
/// ```
/// let function = ExpFn::new("x", "y");
/// 
/// let args = hashmap!{
///     "x" => 2.0,
///     "y" => 3.0
/// };
/// 
/// let result = function.evaluate(&args).unwrap();
/// let expected = 8.0;
/// 
/// assert_eq!(result, expected);
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

    fn evaluate(&self, args: &HashMap<&str, f64>) -> Result<f64, EvalError> {
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

    fn substitute(&self, args: &HashMap<&str, ChildFn>) -> ChildFn {
        let base = self.base.substitute(args);
        let exp = self.exponent.substitute(args);

        Self::new(base, exp).to_child_fn()
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


///# Functionality
/// Struct that represents function that is logarithm of expression with base
/// 
/// # Example
/// ```
/// let function = LogFn::new("x", "y");
/// 
/// let args = hashmap!{
///     "x" => 2.0
///     "y" => 8.0
/// };
/// 
/// let result = function.evaluate(&args).unwrap();
/// let expected = 3.0;
/// 
/// assert_eq!(result, expected);
/// ```
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

    fn evaluate(&self, args: &HashMap<&str, f64>) -> Result<f64, EvalError> {
        let base_value = self.base.evaluate(args)?;
        let arg_value = self.argument.evaluate(args)?;

        match (base_value, arg_value) {
            (_, a) if a <= 0.0 => Err(NonPositiveLogArgError),
            (b, _) if b <= 0.0 => Err(NonPositiveLogBaseError),
            (b, _) if b == 1.0 => Err(LogBaseOneError),
            _ => Ok(arg_value.log(base_value))
        }
    }

    fn substitute(&self, args: &HashMap<&str, ChildFn>) -> ChildFn {
        let base = self.base.substitute(args);
        let arg = self.argument.substitute(args);

        Self::new(base, arg).to_child_fn()
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


///# Functionality
/// Struct that represents function that is sine of expression
/// 
/// # Example
/// ```
/// let function = SinFn::new("x");
/// 
/// let args = hashmap!{
///     "x" => 0.0
/// };
/// 
/// let result = function.evaluate(&args).unwrap();
/// let expected = 0.0;
/// 
/// assert_eq!(result, expected);
/// ```
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

    fn evaluate(&self, args: &HashMap<&str, f64>) -> Result<f64, EvalError> {
        self.child
            .evaluate(args)
            .map(f64::sin)
            .map(utilities::round)
    }

    fn substitute(&self, args: &HashMap<&str, ChildFn>) -> ChildFn {
        let child = self.child.substitute(args);
        Self::new(child).to_child_fn()
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


///# Functionality
/// Struct that represents function that is cosine of expression
/// 
/// # Example
/// ```
/// let function = CosFn::new("x");
/// 
/// let args = hashmap!{
///     "x" => 0.0
/// };
/// 
/// let result = function.evaluate(&args).unwrap();
/// let expected = 1.0;
/// 
/// assert_eq!(result, expected);
/// ```
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

    fn evaluate(&self, args: &HashMap<&str, f64>) -> Result<f64, EvalError> {
        self.child
            .evaluate(args)
            .map(f64::cos)
            .map(utilities::round)
    }

    fn substitute(&self, args: &HashMap<&str, ChildFn>) -> ChildFn {
        let child = self.child.substitute(args);
        Self::new(child).to_child_fn()
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


///# Functionality
/// Struct that represents function that is tangent of expression
/// 
/// # Example
/// ```
/// let function = TanFn::new("x");
/// 
/// let args = hashmap!{
///     "x" => 0.0
/// };
/// 
/// let result = function.evaluate(&args).unwrap();
/// let expected = 0.0;
/// 
/// assert_eq!(result, expected);
/// ```
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

    fn evaluate(&self, args: &HashMap<&str, f64>) -> Result<f64, EvalError> {
        let child_value = self.child.evaluate(args)?;

        if child_value == FRAC_PI_2 {
            return Err(TanAtPiOverTwoError)
        }
        Ok(utilities::round(child_value.tan()))
    }

    fn substitute(&self, args: &HashMap<&str, ChildFn>) -> ChildFn {
        let child = self.child.substitute(args);
        Self::new(child).to_child_fn()
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

///# Functionality
/// Struct that represents function that is sum of arbitrary number of expressions
/// 
/// # Example
/// ```
/// let function = SeqAddFn::new(vec!["x", "y", "z"]);
/// 
/// let args = hashmap!{
///     "x" => 1.0,
///     "y" => 2.0,
///     "z" => 3.0
/// };
/// 
/// let result = function.evaluate(&args).unwrap();
/// let expected = 6.0;
/// 
/// assert_eq!(result, expected);
/// ```
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

    fn evaluate(&self, args: &HashMap<&str, f64>) -> Result<f64, EvalError> {
        let mut result: f64 = 0.0;

        for child in &self.children {
            let child_result = child.evaluate(args)?;
            result += child_result;
        }
        Ok(result)
    }

    fn substitute(&self, args: &HashMap<&str, ChildFn>) -> ChildFn {
        let children: Vec<ChildFn> = self.children.clone()
            .into_iter()
            .map(|c| c.substitute(args))
            .collect();

        Self::new(children).to_child_fn()
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


///# Functionality
/// Struct that represents function that is product of arbitrary number of expressions
/// 
/// # Example
/// ```
/// let function = SeqMulFn::new(vec!["x", "y", "z"]);
/// 
/// let args = hashmap!{
///     "x" => 1.0,
///     "y" => 2.0,
///     "z" => 3.0
/// };
/// 
/// let result = function.evaluate(&args).unwrap();
/// let expected = 6.0;
/// 
/// assert_eq!(result, expected);
/// ```
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

    fn evaluate(&self, args: &HashMap<&str, f64>) -> Result<f64, EvalError> {
        let mut result: f64 = 1.0;

        for child in &self.children {
            let child_result = child.evaluate(args)?;
            result *= child_result;
        }
        Ok(result)
    }

    fn substitute(&self, args: &HashMap<&str, ChildFn>) -> ChildFn {
        let children: Vec<ChildFn> = self.children.clone()
            .into_iter()
            .map(|c| c.substitute(args))
            .collect();

        Self::new(children).to_child_fn()
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


