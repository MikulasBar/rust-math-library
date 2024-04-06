use std::any::Any;

use crate::{antlr_parser::mathparser::FunctionContextAttrs, function_tree::FnTree, functions::*};

// trait for converting a type to a ChildFn
// used instead of Into<ChildFn> because Into cannot be used to convert Function dynamic object
pub trait ToChildFn {
    fn to_child_fn(self) -> ChildFn;
}

impl<T: Function + 'static> ToChildFn for T {
    fn to_child_fn(self) -> ChildFn {
        let tree = FnTree::new(self);
        ChildFn::Fn(Box::new(tree))
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

impl ToChildFn for String {
    fn to_child_fn(self) -> ChildFn {
        ChildFn::Var(self.into_boxed_str())
    }
}

impl<T: Function + 'static> ToChildFn for Option<T> {
    fn to_child_fn(self) -> ChildFn {
        if let Some(res) = self {
            return res.to_child_fn()
        }
        panic!("Cannot convert None to ChildFn")
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
