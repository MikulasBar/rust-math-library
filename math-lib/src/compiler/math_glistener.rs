#![allow(nonstandard_style)]
// Generated from C:\Users\barta\code\rust\horizon-sphere\math-lib\src\compiler\math_g.g4 by ANTLR 4.8
use antlr_rust::tree::ParseTreeListener;
use super::math_gparser::*;

pub trait math_gListener<'input> : ParseTreeListener<'input,math_gParserContextType>{
/**
 * Enter a parse tree produced by {@link math_gParser#expr}.
 * @param ctx the parse tree
 */
fn enter_expr(&mut self, _ctx: &ExprContext<'input>) { }
/**
 * Exit a parse tree produced by {@link math_gParser#expr}.
 * @param ctx the parse tree
 */
fn exit_expr(&mut self, _ctx: &ExprContext<'input>) { }
/**
 * Enter a parse tree produced by {@link math_gParser#term}.
 * @param ctx the parse tree
 */
fn enter_term(&mut self, _ctx: &TermContext<'input>) { }
/**
 * Exit a parse tree produced by {@link math_gParser#term}.
 * @param ctx the parse tree
 */
fn exit_term(&mut self, _ctx: &TermContext<'input>) { }
/**
 * Enter a parse tree produced by {@link math_gParser#factor}.
 * @param ctx the parse tree
 */
fn enter_factor(&mut self, _ctx: &FactorContext<'input>) { }
/**
 * Exit a parse tree produced by {@link math_gParser#factor}.
 * @param ctx the parse tree
 */
fn exit_factor(&mut self, _ctx: &FactorContext<'input>) { }

}

antlr_rust::coerce_from!{ 'input : math_gListener<'input> }


