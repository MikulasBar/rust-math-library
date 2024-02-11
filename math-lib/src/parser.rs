/// Used to parse `String` to structure like function <br>
/// Use the `functions` module from this library
pub mod parser {

    use std::{collections::HashMap, fmt::Error, ops::Index, process::Child};

    use regex::*;
    use once_cell::sync::Lazy;

    use crate::{functions::functions::*, lazy_regex};
    
    


    lazy_regex!(
        name: NAME_RE,
        def: r"
            \s*( \w+ )
        "
    );

    lazy_regex!(
        name: VARS_RE,
        def: r"
            \s*( \w+ )\(( .* )\) = 
        "
    );


    pub struct FuncStructure {
        pub name: String,
        pub variables: Vec<String>,
        definition: BoxedFunc,
    }

    impl FuncStructure { 
        fn parse_name(string_def: &str) -> String {
            NAME_RE
                .captures(string_def)
                .unwrap()[0]
                .to_string()
        }

        fn parse_variables(string_def: &str) -> Vec<String> {
            let mut vars = VARS_RE
                .captures(string_def)
                .unwrap()[0]
                .to_string();

            vars = vars.replace(" ", "");
            vars.split(",")
                .map(|x| x.to_string())
                .collect()
            
        }

        pub fn parse_definition(string_def: String) -> Vec<String> {
            let mut tokens: Vec<String> = Vec::new();
            let mut depth = 0;
            let mut token = "".to_string();

            let string_def = string_def.replace(" ", "");

            for c in string_def.chars() {
                match c {
                    op @ ('+' | '-' | '*' | '/' | '^') if depth == 0 => {
                        tokens.push(token.clone());
                        tokens.push(op.to_string());
                        token = "".to_string();
                    },
                    n => {
                        depth += match n {
                            '(' => 1,
                            ')' => -1,
                            _ => 0, 
                        };
                        token.push(n);
                    },
                }
            }
            tokens.push(token.clone());

            // if tokens.len() == 1 {

            // } else {

            // }

            return tokens;
        }

        // pub fn parse(string_def: &str) -> Self {
        //     let name = FuncStructure::parse_name(string_def);
        //     let variables = FuncStructure::parse_variables(string_def);
        //     let definition = FuncStructure::parse_definition((*string_def).to_string());
        //     Self {
        //         name,
        //         variables,
        //         definition
        //     }
        // }

        fn apply(&self, args: &FuncArgs) -> f32 {
            self.definition.apply(args)
        }

        // fn derivative(&self) -> Self {
        //     Self {
        //         definition: self.definition.
        //         ..*self
        //     }
        // }
    }



}


#[cfg(test)]
mod tests {

    use super::parser::*;

    

    #[test]
    fn test_simple() {
        let string = "5+ (  456*(78 +2 ))  + func( n + x)^ ( j)".to_string();

        let vec = FuncStructure::parse_definition(string);
        let expected = vec!["5", "+", "(456*(78+2))", "+", "func(n+x)", "^", "(j)"];
        let expected: Vec<String> = expected.into_iter()
            .map(|x| x.to_string())
            .collect();

        assert_eq!(vec, expected);
    }
}