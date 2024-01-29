

/// Used for structure like functions <br>
/// Use `f32` as one and only numeric type  
pub mod functions {
    use std::{collections::HashMap};
    use crate::{impl_sequence_func, impl_variable_func};

    //use super::super::utilities::*;

    pub const PI: f32 = 3.1415927;
    pub const E: f32 = 2.7182818;

    /// Type used for `child` fields
    pub type BoxedFunc = Box<dyn Func>;
    pub type FuncArgs = HashMap<String, f32>;

    pub trait Func {
        fn apply(&self, args: &FuncArgs) -> f32;
    }



    pub struct AddFunc {           
        children: Vec<BoxedFunc>
    }

    pub struct MulFunc {
        children: Vec<BoxedFunc>
    }

    pub struct PowFunc {
        pub variable: String,
        pub power: f32,

        child: Option<BoxedFunc>
    }

    pub struct ExpFunc {
        pub variable: String,
        pub base: f32,

        child: Option<BoxedFunc>
    }

    pub struct LogFunc {
        pub variable: String,
        pub base: f32,

        child: Option<BoxedFunc>
    }





    impl_sequence_func!(AddFunc);

    impl Func for AddFunc {
        fn apply(&self, args: &FuncArgs) -> f32 {
            let mut result: f32 = 0.0;

            for child in &self.children {
                let child_res = child.apply(args);
                result += child_res;
            }

            result
        }
    }
    

    impl_sequence_func!(MulFunc);

    impl Func for MulFunc {
        fn apply(&self, args: &FuncArgs) -> f32 {
            let mut result: f32 = 1.0;

            for child in &self.children {
                let child_res = child.apply(args);
                result *= child_res;
            }

            result
        }
    }


    impl_variable_func!(PowFunc, power);

    impl Func for PowFunc {
        fn apply(&self, args: &FuncArgs) -> f32 {
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


    impl_variable_func!(ExpFunc, base);

    impl Func for ExpFunc {
        fn apply(&self, args: &FuncArgs) -> f32 {
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


    impl_variable_func!(LogFunc, base);

    impl Func for LogFunc {
        fn apply(&self, args: &FuncArgs) -> f32 {
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
} 


#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use super::functions::{
        *,
    };

    

    #[test]
    fn test_powfunc() {
        let var1 = String::from("x");
        let var2 = String::from("y");

        let mut pow_func1 =  PowFunc::new(2.0, var1.clone());
        let pow_func2 =      PowFunc::new(3.0, var2.clone());

        let mut args1: FuncArgs = HashMap::new();
        args1.insert(var1,          4.0);
        args1.insert(var2.clone(),  5.0);

        assert_eq!(pow_func1.apply(&args1),  16.0);
        assert_eq!(pow_func2.apply(&args1),  125.0);


        pow_func1.set_child(Box::new(pow_func2));

        let mut args2: FuncArgs = HashMap::new();
        args2.insert(var2, 2.0);

        assert_eq!(pow_func1.apply(&args2), 64.0);
    }

    #[test]
    fn test_addfunc() {
        let (w, x, y, z) = ("w", "x", "y", "z");
        
        let f1 = Box::new(PowFunc::new(1.0, w));
        let f2 = Box::new(PowFunc::new(2.0, x));
        let f3 = Box::new(PowFunc::new(3.0, y));
        let f4 = Box::new(PowFunc::new(4.0, z));

        let add_func = AddFunc::new(vec![f1, f2, f3, f4]);

        let mut args: FuncArgs = HashMap::new();

        args.insert(w.to_string(), 1.0);
        args.insert(x.to_string(), 2.0);
        args.insert(y.to_string(), 3.0);
        args.insert(z.to_string(), 4.0);


        assert_eq!(add_func.apply(&args), 288.0);
    }

    #[test]
    fn test_mulfunc() {
        let (w, x, y, z) = ("w", "x", "y", "z");
        
        let f1 = Box::new(PowFunc::new(1.0, w));
        let f2 = Box::new(PowFunc::new(2.0, x));
        let f3 = Box::new(PowFunc::new(3.0, y));
        let f4 = Box::new(PowFunc::new(4.0, z));

        let add_func = MulFunc::new(vec![f1, f2, f3, f4]);

        let mut args: FuncArgs = HashMap::new();

        args.insert(w.to_string(), 1.0);
        args.insert(x.to_string(), 2.0);
        args.insert(y.to_string(), 3.0);
        args.insert(z.to_string(), 4.0);


        assert_eq!(add_func.apply(&args), 27648.0);
    }
}