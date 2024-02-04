/// Used to parse `String` to structure like function <br>
/// Use the `functions` module from this library
pub mod parser {

    use std::collections::HashMap;

    use regex::*;
    use once_cell::sync::Lazy;

    use crate::functions::functions::*;

    static FUNC_REGEX: Lazy<Regex> = Lazy::new(||
        Regex::new(r"
            \s*
            ( ?P<name>\w+ )
            \(( ?P<parameter>\w+ )\)
            \s*
            =
            \s*
            ( ?P<variable>\w+ )
            \^
            ( ?P<power>[-+]?[0-9]*\.?[0-9]+([eE][-+]?[0-9]+)? )
            \s*
        ").unwrap()
    );

    pub fn will_parse_to_function(haystack: &str) -> bool {
        let Some(caps) = FUNC_REGEX.captures(&haystack) else {
            println!("Regex was not compiled");
            return false;
        };
        
        let Ok(power) = caps["power"].parse::<f32>() else {
            println!("Power cannot be parsed");
            return false;
        };

        let parameter: String = caps["parameter"].to_string();
        let variable: String = caps["variable"].to_string();

        if parameter != variable {
            println!("Parameter and variable is not the same");
            return false;
        }

        if power <= 0.0 {
            println!("Power is less or equal to 0");
            return false;
        }

        let func = PowFunc::new(power, parameter.clone());

        let mut args1: FuncArgs = HashMap::new();
        //let mut arg2: FuncArgs = HashMap::new();

        args1.insert(parameter, 2.0);
        args1.insert("not_a_parameter".to_string(), 1.2);
        
        if func.apply(&args1) != (2.0 as f32).powf(power) {
            println!("The result is not correct");
            return false;
        }

        return true;

    }


}


#[cfg(test)]
mod tests {

    use super::parser::*;

    

    #[test]
    fn test_simple_parser_correct() {
        will_parse_to_function(&"  f(x) =  x^2.45");
        will_parse_to_function(&"fun_ction(x) =  x^2");
        will_parse_to_function(&" f(parameter) =    parameter");
    }

    #[test]
    fn test_simple_parser_incorrect() {
        assert!(
            will_parse_to_function(&" not a function(x) =  x^2")
            will_parse_to_function(&"  f(not a parameter) =  x^2");
            will_parse_to_function(&"  f (x) =  x-2");
            will_parse_to_function(&"  f(x) =  y^2");
            will_parse_to_function(&"  f(y)  y^2");
        );
    }
}