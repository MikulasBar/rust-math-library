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

#[macro_export]
macro_rules! unary_fn {
    ($name:ident => $function:expr) => {
        struct $name<'a> {
            child: ChildFn<'a>
        }

        impl<'a> $name<'a> {
            pub fn new<T: Into<ChildFn<'a>>>(child: T) -> Self {
                Self {
                    child: child.into()
                }
            }
        }

        impl<'a> Function for $name<'a> {
            fn apply(&self, args: &FnArgs) -> FnResult {
                let child_result = self.child.apply(args);

                $function(child_result)
            }
        }
    };
    (list: $($name:ident => $function:expr),+) => {
        $(unary_fn!($name => $function);)+
    }
}
