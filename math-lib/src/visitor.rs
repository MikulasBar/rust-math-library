// --> see visitor on https://github.com/rrevenantt/antlr4rust/blob/master/tests/visitors_tests.rs
#![cfg_attr(test, warn(unused_imports))]

use antlr_rust::{
    common_token_stream::CommonTokenStream,
    tree::{ParseTree, ParseTreeVisitorCompat},
    InputStream
};

use crate::{
    antlr_parser::{
        mathlexer::*,
        mathparser::*,
        mathvisitor::*
    },
    functions::*,
    utilities::*,
    function_tree::{
        ParsingResult,
        ParsingError,
        ParsingRules,
    }
};



pub struct MathVisitor{
    rules: Box<dyn ParsingRules>,
    temp: ParsingResult
}

impl MathVisitor {
    pub fn new<T>(rules: T) -> Self
    where
        T: ParsingRules + 'static
    {
        Self{
            rules: Box::new(rules),
            temp: ParsingResult::default(),
        }  
    }
}


impl ParseTreeVisitorCompat<'_> for MathVisitor {
    type Node = mathParserContextType;
    type Return = ParsingResult;

    fn temp_result(&mut self) -> &mut Self::Return {
        &mut self.temp
    }

    fn aggregate_results(&self, aggregate: Self::Return, next: Self::Return) -> Self::Return {
        panic!("This should not be reacheble")
    }
}

impl mathVisitorCompat<'_> for MathVisitor {
    fn visit_root(&mut self, ctx: &RootContext<'_>) -> Self::Return {
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

        if base.is_err() {
            return base
        }

        if arg.is_err() {
            return arg
        }

        let base = base.unwrap();
        let arg = arg.unwrap();

        ParsingResult::Ok(
            LogFn::new(base, arg).to_child_fn()
        )
    }

    fn visit_function(&mut self, ctx: &FunctionContext<'_>) -> Self::Return {
        let name = ctx.ID().unwrap().get_text();
        let result_args: Vec<ParsingResult> = ctx.expr_all()
            .into_iter()
            .map(|x| self.visit(&*x))
            .collect();

        let mut args: Vec<ChildFn> = vec![];
        for arg in result_args {
            if arg.is_err() {
                return arg
            }
            args.push(arg.unwrap())
        }

        if let Some(result) = self.rules.get_function(&name, args) {
            return ParsingResult::Ok(result)
        }

        return ParsingResult::Err(
            ParsingError::UnrecognizedFunctionNameError
        )
    }
}

