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
        pub fn parse_definition(string: &str) -> Self {
            let mut tokens = split_surface(string, '+');

            if tokens.len() == 1 {
                tokens = split_surface(string, '*');
                
            } else {

            }

            return FuncStructure::default();
        }

        fn apply(&self, args: &FuncArgs) -> f32 {
            self.definition.apply(args)
        }
    }

    impl Default for FuncStructure {
        fn default() -> Self {
            Self {
                name: String::new(),
                variables: Vec::new(),
                definition: Box::new(ConstFunc::new(0.0))
            }
        }
    }

}




#[cfg(test)]
mod tests {

    use super::parser::*;

    

    #[test]
    fn test_simple() {

    }
}