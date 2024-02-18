use std::{collections::HashMap};
use crate::{impl_sequence_func, utilities::function_utils::{apply_on_result, apply_on_result2}};
use ChildFn::*;



pub type BoxedFn = Box<dyn Function>;
pub type FnArgs = HashMap<String, f32>;
pub type FnError = ();
pub type FnResult = Result<f32, FnError>;


pub trait Function{
    fn apply(&self, args: FnArgs) -> FnResult;
    //fn derivative(&self) -> Self;
}

/// Type used for `child` fields
pub enum ChildFn {
    Fn(BoxedFn),
    Var(String),
    Const(f32)
}

impl Function for ChildFn {
    fn apply(&self, args: FnArgs) -> FnResult {
        match self {
            Fn(f) => f.apply(args),
            Var(s) => {
                if let Some(v) = args.get(s) {
                    Ok(*v)
                } else {
                    Err(())
                }
            },
            Const(c) => Ok(*c)
        }
    }
}

impl Into<ChildFn> for BoxedFn {
    fn into(self) -> ChildFn {
        Fn(self)
    }
}

impl Into<ChildFn> for String {
    fn into(self) -> ChildFn {
        Var(self)
    }
}

impl Into<ChildFn> for &str {
    fn into(self) -> ChildFn {
        Var(self.to_string())
    }
}

impl Into<ChildFn> for f32 {
    fn into(self) -> ChildFn {
        Const(self)
    }
}



pub struct AddFn {           
    pub children: Vec<ChildFn>
}

impl_sequence_func!(AddFn);

impl Function for AddFn {
    fn apply(&self, args: FnArgs) -> FnResult {
        let mut result: f32 = 0.0;

        for child in &self.children {
            let child_res = child.apply(args.clone());
            result += child_res?;
        }

        Ok(result)
    }
}



pub struct MulFn {
    pub children: Vec<ChildFn>
}

impl_sequence_func!(MulFn);

impl Function for MulFn {
    fn apply(&self, args: FnArgs) -> FnResult {
        let mut result: f32 = 1.0;

        for child in &self.children {
            let child_result = child.apply(args.clone());
            if let Ok(child_result) = child_result {
                result *= child_result;
            } else {
                return Err(());
            }
        }
        Ok(result)
    }
}


pub struct DivFn {
    pub numerator: ChildFn,
    pub denominator: ChildFn
}

impl Function for DivFn {
    fn apply(&self, args: FnArgs) -> FnResult {
        let num_result = self.numerator.apply(args.clone());
        let den_result = self.denominator.apply(args);

        apply_on_result2(num_result, den_result, |n, d| {
            if d == 0.0 {
                Err(())
            } else {
                Ok(n / d)
            }
        })
    }
}



/// This function is used for adding coefficient without using `MulFn` <br>
pub struct CoefFn {
    pub coefficient: f32,
    pub child: ChildFn
}

impl CoefFn {
    pub fn new(coefficient: f32, child: impl Into<ChildFn>) -> Self {
        Self {
            coefficient,
            child: child.into()
        }
    }
}

impl Function for CoefFn {
    fn apply(&self, args: FnArgs) -> FnResult {
        let child_result = self.child.apply(args);
        apply_on_result(child_result, |r| Ok(self.coefficient * r))
    }
}

pub struct ExpFn {
    pub base: ChildFn,
    pub exponent: ChildFn
}

impl Function for ExpFn {
    fn apply(&self, args: FnArgs) -> FnResult {
        let base_result = self.base.apply(args.clone());
        let exp_result = self.exponent.apply(args);

        apply_on_result2(base_result, exp_result, |b, e| Ok(b.powf(e)))
    }
}



pub struct LogFn {
    pub base: ChildFn,
    pub argument: ChildFn
}

impl Function for LogFn {
    fn apply(&self, args: FnArgs) -> FnResult {
        let base_result = self.base.apply(args.clone());
        let arg_result = self.argument.apply(args);

        apply_on_result2(base_result, arg_result, |b, a| Ok(a.log(b)))
    }
}



#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::utilities::function_utils::ToChildFn;
    use super::*;


    #[test]
    fn test_AddFn() {
        let x = "x".to_string();
        let y = "y".to_string();
        
        let fx = CoefFn::new(2.0, x.clone()).to_child();
        let fy = CoefFn::new(3.0, y.clone()).to_child();

        let add_func = AddFn::new(vec![fx, fy]);

        let mut args: FnArgs = HashMap::new();

        args.insert(x, 4.0);
        args.insert(y, 5.0);


        assert_eq!(add_func.apply(args), Ok(23.0));
    }

    #[test]
    fn test_MulFn() {
        let x = "x".to_string();
        let y = "y".to_string();
        
        let fx = CoefFn::new(2.0, x.clone()).to_child();
        let fy = CoefFn::new(3.0, y.clone()).to_child();

        let mul_func = MulFn::new(vec![fx, fy]);

        let mut args: FnArgs = HashMap::new();

        args.insert(x, 4.0);
        args.insert(y, 5.0);


        assert_eq!(mul_func.apply(args), Ok(120.0));
    }
}