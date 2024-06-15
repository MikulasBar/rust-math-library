use std::{any::type_name, f64::EPSILON};

use crate::functions::*;


/// Get type name of a variable and converts it to string format <br>
/// Example: <br>
/// ```
/// let a = 5;
/// let b = "hello";
/// let c = 5.0;
/// 
/// assert_eq!(type_of(a), "i32");
/// assert_eq!(type_of(b), "&str");
/// assert_eq!(type_of(c), "f64");
/// ```
pub fn type_of<T>(_: T) -> &'static str {
    let full_name = type_name::<T>();
    full_name.split("::").last().unwrap()
}

/// Check if a number is zero <br>
/// Uses `f64::EPSILON` as the threshold
pub fn is_zero(num: f64) -> bool {
    num.abs() < EPSILON
}

/// Use to result from goniometric or other non-precise functions <br>
/// Rounds only if the fractional part is less than `f64::EPSILON`
pub fn round(num: f64) -> f64 {
    let small_part = (num.fract() * 1e15).fract() * 1e-15;
    num - small_part
}