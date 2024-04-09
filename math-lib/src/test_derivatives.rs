#![allow(non_snake_case, unused)]

use crate::{
    fn_args, functions::*
};

#[test]
fn test_derivative_AddFn() {
    let func = AddFn::new("x", "y");
    let dfunc = func.derivative("x");

    let args = fn_args!(
        "x" => 8,
        "y" => 6,
    );

    assert_eq!(func.apply(&args), Ok(14.0));
    assert_eq!(dfunc.apply(&args), Ok(7.0));
}

#[test]
fn test_derivative_CoefFn() {
    let func = CoefFn::new(5.0, "x");
    let dfunc = func.derivative("x");

    let args = fn_args!(
        "x" => 6
    );

    assert_eq!(func)
}


// #[test]
// fn test_string_three() {
//     let f = AddFn::new("x", "y");

//     let string = f.get_string_tree();
//     println!("{}", string);
// }