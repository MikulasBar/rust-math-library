pub mod utilities {
    
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
        
                pub fn set_child(&mut self, child: BoxedFunc) {
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
                pub fn new(children: Vec<BoxedFunc>) -> Self {
                    Self {
                        children
                    }
                }
        
                pub fn add_child(&mut self, child: BoxedFunc) {
                    self.children.push(child);
                }
            }
        };
    }


    
    #[macro_export]
    macro_rules! lazy_regex {
        (name: $name:ident, def: $value:literal) => {
            pub static $name: Lazy<Regex> = Lazy::new(||
                Regex::new($value).unwrap()
            );
        };
    }
}
