#[allow(non_snake_case)]
#[cfg(test)]
mod tests {
    use crate::functions::*;
    use crate::utilities::{
        ToChildFn, ToFnArgs
    };


    #[test]
    fn test_AddFn() {
        let fx = CoefFn::new(2.0, "x").to_child();
        let fy = CoefFn::new(3.0, "y").to_child();

        let add_fn = AddFn::new(vec![fx, fy]);

        let args: FnArgs = vec![
            ("x", 4.0),
            ("y", 5.0),
        ].to_fn_args();

        assert_eq!(add_fn.apply(&args), Ok(23.0));
    }

    #[test]
    fn test_MulFn() {
        let fx = CoefFn::new(2.0, "x").to_child();
        let fy = CoefFn::new(3.0, "y").to_child();

        let mul_fn = MulFn::new(vec![fx, fy]);

        let args: FnArgs = vec![
            ("x", 4.0),
            ("y", 5.0),
        ].to_fn_args();

        assert_eq!(mul_fn.apply(&args), Ok(120.0));
    }

    #[test]
    fn test_DivFn() {
        let div_fn = DivFn::new("x", "y");

        let args: FnArgs = vec![
            ("x", 48.0),
            ("y", 4.0),
        ].to_fn_args();

        assert_eq!(div_fn.apply(&args), Ok(12.0));
    }

    #[test]
    fn test_CoefFn() {
        let coef_fn = CoefFn::new(5.0, "x");

        let args: FnArgs = vec![
            ("x", 6.0),
        ].to_fn_args();

        assert_eq!(coef_fn.apply(&args), Ok(30.0));
    }

    #[test]
    fn test_ExpFn() {
        let exp_fn = ExpFn::new("x", "y");

        let args: FnArgs = vec![
            ("x", 5.0),
            ("y", 3.0),
        ].to_fn_args();

        assert_eq!(exp_fn.apply(&args), Ok(125.0));
    }

    #[test]
    fn test_LogFn() {
        let log_fn = LogFn::new("x", "y");

        let args: FnArgs = vec![
            ("x", 2.0),
            ("y", 16.0),
        ].to_fn_args();

        assert_eq!(log_fn.apply(&args), Ok(4.0));
    }
}