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