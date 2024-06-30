#![allow(dead_code)]

use maplit::hashmap;
use std::collections::HashMap;
use std::f64::consts::{PI, E};

use crate::function::EvalError;
use crate::function::{Function, EvalError::{FunctionNotDefined, VariableNotDefined}};
use crate::functions::trigonometric::{CosFn, SinFn, TanFn};

use builder::Builder;


pub fn elementary() -> Context {
    let vars = hashmap!{
        "e" => E,
        "pi" => PI,
    };

    let funcs = hashmap! {
        "sin" => (vec!["x".to_string()], SinFn::new("x")),
        "cos" => (vec!["x".to_string()], CosFn::new("x")),
        "tan" => (vec!["x".to_string()], TanFn::new("x")),
        "ln" => (vec!["x".to_string()], Function::Log(E.into(), "x".into())),
    };

    Context::builder()
        .set_vars(vars)
        .set_funcs(funcs)
        .build()
}




#[derive(Debug, Clone)]
pub struct Context {
    vars:  HashMap<String, f64>,
    funcs: HashMap<String, Parametric>,
}



impl Context {
    pub fn builder() -> Builder {
        Builder::new()
    }

    pub fn new() -> Self {
        Context {
            vars:  HashMap::new(),
            funcs: HashMap::new(),
        }
    }

    // cloning is not the best solution
    // it will be changed in the future
    fn clone_components(&self) -> (HashMap<String, f64>, HashMap<String, Parametric>) {             // TODO
        (
            self.vars.clone(),
            self.funcs.clone()
        )
    }

    pub fn merge(&self, other: &Context) -> Context {
        let (mut vars, mut funcs) = self.clone_components();
        let (other_vars, other_funcs) = other.clone_components();

        vars.extend(other_vars);

        funcs.extend(other_funcs);

        Context {
            vars,
            funcs,
        }
    }

    // pub fn add_vars(&mut self, params: Vec<String>, values: Vec<f64>) {
    //     let pairs: Vec<_> = params.into_iter()
    //         .map(Box::leak)
    //         .zip(values.into_iter())
    //         .collect();

    //     for (param, value) in pairs {
    //         self.vars.insert(param, value);
    //     }
    // }

    pub fn get_var(&self, name: &str) -> Result<f64, EvalError> {
        self.vars.get(name)
            .map_or(Err(VariableNotDefined(name.to_string())), |var| {
                Ok(*var)
            })
    }

    pub fn get_func(&self, name: &str) -> Result<Parametric, EvalError> {
        self.funcs.get(name)
            .cloned()
            .map_or(Err(FunctionNotDefined(name.to_string())), |func| {
                Ok(func)
            })
    }
}

impl From<HashMap<&str, f64>> for Context {
    fn from(vars: HashMap<&str, f64>) -> Self {
        let vars: HashMap<String, f64> = vars.into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect();

        Context {
            vars: vars,
            funcs: HashMap::new(),
        }
    }    
}

impl From<(Vec<String>, Vec<f64>)> for Context {
    fn from((params, values): (Vec<String>, Vec<f64>)) -> Self {
        let vars: HashMap<String, f64> = params.into_iter()
            .zip(values.into_iter())
            .collect();

        Context {
            vars: vars,
            funcs: HashMap::new(),
        }
    }
}


pub type Parametric = (Vec<String>, Function);

mod builder {
    use super::*;

    pub struct Builder {
        vars: HashMap<String, f64>,
        funcs: HashMap<String, Parametric>,
    }

    impl Builder {
        pub fn new() -> Self {
            Builder {
                vars: HashMap::new(),
                funcs: HashMap::new(),
            }
        }

        pub fn set_vars(mut self, vars: HashMap<impl Into<String>, f64>) -> Self {
            self.vars = vars.into_iter()
                .map(|(k, v)| (k.into(), v))
                .collect();

            self
        }

        pub fn set_funcs(mut self, funcs: HashMap<impl Into<String>, Parametric>) -> Self {
            self.funcs = funcs.into_iter()
                .map(|(k, v)| (k.into(), v))
                .collect();

            self
        }

        pub fn build(self) -> Context {
            Context {
                vars:  self.vars,
                funcs: self.funcs,
            }
        }
    }
} 

