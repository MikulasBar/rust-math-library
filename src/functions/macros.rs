

#[macro_export]
macro_rules! binary_new {
    ($func:ident, $left:ident, $right:ident) => {
        pub fn new<T, U>($left: T, $right: U) -> Function
        where
            T: Into<Child>,
            U: Into<Child>,
        {
            let left = $left.into();
            let right = $right.into();
            Function::$func(left, right)
        }
    };
}

#[macro_export]
macro_rules! unary_new {
    ($func:ident, $child:ident) => {
        pub fn new<T>($child: T) -> Function
        where
            T: Into<Child>,
        {
            let child = $child.into();
            Function::$func(child)
        }
    };
}