#![allow(nonstandard_style)]
// Generated from C:\Users\barta\code\rust\horizon-sphere\math-lib\src\antlr_parser\math.g4 by ANTLR 4.8
use antlr_rust::tree::{ParseTreeVisitor,ParseTreeVisitorCompat};
use super::mathparser::*;

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link mathParser}.
 */
pub trait mathVisitor<'input>: ParseTreeVisitor<'input,mathParserContextType>{
	/**
	 * Visit a parse tree produced by {@link mathParser#root}.
	 * @param ctx the parse tree
	 */
	fn visit_root(&mut self, ctx: &RootContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code add}
	 * labeled alternative in {@link mathParser#expr}.
	 * @param ctx the parse tree
	 */
	fn visit_add(&mut self, ctx: &AddContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code number}
	 * labeled alternative in {@link mathParser#expr}.
	 * @param ctx the parse tree
	 */
	fn visit_number(&mut self, ctx: &NumberContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code parens}
	 * labeled alternative in {@link mathParser#expr}.
	 * @param ctx the parse tree
	 */
	fn visit_parens(&mut self, ctx: &ParensContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code log}
	 * labeled alternative in {@link mathParser#expr}.
	 * @param ctx the parse tree
	 */
	fn visit_log(&mut self, ctx: &LogContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code var}
	 * labeled alternative in {@link mathParser#expr}.
	 * @param ctx the parse tree
	 */
	fn visit_var(&mut self, ctx: &VarContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code function}
	 * labeled alternative in {@link mathParser#expr}.
	 * @param ctx the parse tree
	 */
	fn visit_function(&mut self, ctx: &FunctionContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code power}
	 * labeled alternative in {@link mathParser#expr}.
	 * @param ctx the parse tree
	 */
	fn visit_power(&mut self, ctx: &PowerContext<'input>) { self.visit_children(ctx) }

	/**
	 * Visit a parse tree produced by the {@code multiply}
	 * labeled alternative in {@link mathParser#expr}.
	 * @param ctx the parse tree
	 */
	fn visit_multiply(&mut self, ctx: &MultiplyContext<'input>) { self.visit_children(ctx) }

}

pub trait mathVisitorCompat<'input>:ParseTreeVisitorCompat<'input, Node= mathParserContextType>{
	/**
	 * Visit a parse tree produced by {@link mathParser#root}.
	 * @param ctx the parse tree
	 */
		fn visit_root(&mut self, ctx: &RootContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code add}
	 * labeled alternative in {@link mathParser#expr}.
	 * @param ctx the parse tree
	 */
		fn visit_add(&mut self, ctx: &AddContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code number}
	 * labeled alternative in {@link mathParser#expr}.
	 * @param ctx the parse tree
	 */
		fn visit_number(&mut self, ctx: &NumberContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code parens}
	 * labeled alternative in {@link mathParser#expr}.
	 * @param ctx the parse tree
	 */
		fn visit_parens(&mut self, ctx: &ParensContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code log}
	 * labeled alternative in {@link mathParser#expr}.
	 * @param ctx the parse tree
	 */
		fn visit_log(&mut self, ctx: &LogContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code var}
	 * labeled alternative in {@link mathParser#expr}.
	 * @param ctx the parse tree
	 */
		fn visit_var(&mut self, ctx: &VarContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code function}
	 * labeled alternative in {@link mathParser#expr}.
	 * @param ctx the parse tree
	 */
		fn visit_function(&mut self, ctx: &FunctionContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code power}
	 * labeled alternative in {@link mathParser#expr}.
	 * @param ctx the parse tree
	 */
		fn visit_power(&mut self, ctx: &PowerContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

	/**
	 * Visit a parse tree produced by the {@code multiply}
	 * labeled alternative in {@link mathParser#expr}.
	 * @param ctx the parse tree
	 */
		fn visit_multiply(&mut self, ctx: &MultiplyContext<'input>) -> Self::Return {
			self.visit_children(ctx)
		}

}

impl<'input,T> mathVisitor<'input> for T
where
	T: mathVisitorCompat<'input>
{
	fn visit_root(&mut self, ctx: &RootContext<'input>){
		let result = <Self as mathVisitorCompat>::visit_root(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_add(&mut self, ctx: &AddContext<'input>){
		let result = <Self as mathVisitorCompat>::visit_add(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_number(&mut self, ctx: &NumberContext<'input>){
		let result = <Self as mathVisitorCompat>::visit_number(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_parens(&mut self, ctx: &ParensContext<'input>){
		let result = <Self as mathVisitorCompat>::visit_parens(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_log(&mut self, ctx: &LogContext<'input>){
		let result = <Self as mathVisitorCompat>::visit_log(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_var(&mut self, ctx: &VarContext<'input>){
		let result = <Self as mathVisitorCompat>::visit_var(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_function(&mut self, ctx: &FunctionContext<'input>){
		let result = <Self as mathVisitorCompat>::visit_function(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_power(&mut self, ctx: &PowerContext<'input>){
		let result = <Self as mathVisitorCompat>::visit_power(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

	fn visit_multiply(&mut self, ctx: &MultiplyContext<'input>){
		let result = <Self as mathVisitorCompat>::visit_multiply(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
	}

}