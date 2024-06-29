use std::borrow::Borrow;
use std::collections::HashMap;
use std::rc::Rc;

use crate::function::EvalError;
use crate::function::{Function, EvalError::{FunctionNotDefined, VariableNotDefined}};

#[derive(Debug, Clone)]
pub struct Context {
    vars: Rc<HashMap<String, f64>>,
    funcs: Rc<HashMap<String, Parametric>>,
}

impl Context {
    pub fn new() -> Self {
        Context {
            vars: Rc::new(HashMap::new()),
            funcs: Rc::new(HashMap::new()),
        }
    }

    fn get_components(&self) -> (Rc<HashMap<String, f64>>, Rc<HashMap<String, Parametric>>) {
        (
            Rc::clone(&self.vars),
            Rc::clone(&self.funcs)
        )
    }

    pub fn merge(&self, other: &Context) -> Context {
        todo!();

        let mut new_ctx = Context::new();

        let (vars, funcs) = self.get_components();
        let (other_vars, other_funcs) = other.get_components();

        let mut vars = vars;
        let mut funcs = funcs;

        vars.extend(other_vars.into_iter());

        new_ctx.vars = Rc::clone(&vars);
        new_ctx.funcs = Rc::clone(&funcs);

        new_ctx
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
            vars: Rc::new(vars),
            funcs: Rc::new(HashMap::new()),
        }
    }    
}

impl From<(Vec<String>, Vec<f64>)> for Context {
    fn from((params, values): (Vec<String>, Vec<f64>)) -> Self {
        let vars: HashMap<String, f64> = params.into_iter()
            .zip(values.into_iter())
            .collect();

        Context {
            vars: Rc::new(vars),
            funcs: Rc::new(HashMap::new()),
        }
    }
}

pub type Parametric = (Vec<String>, Function);
