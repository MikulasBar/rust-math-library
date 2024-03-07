use crate::functions::*;

pub trait ToChildFn<'a> {
    fn to_child(&'a self) -> ChildFn<'a>;
}

impl<'a, T: Function + Sized + 'a> ToChildFn<'a> for T {
    fn to_child(&self) -> ChildFn<'a> {
        ChildFn::Fn(Box::new(self))
    }
}

impl<'a> ToChildFn<'a> for f32 {
    fn to_child(&self) -> ChildFn<'a> {
        ChildFn::Const(*self)
    }
}

impl<'a> ToChildFn<'a> for &'a str {
    fn to_child(&self) -> ChildFn<'a> {
        ChildFn::Var(self)
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
