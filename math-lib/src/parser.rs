/// Used to parse `String` to structure like function <br>
/// Use the `functions` module from this library
pub mod parser {

    use std::{collections::HashMap, ops::Index};

    use crate::functions::*;
    use crate::utilities::parser_utils::*;
    


    pub struct FuncStructure {
        pub name: String,
        pub variables: Vec<String>,
        definition: BoxedFunc,
    }

    impl FuncStructure { 
        pub fn parse_definition(string_def: &str) -> Vec<String> {
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

            if tokens.contains(&"+".to_string()) {
                tokens.retain(|x| x != &"".to_string());


            }

            return tokens;
        }

        fn apply(&self, args: &FuncArgs) -> f32 {
            self.definition.apply(args)
        }
    }



}


#[cfg(test)]
mod tests {

    use super::parser::*;

    

    #[test]
    fn test_simple() {
        let string = "5+ (  456*(78 +2 ))  + func( n + x)^ ( j)";

        let vec = FuncStructure::parse_definition(string);
        let expected = vec!["5", "+", "(456*(78+2))", "+", "func(n+x)", "^", "(j)"];
        let expected: Vec<String> = expected.into_iter()
            .map(|x| x.to_string())
            .collect();

        assert_eq!(vec, expected);
    }
}