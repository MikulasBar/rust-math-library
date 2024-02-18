pub mod function_utils {
    use crate::functions::*;

    /// Trait for everything that can be boxed
    pub trait Boxed {
        /// Create a boxed version of the object
        fn boxed(self) -> Box<Self>;
    }
    
    impl<T: Function + Sized> Boxed for T {
        fn boxed(self) -> Box<Self> {
            Box::new(self)
        }
    }

    pub trait ToChildFn {
        fn to_child(self) -> ChildFn;
    }

    impl<T: Function + Sized + 'static> ToChildFn for T {
        fn to_child(self) -> ChildFn {
            ChildFn::Fn(self.boxed())
        }
    }

    pub fn apply_on_result(res: FnResult, f: impl Fn(f32) -> FnResult) -> FnResult {
        if let Ok(v) = res {
            f(v)
        } else {
            Err(())
        }
    }

    pub fn apply_on_result2(res1: FnResult, res2: FnResult, f: impl Fn(f32, f32) -> FnResult) -> FnResult {
        if let (Ok(v1), Ok(v2)) = (res1, res2) {
            f(v1, v2)
        } else {
            Err(())
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
    fn split_surface(string: &str, delimiter: char) -> Vec<String> {
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


    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_split_surface() {
            let string = "45 + 2*(78 + (x + y) + 7) / 2 + 2";
            let vec = split_surface(string, '+');
            let expected = vec!["45", "2*(78+(x+y)+7)/2", "2"];
            let expected: Vec<String> = expected.into_iter()
                .map(|x| x.to_string())
                .collect();

            assert_eq!(vec, expected);
        }
    }

}