#![allow(dead_code)]

use antlr_rust::{
    common_token_stream::CommonTokenStream,
    InputStream,
    tree::ParseTreeVisitorCompat,
};

#[rustfmt::skip]
use crate::{
    antlr::{
        mathlexer::*,
        mathparser::*,
    },
    visitor::*,
    child::*,
    parsing_result::*,
};



// facade for parsing the function from text format
pub fn parse(input: &str) -> Result<Child, ParsingError> {
    let lexer = mathLexer::new(InputStream::new(input.into()));
    let token_source = CommonTokenStream::new(lexer);
    let mut parser = mathParser::new(token_source);

    let root = parser.root();
    if root.is_err() {
        return Err(ParsingError::Antlr)
    }

    let root = root.unwrap();
    let mut visitor = Visitor::new();

    let result = visitor.visit(&*root);

    result.into()
}


#[cfg(test)]
mod test {
    use maplit::hashmap;
    use std::f64::consts::PI;
    
    use crate::_context::{self, Context};

    use super::parse;

    // 2^(3 - 1) * (1 - cos(pi/x)) + log_5(y + ln(e))
    // not yet implemented
    //#[should_panic]
    #[test]
    fn parser() {
        std::env::set_var("RUST_BACKTRACE", "0");

        let func = parse("2^(3 - 1) * (1 - cos(pi/x)) + log_5(y + ln(e))").unwrap();
        // let dfunc = func.derivative("x");

        let elem_ctx = _context::elementary();
        let ctx: Context = hashmap!{
            "x" => 2.0,
            "y" => 4.0,
        }.into();

        let full_ctx = ctx.merge(&elem_ctx);

        assert_eq!(func.eval(&full_ctx), Ok(5.0));
        // assert_eq!(dfunc.eval(&full_ctx), Ok(-PI));
    }
}
