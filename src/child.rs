use std::collections::HashMap;

use crate::function::{
    Function,
    EvalError
};

use Child::*;



/// Type used for fields like `child` or `exponent` ... <br>
/// Can store function, variable or constant
#[derive(Debug, PartialEq, Clone)]
pub enum Child {
    Fn(Box<Function>),
    Var(Box<str>),
    Const(f64),

    // NamedFn(String, Vec<Child>),
}

impl Default for Child {
    fn default() -> Self {
        Const(0.0)
    }
}

impl Child {
    pub fn eval(&self, args: &HashMap<&str, f64>) -> Result<f64, EvalError> {
        match self {
            Fn(f) => f.eval(args),
            Const(c) => Ok(*c),
            Var(v) => {
                args.get(v.as_ref()).copied()
                    .ok_or(EvalError::ParameterNotFound(v.clone()))
            },
        }
    }

    // fn substitute(&self, args: &HashMap<&str, Child>) -> Child {
    //     match self {
    //         Const(_) => self.clone(),
    //         Fn(f) => f.substitute(args),
    //         Var(v) => {
    //             if let Some(c) = args.get(v) {
    //                 return c.clone().into()
    //             }
    //             self.clone()
    //         },
    //     }
    // }

    // fn depends_on(&self, variable: &str) -> bool {
    //     match self {
    //         Fn(f) => f.depends_on(variable),
    //         Var(v) => **v == *variable,
    //         Const(_) => false,
    //     }
    // }

    pub fn derivative(&self, var: &str) -> Child {
        match self {
            Fn(f) => f.derivative(var),
            Const(_) => (0.0).into(),
            Var(v) => {
                match v.as_ref() == var {
                    true => 1.0,
                    false => 0.0,
                }.into()
            },
            // NamedFn(name, args) => {
            //     let args = args.iter()
            //         .map(|a| a.derivative(variable))
            //         .collect();
            //     NamedFn(name.clone(), args)
            // },
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Const(c) => c.to_string(),
            Var(v) => v.to_string(),
            Fn(f) => f.to_string(),
            
            // NamedFn(name, args) => {
            //     let args = args.iter()
            //         .map(|a| a.to_string())
            //         .collect::<Vec<String>>()
            //         .join(", ");
            //     format!("{}({})", name, args)
            // },
        }
    }
}





impl From<Function> for Child {
    #[inline]
    fn from(f: Function) -> Self {
        Child::Fn(Box::new(f))
    }
}

impl From<f64> for Child {
    #[inline]
    fn from(c: f64) -> Self {
        Child::Const(c)
    }
}

impl From<&str> for Child {
    #[inline]
    fn from(v: &str) -> Self {
        Child::Var(Box::from(v))
    }
}

impl From<String> for Child {
    #[inline]
    fn from(v: String) -> Self {
        Child::Var(v.into_boxed_str())
    }
}



// pub trait ToChild {
//     fn into(self) -> Child;
// }

// impl ToChild for Child {
//     #[inline]
//     fn into(self) -> Child {
//         self
//     }
// }

// impl ToChild for Function {
//     #[inline]
//     fn into(self) -> Child {
//         Child::Fn(Box::new(self))
//     }
// }

// impl ToChild for f64 {
//     #[inline]
//     fn into(self) -> Child {
//         Child::Const(self)
//     }
// }

// impl ToChild for &str {
//     #[inline]
//     fn into(self) -> Child {
//         Child::Var(self.to_string())
//     }
// }

// impl ToChild for String {
//     #[inline]
//     fn into(self) -> Child {
//         Child::Var(self)
//     }
// }

