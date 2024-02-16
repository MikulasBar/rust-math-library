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
    

    /// Used for implementing `new`, `set_child`, `remove_child` functions <br>
    /// Use it for functions that have `variable` field
    #[macro_export]
    macro_rules! impl_variable_func {
        ($func_type:ty, $($fields:ident),* ) => {
            impl $func_type {
                /// Initialise new function with no child
                pub fn new($($fields),*: f32, variable: impl Into<String>) -> Self {
                    Self {
                        $($fields: $fields),* ,
                        variable: variable.into(),
                        child: None,
                    }
                }
        
                pub fn set_child(&mut self, child: BoxedFunction) {
                    self.child = Some(child);
                }
        
                pub fn remove_child(&mut self) {
                    self.child = None;
                }
        
                pub fn has_child(&self) -> bool {
                    if let Option::Some(_) = self.child {
                        return true;
                    }
                    false
                } 
            }
        };
    }

    /// Used for implementing `new`, `add_child` functions <br>
    /// Use it for functions that have `children` field (not `child`)
    #[macro_export]
    macro_rules! impl_sequence_func {
        ($func_type:ty) => {
            impl $func_type {
                /// Initialise new function with no children
                pub fn new(children: Vec<BoxedFunction>) -> Self {
                    Self {
                        children,
                    }
                }
        
                pub fn add_child(&mut self, child: BoxedFunction) {
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
    use std::ops::Add;

    use crate::functions::*;
    use super::function_utils::Boxed;

    
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

    /// Parse string to `BoxedFunction`
    pub fn parse_to_func(string: &str) -> BoxedFunction {
        let mut tokens = split_surface(string, '+');

        if tokens.len() == 1 {
            tokens = split_surface(string, '*');
            if tokens.len() == 1 {
                
            }
            
        } else {
            return AddF::new(
                tokens.into_iter()
                .map(|x| parse_to_func(&x))
                .collect()
            ).boxed();
        }

        return ConstF::new(0.0).boxed();
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