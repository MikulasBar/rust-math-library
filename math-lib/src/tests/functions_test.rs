#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use crate::{fn_args, functions::*};
    use crate::utilities::ToChildFn;
    use std::f32::{
        consts::{FRAC_PI_2, FRAC_PI_4},
        EPSILON,
    };


    #[test]
    fn test_AddFn() {
        let fx = CoefFn::new(2.0, "x").to_child_fn();
        let fy = CoefFn::new(3.0, "y").to_child_fn();

        let add_fn = AddFn::new(vec![fx, fy]);

        let args = fn_args!{
            "x" => 4.0,
            "y" => 5.0,
        };

        assert_eq!(add_fn.apply(&args), Ok(23.0));
    }

    #[test]
    fn test_MulFn() {
        let fx = CoefFn::new(2.0, "x").to_child_fn();
        let fy = CoefFn::new(3.0, "y").to_child_fn();

        let mul_fn = MulFn::new(vec![fx, fy]);

        let args = fn_args!{
            "x" => 4.0,
            "y" => 5.0,
        };

        assert_eq!(mul_fn.apply(&args), Ok(120.0));
    }

    #[test]
    fn test_DivFn() {
        let div_fn = DivFn::new("x", "y");

        let args = fn_args!{
            "x" => 48.0,
            "y" => 4.0,
        };

        assert_eq!(div_fn.apply(&args), Ok(12.0));
    }

    #[test]
    fn test_CoefFn() {
        let coef_fn = CoefFn::new(5.0, "x");

        let args = fn_args!{
            "x" => 6.0,
        };

        assert_eq!(coef_fn.apply(&args), Ok(30.0));
    }

    #[test]
    fn test_ExpFn() {
        let exp_fn = ExpFn::new("x", "y");

        let args = fn_args!{
            "x" => 5.0,
            "y" => 3.0,
        };

        assert_eq!(exp_fn.apply(&args), Ok(125.0));
    }

    #[test]
    fn test_LogFn() {
        let log_fn = LogFn::new("x", "y");

        let args = fn_args!{
            "x" => 2.0,
            "y" => 16.0,
        };

        assert_eq!(log_fn.apply(&args), Ok(4.0));
    }

    #[test]
    fn test_SinFn() {
        let sin_fn = SinFn::new("x");

        let args = fn_args!{
            "x" => FRAC_PI_2,
        };
        let value = sin_fn.apply(&args).unwrap() - 1f32;

        assert!(value.abs() <= EPSILON);
    }

    #[test]
    fn test_CosFn() {
        let cos_fn = CosFn::new("x");

        let args = fn_args!{
            "x" => FRAC_PI_2,
        };
        let value = cos_fn.apply(&args).unwrap();

        assert!(value <= EPSILON);
    }

    #[test]
    fn test_TanFn() {
        let tan_fn = TanFn::new("x");

        let args = fn_args!{
            "x" => FRAC_PI_4,
        };
        let value = tan_fn.apply(&args).unwrap() - 1f32;

        assert!(value.abs() <= EPSILON);
    }
}