

///! Used for structure like functions <br>
///! Use `f32` as one and only numeric type  


use std::{borrow::Borrow, collections::HashMap};
use crate::{impl_sequence_func, impl_variable_func};


pub const PI: f32 = 3.1415927;
pub const E: f32 = 2.7182818;

/// Type used for `child` fields
pub type BoxedFunction = Box<dyn Function>;
pub type FunctionArgs = HashMap<String, f32>;

pub trait Function{
    fn apply(&self, args: FunctionArgs) -> f32;
    //fn derivative(&self) -> Self;
}


pub struct ConstF {
    pub value: f32
}

pub struct AddF {           
    pub children: Vec<BoxedFunction>
}

pub struct MulF {
    pub children: Vec<BoxedFunction>
}

pub struct PowF {
    pub power: f32,
    pub variable: String,
    pub child: Option<BoxedFunction>
}

pub struct ExpF {
    pub base: f32,
    pub variable: String,
    pub child: Option<BoxedFunction>
}

pub struct LogF {
    pub base: f32,
    pub variable: String,
    pub child: Option<BoxedFunction>
}




impl ConstF {
    pub fn new(value: f32) -> Self {
        Self {
            value
        }
    }
}
impl Function for ConstF {
    fn apply(&self, _args: FunctionArgs) -> f32 {
        self.value
    }
}

impl_sequence_func!(AddF);

impl Function for AddF {
    fn apply(&self, args: FunctionArgs) -> f32 {
        let mut result: f32 = 0.0;

        for child in &self.children {
            let child_res = child.apply(args.clone());
            result += child_res;
        }

        result
    }
}


impl_sequence_func!(MulF);

impl Function for MulF {
    fn apply(&self, args: FunctionArgs) -> f32 {
        let mut result: f32 = 1.0;

        for child in &self.children {
            let child_res = child.apply(args.clone());
            result *= child_res;
        }

        result
    }
}


impl_variable_func!(PowF, power);

impl Function for PowF {
    fn apply(&self, args: FunctionArgs) -> f32 {
        // has child
        if let Some(child) = &self.child {
            let child_result = child.apply(args);
            return child_result.powf(self.power);
        }
        
        // has not child
        let base = args.get(&self.variable).unwrap();
        return base.powf(self.power);
    }
}


impl_variable_func!(ExpF, base);

impl Function for ExpF {
    fn apply(&self, args: FunctionArgs) -> f32 {
        // has child
        if let Some(child) = &self.child {
            let child_result = child.apply(args);
            return  self.base.powf(child_result);
        }
        
        // has not child
        let power = args.get(&self.variable).unwrap();
        return self.base.powf(*power);
    }
}


impl_variable_func!(LogF, base);

impl Function for LogF {
    fn apply(&self, args: FunctionArgs) -> f32 {
        // has child
        if let Some(child) = &self.child {
            let child_result = child.apply(args);
            return  child_result.log(self.base);
        }

        // has not child
        let log_arg = args.get(&self.variable).unwrap();
        return log_arg.log(self.base);
    }
}


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::utilities::function_utils::Boxed;

    use super::*;

    

    #[test]
    fn test_PowF() {
        let var1 = String::from("x");
        let var2 = String::from("y");

        let mut pow_func1 =  PowF::new(2.0, var1.clone());
        let pow_func2 =      PowF::new(3.0, var2.clone());

        let mut args1: FunctionArgs = HashMap::new();
        args1.insert(var1,          4.0);
        args1.insert(var2.clone(),  5.0);

        assert_eq!(pow_func1.apply(args1.clone()), 16.0);
        assert_eq!(pow_func2.apply(args1), 125.0);


        pow_func1.set_child(pow_func2.boxed());

        let mut args2: FunctionArgs = HashMap::new();
        args2.insert(var2, 2.0);

        assert_eq!(pow_func1.apply(args2), 64.0);
    }

    #[test]
    fn test_AddF() {
        let (w, x, y, z) = ("w", "x", "y", "z");
        
        let f1 = PowF::new(1.0, w).boxed();
        let f2 = PowF::new(2.0, x).boxed();
        let f3 = PowF::new(3.0, y).boxed();
        let f4 = PowF::new(4.0, z).boxed();

        let add_func = AddF::new(vec![f1, f2, f3, f4]);

        let mut args: FunctionArgs = HashMap::new();

        args.insert(w.to_string(), 1.0);
        args.insert(x.to_string(), 2.0);
        args.insert(y.to_string(), 3.0);
        args.insert(z.to_string(), 4.0);


        assert_eq!(add_func.apply(args), 288.0);
    }

    #[test]
    fn test_MulF() {
        let (w, x, y, z) = ("w", "x", "y", "z");
        
        let f1 = PowF::new(1.0, w).boxed();
        let f2 = PowF::new(2.0, x).boxed();
        let f3 = PowF::new(3.0, y).boxed();
        let f4 = PowF::new(4.0, z).boxed();

        let add_func = MulF::new(vec![f1, f2, f3, f4]);

        let mut args: FunctionArgs = HashMap::new();

        args.insert(w.to_string(), 1.0);
        args.insert(x.to_string(), 2.0);
        args.insert(y.to_string(), 3.0);
        args.insert(z.to_string(), 4.0);


        assert_eq!(add_func.apply(args), 27648.0);
    }
}