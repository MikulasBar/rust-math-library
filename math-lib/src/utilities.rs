use std::any::Any;

use crate::functions::*;

// trait for converting a type to a ChildFn
// used instead of Into<ChildFn> because Into cannot be used to convert Function trait
pub trait ToChildFn {
    fn to_child_fn(self) -> ChildFn;
}

impl<T: Function + 'static> ToChildFn for T {
    fn to_child_fn(self) -> ChildFn {
        ChildFn::Fn(Box::new(self))
    }
}

impl ToChildFn for f64 {
    fn to_child_fn(self) -> ChildFn {
        ChildFn::Const(self)
    }
}

impl ToChildFn for &str {
    fn to_child_fn(self) -> ChildFn {
        ChildFn::Var(Into::into(self))
    }
}




#[macro_export]
macro_rules! fn_args {
    ($($key:expr => $value:expr),* $(,)?) => {
        {
            let mut args = FnArgs::new();
            $(
                args.insert($key, $value as f64);
            )*
            args
        }
    };
}
