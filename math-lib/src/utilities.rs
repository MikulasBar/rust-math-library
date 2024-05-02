use std::{
    any::type_name,
};

use crate::{
    functions::*,
};

// trait for converting a type to a ChildFn
// used instead of Into<ChildFn> because Into cannot be used to convert Function dynamic object
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


pub fn type_of<T>(_: T) -> &'static str {
    let full_name = type_name::<T>();
    full_name.split("::").last().unwrap()
}


