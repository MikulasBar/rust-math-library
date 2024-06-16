
#[macro_export]
macro_rules! binary_new {
    ($func:ident, $left:ident, $right:ident) => {
        pub fn new<T, U>($left: T, $right: U) -> Function
        where
            T: ToChild,
            U: ToChild,
        {
            let left = $left.to_child();
            let right = $right.to_child();
            Function::$func(left, right)
        }
    };
}

#[macro_export]
macro_rules! unary_new {
    ($func:ident, $child:ident) => {
        pub fn new<T>($child: T) -> Function
        where
            T: ToChild,
        {
            let child = $child.to_child();
            Function::$func(child)
        }
    };
}