pub mod function_utils {

    use crate::functions::*;

    pub trait ToFnArgs {
        fn to_fn_args(self) -> FnArgs;
    }

    

    impl ToFnArgs for Vec<(&str, f32)> {
        fn to_fn_args(self) -> FnArgs {
            let mut args = FnArgs::new();

            for t in &self {
                args.insert(t.0.to_string(), t.1);
            }
            args
        }
    }

    impl ToFnArgs for Vec<(&str, f64)> {
        fn to_fn_args(self) -> FnArgs {
            let mut args = FnArgs::new();

            for t in &self {
                args.insert(t.0.to_string(), t.1 as f32);
            }
            args
        }
    }

    pub trait ToChildFn {
        fn to_child(self) -> ChildFn;
    }

    impl<T: Function + Sized + 'static> ToChildFn for T {
        fn to_child(self) -> ChildFn {
            ChildFn::Fn(Box::new(self))
        }
    }

    /// Used for implementing `new`, `add_child` functions <br>
    /// Use it for functions that have `children` field (not `child`)
    #[macro_export]
    macro_rules! impl_sequence_func {
        ($func_type:ty) => {
            impl $func_type {
                /// Initialise new function with no children
                pub fn new(children: Vec<ChildFn>) -> Self {
                    Self {
                        children,
                    }
                }
        
                pub fn add_child(&mut self, child: ChildFn) {
                    self.children.push(child);
                }
            }

            impl Default for $func_type {
                fn default() -> Self {
                    Self {
                        children: Vec::new()
                    }
                }
            }
        };
    }
}

pub mod parser_utils {

    /// Split string on specified delimiter, but only on surface level <br>
    /// Also removes all spaces
    pub fn split_surface(string: &str, delimiter: char) -> Vec<String> {
        let mut result = Vec::new();
        let mut start = 0;
        let mut depth = 0;

        let string = string.replace(" ", "");

        for (i, c) in string.chars().enumerate() {
            match c {
                '(' => depth += 1,
                ')' => depth -= 1,
                _ if c == delimiter && depth == 0 => {
                    result.push(string[start..i].to_string());
                    start = i + 1;
                },
                _ => {}
            }
        }
        result.push(string[start..].to_string());

        result
    }



}