use std::collections::HashMap;
use std::ops::RangeInclusive;
use maplit::hashmap;

use crate::child::*;
use crate::utils::type_of;



///# Functionality
/// Implement this trait for your struct if you want to treat it like mathematical function
/// 
/// All functions that manipulates with objects should be implemented such that they will produce new object <br>
/// Derive `Clone` trait for your struct <br>
/// 
/// Override implicit definition of `get_type` if you can <br>
/// If you do that some functions don't have to be implemented
pub trait FnBehaviour {

    ///# Functionality
    /// Clones `self` and wraps it in `Box`
    /// 
    /// Derive `Clone` trait for your struct, so you can implement this function <br> 
    /// This function needs to be implemented, so `Child` can be cloned
    fn clone_box(&self) -> Box<dyn FnBehaviour>;

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
    fn derivative(&self, variable: &str) -> Child;

    ///# Functionality
    /// Takes nth derivative of function with respect to a `variable` <br>
    fn nth_derivative(&self, variable: &str, n: u32) -> Child {
        let mut result = self.derivative(variable);

        for _ in 0..n-1 {
            result = result.derivative(variable);
        }
        result.to_child()
    }

    ///# Functionality
    /// Replace all matched variables with possibly non constant Child
    /// 
    /// The function shouldn't return self <br>
    /// Instead use .clone() to make new object
    /// 
    ///# Example
    /// ```
    /// let
    /// ```
    fn substitute(&self, args: &HashMap<&str, Child>) -> Child;

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
    /// impl FnBehaviour for AddFn {
    ///     fn get_type(&self) -> FnType {
    ///         // AddFn has binary type because it has two children: left + right side
    ///         // You have to pass in the references to children, so other functions can use it
    ///         FnType::Binary(&self.left, &self.right)
    ///     }
    ///     // ...
    /// }
    /// ```
    fn get_type(&self) -> FnType {
        FnType::None
    }
    
    ///# Functionality
    /// Simply returns string representation of `self` object
    /// 
    /// This function don't format the output (output has only 1 line)
    fn get_string_tree(&self) -> String {
        use FnType::*;

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
        use FnType::*;

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



/// Error you get if you use implicitly implemented function that only works if you implement `get_type` function in `Function` trait
const FUNCTION_TYPE_ERROR: &str = 
    "Function of this FnType cannot use implicit definition of the this function,
    implement this function / change function type in get_type function"
;


///# Functionality
/// Helps recognizing how many children struct has <br>
/// Stores references to all the children
pub enum FnType<'a> {

    /// Default type of struct <br>
    /// Struct with this type can't use implicit definitions of some functions in `Function` trait 
    None,

    /// Has only one child
    Unary(&'a Child),

    /// Has two children <br>
    /// often usecase is binary operators: `+ - * /`
    Binary(&'a Child, &'a Child),

    /// You can use it if you have 1 or more children <br>
    /// Recommended only for structs that have 3 or more children <br>
    /// 
    /// Works even if number of children can be changed
    Variadic(&'a Vec<Child>)
}


///# Functionality
/// Enum with all errors that can occur during evaluating function <br>
/// contains only errors for elementary functions
#[derive(Debug, PartialEq)]
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

// impl PartialEq for EvalError {
//     fn eq(&self, other: &Self) -> bool {
//         self == other
//     }
// }

