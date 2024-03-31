// --> see visitor on https://github.com/rrevenantt/antlr4rust/blob/master/tests/visitors_tests.rs
#![cfg_attr(test, warn(unused_imports))]

use std::f64::consts::{E, PI};

use antlr_rust::{
    common_token_stream::CommonTokenStream,
    tree::{ParseTree, ParseTreeVisitorCompat},
    InputStream
};

use crate::antlr_parser::{
    mathlexer::*,
    mathparser::*,
    mathvisitor::*
};
use crate::{
    function_tree::{
        ParsingResult,
        ParsingRules,
    },
    functions::*,
    utilities::*,
    fn_args,
};


// fn string_to_fn(name: &str, mut args: Vec<ChildFn>) -> Option<ChildFn> {
//     Some (
//         match name {
//             "sin" => args.pop().map(SinFn::new).to_child_fn(),
//             "cos" => args.pop().map(CosFn::new).to_child_fn(),
//             "tan" => args.pop().map(TanFn::new).to_child_fn(),
//             "log" => {
//                 let arg = args.pop()?;
//                 LogFn::new(10.0, arg).to_child_fn()
//             },
//             "ln" => {
//                 let arg = args.pop()?;
//                 LogFn::new(E, arg).to_child_fn()
//             },
//             _ => return None
//         }
//     )
// }


pub struct MathVisitor{
    value: ParsingResult,
    rules: Box<dyn ParsingRules>
}

impl MathVisitor {
    pub fn new(rules: Box<dyn ParsingRules>) -> Self {
        Self{
            value: ParsingResult::default(),
            rules,
        }  
    }
}


impl ParseTreeVisitorCompat<'_> for MathVisitor {
    type Node = mathParserContextType;
    type Return = ParsingResult;

    fn temp_result(&mut self) -> &mut Self::Return {
        &mut self.value
    }

    fn aggregate_results(&self, _: Self::Return, _: Self::Return) -> Self::Return {
        panic!("AGGREGATE RESULTS USED")
    }
}

impl mathVisitorCompat<'_> for MathVisitor {
    fn visit_prog(&mut self, ctx: &ProgContext<'_>) -> Self::Return {
        self.visit(&*ctx.expr().unwrap())
    }

    fn visit_number(&mut self, ctx: &NumberContext<'_>) -> Self::Return {
        ParsingResult::Ok(
            ChildFn::Const(
                ctx.NUMBER()
                    .unwrap()
                    .get_text()
                    .parse()
                    .unwrap()
            )
        )
    }

    fn visit_var(&mut self, ctx: &VarContext<'_>) -> Self::Return {
        let name = ctx.ID()
            .unwrap()
            .get_text();

        let value = self.rules.get_constant(&name);

        ParsingResult::Ok(
            match value {
                Some(v) => v.to_child_fn(),
                _ => name.to_child_fn()
            }
        )
    }

    fn visit_parens(&mut self, ctx: &ParensContext<'_>) -> Self::Return {
        self.visit(&*ctx.expr().unwrap())
    }

    fn visit_add(&mut self, ctx: &AddContext<'_>) -> Self::Return {
        let left = self.visit(&*ctx.expr(0).unwrap());
        let right = self.visit(&*ctx.expr(1).unwrap());

        if left.is_err() {
            return left
        }

        if right.is_err() {
            return right
        }

        let left = left.unwrap();
        let right = right.unwrap();

        ParsingResult::Ok(
            match ctx.SUB() {
                Some(_) => SubFn::new(left, right).to_child_fn(),
                _ =>  AddFn::new(left, right).to_child_fn()
            }
        )
    }

    fn visit_multiply(&mut self, ctx: &MultiplyContext<'_>) -> Self::Return {
        let left = self.visit(&*ctx.expr(0).unwrap());
        let right = self.visit(&*ctx.expr(1).unwrap());

        if left.is_err() {
            return left
        }

        if right.is_err() {
            return right
        }

        let left = left.unwrap();
        let right = right.unwrap();

        ParsingResult::Ok(
            match ctx.DIV() {
                Some(_) => DivFn::new(left, right).to_child_fn(),
                _ =>  MulFn::new(left, right).to_child_fn()
            }
        )
    }

    fn visit_power(&mut self, ctx: &PowerContext<'_>) -> Self::Return {
        let base = self.visit(&*ctx.expr(0).unwrap());
        let power = self.visit(&*ctx.expr(1).unwrap());

        if base.is_err() {
            return base
        }

        if power.is_err() {
            return power
        }

        let base = base.unwrap();
        let power = power.unwrap();

        ParsingResult::Ok(
            ExpFn::new(base, power).to_child_fn()
        )
    }

    fn visit_log(&mut self, ctx: &LogContext<'_>) -> Self::Return {
        let base = self.visit(&*ctx.expr(0).unwrap());
        let arg = self.visit(&*ctx.expr(1).unwrap());

        LogFn::new(base, arg).to_child_fn()
    }

    fn visit_function(&mut self, ctx: &FunctionContext<'_>) -> Self::Return {
        let name = ctx.ID().unwrap().get_text();
        let args: Vec<_> = ctx.expr_all()
            .into_iter()
            .map(|x| self.visit(&*x))
            .collect();

        if let Some(result) = string_to_fn(&name, args) {
            return result
        }

        panic!("Unrecognized function name: {}", name);
    }
}




#[test]
fn test_parser() {
    let lexer = mathLexer::new(InputStream::new("2^(3 - 1) * (1 - cos(pi/x)) + log_5(y + ln(e))".into()));

    let token_source = CommonTokenStream::new(lexer);
    let mut parser = mathParser::new(token_source);

    let root = parser.prog().unwrap();

    let func = MathVisitor::new().visit(&*root);

    let result = func.apply(&fn_args!(
        "x" => 2,
        "y" => 4,
    )).unwrap();

    println!("{}", root.to_string_tree(&*parser));

    assert_eq!(result, 5.0);
}


