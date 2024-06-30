
use crate::function::{
    Function,
    EvalError
};

use Child::*;
use crate::_context::Context;

// use EvalError::{
//     VariableNotDefined,
//     FunctionNotDefined
// };


/// Type used for fields like `child` or `exponent` ... <br>
/// Can store function, variable or constant
#[derive(Debug, PartialEq, Clone)]
pub enum Child {
    Fn(Box<Function>),
    Var(String),
    Const(f64),

    NamedFn(String, Vec<Child>),
}

impl Default for Child {
    fn default() -> Self {
        Const(0.0)
    }
}

impl Child {
    pub fn eval(&self, ctx: &Context) -> Result<f64, EvalError> {
        match self {
            Fn(f) => f.eval(ctx),
            Const(c) => Ok(*c),
            Var(v) => ctx.get_var(v),

            NamedFn(name, args) => {
                // from parsed string we get function name and arguments
                // from context we get function body and parameters
                let (params, body) = ctx.get_func(name)?;
                
                // we use the context to evaluate the argumencts
                let values = args.into_iter()
                    .map(|a| a.eval(ctx))
                    .collect::<Result<Vec<f64>, EvalError>>()?;
            
                // then we create new context using the parameters and evaluated arguments
                let arg_ctx = Context::from((params, values));

                // note that this context shares same values as the original context
                // that is because of the Rc type used in the Context struct
                let full_ctx = ctx.merge(&arg_ctx);
            
                // then we evaluate the function body using both contexts by merging them
                body.eval(&full_ctx)
            },
        }
    }

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
                match v == var {
                    true => 1.0,
                    false => 0.0,
                }.into()
            },
            NamedFn(name, args) => {                                // TODO
                // let args = args.iter()
                //     .map(|a| a.derivative(var))
                //     .collect();
                // NamedFn(name.clone(), args)
                todo!()
            },
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Const(c) => c.to_string(),
            Var(v) => v.to_string(),
            Fn(f) => f.to_string(),
            
            NamedFn(name, args) => {                                // TODO
                // let args = args.iter()
                //     .map(|a| a.to_string())
                //     .collect::<Vec<String>>()
                //     .join(", ");
                // format!("{}({})", name, args)
                todo!()
            },
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
        Child::Var(v.to_string())
    }
}

impl From<String> for Child {
    #[inline]
    fn from(v: String) -> Self {
        Child::Var(v)
    }
}