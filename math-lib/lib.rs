
/// module for math <br>
/// uses f32 as one and only numeric type
pub mod my_math {
    use std::ops::{
        Add, Sub, Mul, Div,
        AddAssign,
    };

    pub const PI: f32 = 3.1415927;
    pub const E: f32 = 2.7182818;

    // pub struct DebugInfo {
    //     /// Warns you when undefined value is computed
    //     pub undefined_value_warning: bool,

    //     /// Print all Values that was computed <br>
    //     /// That includes nested function structs (`FuncObject`)
    //     pub print_computed_values: bool,
        
    // }

    // /// Allows you to print hidden calculations to console <br>
    // /// All fields are `false` by default
    // pub static mut DEBUG_INFO: DebugInfo = DebugInfo {
    //     undefined_value_warning: false,
    //     print_computed_values: false,
    // };



    /// Holds `f32` or it's `Undefined`
    /// 
    /// Not designed as alternative to `f32` <br>
    /// Doesn't have all functions that `f32` has
    /// 
    /// Designed for `my_math` module 
    #[derive(Clone, Debug, PartialEq)]
    pub enum Value {
        Num(f32),
        Undefined,
    }

    pub const UNDEFINED: Value = Value::Undefined; 
    

    impl Value {

        /// Unwraps `Value` into `f32` <br>
        /// Panics if the value is `Undefined`
        pub fn unwrap(&self) -> f32 {
            if let Value::Num(n) = self {
                return n.clone();
            }
            panic!("The unwrap method for 'Value' type wasn't succesfull");
        }
    }

    impl Add for Value {
        type Output = Self;

        fn add(self, other: Self) -> Self::Output {
            match (self, other) {
                (Value::Num(n), Value::Num(m)) => {
                    Value::Num(n + m)
                }
                _ => Value::Undefined
            }
        }
    }

    impl AddAssign for Value {
        fn add_assign(&mut self, rhs: Self) {
            *self = self.clone() + rhs;
        }
    }

    impl Sub for Value {
        type Output = Self;

        fn sub(self, other: Self) -> Self::Output {
            match (self, other) {
                (Value::Num(n), Value::Num(m)) => {
                    Value::Num(n - m)
                }
                _ => Value::Undefined
            }
        }
    }

    impl Mul for Value {
        type Output = Self;

        fn mul(self, other: Self) -> Self::Output {
            match (self, other) {
                (Value::Num(n), Value::Num(m)) => {
                    Value::Num(n * m)
                }
                _ => Value::Undefined
            }
        }
    }

    impl Div for Value {
        type Output = Self;

        fn div(self, other: Self) -> Self::Output {
            match (self, other) {
                (Value::Num(n), Value::Num(m)) if m != 0.0 => {
                    Value::Num(n / m)
                }
                _ => Value::Undefined
            }
        }
    }

    impl Into<Value> for f32 {
        fn into(self) -> Value {
            Value::Num(self)
        }    
    }





    pub trait Func {
        fn get_variables(&self) -> Vec<String>;
        fn apply(&self, args: &Vec<Value>) -> Value;
    }

    mod utilities {
        use super::*;

        /// Use to warn about invalid arguments <br>
        /// This doesn't throw error
        /// 
        /// Returns `Value::Undefined`
        pub fn invalid_arguments(args_len: usize, var_len: usize) -> Value {
            println!("Invalid number of arguments provided: {args_len}, expected: {var_len}");
            return Value::Undefined;
        }
    }

    pub mod func_structs {
        use super::{
            *,
            utilities::*,
        };


        type FuncObject = Box<dyn Func>;
        


        pub struct PowFunc {
            pub power: f32,

            variables: Vec<String>,
            child: Option<FuncObject>
        }

        pub struct AddFunc { 
            variables: Vec<String>,          
            children: Vec<FuncObject>
        }

    


        impl PowFunc {
            /// Initialise new `PowFunc` with no child
            pub fn new(power: f32, variable: impl Into<String>) -> Self {
                PowFunc {
                    power,
                    variables: vec![variable.into()],
                    child: None,
                }
            }

            pub fn set_child(&mut self, child: FuncObject) {
                self.variables = child.get_variables();
                self.child = Some(child);
            }

            pub fn remove_child(&mut self, variable: String) {
                self.child = None;
                self.variables = vec![variable];
            }
        }
    
        impl Func for PowFunc {
            fn get_variables(&self) -> Vec<String> {
                self.variables.clone()
            }
            
            fn apply(&self, args: &Vec<Value>) -> Value {
                let var_len = self.variables.len();
                let args_len = args.len();

                if args_len != var_len {
                    return invalid_arguments(args_len, var_len);
                }


    
                // has child
                if let Some(child) = &self.child {
                    let child_result = child.apply(args);

                    if let Value::Num(base) = child_result {
                        return Value::Num(base.powf(self.power));
                    }
                }
                
                // has not child
                if let Value::Num(base) = &args[0] {
                    return Value::Num(base.powf(self.power));
                }



                Value::Undefined
            }
        }

        

        impl AddFunc {
            pub fn new(children: Vec<FuncObject>) -> Self {
                AddFunc {
                    variables: vec![],
                    children
                }
            }

            // pub fn add_child(&mut self, child: FuncObject) {
            //     self.children.push(child);
            // }
        }

        impl Func for AddFunc {
            fn get_variables(&self) -> Vec<String> {
                self.variables.clone()
            }

            fn apply(&self, args: &Vec<Value>) -> Value {
                let mut result = Value::Num(0.0);

                for child in &self.children {
                    let child_res = child.apply(args);
                    result += child_res;
                }

                result
            }

        }


    }
}


#[cfg(test)]
mod tests {
    use super::my_math::{
        *,
        func_structs::*,
    };

    

    #[test]
    fn test_powfunc() {
        let mut pow1 = PowFunc::new(2.0, "x");
        let pow2 = PowFunc::new(3.0, "y");

        const TEST_VAL1: Value = Value::Num(4.0);
        const TEST_VAL2: Value = Value::Num(2.0);

        assert_eq!(pow1.apply(&vec![TEST_VAL1]), Value::Num(16.0));
        assert_eq!(pow2.apply(&vec![TEST_VAL2]), Value::Num(8.0));
        assert_eq!(pow1.apply(&vec![UNDEFINED]), UNDEFINED);


        pow1.set_child(Box::new(pow2));

        assert_eq!(pow1.apply(&vec![TEST_VAL2]), Value::Num(64.0));
        assert_eq!(pow1.apply(&vec![UNDEFINED]), UNDEFINED);

    }

}