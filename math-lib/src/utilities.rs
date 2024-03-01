use std::collections::HashMap;

use crate::functions::*;

pub trait ToChildFn {
    fn to_child(self) -> ChildFn<'static>;
}

impl<T: Function + Sized + 'static> ToChildFn for T {
    fn to_child(self) -> ChildFn<'static> {
        ChildFn::Fn(Box::new(self))
    }
}


#[macro_export]
macro_rules! fn_args {
    ($($key:expr => $value:expr),* $(,)?) => {
        {
            let mut args = FnArgs::new();
            $(
                args.insert($key, $value as f32);
            )*
            args
        }
    };
}