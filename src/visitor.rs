// --> see visitor on https://github.com/rrevenantt/antlr4rust/blob/master/tests/visitors_tests.rs
//#![cfg_attr(test, warn(unused_imports))]

use antlr_rust::tree::{ParseTree, ParseTreeVisitorCompat};

use crate::{
    antlr_parser::{
        mathparser::*,
        mathvisitor::*
    },
    child::*,
    parsing_result::{ParsingError, ParsingResult},
    functions::{AddFn, MulFn, SubFn, DivFn, ExpFn, LogFn},
    function::*,
};


pub struct Visitor {
    /// this value isn't really used, but it's required by the trait
    temp: ParsingResult
}

impl Visitor {
    pub fn new() -> Self {
        Self {
            temp: ParsingResult::default(),
        }  
    }
}


impl ParseTreeVisitorCompat<'_> for Visitor {
    type Node = mathParserContextType;
    type Return = ParsingResult;

    fn temp_result(&mut self) -> &mut Self::Return {
        &mut self.temp
    }

    fn aggregate_results(&self, _: Self::Return, _: Self::Return) -> Self::Return {
        unreachable!()
    }
}

impl mathVisitorCompat<'_> for Visitor {
    fn visit_root(&mut self, ctx: &RootContext<'_>) -> Self::Return {
        self.visit(&*ctx.expr().unwrap())
    }
    
    fn visit_number(&mut self, ctx: &NumberContext<'_>) -> Self::Return {
        ctx.NUMBER()
            .unwrap()
            .get_text()
            .parse::<f64>()
            .unwrap()
            .to_child()
            .into()
    }

    // we don't check if its constant like pi - it will be checked in the function by `Context`
    fn visit_var(&mut self, ctx: &VarContext<'_>) -> Self::Return {
        ctx.ID()
            .unwrap()
            .get_text()
            .to_child()
            .into()
    }

    fn visit_parens(&mut self, ctx: &ParensContext<'_>) -> Self::Return {
        self.visit(&*ctx.expr().unwrap())
    }

    // returns 'Add' or 'Sub' Function
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
        
        match ctx.SUB() {
            Some(_) => SubFn::new(left, right),
            _ =>  AddFn::new(left, right)
        }
        .to_child()
        .into()
    }

    // returns 'Mul' or 'Div' Function
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

        match ctx.DIV() {
            Some(_) => DivFn::new(left, right),
            _ =>  MulFn::new(left, right)
        }
        .to_child()
        .into()
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

        ExpFn::new(base, power)
            .to_child()
            .into()
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

        LogFn::new(base, arg)
            .to_child()
            .into()
    }

    // Visit function with arguments (sin, tan, your custom function)
    fn visit_function(&mut self, ctx: &FunctionContext<'_>) -> Self::Return {
        let name = ctx.ID().unwrap().get_text();
        let mut args: Vec<Child> = vec![];
        
        for expr in ctx.expr_all() {
            let visited = self.visit(&*expr);
            // checks if the result is an error - returns the first error
            if visited.is_err() {
                return visited
            }
            args.push(visited.unwrap())
        }

        todo!();
        // if let Some(result) = self.rules.get_function(&name, args) {
        //     return ParsingResult::Ok(result)
        // }

        return ParsingResult::Err(
            ParsingError::UnrecognizedFunctionName
        )
    }
}

