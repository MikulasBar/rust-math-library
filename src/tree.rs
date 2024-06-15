use std::collections::HashMap;

use crate::{
    child::*,
    fn_behaviour::*,
};


#[derive(Clone)]
pub struct FnTree {
    definition: Child,
    string_tree: String,
}

impl FnTree {
    pub fn new<T>(definition: T) -> Self 
    where
        T: ToChild
    {
        let mut fn_tree = Self {
            definition: definition.to_child(),
            string_tree: "".to_string()
        };
        fn_tree.string_tree = fn_tree.definition.get_string_tree();
        fn_tree
    }
}

impl FnBehaviour for FnTree {
    fn clone_box(&self) -> Box<dyn FnBehaviour> {
        Box::new(self.clone())
    }

    fn evaluate(&self, args: &HashMap<&str, f64>) -> Result<f64, EvalError> {
        self.definition.evaluate(args)
    }

    fn substitute(&self, args: &HashMap<&str, Child>) -> Child {
        self.definition.substitute(args)
    }

    fn get_string_tree(&self) -> String {
        self.string_tree.clone()
    }

    fn derivative(&self, variable: &str) -> Child {
        self.definition.derivative(variable)
    }
}


